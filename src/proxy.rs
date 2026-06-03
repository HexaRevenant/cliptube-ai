use std::collections::HashSet;
use std::sync::RwLock;

use reqwest::Proxy;
use tracing::{info, warn};

#[derive(Debug, Clone, PartialEq)]
pub enum ProxyScheme {
    Http,
    Https,
    Socks5,
}

impl ProxyScheme {
    fn from_url(url: &str) -> Option<Self> {
        if url.starts_with("http://") {
            Some(ProxyScheme::Http)
        } else if url.starts_with("https://") {
            Some(ProxyScheme::Https)
        } else if url.starts_with("socks5://") || url.starts_with("socks5h://") {
            Some(ProxyScheme::Socks5)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub url: String,
    pub scheme: ProxyScheme,
}

pub struct ProxyManager {
    state: RwLock<ProxyState>,
}

struct ProxyState {
    enabled: bool,
    proxies: Vec<ProxyConfig>,
    current_index: usize,
    dead: HashSet<usize>,
    version: u64,
}

impl ProxyManager {
    /// Crea un ProxyManager leyendo las variables de entorno:
    /// `YOUTUBE_PROXY_LIST` (JSON array, prioridad), `HTTP_PROXY`, `HTTPS_PROXY`
    pub fn from_env() -> Self {
        let proxies = Self::parse_proxy_list();

        let enabled = !proxies.is_empty();
        if enabled {
            info!("ProxyManager: {} proxy(es) configurado(s)", proxies.len());
            for (i, p) in proxies.iter().enumerate() {
                info!("  Proxy #{}: {} ({:?})", i + 1, p.url, p.scheme);
            }
        } else {
            info!("ProxyManager: sin proxies configurados (deshabilitado)");
        }

        Self {
            state: RwLock::new(ProxyState {
                enabled,
                proxies,
                current_index: 0,
                dead: HashSet::new(),
                version: 0,
            }),
        }
    }

    fn parse_proxy_list() -> Vec<ProxyConfig> {
        // YOUTUBE_PROXY_LIST tiene prioridad: JSON array de URLs
        if let Ok(raw) = std::env::var("YOUTUBE_PROXY_LIST") {
            let raw = raw.trim().to_string();
            if !raw.is_empty() {
                if let Ok(urls) = serde_json::from_str::<Vec<String>>(&raw) {
                    let configs: Vec<ProxyConfig> = urls
                        .into_iter()
                        .filter_map(|url| {
                            let scheme = ProxyScheme::from_url(&url)?;
                            Some(ProxyConfig { url, scheme })
                        })
                        .collect();
                    if !configs.is_empty() {
                        return configs;
                    }
                    warn!("YOUTUBE_PROXY_LIST no contiene URLs válidas");
                } else {
                    warn!("YOUTUBE_PROXY_LIST no es un JSON array válido");
                }
            }
        }

        // Fallback: HTTP_PROXY / HTTPS_PROXY estándar
        let mut proxies = Vec::new();
        if let Ok(url) = std::env::var("HTTPS_PROXY") {
            let url = url.trim().to_string();
            if !url.is_empty() {
                if let Some(scheme) = ProxyScheme::from_url(&url) {
                    proxies.push(ProxyConfig { url, scheme });
                }
            }
        }
        if proxies.is_empty() {
            if let Ok(url) = std::env::var("HTTP_PROXY") {
                let url = url.trim().to_string();
                if !url.is_empty() {
                    if let Some(scheme) = ProxyScheme::from_url(&url) {
                        proxies.push(ProxyConfig { url, scheme });
                    }
                }
            }
        }
        proxies
    }

    /// ¿El proxy está habilitado Y hay proxies configurados?
    pub fn is_enabled(&self) -> bool {
        let state = self.state.read().expect("proxy lock poisoned");
        state.enabled && !state.proxies.is_empty()
    }

    pub fn set_enabled(&self, enabled: bool) {
        let mut state = self.state.write().expect("proxy lock poisoned");
        if state.enabled != enabled {
            state.enabled = enabled;
            state.version += 1;
            info!(
                "ProxyManager: {} (version={})",
                if enabled { "habilitado" } else { "deshabilitado" },
                state.version
            );
        }
    }

    /// Proxy activo actual, si está habilitado y hay proxies vivos
    pub fn active_proxy(&self) -> Option<(ProxyConfig, u64)> {
        let state = self.state.read().expect("proxy lock poisoned");
        if !state.enabled || state.proxies.is_empty() {
            return None;
        }

        // Buscar desde current_index hacia adelante, luego wrap
        let n = state.proxies.len();
        for offset in 0..n {
            let idx = (state.current_index + offset) % n;
            if !state.dead.contains(&idx) {
                return Some((state.proxies[idx].clone(), state.version));
            }
        }
        None // todos muertos
    }

    /// Marca el proxy actual como muerto y avanza al siguiente
    pub fn mark_current_dead(&self) {
        let mut state = self.state.write().expect("proxy lock poisoned");
        if state.proxies.is_empty() {
            return;
        }

        let dead_idx = state.current_index;
        state.dead.insert(dead_idx);
        state.current_index = (dead_idx + 1) % state.proxies.len();
        state.version += 1;

        let alive = state.proxies.len() - state.dead.len();
        warn!(
            "Proxy #{} ({}) marcado como muerto. Vivos: {}. Siguiente: #{} (version={})",
            dead_idx + 1,
            state.proxies[dead_idx].url,
            alive,
            state.current_index + 1,
            state.version,
        );

        if alive == 0 {
            warn!("ProxyManager: TODOS los proxies están muertos");
        }
    }

    /// Setea una URL de proxy desde la UI (reemplaza cualquier config de env vars)
    /// NO cambia el estado enabled — eso lo controla el checkbox
    pub fn set_proxy_url(&self, url: &str) {
        let mut state = self.state.write().expect("proxy lock poisoned");
        let url = url.trim();

        let proxy = if url.is_empty() {
            None
        } else {
            let scheme = ProxyScheme::from_url(url);
            if scheme.is_none() {
                warn!("ProxyManager: URL de proxy inválida: {url}");
                return;
            }
            Some(ProxyConfig {
                url: url.to_string(),
                scheme: scheme.unwrap(),
            })
        };

        let old_first = state.proxies.first().map(|p| p.url.clone());
        let new_first = proxy.as_ref().map(|p| p.url.clone());
        if old_first == new_first {
            return; // misma URL, no hacer nada
        }

        state.proxies = proxy.map(|p| vec![p]).unwrap_or_default();
        state.current_index = 0;
        state.dead.clear();
        // NO tocar state.enabled — eso lo maneja el checkbox
        state.version += 1;

        info!(
            "ProxyManager: URL actualizada desde UI ({} proxy, version={})",
            state.proxies.len(),
            state.version,
        );
    }

    pub fn version(&self) -> u64 {
        let state = self.state.read().expect("proxy lock poisoned");
        state.version
    }

    #[allow(dead_code)]
    pub fn proxy_count(&self) -> usize {
        let state = self.state.read().expect("proxy lock poisoned");
        state.proxies.len()
    }

    /// Construye el `reqwest::Proxy` correspondiente al proxy activo
    pub fn build_reqwest_proxy(&self) -> Option<reqwest::Proxy> {
        let proxy_info = self.active_proxy()?;
        let (config, _version) = proxy_info;

        let no_proxy = std::env::var("NO_PROXY").unwrap_or_default();

        match config.scheme {
            ProxyScheme::Http | ProxyScheme::Https => {
                let mut p = Proxy::http(&config.url).ok()?;
                if !no_proxy.is_empty() {
                    p = p.no_proxy(reqwest::NoProxy::from_string(&no_proxy));
                }
                Some(p)
            }
            ProxyScheme::Socks5 => {
                // SOCKS5 requiere feature "socks" en reqwest (agregar a Cargo.toml)
                warn!("Proxy SOCKS5 ({}) requiere feature 'socks' en reqwest. Agregá 'socks' a features en Cargo.toml", config.url);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scheme() {
        assert_eq!(ProxyScheme::from_url("http://proxy:8080"), Some(ProxyScheme::Http));
        assert_eq!(ProxyScheme::from_url("https://proxy:8443"), Some(ProxyScheme::Https));
        assert_eq!(ProxyScheme::from_url("socks5://127.0.0.1:9050"), Some(ProxyScheme::Socks5));
        assert_eq!(ProxyScheme::from_url("socks5h://proxy:1080"), Some(ProxyScheme::Socks5));
        assert_eq!(ProxyScheme::from_url(""), None);
    }

    #[test]
    fn test_proxy_manager_no_env_disabled() {
        let pm = ProxyManager::from_env();
        // Sin vars de entorno, no debería haber proxies
        assert!(!pm.is_enabled());
        assert_eq!(pm.proxy_count(), 0);
        assert!(pm.active_proxy().is_none());
    }

    #[test]
    fn test_proxy_manager_enable_toggle() {
        let pm = ProxyManager::from_env();
        let v0 = pm.version();
        pm.set_enabled(true);
        // Sin proxies configurados, is_enabled es false incluso con toggle
        assert!(!pm.is_enabled());
        // La versión incrementa porque el toggle cambió (aunque no tenga efecto sin proxies)
        assert!(pm.version() > v0);
        // Toggle de vuelta: sin proxies, version vuelve a incrementar
        pm.set_enabled(false);
        assert!(!pm.is_enabled());
        assert!(pm.version() > v0 + 0);
    }

    #[test]
    fn test_mark_dead_on_empty() {
        let pm = ProxyManager::from_env();
        pm.mark_current_dead(); // No debería panic
        assert!(pm.active_proxy().is_none());
    }
}

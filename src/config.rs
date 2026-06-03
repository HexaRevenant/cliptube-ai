use std::env;

pub fn validate_startup_config() -> Result<(), String> {
    if let Ok(url) = env::var("OLLAMA_CHAT_URL")
        && !url.trim().is_empty()
    {
        validate_http_url("OLLAMA_CHAT_URL", &url)?;
    } else {
        let host = env::var("OLLAMA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        if host.trim().is_empty() {
            return Err("OLLAMA_HOST no puede estar vacío".to_string());
        }

        let port_raw = env::var("OLLAMA_PORT").unwrap_or_else(|_| "11434".to_string());
        let port = port_raw
            .trim()
            .parse::<u16>()
            .map_err(|_| "OLLAMA_PORT debe ser un número válido entre 1 y 65535".to_string())?;
        if port == 0 {
            return Err("OLLAMA_PORT debe ser mayor a 0".to_string());
        }
    }

    parse_positive_usize_env("MAX_TRANSCRIPT_CHARS_FOR_AI")?;
    parse_positive_usize_env("OLLAMA_CHUNK_SIZE")?;
    parse_positive_usize_env("OLLAMA_MAX_CHUNKS")?;

    validate_proxy_config()?;

    Ok(())
}

fn validate_proxy_config() -> Result<(), String> {
    if let Ok(raw) = env::var("YOUTUBE_PROXY_LIST") {
        let raw = raw.trim().to_string();
        if !raw.is_empty() {
            let urls: Vec<String> = serde_json::from_str(&raw)
                .map_err(|e| format!("YOUTUBE_PROXY_LIST no es un JSON array válido: {e}"))?;
            for (i, url) in urls.iter().enumerate() {
                validate_proxy_url(&format!("YOUTUBE_PROXY_LIST[{i}]"), url)?;
            }
        }
    }

    if let Ok(url) = env::var("HTTP_PROXY") {
        let url = url.trim().to_string();
        if !url.is_empty() {
            validate_proxy_url("HTTP_PROXY", &url)?;
        }
    }

    if let Ok(url) = env::var("HTTPS_PROXY") {
        let url = url.trim().to_string();
        if !url.is_empty() {
            validate_proxy_url("HTTPS_PROXY", &url)?;
        }
    }

    Ok(())
}

fn validate_proxy_url(var_name: &str, value: &str) -> Result<(), String> {
    let valid_schemes = ["http://", "https://", "socks5://", "socks5h://"];
    let has_valid_scheme = valid_schemes.iter().any(|s| value.starts_with(s));
    if !has_valid_scheme {
        return Err(format!(
            "{var_name} debe empezar con http://, https://, socks5:// o socks5h:// (actual: {value})"
        ));
    }

    let parsed = url::Url::parse(value)
        .map_err(|_| format!("{var_name} no es una URL válida: {value}"))?;

    if parsed.host().is_none() {
        return Err(format!("{var_name} debe tener un host: {value}"));
    }

    if parsed.port().is_none() {
        return Err(format!("{var_name} debe tener un puerto: {value}"));
    }

    Ok(())
}

fn validate_http_url(var_name: &str, value: &str) -> Result<(), String> {
    let parsed = url::Url::parse(value.trim())
        .map_err(|_| format!("{var_name} no es una URL válida: {value}"))?;
    let scheme = parsed.scheme();
    if scheme != "http" && scheme != "https" {
        return Err(format!(
            "{var_name} debe usar http:// o https:// (actual: {scheme})"
        ));
    }
    Ok(())
}

fn parse_positive_usize_env(var_name: &str) -> Result<(), String> {
    let Ok(raw) = env::var(var_name) else {
        return Ok(());
    };
    let value = raw
        .trim()
        .parse::<usize>()
        .map_err(|_| format!("{var_name} debe ser un número entero positivo"))?;
    if value == 0 {
        return Err(format!("{var_name} debe ser mayor a 0"));
    }
    Ok(())
}

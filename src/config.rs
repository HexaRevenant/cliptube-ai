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

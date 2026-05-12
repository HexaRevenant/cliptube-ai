pub fn build_share_text(
    chat_text: &str,
    summary_label: &str,
    summary_text: &str,
    key_points_label: &str,
    key_points_text: &str,
    source_url: &str,
) -> String {
    format!(
        "{}\n\n\n{}\n{}\n\n\n{}\n{}\n\n\n{}",
        chat_text.trim(),
        summary_label,
        summary_text.trim(),
        key_points_label,
        key_points_text.trim(),
        source_url.trim()
    )
}

#[cfg(test)]
mod tests {
    use super::build_share_text;

    #[test]
    fn build_share_text_includes_sections_and_url() {
        let out = build_share_text(
            "Hola",
            "Resumen",
            "Contenido",
            "Puntos",
            "• uno",
            "https://youtube.com/watch?v=12345678901",
        );

        assert!(out.contains("Resumen"));
        assert!(out.contains("Puntos"));
        assert!(out.ends_with("https://youtube.com/watch?v=12345678901"));
    }
}

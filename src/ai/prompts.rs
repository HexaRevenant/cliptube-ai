use super::{OutputStyle, text::normalized_lang};

pub(crate) fn build_chunk_prompt(
    idx: usize,
    total: usize,
    chunk: &str,
    _style: OutputStyle,
    response_language: &str,
) -> String {
    format!(
        "Estás procesando el bloque {} de {} de una transcripción de YouTube. 
        Resume este bloque extrayendo los puntos más importantes. 
        Responde en el idioma {}.
        Responde ÚNICAMENTE en JSON con este formato:
        {{
          \"summary\": \"resumen del bloque\",
          \"key_points\": [\"punto 1\", \"punto 2\"]
        }}
        
        Texto del bloque:
        {}
    ",
        idx, total, response_language, chunk
    )
}

pub(crate) fn build_combine_prompt(
    url: &str,
    partials: &[super::OllamaSummaryJson],
    style: OutputStyle,
    response_language: &str,
) -> String {
    let mut combined_text = String::new();
    for (i, p) in partials.iter().enumerate() {
        combined_text.push_str(&format!("Bloque {}: {}\n", i + 1, p.summary));
    }

    format!(
        "A partir de los siguientes resúmenes parciales de un video de YouTube ({url}), genera un resumen final coherente y una lista de puntos clave.
        Sigue estrictamente estas instrucciones de formato:
        {chat_instruction}
        Responde en el idioma {response_language}.
        
        Responde ÚNICAMENTE en JSON con este formato:
        {{
          \"summary\": \"Resumen global consolidado\",
          \"key_points\": [\"punto clave 1\", \"punto clave 2\"],
          \"chat_text\": \"el texto listo para pegar en chat según el estilo solicitado\"
        }}

        Resúmenes parciales:
        {combined_text}
    ",
        url = url,
        chat_instruction = style.chat_instruction(),
        response_language = response_language,
        combined_text = combined_text
    )
}

pub(crate) fn build_final_prompt(
    url: &str,
    text: &str,
    style: OutputStyle,
    response_language: &str,
) -> String {
    format!(
        "Resume la siguiente transcripción de un video de YouTube ({url}).
        Sigue estrictamente estas instrucciones de formato:
        {chat_instruction}
        Responde en el idioma {response_language}.
        
        Responde ÚNICAMENTE en JSON con este formato:
        {{
          \"summary\": \"Resumen global consolidado\",
          \"key_points\": [\"punto clave 1\", \"punto clave 2\"],
          \"chat_text\": \"el texto listo para pegar en chat según el estilo solicitado\"
        }}

        Transcripción:
        {text}
    ",
        url = url,
        chat_instruction = style.chat_instruction(),
        response_language = response_language,
        text = text
    )
}

pub(crate) fn build_chat_text(
    video_url: &str,
    summary: &str,
    key_points: &[String],
    output_style: OutputStyle,
    response_language: &str,
) -> String {
    let bullets = if key_points.is_empty() {
        match normalized_lang(response_language) {
            "en" => String::from("- No key points could be extracted."),
            "pt" => String::from("- Não foi possível extrair pontos importantes."),
            "fr" => String::from("- Aucun point clé n’a pu être extrait."),
            "de" => String::from("- Es konnten keine wichtigen Punkte extrahiert werden."),
            "ja" => String::from("- 重要なポイントを抽出できませんでした。"),
            "zh" => String::from("- 未能提取关键要点。"),
            "ru" => String::from("- Не удалось извлечь ключевые моменты."),
            "ar" => String::from("- تعذّر استخراج النقاط المهمة."),
            "hi" => String::from("- मुख्य बिंदु निकाले नहीं जा सके।"),
            _ => String::from("- No se pudieron extraer puntos importantes."),
        }
    } else {
        key_points
            .iter()
            .map(|point| format!("- {}", point))
            .collect::<Vec<_>>()
            .join("\n")
    };

    match (output_style, normalized_lang(response_language)) {
        (OutputStyle::Chat, "en") => format!(
            "🚨 THIS VIDEO IS WILD\n🔗 {video_url}\n\n🔥 Quick summary:\n{summary}\n\n💥 Most important points:\n{bullets}"
        ),
        (OutputStyle::Executive, "en") => {
            format!("Video: {video_url}\n\nExecutive summary:\n{summary}\n\nKey points:\n{bullets}")
        }
        (OutputStyle::Bullets, "en") => {
            format!("Video:\n{video_url}\n\nQuick summary:\n- {summary}\n\nPoints:\n{bullets}")
        }
        (OutputStyle::Chat, "pt") => format!(
            "🚨 ESTE VÍDEO ESTÁ IMPERDÍVEL\n🔗 {video_url}\n\n🔥 Resumo rápido:\n{summary}\n\n💥 Pontos mais importantes:\n{bullets}"
        ),
        (OutputStyle::Executive, "pt") => format!(
            "Vídeo: {video_url}\n\nResumo executivo:\n{summary}\n\nPontos-chave:\n{bullets}"
        ),
        (OutputStyle::Bullets, "pt") => {
            format!("Vídeo:\n{video_url}\n\nResumo rápido:\n- {summary}\n\nPontos:\n{bullets}")
        }
        (OutputStyle::Chat, "fr") => format!(
            "🚨 CETTE VIDÉO EST INCROYABLE\n🔗 {video_url}\n\n🔥 Résumé rapide :\n{summary}\n\n💥 Points les plus importants :\n{bullets}"
        ),
        (OutputStyle::Executive, "fr") => format!(
            "Vidéo : {video_url}\n\nRésumé exécutif :\n{summary}\n\nPoints clés :\n{bullets}"
        ),
        (OutputStyle::Bullets, "fr") => {
            format!("Vidéo :\n{video_url}\n\nRésumé rapide :\n- {summary}\n\nPoints :\n{bullets}")
        }
        (OutputStyle::Chat, "de") => format!(
            "🚨 DIESES VIDEO IST HEFTIG\n🔗 {video_url}\n\n🔥 Kurzfassung:\n{summary}\n\n💥 Wichtigste Punkte:\n{bullets}"
        ),
        (OutputStyle::Executive, "de") => format!(
            "Video: {video_url}\n\nManagement-Zusammenfassung:\n{summary}\n\nKernpunkte:\n{bullets}"
        ),
        (OutputStyle::Bullets, "de") => {
            format!("Video:\n{video_url}\n\nKurzfassung:\n- {summary}\n\nPunkte:\n{bullets}")
        }
        (OutputStyle::Chat, "ja") => format!(
            "🚨 この動画はかなり重要です\n🔗 {video_url}\n\n🔥 クイック要約:\n{summary}\n\n💥 重要ポイント:\n{bullets}"
        ),
        (OutputStyle::Executive, "ja") => format!(
            "動画: {video_url}\n\nエグゼクティブ要約:\n{summary}\n\n重要ポイント:\n{bullets}"
        ),
        (OutputStyle::Bullets, "ja") => {
            format!("動画:\n{video_url}\n\nクイック要約:\n- {summary}\n\nポイント:\n{bullets}")
        }
        (OutputStyle::Chat, "zh") => format!(
            "🚨 这个视频很值得看\n🔗 {video_url}\n\n🔥 快速总结：\n{summary}\n\n💥 最重要的要点：\n{bullets}"
        ),
        (OutputStyle::Executive, "zh") => {
            format!("视频：{video_url}\n\n执行摘要：\n{summary}\n\n关键要点：\n{bullets}")
        }
        (OutputStyle::Bullets, "zh") => {
            format!("视频：\n{video_url}\n\n快速总结：\n- {summary}\n\n要点：\n{bullets}")
        }
        (OutputStyle::Chat, "ru") => format!(
            "🚨 ЭТО ВИДЕО ОЧЕНЬ СИЛЬНОЕ\n🔗 {video_url}\n\n🔥 Краткое резюме:\n{summary}\n\n💥 Самое важное:\n{bullets}"
        ),
        (OutputStyle::Executive, "ru") => format!(
            "Видео: {video_url}\n\nКраткое резюме:\n{summary}\n\nКлючевые пункты:\n{bullets}"
        ),
        (OutputStyle::Bullets, "ru") => {
            format!("Видео:\n{video_url}\n\nБыстрое резюме:\n- {summary}\n\nПункты:\n{bullets}")
        }
        (OutputStyle::Chat, "ar") => format!(
            "🚨 هذا الفيديو قوي جدًا\n🔗 {video_url}\n\n🔥 ملخص سريع:\n{summary}\n\n💥 أهم النقاط:\n{bullets}"
        ),
        (OutputStyle::Executive, "ar") => format!(
            "الفيديو: {video_url}\n\nملخص تنفيذي:\n{summary}\n\nالنقاط الرئيسية:\n{bullets}"
        ),
        (OutputStyle::Bullets, "ar") => {
            format!("الفيديو:\n{video_url}\n\nملخص سريع:\n- {summary}\n\nالنقاط:\n{bullets}")
        }
        (OutputStyle::Chat, "hi") => format!(
            "🚨 यह वीडियो कमाल का है\n🔗 {video_url}\n\n🔥 त्वरित सारांश:\n{summary}\n\n💥 सबसे महत्वपूर्ण बिंदु:\n{bullets}"
        ),
        (OutputStyle::Executive, "hi") => {
            format!("वीडियो: {video_url}\n\nकार्यकारी सारांश:\n{summary}\n\nमुख्य बिंदु:\n{bullets}")
        }
        (OutputStyle::Bullets, "hi") => {
            format!("वीडियो:\n{video_url}\n\nत्वरित सारांश:\n- {summary}\n\nबिंदु:\n{bullets}")
        }
        (OutputStyle::Chat, _) => format!(
            "🚨 ESTE VIDEO ESTÁ BRUTAL\n🔗 {video_url}\n\n🔥 Resumen rápido:\n{summary}\n\n💥 Lo más importante:\n{bullets}"
        ),
        (OutputStyle::Executive, _) => format!(
            "Video: {video_url}\n\nResumen ejecutivo:\n{summary}\n\nPuntos clave:\n{bullets}"
        ),
        (OutputStyle::Bullets, _) => {
            format!("Video:\n{video_url}\n\nResumen rápido:\n- {summary}\n\nPuntos:\n{bullets}")
        }
    }
}

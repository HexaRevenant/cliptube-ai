use crate::{
    ai::{self, AiSummary},
    share_text::build_share_text,
    transcript::{self, TranscriptBundle},
    ui::i18n::UiLanguage,
};

use super::ResultViewModel;

pub(super) fn to_view_model(
    transcript: TranscriptBundle,
    summary: AiSummary,
    output_style: ai::OutputStyle,
    output_style_index: usize,
    ui_language: UiLanguage,
    model_name: String,
    ollama_endpoint: String,
) -> ResultViewModel {
    let transcript_char_count = transcript.full_text.chars().count();
    let subtitle_kind_label = if transcript.is_generated {
        ui_language.text("auto_subtitles")
    } else {
        ui_language.text("manual_subtitles")
    };
    let key_points_text = if summary.key_points.is_empty() {
        ui_language.text("no_key_points").to_string()
    } else {
        summary
            .key_points
            .iter()
            .map(|item| format!("• {item}"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let final_share_text = build_share_text(
        &summary.chat_text,
        ui_language.text("summary"),
        &summary.summary,
        ui_language.text("key_points"),
        &key_points_text,
        &transcript.source_url,
    );

    let video_meta = format!(
        "{}: {}\nID: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}",
        "URL",
        transcript.source_url,
        transcript.video_id,
        ui_language.text("language"),
        transcript.language_label,
        ui_language.text("type"),
        subtitle_kind_label,
        ui_language.text("output"),
        output_style.label(ui_language.code()),
        ui_language.text("transcript"),
        transcript_char_count,
        "Model",
        model_name,
        "Endpoint",
        ollama_endpoint,
    );

    ResultViewModel {
        video_id: transcript.video_id.clone(),
        source_url: transcript.source_url.clone(),
        title: transcript.title.clone(),
        channel: transcript.channel.clone(),
        video_meta,
        summary: summary.summary,
        key_points_text: key_points_text.clone(),
        chat_text: summary.chat_text.clone(),
        share_text: final_share_text,
        transcript_text: transcript.full_text.clone(),
        transcript_char_count,
        ai_status: summary.status,
        language_label: transcript.language_label.clone(),
        is_generated: transcript.is_generated,
        subtitle_kind: subtitle_kind_label.to_string(),
        output_style: output_style.label(ui_language.code()).to_string(),
        output_style_index,
        ui_language: ui_language.code().to_string(),
        model_name,
        ollama_endpoint,
        segments: transcript.segments.clone(),
    }
}

#[derive(thiserror::Error, Debug)]
pub(super) enum AppError {
    #[error("No pude obtener la transcripción: {0}")]
    Transcript(#[from] transcript::TranscriptError),
    #[error("No pude generar el resumen: {0}")]
    Summary(#[from] ai::SummaryError),
    #[error("{0}")]
    Data(String),
}

use crate::{
    ai::{self, AiSummary, SummaryService},
    db::VideoEntry,
    transcript::{self, TranscriptBundle},
    ui::i18n::UiLanguage,
};

use super::{AppState, ResultViewModel};

pub(super) async fn run_analysis(
    state: &AppState,
    summary_service: &SummaryService,
    url: &str,
    requested_languages: &[String],
    output_style: ai::OutputStyle,
    output_style_index: usize,
    ui_language: UiLanguage,
) -> Result<ResultViewModel, AppError> {
    let transcript = state
        .transcript_service
        .fetch(url, requested_languages)
        .await?;

    let ai_summary = summary_service
        .summarize(&transcript, output_style, ui_language.code())
        .await?;

    Ok(to_view_model(
        transcript,
        ai_summary,
        output_style,
        output_style_index,
        ui_language,
        summary_service.model_name().to_string(),
        summary_service.endpoint().to_string(),
    ))
}

pub(super) async fn rerun_summary_from_stored(
    entry: &VideoEntry,
    segments: Vec<transcript::TranscriptSegment>,
    summary_service: &SummaryService,
    output_style: ai::OutputStyle,
    output_style_index: usize,
    ui_language: UiLanguage,
) -> Result<ResultViewModel, AppError> {
    if entry.transcript_text.trim().is_empty() {
        return Err(AppError::Data(
            "No hay transcript guardado para regenerar resumen".to_string(),
        ));
    }

    let transcript = TranscriptBundle {
        source_url: entry.source_url.clone(),
        video_id: entry.video_id.clone(),
        title: entry.title.clone(),
        channel: entry.channel.clone(),
        language_label: if entry.language_label.trim().is_empty() {
            "No especificado".to_string()
        } else {
            entry.language_label.clone()
        },
        is_generated: entry.is_generated,
        full_text: entry.transcript_text.clone(),
        segments,
    };

    let ai_summary = summary_service
        .summarize(&transcript, output_style, ui_language.code())
        .await?;

    Ok(to_view_model(
        transcript,
        ai_summary,
        output_style,
        output_style_index,
        ui_language,
        summary_service.model_name().to_string(),
        summary_service.endpoint().to_string(),
    ))
}

fn to_view_model(
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

    let final_share_text = format!(
        "{}\n\n\n{}\n{}\n\n\n{}\n{}\n\n\n{}",
        summary.chat_text.trim(),
        ui_language.text("summary"),
        summary.summary.trim(),
        ui_language.text("key_points"),
        key_points_text.trim(),
        transcript.source_url
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

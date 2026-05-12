use crate::{
    ai::{self, SummaryService},
    db::VideoEntry,
    transcript::{self, TranscriptBundle},
    ui::i18n::UiLanguage,
};

use super::{AppState, ResultViewModel, view_model};

pub(super) async fn run_analysis(
    state: &AppState,
    summary_service: &SummaryService,
    url: &str,
    requested_languages: &[String],
    output_style: ai::OutputStyle,
    output_style_index: usize,
    ui_language: UiLanguage,
) -> Result<ResultViewModel, view_model::AppError> {
    let transcript = state
        .transcript_service
        .fetch(url, requested_languages)
        .await?;

    let ai_summary = summary_service
        .summarize(&transcript, output_style, ui_language.code())
        .await?;

    Ok(view_model::to_view_model(
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
) -> Result<ResultViewModel, view_model::AppError> {
    if entry.transcript_text.trim().is_empty() {
        return Err(view_model::AppError::Data(
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

    Ok(view_model::to_view_model(
        transcript,
        ai_summary,
        output_style,
        output_style_index,
        ui_language,
        summary_service.model_name().to_string(),
        summary_service.endpoint().to_string(),
    ))
}

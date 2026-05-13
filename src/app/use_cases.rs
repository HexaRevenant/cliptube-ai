use crate::{
    ai::{self, SummaryService},
    db::VideoEntry,
    transcript::{self, TranscriptBundle},
    ui::i18n::UiLanguage,
};
use tracing::info;

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
    info!("analysis stage: transcript_fetch_start url={url}");
    let transcript = state
        .transcript_service
        .fetch(url, requested_languages)
        .await?;
    info!(
        "analysis stage: transcript_fetch_done video_id={} chars={} segments={}",
        transcript.video_id,
        transcript.full_text.chars().count(),
        transcript.segments.len()
    );

    info!(
        "analysis stage: summarize_start model={} endpoint={}",
        summary_service.model_name(),
        summary_service.endpoint()
    );
    let ai_summary = summary_service
        .summarize(&transcript, output_style, ui_language.code())
        .await?;
    info!(
        "analysis stage: summarize_done summary_chars={} key_points={}",
        ai_summary.summary.chars().count(),
        ai_summary.key_points.len()
    );

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

    info!(
        "analysis stage: rerun_summarize_start video_id={} model={}",
        entry.video_id,
        summary_service.model_name()
    );
    let ai_summary = summary_service
        .summarize(&transcript, output_style, ui_language.code())
        .await?;
    info!(
        "analysis stage: rerun_summarize_done video_id={} summary_chars={} key_points={}",
        transcript.video_id,
        ai_summary.summary.chars().count(),
        ai_summary.key_points.len()
    );

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

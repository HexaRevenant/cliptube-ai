use crate::{
    ai::{self, AiSummary, SummaryService},
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
        ui_language,
    ))
}

fn to_view_model(
    transcript: TranscriptBundle,
    summary: AiSummary,
    output_style: ai::OutputStyle,
    ui_language: UiLanguage,
) -> ResultViewModel {
    let transcript_char_count = transcript.full_text.chars().count();
    let subtitle_kind = if transcript.is_generated {
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

    ResultViewModel {
        source_url: transcript.source_url.clone(),
        video_meta: format!(
            "{}: {}\nID: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}",
            "URL",
            transcript.source_url,
            transcript.video_id,
            ui_language.text("language"),
            transcript.language_label,
            ui_language.text("type"),
            subtitle_kind,
            ui_language.text("output"),
            output_style.label(ui_language.code()),
            ui_language.text("transcript"),
            transcript_char_count,
        ),
        summary: summary.summary,
        key_points_text,
        share_text: final_share_text,
        transcript_text: transcript.full_text,
        ai_status: summary.status,
    }
}

#[derive(thiserror::Error, Debug)]
pub(super) enum AppError {
    #[error("No pude obtener la transcripción: {0}")]
    Transcript(#[from] transcript::TranscriptError),
    #[error("No pude generar el resumen: {0}")]
    Summary(#[from] ai::SummaryError),
}

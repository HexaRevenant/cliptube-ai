mod actions;
mod assets;
mod render;
mod render_chat;
mod render_main;
mod render_topbar;
mod view_model;

use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use eframe::egui;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

use crate::{
    ai::SummaryService,
    db::{self, open_db, VideoEntry},
    transcript::{TranscriptSegment, TranscriptService},
    ui::{components::install_multilingual_fonts, i18n::UiLanguage},
};

use self::{
    actions::{load_persisted_settings, ollama_settings_from_endpoint},
    assets::load_embedded_svg_color_image,
};

const APP_SETTINGS_KEY: &str = "cliptube_app_settings";
const COPY_FEEDBACK_DURATION: Duration = Duration::from_millis(1400);

#[derive(Clone)]
struct AppState {
    transcript_service: Arc<TranscriptService>,
    summary_service: Arc<SummaryService>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PersistedUiSettings {
    ui_language: String,
    languages: String,
    model_name: String,
    output_style_index: usize,
    ollama_host: String,
    ollama_port: String,
    ollama_endpoint_override: String,
}

#[derive(Clone)]
struct ResultViewModel {
    video_id: String,
    source_url: String,
    title: Option<String>,
    channel: Option<String>,
    video_meta: String,
    summary: String,
    key_points_text: String,
    chat_text: String,
    share_text: String,
    transcript_text: String,
    transcript_char_count: usize,
    ai_status: String,
    language_label: String,
    is_generated: bool,
    subtitle_kind: String,
    output_style: String,
    output_style_index: usize,
    ui_language: String,
    model_name: String,
    ollama_endpoint: String,
    segments: Vec<TranscriptSegment>,
}

enum BackgroundMessage {
    Success(ResultViewModel),
    ChatSuccess {
        reply: String,
        replace_share_text: bool,
    },
    ModelsLoaded(Vec<String>),
    ImportProgress {
        current: usize,
        total: usize,
    },
    ImportComplete {
        new_count: usize,
        updated_count: usize,
        entries: Vec<VideoEntry>,
    },
    AutoQueueProgress {
        current: usize,
        total: usize,
        video_id: String,
    },
    AutoQueueItemComplete {
        video_id: String,
        result: ResultViewModel,
    },
    AutoQueueError {
        video_id: String,
        error: String,
        is_fatal: bool,
    },
    AutoQueueComplete,
    ReprocessSegmentsProgress {
        current: usize,
        total: usize,
        video_id: String,
    },
    ReprocessSegmentsItemComplete {
        video_id: String,
    },
    ReprocessSegmentsError {
        video_id: String,
        error: String,
    },
    ReprocessSegmentsComplete {
        processed: usize,
        failed: usize,
    },
    ReprocessSummariesProgress {
        current: usize,
        total: usize,
        video_id: String,
    },
    ReprocessSummariesItemComplete {
        video_id: String,
        result: ResultViewModel,
    },
    ReprocessSummariesError {
        video_id: String,
        error: String,
    },
    ReprocessSummariesComplete {
        processed: usize,
        failed: usize,
    },
    Error(String),
}

#[derive(Clone)]
struct ChatMessage {
    role: ChatRole,
    content: String,
}

#[derive(Clone, Copy)]
enum ChatRole {
    User,
    Assistant,
}

pub struct YoutubeNativeApp {
    state: AppState,
    runtime: tokio::runtime::Runtime,
    tx: UnboundedSender<BackgroundMessage>,
    rx: UnboundedReceiver<BackgroundMessage>,
    url: String,
    languages: String,
    ui_language: UiLanguage,
    model_name: String,
    model_options: Vec<String>,
    output_style_index: usize,
    busy: bool,
    status_text: String,
    source_url: String,
    video_meta: String,
    summary: String,
    key_points_text: String,
    share_text: String,
    transcript_text: String,
    chat_input: String,
    ollama_host: String,
    ollama_port: String,
    ollama_endpoint_override: String,
    chat_messages: Vec<ChatMessage>,
    brand_logo: egui::TextureHandle,
    copy_feedback_started_at: Option<Instant>,
    pub(super) db_conn: Option<rusqlite::Connection>,
    pub(super) history_entries: Vec<VideoEntry>,
    pub(super) show_history_panel: bool,
    pub(super) history_import_status: String,
    pub(super) history_file_path: String,
    pub(super) importing_history: bool,
    pub(super) import_progress_current: usize,
    pub(super) import_progress_total: usize,
    pub(super) show_file_browser: bool,
    pub(super) file_browser_current_dir: std::path::PathBuf,
    pub(super) file_browser_entries: Vec<(String, bool)>,
    pub(super) force_reanalyze: bool,
    pub(super) auto_processing: bool,
    pub(super) auto_queue: Vec<String>,
    pub(super) auto_current: usize,
    pub(super) auto_total: usize,
    pub(super) auto_status: String,
    pub(super) auto_stop_requested: bool,
    pub(super) segments: Vec<TranscriptSegment>,
    pub(super) reprocess_segments_processing: bool,
    pub(super) reprocess_segments_queue: Vec<String>,
    pub(super) reprocess_segments_current: usize,
    pub(super) reprocess_segments_total: usize,
    pub(super) reprocess_segments_status: String,
    pub(super) reprocess_segments_stop_requested: bool,
    pub(super) reprocess_summaries_processing: bool,
    pub(super) reprocess_summaries_queue: Vec<String>,
    pub(super) reprocess_summaries_current: usize,
    pub(super) reprocess_summaries_total: usize,
    pub(super) reprocess_summaries_status: String,
    pub(super) reprocess_summaries_stop_requested: bool,
}
impl YoutubeNativeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        install_multilingual_fonts(&cc.egui_ctx);
        let (tx, rx) = unbounded_channel();
        let summary_service = Arc::new(SummaryService::from_env());
        let persisted = cc.storage.and_then(load_persisted_settings);
        let ui_language = persisted
            .as_ref()
            .and_then(|settings| UiLanguage::from_code(&settings.ui_language))
            .unwrap_or_else(UiLanguage::detect_system);
        let default_model = persisted
            .as_ref()
            .map(|settings| settings.model_name.clone())
            .filter(|name: &String| !name.trim().is_empty())
            .unwrap_or_else(|| summary_service.model_name().to_string());
        let (default_host, default_port, default_override) =
            ollama_settings_from_endpoint(summary_service.endpoint());
        let mut model_options = vec![
            default_model.clone(),
            "gemma4:31b-cloud".to_string(),
            "qwen3:32b".to_string(),
            "llama3.3:70b".to_string(),
            "mistral-large".to_string(),
        ];
        model_options.sort();
        model_options.dedup();

        let db_conn = match open_db() {
            Ok(conn) => {
                eprintln!("[DB] Database opened successfully at {:?}", db::db_path());
                Some(conn)
            }
            Err(e) => {
                eprintln!("[DB] Failed to open database: {e}");
                None
            }
        };
        let history_entries = db_conn
            .as_ref()
            .and_then(|conn| db::load_videos(conn).ok())
            .unwrap_or_default();

        Self {
            state: AppState {
                transcript_service: Arc::new(TranscriptService::new()),
                summary_service,
            },
            runtime: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("no se pudo crear runtime tokio"),
            tx,
            rx,
            url: String::new(),
            languages: persisted
                .as_ref()
                .map(|settings| settings.languages.clone())
                .filter(|value: &String| !value.trim().is_empty())
                .unwrap_or_else(|| ui_language.prefers_transcript_languages().into()),
            ui_language,
            model_name: default_model,
            model_options,
            output_style_index: persisted
                .as_ref()
                .map(|settings| settings.output_style_index)
                .unwrap_or(0),
            busy: false,
            status_text: ui_language.text("status_init").into(),
            source_url: String::new(),
            video_meta: String::new(),
            summary: String::new(),
            key_points_text: String::new(),
            share_text: String::new(),
            transcript_text: String::new(),
            chat_input: String::new(),
            ollama_host: persisted
                .as_ref()
                .map(|settings| settings.ollama_host.clone())
                .filter(|value: &String| !value.trim().is_empty())
                .unwrap_or(default_host),
            ollama_port: persisted
                .as_ref()
                .map(|settings| settings.ollama_port.clone())
                .filter(|value: &String| !value.trim().is_empty())
                .unwrap_or(default_port),
            ollama_endpoint_override: persisted
                .as_ref()
                .map(|settings| settings.ollama_endpoint_override.clone())
                .unwrap_or(default_override),
            chat_messages: vec![ChatMessage {
                role: ChatRole::Assistant,
                content: ui_language.text("chat_initial").to_string(),
            }],
            brand_logo: cc.egui_ctx.load_texture(
                "cliptube-brand-logo",
                load_embedded_svg_color_image(),
                egui::TextureOptions::LINEAR,
            ),
            copy_feedback_started_at: None,
            db_conn,
            history_entries,
            show_history_panel: false,
            history_import_status: String::new(),
            history_file_path: "/home/hexa/Descargas/Takeout/YouTube y YouTube Music/historial de videos/historial de reproducciones.html".to_string(),
            importing_history: false,
            import_progress_current: 0,
            import_progress_total: 0,
            show_file_browser: false,
            file_browser_current_dir: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
            file_browser_entries: Vec::new(),
            force_reanalyze: false,
            auto_processing: false,
            auto_queue: Vec::new(),
            auto_current: 0,
            auto_total: 0,
            auto_status: String::new(),
            auto_stop_requested: false,
            segments: Vec::new(),
            reprocess_segments_processing: false,
            reprocess_segments_queue: Vec::new(),
            reprocess_segments_current: 0,
            reprocess_segments_total: 0,
            reprocess_segments_status: String::new(),
            reprocess_segments_stop_requested: false,
            reprocess_summaries_processing: false,
            reprocess_summaries_queue: Vec::new(),
            reprocess_summaries_current: 0,
            reprocess_summaries_total: 0,
            reprocess_summaries_status: String::new(),
            reprocess_summaries_stop_requested: false,
        }
        .with_initial_model_refresh()
    }

    fn with_initial_model_refresh(mut self) -> Self {
        self.refresh_models();
        self
    }

    pub(super) fn copy_feedback_progress(&self) -> Option<f32> {
        let started_at = self.copy_feedback_started_at?;
        let elapsed = started_at.elapsed();
        if elapsed >= COPY_FEEDBACK_DURATION {
            None
        } else {
            Some(elapsed.as_secs_f32() / COPY_FEEDBACK_DURATION.as_secs_f32())
        }
    }

    pub(super) fn copy_feedback_active(&self) -> bool {
        self.copy_feedback_progress().is_some()
    }
}

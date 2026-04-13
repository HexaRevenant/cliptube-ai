mod actions;
mod assets;
mod render;
mod render_chat;
mod render_main;
mod render_topbar;
mod view_model;

use std::sync::Arc;

use eframe::egui;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

use crate::{
    ai::SummaryService,
    transcript::TranscriptService,
    ui::{components::install_multilingual_fonts, i18n::UiLanguage},
};

use self::{
    actions::{load_persisted_settings, ollama_settings_from_endpoint},
    assets::load_embedded_svg_color_image,
};

const APP_SETTINGS_KEY: &str = "cliptube_app_settings";

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
    source_url: String,
    video_meta: String,
    summary: String,
    key_points_text: String,
    share_text: String,
    transcript_text: String,
    ai_status: String,
}

enum BackgroundMessage {
    Success(ResultViewModel),
    ChatSuccess {
        reply: String,
        replace_share_text: bool,
    },
    ModelsLoaded(Vec<String>),
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
        }
        .with_initial_model_refresh()
    }

    fn with_initial_model_refresh(mut self) -> Self {
        self.refresh_models();
        self
    }
}

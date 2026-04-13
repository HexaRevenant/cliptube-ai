use arboard::Clipboard;

use super::*;
use crate::ai::{self, SummaryService};

impl YoutubeNativeApp {
    pub(super) fn switch_ui_language(&mut self, new_language: UiLanguage) {
        if self.ui_language == new_language {
            return;
        }

        let previous = self.ui_language;
        self.ui_language = new_language;
        self.languages = new_language.prefers_transcript_languages().into();

        if self.status_text == previous.text("status_init")
            || self.status_text == previous.text("status_context_ready")
            || self.status_text == previous.text("status_improved")
            || self.status_text == previous.text("status_chat_ready")
        {
            self.status_text = new_language.text("status_init").into();
        }

        if let Some(first) = self.chat_messages.first_mut()
            && matches!(first.role, ChatRole::Assistant)
            && (first.content == previous.text("chat_initial")
                || first.content == previous.text("status_context_ready"))
        {
            first.content = new_language.text("chat_initial").into();
        }
    }

    pub(super) fn effective_ollama_base_url(&self) -> Result<String, String> {
        if !self.ollama_endpoint_override.trim().is_empty() {
            let override_url = self.ollama_endpoint_override.trim().trim_end_matches('/');
            return if override_url.starts_with("http://") || override_url.starts_with("https://") {
                Ok(override_url.to_string())
            } else {
                Err(self
                    .ui_language
                    .text("status_invalid_connection")
                    .to_string())
            };
        }

        let host = self.ollama_host.trim();
        if host.is_empty() {
            return Err(self
                .ui_language
                .text("status_invalid_connection")
                .to_string());
        }

        let port = self.ollama_port.trim().parse::<u16>().map_err(|_| {
            self.ui_language
                .text("status_invalid_connection")
                .to_string()
        })?;
        let scheme = if host.starts_with("http://") || host.starts_with("https://") {
            ""
        } else {
            "http://"
        };

        Ok(format!("{scheme}{host}:{port}"))
    }

    fn effective_ollama_chat_url(&self) -> Result<String, String> {
        Ok(format!(
            "{}/api/chat",
            self.effective_ollama_base_url()?.trim_end_matches('/')
        ))
    }

    fn summary_service_for_current_settings(&self) -> Result<SummaryService, String> {
        let endpoint = self.effective_ollama_chat_url()?;
        Ok(self
            .state
            .summary_service
            .with_endpoint(endpoint)
            .with_model(self.model_name.trim().to_string()))
    }

    pub(super) fn persisted_settings(&self) -> PersistedUiSettings {
        PersistedUiSettings {
            ui_language: self.ui_language.code().to_string(),
            languages: self.languages.clone(),
            model_name: self.model_name.clone(),
            output_style_index: self.output_style_index,
            ollama_host: self.ollama_host.clone(),
            ollama_port: self.ollama_port.clone(),
            ollama_endpoint_override: self.ollama_endpoint_override.clone(),
        }
    }

    pub(super) fn start_analysis(&mut self) {
        if self.busy {
            return;
        }

        let url = self.url.trim().to_string();
        if url.is_empty() {
            self.status_text = self.ui_language.text("status_need_url").into();
            return;
        }

        self.busy = true;
        self.status_text = self.ui_language.text("status_fetching").into();

        let state = self.state.clone();
        let tx = self.tx.clone();
        let languages = parse_languages(&self.languages);
        let output_style = parse_output_style_index(self.output_style_index);
        let ui_language = self.ui_language;
        let summary_service = match self.summary_service_for_current_settings() {
            Ok(service) => service,
            Err(error) => {
                self.status_text = error;
                self.busy = false;
                return;
            }
        };

        self.runtime.spawn(async move {
            let message = match super::view_model::run_analysis(
                &state,
                &summary_service,
                &url,
                &languages,
                output_style,
                ui_language,
            )
            .await
            {
                Ok(view) => BackgroundMessage::Success(view),
                Err(error) => BackgroundMessage::Error(error.to_string()),
            };
            let _ = tx.send(message);
        });
    }

    pub(super) fn handle_background_messages(&mut self) {
        while let Ok(message) = self.rx.try_recv() {
            self.busy = false;
            match message {
                BackgroundMessage::Success(result) => {
                    self.source_url = result.source_url;
                    self.video_meta = result.video_meta;
                    self.summary = result.summary;
                    self.key_points_text = result.key_points_text;
                    self.share_text = result.share_text;
                    self.transcript_text = result.transcript_text;
                    self.status_text = result.ai_status;
                    self.chat_messages.push(ChatMessage {
                        role: ChatRole::Assistant,
                        content: self.ui_language.text("status_context_ready").into(),
                    });
                }
                BackgroundMessage::ChatSuccess {
                    reply,
                    replace_share_text,
                } => {
                    if replace_share_text {
                        self.share_text = reply.clone();
                        self.status_text = self.ui_language.text("status_improved").into();
                    } else {
                        self.status_text = self.ui_language.text("status_chat_ready").into();
                    }

                    self.chat_messages.push(ChatMessage {
                        role: ChatRole::Assistant,
                        content: reply,
                    });
                }
                BackgroundMessage::ModelsLoaded(models) => {
                    if !models.is_empty() {
                        self.model_options = models;
                        if !self
                            .model_options
                            .iter()
                            .any(|model| model == &self.model_name)
                        {
                            self.model_options.insert(0, self.model_name.clone());
                        }
                        self.status_text = format!(
                            "Ollama · {} {} · {}",
                            self.model_options.len(),
                            self.ui_language.text("models_noun"),
                            self.effective_ollama_base_url().unwrap_or_default()
                        );
                    }
                }
                BackgroundMessage::Error(error) => {
                    self.status_text = format!("Error: {error}");
                    self.source_url.clear();
                    self.video_meta.clear();
                    self.summary.clear();
                    self.key_points_text.clear();
                    self.share_text.clear();
                    self.transcript_text.clear();
                }
            }
        }
    }

    pub(super) fn start_chat_request(&mut self, improve_share_text: bool) {
        if self.busy {
            return;
        }

        if self.transcript_text.trim().is_empty() {
            self.status_text = self.ui_language.text("status_need_transcript").into();
            return;
        }

        let user_prompt = self.chat_input.trim().to_string();
        if !improve_share_text && user_prompt.is_empty() {
            self.status_text = self.ui_language.text("status_need_question").into();
            return;
        }

        if improve_share_text {
            let prompt = if user_prompt.is_empty() {
                "Mejora el texto listo para pegar ya creado. Mantén el orden actual: texto inicial, Resumen, Puntos Importantes y URL final. Hazlo más claro, más potente y más natural, sin inventar nada.".to_string()
            } else {
                format!(
                    "Mejora el texto listo para pegar ya creado. Mantén el orden actual: texto inicial, Resumen, Puntos Importantes y URL final. Instrucciones extra del usuario: {}",
                    user_prompt
                )
            };

            self.chat_messages.push(ChatMessage {
                role: ChatRole::User,
                content: format!("Mejorar texto final: {prompt}"),
            });

            self.spawn_chat_task(prompt, true);
        } else {
            self.chat_messages.push(ChatMessage {
                role: ChatRole::User,
                content: user_prompt.clone(),
            });
            self.spawn_chat_task(user_prompt, false);
        }

        self.chat_input.clear();
        self.busy = true;
        self.status_text = self.ui_language.text("status_consulting_ai").into();
    }

    pub(super) fn refresh_models(&mut self) {
        let tx = self.tx.clone();
        let summary_service = match self.summary_service_for_current_settings() {
            Ok(service) => service,
            Err(error) => {
                self.status_text = error;
                self.busy = false;
                return;
            }
        };
        self.busy = true;
        self.status_text = format!(
            "{} {}",
            self.ui_language.text("status_loading_models"),
            self.effective_ollama_base_url().unwrap_or_default()
        );

        self.runtime.spawn(async move {
            let message = match summary_service.list_models().await {
                Ok(models) => BackgroundMessage::ModelsLoaded(models),
                Err(error) => BackgroundMessage::Error(error.to_string()),
            };
            let _ = tx.send(message);
        });
    }

    fn spawn_chat_task(&self, user_prompt: String, replace_share_text: bool) {
        let tx = self.tx.clone();
        let transcript_text = self.transcript_text.clone();
        let share_text = self.share_text.clone();
        let source_url = self.source_url.clone();
        let response_language = self.ui_language.code().to_string();
        let summary_service = match self.summary_service_for_current_settings() {
            Ok(service) => service,
            Err(error) => {
                let _ = self.tx.send(BackgroundMessage::Error(error));
                return;
            }
        };

        self.runtime.spawn(async move {
            let message = match summary_service
                .ask_about_video(
                    &source_url,
                    &transcript_text,
                    &share_text,
                    &user_prompt,
                    replace_share_text,
                    &response_language,
                )
                .await
            {
                Ok(reply) => BackgroundMessage::ChatSuccess {
                    reply,
                    replace_share_text,
                },
                Err(error) => BackgroundMessage::Error(error.to_string()),
            };
            let _ = tx.send(message);
        });
    }

    pub(super) fn copy_share_text(&mut self) {
        match Clipboard::new() {
            Ok(mut cb) => match cb.set_text(self.share_text.clone()) {
                Ok(()) => {
                    self.status_text = self.ui_language.text("clipboard_ok").into();
                    self.copy_feedback_started_at = Some(Instant::now());
                }
                Err(err) => self.status_text = format!("No pude copiar al portapapeles: {err}"),
            },
            Err(err) => {
                self.status_text = format!("No pude abrir el portapapeles: {err}");
            }
        }
    }
}

pub(super) fn parse_output_style_index(index: usize) -> ai::OutputStyle {
    match index {
        1 => ai::OutputStyle::Executive,
        2 => ai::OutputStyle::Bullets,
        _ => ai::OutputStyle::Chat,
    }
}

fn parse_languages(input: &str) -> Vec<String> {
    let mut langs: Vec<String> = input
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| part.to_lowercase())
        .collect();

    if langs.is_empty() {
        langs.extend(
            UiLanguage::detect_system()
                .prefers_transcript_languages()
                .split(',')
                .map(|s| s.to_string()),
        );
    }

    langs
}

pub(super) fn ollama_settings_from_endpoint(endpoint: &str) -> (String, String, String) {
    let trimmed = endpoint.trim();
    let without_suffix = trimmed.trim_end_matches("/api/chat").trim_end_matches('/');
    let normalized =
        if without_suffix.starts_with("http://") || without_suffix.starts_with("https://") {
            without_suffix.to_string()
        } else {
            format!("http://{without_suffix}")
        };

    if let Ok(url) = url::Url::parse(&normalized) {
        let host = url.host_str().unwrap_or("127.0.0.1").to_string();
        let port = url.port_or_known_default().unwrap_or(11434).to_string();
        (host, port, String::new())
    } else {
        (
            "127.0.0.1".to_string(),
            "11434".to_string(),
            without_suffix.to_string(),
        )
    }
}

pub(super) fn load_persisted_settings(
    storage: &dyn eframe::Storage,
) -> Option<PersistedUiSettings> {
    storage
        .get_string(APP_SETTINGS_KEY)
        .and_then(|value| serde_json::from_str::<PersistedUiSettings>(&value).ok())
}

pub(super) fn save_persisted_settings(
    storage: &mut dyn eframe::Storage,
    settings: &PersistedUiSettings,
) {
    if let Ok(value) = serde_json::to_string(settings) {
        storage.set_string(APP_SETTINGS_KEY, value);
    }
}

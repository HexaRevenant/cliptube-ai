use arboard::Clipboard;

use super::*;
use crate::ai::{self, SummaryService};
use crate::db::{self, VideoEntry};
use crate::history;

fn summary_needs_reprocess(entry: &VideoEntry) -> bool {
    let status = entry.ai_status.to_lowercase();
    entry.summary.trim().is_empty()
        || status.contains("429")
        || status.contains("too many requests")
        || status.contains("fallback extractivo")
        || status.contains("no tiene el formato esperado")
}

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

        // Check if already analyzed in database (unless forcing re-analysis)
        if !self.force_reanalyze {
            if let Ok(video_id) = crate::transcript_helpers::extract_video_id(&url) {
                if let Some(conn) = self.db_conn.as_mut() {
                    if let Ok(Some(entry)) = db::load_video_by_id(conn, &video_id) {
                        if !entry.summary.is_empty() {
                            self.load_video_from_db(&entry);
                            self.status_text = self.ui_language.text("status_loaded_from_history").into();
                            return;
                        }
                    }
                }
            }
        }

        self.busy = true;
        self.status_text = self.ui_language.text("status_fetching").into();

        let state = self.state.clone();
        let tx = self.tx.clone();
        let languages = parse_languages(&self.languages);
        let output_style_index = self.output_style_index;
        let output_style = parse_output_style_index(output_style_index);
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
                output_style_index,
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
                    self.source_url = result.source_url.clone();
                    self.video_meta = result.video_meta.clone();
                    self.summary = result.summary.clone();
                    self.key_points_text = result.key_points_text.clone();
                    self.share_text = result.share_text.clone();
                    self.transcript_text = result.transcript_text.clone();
                    self.segments = result.segments.clone();
                    self.status_text = result.ai_status.clone();
                    self.chat_messages.push(ChatMessage {
                        role: ChatRole::Assistant,
                        content: self.ui_language.text("status_context_ready").into(),
                    });
                    self.save_current_result_to_db(&result);
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
                BackgroundMessage::ImportProgress { current, total } => {
                    self.import_progress_current = current;
                    self.import_progress_total = total;
                    self.history_import_status =
                        format!("Procesando {current} / {total} videos...");
                }
                BackgroundMessage::ImportComplete {
                    new_count,
                    updated_count,
                    entries,
                } => {
                    self.history_entries = entries;
                    self.history_import_status =
                        format!("Nuevos: {new_count} | Actualizados: {updated_count}");
                    self.importing_history = false;
                }
                BackgroundMessage::AutoQueueProgress {
                    current,
                    total,
                    video_id,
                } => {
                    self.auto_current = current;
                    self.auto_total = total;
                    let remaining = total.saturating_sub(current);
                    self.auto_status =
                        format!("Analizando video {current}/{total} (faltan {remaining}): {video_id}");
                    self.url = format!("https://www.youtube.com/watch?v={}", video_id);
                }
                BackgroundMessage::AutoQueueItemComplete { video_id, result } => {
                    self.segments = result.segments.clone();
                    self.save_current_result_to_db(&result);
                    let remaining = self.auto_total.saturating_sub(self.auto_current);
                    self.auto_status =
                        format!("✅ Video {} guardado | {}/{} completados | {} faltan",
                            video_id, self.auto_current, self.auto_total, remaining);
                    if !self.auto_stop_requested {
                        self.process_next_auto_queue();
                    }
                }
                BackgroundMessage::AutoQueueError {
                    video_id,
                    error,
                    is_fatal,
                } => {
                    if is_fatal {
                        self.auto_processing = false;
                        self.auto_status = format!("🛑 DETENIDO en video {}: {}", video_id, error);
                        self.status_text = format!("Error fatal en auto-proceso: {error}");
                    } else {
                        let remaining = self.auto_total.saturating_sub(self.auto_current);
                        self.auto_status =
                            format!("⚠️  Error en {} (continúa): {} | {}/{} | {} faltan",
                                video_id, error, self.auto_current, self.auto_total, remaining);
                        if !self.auto_stop_requested {
                            self.process_next_auto_queue();
                        }
                    }
                }
                BackgroundMessage::AutoQueueComplete => {
                    self.auto_processing = false;
                    self.auto_status = "🎉 Auto-proceso completado - todos los videos analizados".to_string();
                    if let Some(conn) = self.db_conn.as_mut() {
                        if let Ok(list) = db::load_videos(conn) {
                            self.history_entries = list;
                        }
                    }
                }
                BackgroundMessage::ReprocessSegmentsProgress {
                    current,
                    total,
                    video_id,
                } => {
                    self.reprocess_segments_current = current;
                    self.reprocess_segments_total = total;
                    self.reprocess_segments_status =
                        format!("Reprocesando tiempos {current}/{total}: {video_id}");
                }
                BackgroundMessage::ReprocessSegmentsItemComplete { video_id } => {
                    let remaining = self.reprocess_segments_total.saturating_sub(self.reprocess_segments_current);
                    self.reprocess_segments_status =
                        format!("✅ Tiempos guardados para {} | {}/{} | {} faltan",
                            video_id, self.reprocess_segments_current, self.reprocess_segments_total, remaining);
                    if !self.reprocess_segments_stop_requested {
                        self.process_next_reprocess_segment();
                    }
                }
                BackgroundMessage::ReprocessSegmentsError { video_id, error } => {
                    let remaining = self.reprocess_segments_total.saturating_sub(self.reprocess_segments_current);
                    self.reprocess_segments_status =
                        format!("⚠️ Error en {}: {} | {}/{} | {} faltan",
                            video_id, error, self.reprocess_segments_current, self.reprocess_segments_total, remaining);
                    if !self.reprocess_segments_stop_requested {
                        self.process_next_reprocess_segment();
                    }
                }
                BackgroundMessage::ReprocessSegmentsComplete { processed, failed } => {
                    self.reprocess_segments_processing = false;
                    self.reprocess_segments_status =
                        format!("🎉 Reproceso completado: {} guardados, {} fallidos", processed, failed);
                    if let Some(conn) = self.db_conn.as_mut() {
                        if let Ok(list) = db::load_videos(conn) {
                            self.history_entries = list;
                        }
                    }
                }
                BackgroundMessage::ReprocessSummariesProgress {
                    current,
                    total,
                    video_id,
                } => {
                    self.reprocess_summaries_current = current;
                    self.reprocess_summaries_total = total;
                    self.reprocess_summaries_status =
                        format!("Reprocesando resumen {current}/{total}: {video_id}");
                }
                BackgroundMessage::ReprocessSummariesItemComplete { video_id, result } => {
                    self.segments = result.segments.clone();
                    self.save_current_result_to_db(&result);
                    let remaining = self
                        .reprocess_summaries_total
                        .saturating_sub(self.reprocess_summaries_current);
                    self.reprocess_summaries_status = format!(
                        "✅ Resumen regenerado para {} | {}/{} | {} faltan",
                        video_id,
                        self.reprocess_summaries_current,
                        self.reprocess_summaries_total,
                        remaining
                    );
                    if !self.reprocess_summaries_stop_requested {
                        self.process_next_reprocess_summary();
                    }
                }
                BackgroundMessage::ReprocessSummariesError { video_id, error } => {
                    let remaining = self
                        .reprocess_summaries_total
                        .saturating_sub(self.reprocess_summaries_current);
                    self.reprocess_summaries_status = format!(
                        "⚠️ Error en resumen {}: {} | {}/{} | {} faltan",
                        video_id,
                        error,
                        self.reprocess_summaries_current,
                        self.reprocess_summaries_total,
                        remaining
                    );
                    if !self.reprocess_summaries_stop_requested {
                        self.process_next_reprocess_summary();
                    }
                }
                BackgroundMessage::ReprocessSummariesComplete { processed, failed } => {
                    self.reprocess_summaries_processing = false;
                    self.reprocess_summaries_status = format!(
                        "🎉 Reproceso de resúmenes completado: {} regenerados, {} fallidos",
                        processed, failed
                    );
                    if let Some(conn) = self.db_conn.as_mut() {
                        if let Ok(list) = db::load_videos(conn) {
                            self.history_entries = list;
                        }
                    }
                }
                BackgroundMessage::Error(error) => {
                    self.status_text = format!("Error: {error}");
                    self.importing_history = false;
                    self.auto_processing = false;
                    self.reprocess_summaries_processing = false;
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

    fn save_current_result_to_db(&mut self, result: &ResultViewModel) {
        let Some(conn) = self.db_conn.as_mut() else {
            eprintln!("[DB] db_conn is None, cannot save");
            return;
        };
        if result.video_id.is_empty() {
            eprintln!("[DB] video_id is empty, cannot save");
            return;
        }
        eprintln!("[DB] Saving video_id={}", result.video_id);
        let now = chrono::Local::now().to_rfc3339();
        let entry = VideoEntry {
            id: 0,
            video_id: result.video_id.clone(),
            source_url: result.source_url.clone(),
            title: result.title.clone(),
            channel: result.channel.clone(),
            summary: result.summary.clone(),
            key_points: result.key_points_text.clone(),
            chat_text: result.chat_text.clone(),
            share_text: result.share_text.clone(),
            transcript_text: result.transcript_text.clone(),
            transcript_char_count: result.transcript_char_count as i64,
            ai_status: result.ai_status.clone(),
            language_label: result.language_label.clone(),
            is_generated: result.is_generated,
            subtitle_kind: result.subtitle_kind.clone(),
            output_style: result.output_style.clone(),
            output_style_index: result.output_style_index as i64,
            ui_language: result.ui_language.clone(),
            model_name: result.model_name.clone(),
            ollama_endpoint: result.ollama_endpoint.clone(),
            video_meta: result.video_meta.clone(),
            watched_at: None,
            watched_at_sortable: 0,
            created_at: now,
        };
        if let Err(e) = db::save_video(conn, &entry) {
            eprintln!("[DB] Failed to save video to db: {e}");
        } else {
            eprintln!("[DB] Video saved successfully");
            if let Err(e) = db::save_segments(conn, &result.video_id, &result.segments) {
                eprintln!("[DB] Failed to save segments: {e}");
            }
            if let Ok(list) = db::load_videos(conn) {
                self.history_entries = list;
            }
        }
    }

    pub(super) fn load_video_from_db(&mut self, entry: &VideoEntry) {
        self.url = entry.source_url.clone();
        self.source_url = entry.source_url.clone();
        self.summary = entry.summary.clone();
        self.key_points_text = entry.key_points.clone();
        self.chat_input = String::new();
        self.share_text = entry.share_text.clone();
        self.transcript_text = entry.transcript_text.clone();
        self.video_meta = entry.video_meta.clone();
        if let Ok(style_index) = usize::try_from(entry.output_style_index) {
            self.output_style_index = style_index;
        }
        if !entry.model_name.is_empty() {
            self.model_name = entry.model_name.clone();
        }
        if !entry.language_label.is_empty() {
            // Stored video_meta already contains language info
        }
        self.segments = if let Some(conn) = self.db_conn.as_ref() {
            db::load_segments(conn, &entry.video_id).unwrap_or_default()
        } else {
            Vec::new()
        };
        self.chat_messages = vec![
            ChatMessage {
                role: ChatRole::Assistant,
                content: self.ui_language.text("chat_initial").to_string(),
            },
            ChatMessage {
                role: ChatRole::Assistant,
                content: self.ui_language.text("status_context_ready").to_string(),
            },
        ];
        self.status_text = self.ui_language.text("status_context_ready").into();
    }

    pub(super) fn import_youtube_history(&mut self) {
        let path = self.history_file_path.trim().to_string();
        if path.is_empty() {
            self.history_import_status = "Ruta vacía".to_string();
            return;
        }
        self.importing_history = true;
        self.import_progress_current = 0;
        self.import_progress_total = 0;
        self.history_import_status = "Leyendo archivo HTML...".to_string();

        let tx = self.tx.clone();
        self.runtime.spawn(async move {
            let message = match history::parse_watch_history(&path) {
                Ok(entries) => {
                    let total = entries.len();
                    let mut new_count = 0;
                    let mut updated_count = 0;

                    // Open a separate DB connection in this thread
                    match db::open_db() {
                        Ok(conn) => {
                            for (i, entry) in entries.into_iter().enumerate() {
                                let _ = tx.send(BackgroundMessage::ImportProgress {
                                    current: i + 1,
                                    total,
                                });
                                let exists = db::video_exists(&conn, &entry.video_id).unwrap_or(false);
                                if !exists {
                                    let watched_at_sortable = entry.watched_at.as_ref()
                                        .map(|d| db::parse_spanish_date(d))
                                        .unwrap_or(0);
                                    let db_entry = VideoEntry {
                                        id: 0,
                                        video_id: entry.video_id,
                                        source_url: entry.source_url,
                                        title: Some(entry.title),
                                        channel: Some(entry.channel),
                                        summary: String::new(),
                                        key_points: String::new(),
                                        chat_text: String::new(),
                                        share_text: String::new(),
                                        transcript_text: String::new(),
                                        transcript_char_count: 0,
                                        ai_status: String::new(),
                                        language_label: String::new(),
                                        is_generated: false,
                                        subtitle_kind: String::new(),
                                        output_style: String::new(),
                                        output_style_index: 0,
                                        ui_language: String::new(),
                                        model_name: String::new(),
                                        ollama_endpoint: String::new(),
                                        video_meta: String::new(),
                                        watched_at: entry.watched_at,
                                        watched_at_sortable,
                                        created_at: chrono::Local::now().to_rfc3339(),
                                    };
                                    if db::save_video(&conn, &db_entry).is_ok() {
                                        new_count += 1;
                                    }
                                } else {
                                    let title_ref = entry.title.as_str();
                                    let channel_ref = entry.channel.as_str();
                                    let watched_ref = entry.watched_at.as_deref();
                                    match db::update_history_fields(
                                        &conn,
                                        &entry.video_id,
                                        Some(title_ref),
                                        Some(channel_ref),
                                        watched_ref,
                                    ) {
                                        Ok(true) => updated_count += 1,
                                        Ok(false) => {}
                                        Err(e) => eprintln!("Update history fields failed: {e}"),
                                    }
                                }
                            }
                            let entries = db::load_videos(&conn).unwrap_or_default();
                            BackgroundMessage::ImportComplete {
                                new_count,
                                updated_count,
                                entries,
                            }
                        }
                        Err(e) => BackgroundMessage::Error(format!("DB error: {e}")),
                    }
                }
                Err(e) => BackgroundMessage::Error(format!("Parse error: {e}")),
            };
            let _ = tx.send(message);
        });
    }

    pub(super) fn start_reprocess_segments(&mut self) {
        if self.reprocess_segments_processing {
            return;
        }
        let Some(conn) = self.db_conn.as_ref() else {
            self.reprocess_segments_status = "DB no disponible".to_string();
            return;
        };
        let pending: Vec<String> = match db::load_videos_without_segments(conn) {
            Ok(entries) => entries.into_iter().map(|e| e.source_url).collect(),
            Err(e) => {
                self.reprocess_segments_status = format!("Error cargando videos: {e}");
                return;
            }
        };
        if pending.is_empty() {
            self.reprocess_segments_status = "Todos los videos ya tienen tiempos".to_string();
            return;
        }
        self.reprocess_segments_processing = true;
        self.reprocess_segments_stop_requested = false;
        self.reprocess_segments_queue = pending;
        self.reprocess_segments_current = 0;
        self.reprocess_segments_total = self.reprocess_segments_queue.len();
        self.reprocess_segments_status = format!(
            "Iniciando reproceso de tiempos para {} videos...",
            self.reprocess_segments_total
        );
        self.process_next_reprocess_segment();
    }

    pub(super) fn stop_reprocess_segments(&mut self) {
        self.reprocess_segments_stop_requested = true;
        self.reprocess_segments_processing = false;
        self.reprocess_segments_status = "Reproceso de tiempos detenido por usuario".to_string();
    }

    fn process_next_reprocess_segment(&mut self) {
        if self.reprocess_segments_stop_requested || self.reprocess_segments_queue.is_empty() {
            let processed = self.reprocess_segments_current;
            let failed = self.reprocess_segments_total.saturating_sub(processed);
            let _ = self
                .tx
                .send(BackgroundMessage::ReprocessSegmentsComplete { processed, failed });
            return;
        }
        let url = self.reprocess_segments_queue.remove(0);
        self.reprocess_segments_current += 1;
        let current = self.reprocess_segments_current;
        let total = self.reprocess_segments_total;

        let video_id = match crate::transcript_helpers::extract_video_id(&url) {
            Ok(id) => id,
            Err(_) => {
                let _ = self.tx.send(BackgroundMessage::ReprocessSegmentsError {
                    video_id: url,
                    error: "URL inválida".to_string(),
                });
                return;
            }
        };

        let _ = self.tx.send(BackgroundMessage::ReprocessSegmentsProgress {
            current,
            total,
            video_id: video_id.clone(),
        });

        let state = self.state.clone();
        let tx = self.tx.clone();
        let languages = parse_languages(&self.languages);

        self.runtime.spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;

            match state.transcript_service.fetch(&url, &languages).await {
                Ok(bundle) => {
                    match db::open_db() {
                        Ok(conn) => {
                            if let Err(e) = db::save_segments(&conn, &video_id, &bundle.segments) {
                                let _ = tx.send(BackgroundMessage::ReprocessSegmentsError {
                                    video_id: video_id.clone(),
                                    error: format!("DB error: {e}"),
                                });
                            } else {
                                let _ = tx
                                    .send(BackgroundMessage::ReprocessSegmentsItemComplete {
                                        video_id: video_id.clone(),
                                    });
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(BackgroundMessage::ReprocessSegmentsError {
                                video_id: video_id.clone(),
                                error: format!("DB open error: {e}"),
                            });
                        }
                    }
                }
                Err(error) => {
                    let _ = tx.send(BackgroundMessage::ReprocessSegmentsError {
                        video_id: video_id.clone(),
                        error: error.to_string(),
                    });
                }
            }
        });
    }

    pub(super) fn start_auto_processing(&mut self) {
        if self.auto_processing {
            return;
        }
        let Some(conn) = self.db_conn.as_ref() else {
            self.auto_status = "DB no disponible".to_string();
            return;
        };
        // Load all videos without summary
        let pending: Vec<String> = match db::load_videos(conn) {
            Ok(entries) => {
                entries
                    .into_iter()
                    .filter(|e| e.summary.is_empty())
                    .map(|e| e.source_url)
                    .collect()
            }
            Err(e) => {
                self.auto_status = format!("Error cargando videos: {e}");
                return;
            }
        };
        if pending.is_empty() {
            self.auto_status = "No hay videos pendientes".to_string();
            return;
        }
        self.auto_processing = true;
        self.auto_stop_requested = false;
        self.auto_queue = pending;
        self.auto_current = 0;
        self.auto_total = self.auto_queue.len();
        self.auto_status = format!("Iniciando auto-proceso de {} videos...", self.auto_total);
        self.process_next_auto_queue();
    }

    pub(super) fn retry_pending_and_failed(&mut self) {
        if self.auto_processing {
            return;
        }
        let pending: Vec<String> = self
            .history_entries
            .iter()
            .filter(|e| summary_needs_reprocess(e))
            .map(|e| e.source_url.clone())
            .collect();
        if pending.is_empty() {
            self.auto_status = "No hay pendientes/fallidos para reintentar".to_string();
            return;
        }
        self.auto_processing = true;
        self.auto_stop_requested = false;
        self.auto_queue = pending;
        self.auto_current = 0;
        self.auto_total = self.auto_queue.len();
        self.auto_status = format!(
            "Iniciando reintento de {} videos (pendientes/fallidos)...",
            self.auto_total
        );
        self.process_next_auto_queue();
    }

    pub(super) fn start_reprocess_summaries(&mut self) {
        if self.reprocess_summaries_processing {
            return;
        }
        let pending: Vec<String> = self
            .history_entries
            .iter()
            .filter(|e| summary_needs_reprocess(e) && !e.transcript_text.trim().is_empty())
            .map(|e| e.video_id.clone())
            .collect();
        if pending.is_empty() {
            self.reprocess_summaries_status = "No hay resúmenes para reprocesar".to_string();
            return;
        }
        self.reprocess_summaries_processing = true;
        self.reprocess_summaries_stop_requested = false;
        self.reprocess_summaries_queue = pending;
        self.reprocess_summaries_current = 0;
        self.reprocess_summaries_total = self.reprocess_summaries_queue.len();
        self.reprocess_summaries_status = format!(
            "Iniciando reproceso de {} resúmenes...",
            self.reprocess_summaries_total
        );
        self.process_next_reprocess_summary();
    }

    pub(super) fn stop_reprocess_summaries(&mut self) {
        self.reprocess_summaries_stop_requested = true;
        self.reprocess_summaries_processing = false;
        self.reprocess_summaries_status = "Reproceso de resúmenes detenido por usuario".to_string();
    }

    fn process_next_reprocess_summary(&mut self) {
        if self.reprocess_summaries_stop_requested || self.reprocess_summaries_queue.is_empty() {
            let processed = self.reprocess_summaries_current;
            let failed = self.reprocess_summaries_total.saturating_sub(processed);
            let _ = self
                .tx
                .send(BackgroundMessage::ReprocessSummariesComplete { processed, failed });
            return;
        }
        let video_id = self.reprocess_summaries_queue.remove(0);
        self.reprocess_summaries_current += 1;
        let current = self.reprocess_summaries_current;
        let total = self.reprocess_summaries_total;
        let _ = self.tx.send(BackgroundMessage::ReprocessSummariesProgress {
            current,
            total,
            video_id: video_id.clone(),
        });

        let tx = self.tx.clone();
        let output_style_index = self.output_style_index;
        let output_style = parse_output_style_index(output_style_index);
        let ui_language = self.ui_language;
        let summary_service = match self.summary_service_for_current_settings() {
            Ok(service) => service,
            Err(error) => {
                let _ = tx.send(BackgroundMessage::ReprocessSummariesError { video_id, error });
                return;
            }
        };

        self.runtime.spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            match db::open_db() {
                Ok(conn) => match db::load_video_by_id(&conn, &video_id) {
                    Ok(Some(entry)) => {
                        let segments = db::load_segments(&conn, &video_id).unwrap_or_default();
                        match super::view_model::rerun_summary_from_stored(
                            &entry,
                            segments,
                            &summary_service,
                            output_style,
                            output_style_index,
                            ui_language,
                        )
                        .await
                        {
                            Ok(result) => {
                                let _ = tx.send(BackgroundMessage::ReprocessSummariesItemComplete {
                                    video_id: video_id.clone(),
                                    result,
                                });
                            }
                            Err(error) => {
                                let _ = tx.send(BackgroundMessage::ReprocessSummariesError {
                                    video_id: video_id.clone(),
                                    error: error.to_string(),
                                });
                            }
                        }
                    }
                    Ok(None) => {
                        let _ = tx.send(BackgroundMessage::ReprocessSummariesError {
                            video_id: video_id.clone(),
                            error: "No existe en DB".to_string(),
                        });
                    }
                    Err(e) => {
                        let _ = tx.send(BackgroundMessage::ReprocessSummariesError {
                            video_id: video_id.clone(),
                            error: format!("DB error: {e}"),
                        });
                    }
                },
                Err(e) => {
                    let _ = tx.send(BackgroundMessage::ReprocessSummariesError {
                        video_id: video_id.clone(),
                        error: format!("DB open error: {e}"),
                    });
                }
            }
        });
    }

    pub(super) fn stop_auto_processing(&mut self) {
        self.auto_stop_requested = true;
        self.auto_processing = false;
        self.auto_status = "Auto-proceso detenido por usuario".to_string();
    }

    fn process_next_auto_queue(&mut self) {
        if self.auto_stop_requested || self.auto_queue.is_empty() {
            let _ = self.tx.send(BackgroundMessage::AutoQueueComplete);
            return;
        }
        let url = self.auto_queue.remove(0);
        self.auto_current += 1;
        let current = self.auto_current;
        let total = self.auto_total;

        let video_id = match crate::transcript_helpers::extract_video_id(&url) {
            Ok(id) => id,
            Err(_) => {
                let _ = self.tx.send(BackgroundMessage::AutoQueueError {
                    video_id: url,
                    error: "URL inválida".to_string(),
                    is_fatal: false,
                });
                return;
            }
        };

        let _ = self.tx.send(BackgroundMessage::AutoQueueProgress {
            current,
            total,
            video_id: video_id.clone(),
        });

        let state = self.state.clone();
        let tx = self.tx.clone();
        let languages = parse_languages(&self.languages);
        let output_style_index = self.output_style_index;
        let output_style = parse_output_style_index(output_style_index);
        let ui_language = self.ui_language;
        let summary_service = match self.summary_service_for_current_settings() {
            Ok(service) => service,
            Err(error) => {
                let _ = tx.send(BackgroundMessage::AutoQueueError {
                    video_id,
                    error,
                    is_fatal: true,
                });
                return;
            }
        };

        self.runtime.spawn(async move {
            // Wait 1 second between videos to avoid rate limiting
            tokio::time::sleep(Duration::from_secs(1)).await;

            match super::view_model::run_analysis(
                &state,
                &summary_service,
                &url,
                &languages,
                output_style,
                output_style_index,
                ui_language,
            )
            .await
            {
                Ok(result) => {
                    let _ = tx.send(BackgroundMessage::AutoQueueItemComplete {
                        video_id: result.video_id.clone(),
                        result,
                    });
                }
                Err(error) => {
                    let error_str = error.to_string();
                    let is_fatal = is_fatal_error(&error);
                    let _ = tx.send(BackgroundMessage::AutoQueueError {
                        video_id,
                        error: error_str,
                        is_fatal,
                    });
                }
            }
        });
    }

    pub(super) fn delete_history_entry(&mut self, video_id: &str) {
        let Some(conn) = self.db_conn.as_mut() else { return };
        if let Err(e) = db::delete_video(conn, video_id) {
            eprintln!("Failed to delete video: {e}");
        } else if let Ok(list) = db::load_videos(conn) {
            self.history_entries = list;
        }
    }

    pub(super) fn copy_entry_share_text(&mut self, text: &str) {
        match Clipboard::new() {
            Ok(mut cb) => match cb.set_text(text.to_string()) {
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

    pub(super) fn open_file_browser(&mut self) {
        self.show_file_browser = true;
        self.refresh_file_browser();
    }

    pub(super) fn refresh_file_browser(&mut self) {
        self.file_browser_entries.clear();
        if let Ok(entries) = std::fs::read_dir(&self.file_browser_current_dir) {
            let mut dirs = Vec::new();
            let mut files = Vec::new();
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                if name.starts_with('.') {
                    continue;
                }
                if is_dir {
                    dirs.push((name, true));
                } else if name.ends_with(".html") {
                    files.push((name, false));
                }
            }
            dirs.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
            files.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
            self.file_browser_entries.push(("📁 ..".to_string(), true));
            self.file_browser_entries.extend(dirs);
            self.file_browser_entries.extend(files);
        }
    }

    pub(super) fn navigate_file_browser(&mut self, name: &str) {
        if name == ".." {
            if let Some(parent) = self.file_browser_current_dir.parent() {
                self.file_browser_current_dir = parent.to_path_buf();
            }
        } else {
            self.file_browser_current_dir.push(name);
        }
        self.refresh_file_browser();
    }

    pub(super) fn select_file_browser(&mut self, name: &str) {
        self.file_browser_current_dir.push(name);
        self.history_file_path = self.file_browser_current_dir.to_string_lossy().to_string();
        self.show_file_browser = false;
    }

}

fn is_fatal_error(error: &super::view_model::AppError) -> bool {
    match error {
        super::view_model::AppError::Transcript(te) => {
            let err_str = te.to_string().to_lowercase();
            err_str.contains("429")
                || err_str.contains("too many requests")
                || err_str.contains("banned")
                || err_str.contains("rate limit")
                || err_str.contains("consent")
                || err_str.contains("forbidden")
                || err_str.contains("unauthorized")
        }
        super::view_model::AppError::Summary(se) => {
            let err_str = se.to_string().to_lowercase();
            err_str.contains("connection refused")
                || err_str.contains("timeout")
                || err_str.contains("model not found")
                || err_str.contains("service unavailable")
        }
        super::view_model::AppError::Data(_) => false,
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

use eframe::egui;

use super::*;
use crate::db;
use crate::ui::{
    components::{
        field_label, full_width_card, full_width_title_block, icon_button, metric_chip,
        output_style_name, primary_button, secondary_button, section_header, section_text,
        section_text_stretch,
    },
    theme::{BrandColors, LayoutSpace},
};

impl YoutubeNativeApp {
    pub(super) fn render_main_panel(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                full_width_title_block(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("ClipTube AI");
                        ui.colored_label(BrandColors::MUTED, self.ui_language.text("app_desc"));
                        ui.add_space(LayoutSpace::MD);
                        metric_chip(
                            ui,
                            format!(
                                "Ollama · {} {}",
                                self.model_options.len(),
                                self.ui_language.text("models_noun")
                            ),
                            BrandColors::CYAN,
                        );
                    });
                });

                ui.add_space(LayoutSpace::MD);

                full_width_card(ui, |ui| {
                    section_header(
                        ui,
                        self.ui_language.text("video"),
                        self.ui_language.text("video_config_desc"),
                        self.ui_language,
                    );
                    ui.add_space(LayoutSpace::SM);

                    field_label(ui, self.ui_language.text("video"), self.ui_language);
                    ui.add_enabled(
                        !self.busy,
                        egui::TextEdit::singleline(&mut self.url)
                            .hint_text("https://www.youtube.com/watch?v=...")
                            .desired_width(f32::INFINITY)
                            .horizontal_align(if self.ui_language.is_rtl() {
                                egui::Align::RIGHT
                            } else {
                                egui::Align::LEFT
                            }),
                    );

                    ui.add_space(LayoutSpace::MD);
                    ui.vertical(|ui| {
                        field_label(
                            ui,
                            self.ui_language.text("ollama_connection"),
                            self.ui_language,
                        );
                        ui.colored_label(
                            BrandColors::MUTED,
                            self.ui_language.text("connection_desc"),
                        );
                        ui.add_space(LayoutSpace::XS);
                        ui.columns(2, |columns| {
                            field_label(
                                &mut columns[0],
                                self.ui_language.text("ollama_host"),
                                self.ui_language,
                            );
                            columns[0].add_enabled(
                                !self.busy,
                                egui::TextEdit::singleline(&mut self.ollama_host)
                                    .desired_width(f32::INFINITY)
                                    .hint_text("127.0.0.1"),
                            );
                            field_label(
                                &mut columns[1],
                                self.ui_language.text("ollama_port"),
                                self.ui_language,
                            );
                            columns[1].add_enabled(
                                !self.busy,
                                egui::TextEdit::singleline(&mut self.ollama_port)
                                    .desired_width(f32::INFINITY)
                                    .hint_text("11434"),
                            );
                        });

                        ui.add_space(LayoutSpace::SM);
                        field_label(
                            ui,
                            self.ui_language.text("endpoint_override"),
                            self.ui_language,
                        );
                        ui.add_enabled(
                            !self.busy,
                            egui::TextEdit::singleline(&mut self.ollama_endpoint_override)
                                .desired_width(f32::INFINITY)
                                .hint_text(self.ui_language.text("endpoint_override_hint")),
                        );
                        ui.add_space(LayoutSpace::XS);
                        if let Ok(effective_url) = self.effective_ollama_base_url() {
                            metric_chip(
                                ui,
                                format!(
                                    "{} · {}",
                                    self.ui_language.text("effective_url"),
                                    effective_url
                                ),
                                BrandColors::CYAN,
                            );
                        }

                        ui.add_space(LayoutSpace::MD);
                        field_label(ui, self.ui_language.text("languages"), self.ui_language);
                        ui.add_enabled(
                            !self.busy,
                            egui::TextEdit::singleline(&mut self.languages)
                                .desired_width(f32::INFINITY)
                                .horizontal_align(if self.ui_language.is_rtl() {
                                    egui::Align::RIGHT
                                } else {
                                    egui::Align::LEFT
                                }),
                        );

                        ui.add_space(LayoutSpace::SM);
                        field_label(ui, self.ui_language.text("model"), self.ui_language);
                        ui.horizontal(|ui| {
                            egui::ComboBox::from_id_salt("model_selector")
                                .width((ui.available_width() - 52.0).max(180.0))
                                .selected_text(self.model_name.clone())
                                .show_ui(ui, |ui| {
                                    for model in &self.model_options {
                                        ui.selectable_value(
                                            &mut self.model_name,
                                            model.clone(),
                                            model,
                                        );
                                    }
                                });
                            if ui
                                .add_enabled(
                                    !self.busy,
                                    icon_button("↻", self.ui_language.text("reload_models")),
                                )
                                .clicked()
                            {
                                self.refresh_models();
                            }
                        });
                        ui.add_space(LayoutSpace::XS);
                        ui.add_enabled(
                            !self.busy,
                            egui::TextEdit::singleline(&mut self.model_name)
                                .desired_width(f32::INFINITY)
                                .hint_text(self.ui_language.text("custom_model")),
                        );

                        ui.add_space(LayoutSpace::SM);
                        field_label(ui, self.ui_language.text("format"), self.ui_language);
                        egui::ComboBox::from_id_salt("output_style")
                            .width(ui.available_width())
                            .selected_text(output_style_name(
                                self.output_style_index,
                                self.ui_language,
                            ))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.output_style_index,
                                    0,
                                    self.ui_language.text("chat_format"),
                                );
                                ui.selectable_value(
                                    &mut self.output_style_index,
                                    1,
                                    self.ui_language.text("executive_format"),
                                );
                                ui.selectable_value(
                                    &mut self.output_style_index,
                                    2,
                                    self.ui_language.text("bullets_format"),
                                );
                            });
                    });

                    ui.add_space(LayoutSpace::SM);
                    ui.checkbox(
                        &mut self.force_reanalyze,
                        "Forzar re-análisis (ignorar cache)",
                    );

                    ui.add_space(LayoutSpace::MD);
                    ui.horizontal_wrapped(|ui| {
                        if ui
                            .add_enabled(
                                !self.busy,
                                primary_button(if self.busy {
                                    self.ui_language.text("processing")
                                } else {
                                    self.ui_language.text("run")
                                }),
                            )
                            .clicked()
                        {
                            self.start_analysis();
                        }

                        if ui
                            .add_enabled(!self.busy && !self.share_text.is_empty(), {
                                let copy_button =
                                    if let Some(progress) = self.copy_feedback_progress() {
                                        let pulse = 1.0 - progress;
                                        let fill = egui::lerp(
                                            egui::Rgba::from(BrandColors::CYAN)
                                                ..=egui::Rgba::from(BrandColors::VIOLET),
                                            pulse.clamp(0.0, 1.0),
                                        );
                                        egui::Button::new(
                                            egui::RichText::new(format!(
                                                "✅ {}",
                                                self.ui_language.text("copy_final")
                                            ))
                                            .strong(),
                                        )
                                        .fill(fill)
                                        .stroke(egui::Stroke::new(
                                            1.0 + (pulse * 1.5),
                                            BrandColors::line_strong(),
                                        ))
                                        .corner_radius(
                                            egui::CornerRadius::same(LayoutSpace::INPUT_RADIUS),
                                        )
                                    } else {
                                        secondary_button(self.ui_language.text("copy_final"))
                                    };
                                copy_button
                            })
                            .clicked()
                        {
                            self.copy_share_text();
                        }
                    });
                });

                ui.add_space(LayoutSpace::MD);

                full_width_card(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(self.ui_language.text("history"))
                                .size(18.0)
                                .strong()
                                .color(BrandColors::TEXT),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .add(secondary_button(if self.show_history_panel {
                                    "▲"
                                } else {
                                    "▼"
                                }))
                                .clicked()
                            {
                                self.show_history_panel = !self.show_history_panel;
                            }
                        });
                    });
                    if !self.history_import_status.is_empty() {
                        ui.add_space(LayoutSpace::XS);
                        ui.colored_label(BrandColors::CYAN, &self.history_import_status);
                    }
                    if self.show_history_panel {
                        ui.add_space(LayoutSpace::SM);
                        ui.horizontal(|ui| {
                            if ui.add(icon_button("📁", "Explorar")).clicked() {
                                self.open_file_browser();
                            }
                            ui.add(
                                egui::TextEdit::singleline(&mut self.history_file_path)
                                    .hint_text("Ruta del archivo HTML de YouTube...")
                                    .desired_width(ui.available_width() - 170.0),
                            );
                            if ui
                                .add_enabled(
                                    !self.busy && !self.importing_history,
                                    secondary_button(self.ui_language.text("import_history")),
                                )
                                .clicked()
                            {
                                self.import_youtube_history();
                            }
                        });
                        ui.add_space(LayoutSpace::SM);
                        if self.importing_history {
                            let progress = if self.import_progress_total > 0 {
                                self.import_progress_current as f32
                                    / self.import_progress_total as f32
                            } else {
                                0.0
                            };
                            ui.add(
                                egui::ProgressBar::new(progress)
                                    .text(format!(
                                        "Importando {} / {}",
                                        self.import_progress_current, self.import_progress_total
                                    ))
                                    .fill(BrandColors::VIOLET)
                                    .desired_width(ui.available_width()),
                            );
                            ui.ctx().request_repaint();
                        }

                        // Auto-processing controls
                        ui.add_space(LayoutSpace::SM);
                        ui.horizontal(|ui| {
                            if self.auto_processing {
                                if ui.add(secondary_button("⏹ Detener auto")).clicked() {
                                    self.stop_auto_processing();
                                }
                            } else {
                                // Count pending videos
                                let pending = self
                                    .history_entries
                                    .iter()
                                    .filter(|e| e.summary.is_empty())
                                    .count();
                                let button_text = if pending > 0 {
                                    format!("▶ Procesar lista ({} pendientes)", pending)
                                } else {
                                    "▶ Procesar lista".to_string()
                                };
                                if ui
                                    .add_enabled(
                                        !self.busy && !self.importing_history && pending > 0,
                                        secondary_button(&button_text),
                                    )
                                    .clicked()
                                {
                                    self.start_auto_processing();
                                }
                            }
                        });

                        ui.add_space(LayoutSpace::XS);
                        ui.horizontal(|ui| {
                            let retryable = self
                                .history_entries
                                .iter()
                                .filter(|e| {
                                    let status = e.ai_status.to_lowercase();
                                    e.summary.trim().is_empty()
                                        || status.contains("429")
                                        || status.contains("too many requests")
                                        || status.contains("error en ia")
                                        || status.contains("fallback extractivo")
                                })
                                .count();
                            let button_text = if retryable > 0 {
                                format!("🔁 Reintentar pendientes/fallidos ({retryable})")
                            } else {
                                "🔁 Reintentar pendientes/fallidos".to_string()
                            };
                            if ui
                                .add_enabled(
                                    !self.busy && !self.importing_history && !self.auto_processing && retryable > 0,
                                    secondary_button(&button_text),
                                )
                                .clicked()
                            {
                                self.retry_pending_and_failed();
                            }
                        });

                        // Re-process timestamps controls
                        ui.add_space(LayoutSpace::SM);
                        ui.horizontal(|ui| {
                            if self.reprocess_segments_processing {
                                if ui.add(secondary_button("⏹ Detener reproceso")).clicked() {
                                    self.stop_reprocess_segments();
                                }
                            } else {
                                // Count videos without timestamps
                                let missing = if let Some(conn) = self.db_conn.as_ref() {
                                    db::load_videos_without_segments(conn)
                                        .map(|v| v.len())
                                        .unwrap_or(0)
                                } else {
                                    0
                                };
                                let button_text = if missing > 0 {
                                    format!("⏱ Reprocesar tiempos ({} sin tiempos)", missing)
                                } else {
                                    "⏱ Reprocesar tiempos".to_string()
                                };
                                if ui
                                    .add_enabled(
                                        !self.busy
                                            && !self.importing_history
                                            && !self.auto_processing
                                            && missing > 0,
                                        secondary_button(&button_text),
                                    )
                                    .clicked()
                                {
                                    self.start_reprocess_segments();
                                }
                            }
                        });

                        // Re-process summaries controls
                        ui.add_space(LayoutSpace::SM);
                        ui.horizontal(|ui| {
                            if self.reprocess_summaries_processing {
                                if ui
                                    .add(secondary_button("⏹ Detener reproceso resúmenes"))
                                    .clicked()
                                {
                                    self.stop_reprocess_summaries();
                                }
                            } else {
                                let retryable = self
                                    .history_entries
                                    .iter()
                                    .filter(|e| {
                                        let status = e.ai_status.to_lowercase();
                                        (!e.transcript_text.trim().is_empty())
                                            && (e.summary.trim().is_empty()
                                                || status.contains("429")
                                                || status.contains("too many requests")
                                                || status.contains("fallback extractivo")
                                                || status.contains("no tiene el formato esperado"))
                                    })
                                    .count();
                                let button_text = if retryable > 0 {
                                    format!(
                                        "🔁 Reprocesar resúmenes ({} pendientes)",
                                        retryable
                                    )
                                } else {
                                    "🔁 Reprocesar resúmenes".to_string()
                                };
                                if ui
                                    .add_enabled(
                                        !self.busy
                                            && !self.importing_history
                                            && !self.auto_processing
                                            && !self.reprocess_segments_processing
                                            && retryable > 0,
                                        secondary_button(&button_text),
                                    )
                                    .clicked()
                                {
                                    self.start_reprocess_summaries();
                                }
                            }
                        });
                        if self.auto_processing && self.auto_total > 0 {
                            let progress = self.auto_current as f32 / self.auto_total as f32;
                            let remaining = self.auto_total.saturating_sub(self.auto_current);
                            ui.vertical(|ui| {
                                ui.add(
                                    egui::ProgressBar::new(progress)
                                        .text(format!(
                                            "Procesando {} de {} | {} faltan | {}%",
                                            self.auto_current,
                                            self.auto_total,
                                            remaining,
                                            (progress * 100.0) as u32
                                        ))
                                        .fill(BrandColors::PINK)
                                        .desired_width(ui.available_width()),
                                );
                                if !self.auto_status.is_empty() {
                                    ui.colored_label(
                                        BrandColors::CYAN,
                                        format!("📋 {}", self.auto_status),
                                    );
                                }
                            });
                            ui.ctx().request_repaint();
                        } else if !self.auto_status.is_empty() {
                            ui.colored_label(BrandColors::CYAN, &self.auto_status);
                        }

                        if self.reprocess_segments_processing && self.reprocess_segments_total > 0 {
                            let progress = self.reprocess_segments_current as f32
                                / self.reprocess_segments_total as f32;
                            let remaining = self
                                .reprocess_segments_total
                                .saturating_sub(self.reprocess_segments_current);
                            ui.vertical(|ui| {
                                ui.add(
                                    egui::ProgressBar::new(progress)
                                        .text(format!(
                                            "Reprocesando tiempos {} de {} | {} faltan | {}%",
                                            self.reprocess_segments_current,
                                            self.reprocess_segments_total,
                                            remaining,
                                            (progress * 100.0) as u32
                                        ))
                                        .fill(BrandColors::CYAN)
                                        .desired_width(ui.available_width()),
                                );
                                if !self.reprocess_segments_status.is_empty() {
                                    ui.colored_label(
                                        BrandColors::CYAN,
                                        format!("⏱ {}", self.reprocess_segments_status),
                                    );
                                }
                            });
                            ui.ctx().request_repaint();
                        } else if !self.reprocess_segments_status.is_empty() {
                            ui.colored_label(BrandColors::CYAN, &self.reprocess_segments_status);
                        }

                        if self.reprocess_summaries_processing && self.reprocess_summaries_total > 0
                        {
                            let progress = self.reprocess_summaries_current as f32
                                / self.reprocess_summaries_total as f32;
                            let remaining = self
                                .reprocess_summaries_total
                                .saturating_sub(self.reprocess_summaries_current);
                            ui.vertical(|ui| {
                                ui.add(
                                    egui::ProgressBar::new(progress)
                                        .text(format!(
                                            "Reprocesando resúmenes {} de {} | {} faltan | {}%",
                                            self.reprocess_summaries_current,
                                            self.reprocess_summaries_total,
                                            remaining,
                                            (progress * 100.0) as u32
                                        ))
                                        .fill(BrandColors::VIOLET)
                                        .desired_width(ui.available_width()),
                                );
                                if !self.reprocess_summaries_status.is_empty() {
                                    ui.colored_label(
                                        BrandColors::VIOLET,
                                        format!("🔁 {}", self.reprocess_summaries_status),
                                    );
                                }
                            });
                            ui.ctx().request_repaint();
                        } else if !self.reprocess_summaries_status.is_empty() {
                            ui.colored_label(BrandColors::VIOLET, &self.reprocess_summaries_status);
                        }
                        if self.history_entries.is_empty() {
                            ui.colored_label(
                                BrandColors::MUTED,
                                self.ui_language.text("no_history"),
                            );
                        } else {
                            egui::ScrollArea::vertical()
                                .max_height(280.0)
                                .show(ui, |ui| {
                                    for entry in self.history_entries.clone() {
                                        ui.horizontal(|ui| {
                                            ui.vertical(|ui| {
                                                ui.label(
                                                    egui::RichText::new(entry.display_title())
                                                        .size(14.0)
                                                        .color(BrandColors::TEXT),
                                                );
                                                if let Some(date) = &entry.watched_at {
                                                    ui.colored_label(
                                                        BrandColors::MUTED,
                                                        egui::RichText::new(date)
                                                            .size(11.0)
                                                            .italics(),
                                                    );
                                                }
                                            });
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    if ui
                                                        .add(icon_button(
                                                            "🗑",
                                                            self.ui_language.text("delete"),
                                                        ))
                                                        .clicked()
                                                    {
                                                        self.delete_history_entry(&entry.video_id);
                                                    }
                                                    if ui
                                                        .add(icon_button(
                                                            "📋",
                                                            self.ui_language.text("copy_final"),
                                                        ))
                                                        .clicked()
                                                    {
                                                        self.copy_entry_share_text(
                                                            &entry.share_text,
                                                        );
                                                    }
                                                    if ui
                                                        .add(icon_button(
                                                            "📂",
                                                            self.ui_language.text("load"),
                                                        ))
                                                        .clicked()
                                                    {
                                                        self.load_video_from_db(&entry);
                                                    }
                                                },
                                            );
                                        });
                                        ui.add_space(LayoutSpace::XS);
                                    }
                                });
                        }
                    }
                });

                ui.add_space(LayoutSpace::MD);

                if !self.video_meta.is_empty() {
                    full_width_card(ui, |ui| {
                        section_header(
                            ui,
                            self.ui_language.text("video_details"),
                            self.ui_language.text("video_meta_desc"),
                            self.ui_language,
                        );
                        ui.add_space(LayoutSpace::XS);
                        ui.add(
                            egui::TextEdit::multiline(&mut self.video_meta)
                                .desired_width(f32::INFINITY)
                                .desired_rows(4)
                                .interactive(false),
                        );
                    });
                    ui.add_space(LayoutSpace::MD);
                }

                section_text(
                    ui,
                    self.ui_language.text("share_text"),
                    &mut self.share_text,
                    12,
                    true,
                    self.ui_language,
                );
                let transcript_height = (ui.available_height() - LayoutSpace::XL).max(220.0);
                section_text_stretch(
                    ui,
                    self.ui_language.text("full_transcript"),
                    &mut self.transcript_text,
                    false,
                    transcript_height,
                    self.ui_language,
                );
                ui.add_space(LayoutSpace::XL);
            });
    }
}

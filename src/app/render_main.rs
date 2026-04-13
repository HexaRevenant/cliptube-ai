use eframe::egui;

use super::*;
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
                            .add_enabled(
                                !self.busy && !self.share_text.is_empty(),
                                secondary_button(self.ui_language.text("copy_final")),
                            )
                            .clicked()
                        {
                            self.copy_share_text();
                        }
                    });
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

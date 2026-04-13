use eframe::egui;

use super::*;
use crate::ui::{
    components::{card_frame, field_label, primary_button, secondary_button, section_header},
    theme::{BrandColors, LayoutSpace},
};

impl YoutubeNativeApp {
    pub(super) fn render_chat_panel(&mut self, ui: &mut egui::Ui) {
        let outer_bottom_gap = 40.0;
        let panel_rect = ui.available_rect_before_wrap();
        let card_rect = egui::Rect::from_min_max(
            panel_rect.min,
            egui::pos2(panel_rect.max.x, panel_rect.max.y - outer_bottom_gap),
        );

        let mut child_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(card_rect)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );

        card_frame().show(&mut child_ui, |ui| {
            ui.set_min_height((card_rect.height() - (LayoutSpace::LG * 2.0)).max(0.0));

            section_header(
                ui,
                self.ui_language.text("chat_title"),
                self.ui_language.text("chat_desc"),
                self.ui_language,
            );
            ui.add_space(LayoutSpace::SM);

            let available_height = ui.available_height();
            let gap = LayoutSpace::MD;
            let composer_height = LayoutSpace::CHAT_COMPOSER_HEIGHT;
            let messages_height =
                (available_height - composer_height - gap).max(LayoutSpace::CHAT_MESSAGES_HEIGHT);

            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), messages_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    egui::Frame::new()
                        .fill(BrandColors::PANEL)
                        .stroke(egui::Stroke::new(1.0, BrandColors::line()))
                        .corner_radius(egui::CornerRadius::same(22))
                        .inner_margin(egui::Margin::same(12))
                        .show(ui, |ui| {
                            ui.set_min_height(messages_height - 24.0);
                            egui::ScrollArea::vertical()
                                .auto_shrink([false; 2])
                                .scroll_bar_visibility(
                                    egui::scroll_area::ScrollBarVisibility::AlwaysVisible,
                                )
                                .show(ui, |ui| {
                                    for message in &self.chat_messages {
                                        self.render_chat_bubble(ui, message);
                                        ui.add_space(LayoutSpace::SM);
                                    }
                                });
                        });
                },
            );
            ui.add_space(gap);
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), composer_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    egui::Frame::new()
                        .fill(BrandColors::PANEL)
                        .stroke(egui::Stroke::new(1.0, BrandColors::line()))
                        .corner_radius(egui::CornerRadius::same(20))
                        .inner_margin(egui::Margin::same(12))
                        .show(ui, |ui| {
                            field_label(ui, self.ui_language.text("question"), self.ui_language);
                            ui.add_space(LayoutSpace::XS);
                            ui.add(
                                egui::TextEdit::multiline(&mut self.chat_input)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(3)
                                    .horizontal_align(if self.ui_language.is_rtl() {
                                        egui::Align::RIGHT
                                    } else {
                                        egui::Align::LEFT
                                    })
                                    .vertical_align(egui::Align::Center)
                                    .margin(egui::Margin {
                                        left: 12,
                                        right: 12,
                                        top: 10,
                                        bottom: 10,
                                    })
                                    .hint_text(self.ui_language.text("question_hint")),
                            );
                            ui.add_space(LayoutSpace::XS);
                            ui.horizontal_wrapped(|ui| {
                                if ui
                                    .add_enabled(
                                        !self.busy,
                                        primary_button(self.ui_language.text("ask_ai")),
                                    )
                                    .clicked()
                                {
                                    self.start_chat_request(false);
                                }

                                if ui
                                    .add_enabled(
                                        !self.busy && !self.share_text.trim().is_empty(),
                                        secondary_button(self.ui_language.text("improve_final")),
                                    )
                                    .clicked()
                                {
                                    self.start_chat_request(true);
                                }
                            });
                        });
                },
            );
        });
    }

    fn render_chat_bubble(&self, ui: &mut egui::Ui, message: &ChatMessage) {
        let (title, accent, fill) = match message.role {
            ChatRole::User => (
                self.ui_language.text("you"),
                BrandColors::CYAN,
                egui::Color32::from_rgb(12, 31, 44),
            ),
            ChatRole::Assistant => (
                self.ui_language.text("assistant"),
                BrandColors::PINK,
                BrandColors::PANEL_STRONG,
            ),
        };

        egui::Frame::new()
            .fill(fill)
            .stroke(egui::Stroke::new(1.0, BrandColors::line()))
            .corner_radius(egui::CornerRadius::same(18))
            .inner_margin(egui::Margin::same(12))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    ui.colored_label(accent, egui::RichText::new(title).strong());
                    if matches!(message.role, ChatRole::User) {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.colored_label(BrandColors::MUTED, self.ui_language.text("message"));
                        });
                    }
                });
                ui.add_space(4.0);
                ui.add(egui::Label::new(&message.content).wrap());
            });
    }
}

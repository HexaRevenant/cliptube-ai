use eframe::egui;

use super::*;
use crate::ui::{
    components::{accent_dot, hero_logo, language_flag, status_chip},
    theme::{BrandColors, LayoutSpace},
};

impl YoutubeNativeApp {
    pub(super) fn render_top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = 14.0;

            hero_logo(ui, &self.brand_logo);

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("ClipTube")
                            .size(28.0)
                            .strong()
                            .color(BrandColors::TEXT),
                    );
                    egui::Frame::new()
                        .fill(egui::Color32::from_rgba_unmultiplied(
                            BrandColors::PINK.r(),
                            BrandColors::PINK.g(),
                            BrandColors::PINK.b(),
                            40,
                        ))
                        .stroke(egui::Stroke::new(
                            1.0,
                            egui::Color32::from_rgba_unmultiplied(
                                BrandColors::PINK.r(),
                                BrandColors::PINK.g(),
                                BrandColors::PINK.b(),
                                90,
                            ),
                        ))
                        .corner_radius(egui::CornerRadius::same(255))
                        .inner_margin(egui::Margin {
                            left: 8,
                            right: 8,
                            top: 3,
                            bottom: 3,
                        })
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new("AI")
                                    .size(15.0)
                                    .strong()
                                    .color(BrandColors::PINK),
                            );
                        });
                });
                ui.horizontal(|ui| {
                    accent_dot(ui, BrandColors::CYAN, 4.5);
                    ui.colored_label(BrandColors::MUTED, self.ui_language.text("app_desc"));
                });
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                status_chip(
                    ui,
                    if self.busy {
                        self.ui_language.text("processing")
                    } else {
                        &self.status_text
                    },
                    if self.busy {
                        BrandColors::PINK
                    } else {
                        BrandColors::CYAN
                    },
                );
                ui.add_space(LayoutSpace::SM);
                egui::ComboBox::from_id_salt("ui_language_selector")
                    .selected_text(format!(
                        "{} · {}",
                        self.ui_language.country_code(),
                        self.ui_language.display_name()
                    ))
                    .width(220.0)
                    .show_ui(ui, |ui| {
                        for language in UiLanguage::all() {
                            ui.horizontal(|ui| {
                                language_flag(ui, *language);
                                let label = format!(
                                    "{} · {}",
                                    language.country_code(),
                                    language.display_name()
                                );
                                if ui
                                    .selectable_label(self.ui_language == *language, label)
                                    .clicked()
                                {
                                    self.switch_ui_language(*language);
                                }
                            });
                        }
                    });
            });
        });
    }
}

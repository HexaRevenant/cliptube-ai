use eframe::egui;

use super::{
    i18n::UiLanguage,
    theme::{BrandColors, LayoutSpace},
};

pub fn section_text(
    ui: &mut egui::Ui,
    title: &str,
    text: &mut String,
    rows: usize,
    emphasize: bool,
    ui_language: UiLanguage,
) {
    full_width_card(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(title)
                    .size(18.0)
                    .strong()
                    .color(BrandColors::TEXT),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                metric_chip(
                    ui,
                    format!(
                        "{} {}",
                        text.chars().count(),
                        ui_language.text("characters")
                    ),
                    if emphasize {
                        BrandColors::PINK
                    } else {
                        BrandColors::CYAN
                    },
                );
            });
        });
        ui.add_space(LayoutSpace::XS);
        ui.add(
            egui::TextEdit::multiline(text)
                .desired_width(f32::INFINITY)
                .desired_rows(rows)
                .interactive(true),
        );
    });
    ui.add_space(LayoutSpace::MD);
}

pub fn section_text_stretch(
    ui: &mut egui::Ui,
    title: &str,
    text: &mut String,
    emphasize: bool,
    min_height: f32,
    ui_language: UiLanguage,
) {
    full_width_card(ui, |ui| {
        ui.set_min_height(min_height);
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(title)
                    .size(18.0)
                    .strong()
                    .color(BrandColors::TEXT),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                metric_chip(
                    ui,
                    format!(
                        "{} {}",
                        text.chars().count(),
                        ui_language.text("characters")
                    ),
                    if emphasize {
                        BrandColors::PINK
                    } else {
                        BrandColors::CYAN
                    },
                );
            });
        });
        ui.add_space(LayoutSpace::XS);
        let editor_height = (min_height - 72.0).max(140.0);
        ui.add_sized(
            [ui.available_width(), editor_height],
            egui::TextEdit::multiline(text)
                .desired_width(f32::INFINITY)
                .interactive(true),
        );
    });
    ui.add_space(LayoutSpace::MD);
}

pub fn full_width_card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    let width = ui.available_width();
    ui.allocate_ui_with_layout(
        egui::vec2(width, 0.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            card_frame().show(ui, |ui| {
                ui.set_min_width(width - (LayoutSpace::LG * 2.0));
                add_contents(ui);
            });
        },
    );
}

pub fn full_width_title_block(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    let width = ui.available_width();
    ui.allocate_ui_with_layout(
        egui::vec2(width, 0.0),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            title_block_frame().show(ui, |ui| {
                ui.set_min_width(width - 40.0);
                add_contents(ui);
            });
        },
    );
}

pub fn card_frame() -> egui::Frame {
    egui::Frame::group(&egui::Style::default())
        .fill(BrandColors::PANEL_STRONG)
        .stroke(egui::Stroke::new(1.0, BrandColors::line()))
        .corner_radius(egui::CornerRadius::same(LayoutSpace::CARD_RADIUS))
        .inner_margin(egui::Margin::same(LayoutSpace::LG as i8))
}

pub fn title_block_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(10, 16, 30))
        .stroke(egui::Stroke::new(1.0, BrandColors::line()))
        .corner_radius(egui::CornerRadius::same(26))
        .inner_margin(egui::Margin::same(20))
}

pub fn top_bar_frame() -> egui::Frame {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(10, 15, 29))
        .stroke(egui::Stroke::new(0.0, egui::Color32::TRANSPARENT))
        .inner_margin(egui::Margin {
            left: 16,
            right: 16,
            top: 14,
            bottom: 12,
        })
}

pub fn primary_button(label: impl Into<egui::WidgetText>) -> egui::Button<'static> {
    egui::Button::new(label)
        .fill(BrandColors::PINK)
        .stroke(egui::Stroke::new(1.0, BrandColors::line_strong()))
        .corner_radius(egui::CornerRadius::same(LayoutSpace::INPUT_RADIUS))
}

pub fn secondary_button(label: impl Into<egui::WidgetText>) -> egui::Button<'static> {
    egui::Button::new(label)
        .fill(BrandColors::PANEL)
        .stroke(egui::Stroke::new(1.0, BrandColors::line()))
        .corner_radius(egui::CornerRadius::same(LayoutSpace::INPUT_RADIUS))
}

pub fn icon_button(label: impl Into<egui::WidgetText>, _hover_text: &str) -> egui::Button<'static> {
    egui::Button::new(label)
        .min_size(egui::vec2(38.0, 38.0))
        .fill(BrandColors::PANEL)
        .stroke(egui::Stroke::new(1.0, BrandColors::line()))
        .corner_radius(egui::CornerRadius::same(LayoutSpace::INPUT_RADIUS))
}

pub fn field_label(ui: &mut egui::Ui, label: &str, ui_language: UiLanguage) {
    if ui_language.is_rtl() {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(label)
                    .size(14.0)
                    .strong()
                    .color(BrandColors::TEXT),
            );
        });
    } else {
        ui.label(
            egui::RichText::new(label)
                .size(14.0)
                .strong()
                .color(BrandColors::TEXT),
        );
    }
}

pub fn section_header(ui: &mut egui::Ui, title: &str, subtitle: &str, ui_language: UiLanguage) {
    if ui_language.is_rtl() {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            ui.label(
                egui::RichText::new(title)
                    .size(24.0)
                    .strong()
                    .color(BrandColors::TEXT),
            );
        });
        ui.add_space(LayoutSpace::XS);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
            ui.colored_label(BrandColors::MUTED, subtitle);
        });
    } else {
        ui.label(
            egui::RichText::new(title)
                .size(24.0)
                .strong()
                .color(BrandColors::TEXT),
        );
        ui.add_space(LayoutSpace::XS);
        ui.colored_label(BrandColors::MUTED, subtitle);
    }
}

pub fn metric_chip(ui: &mut egui::Ui, text: String, accent: egui::Color32) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgba_unmultiplied(
            accent.r(),
            accent.g(),
            accent.b(),
            26,
        ))
        .stroke(egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 80),
        ))
        .corner_radius(egui::CornerRadius::same(255))
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 6,
            bottom: 6,
        })
        .show(ui, |ui| {
            ui.label(egui::RichText::new(text).color(accent).strong());
        });
}

pub fn status_chip(ui: &mut egui::Ui, text: &str, accent: egui::Color32) {
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(13, 19, 34))
        .stroke(egui::Stroke::new(1.0, BrandColors::line()))
        .corner_radius(egui::CornerRadius::same(255))
        .inner_margin(egui::Margin {
            left: 10,
            right: 12,
            top: 8,
            bottom: 8,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                accent_dot(ui, accent, 4.5);
                ui.label(egui::RichText::new(text).color(BrandColors::TEXT));
            });
        });
}

pub fn install_multilingual_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let mut added = Vec::new();

    for (name, path) in [
        (
            "noto-sans",
            "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
        ),
        (
            "noto-sans-arabic",
            "/usr/share/fonts/truetype/noto/NotoSansArabic-Regular.ttf",
        ),
        (
            "noto-sans-devanagari",
            "/usr/share/fonts/truetype/noto/NotoSansDevanagari-Regular.ttf",
        ),
        (
            "noto-sans-cjk",
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        ),
    ] {
        if let Ok(bytes) = std::fs::read(path) {
            fonts
                .font_data
                .insert(name.to_owned(), egui::FontData::from_owned(bytes).into());
            added.push(name.to_owned());
        }
    }

    if !added.is_empty() {
        if let Some(proportional) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
            for name in &added {
                proportional.push(name.clone());
            }
        }
        if let Some(monospace) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
            for name in &added {
                monospace.push(name.clone());
            }
        }
        ctx.set_fonts(fonts);
    }
}

pub fn accent_dot(ui: &mut egui::Ui, color: egui::Color32, radius: f32) {
    let size = egui::vec2(radius * 2.0 + 4.0, radius * 2.0 + 4.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    ui.painter().circle_filled(rect.center(), radius, color);
}

pub fn language_flag(ui: &mut egui::Ui, language: UiLanguage) {
    let size = egui::vec2(18.0, 12.0);
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let painter = ui.painter();
    let round = egui::CornerRadius::same(3);
    painter.rect_filled(rect, round, BrandColors::PANEL_STRONG);

    match language {
        UiLanguage::Es => {
            let top = egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.min.y + 3.0));
            let mid = egui::Rect::from_min_max(
                egui::pos2(rect.min.x, rect.min.y + 3.0),
                egui::pos2(rect.max.x, rect.max.y - 3.0),
            );
            let bot = egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.max.y - 3.0), rect.max);
            painter.rect_filled(top, round, egui::Color32::from_rgb(198, 29, 38));
            painter.rect_filled(
                mid,
                egui::CornerRadius::ZERO,
                egui::Color32::from_rgb(255, 205, 0),
            );
            painter.rect_filled(bot, round, egui::Color32::from_rgb(198, 29, 38));
        }
        UiLanguage::En => {
            painter.rect_filled(rect, round, egui::Color32::from_rgb(191, 10, 48));
            for i in 0..6 {
                let y = rect.min.y + i as f32 * 2.0;
                let stripe = egui::Rect::from_min_max(
                    egui::pos2(rect.min.x, y),
                    egui::pos2(rect.max.x, (y + 1.0).min(rect.max.y)),
                );
                painter.rect_filled(stripe, egui::CornerRadius::ZERO, egui::Color32::WHITE);
            }
            let canton =
                egui::Rect::from_min_max(rect.min, egui::pos2(rect.min.x + 8.0, rect.min.y + 6.5));
            painter.rect_filled(canton, round, egui::Color32::from_rgb(10, 49, 97));
        }
        UiLanguage::Pt => {
            let left = egui::Rect::from_min_max(rect.min, egui::pos2(rect.min.x + 7.0, rect.max.y));
            let right =
                egui::Rect::from_min_max(egui::pos2(rect.min.x + 7.0, rect.min.y), rect.max);
            painter.rect_filled(left, round, egui::Color32::from_rgb(0, 156, 59));
            painter.rect_filled(right, round, egui::Color32::from_rgb(255, 223, 0));
            painter.circle_filled(rect.center(), 2.2, egui::Color32::from_rgb(0, 39, 118));
        }
        UiLanguage::Fr => {
            let w = rect.width() / 3.0;
            painter.rect_filled(
                egui::Rect::from_min_max(rect.min, egui::pos2(rect.min.x + w, rect.max.y)),
                round,
                egui::Color32::from_rgb(0, 85, 164),
            );
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(rect.min.x + w, rect.min.y),
                    egui::pos2(rect.min.x + 2.0 * w, rect.max.y),
                ),
                egui::CornerRadius::ZERO,
                egui::Color32::WHITE,
            );
            painter.rect_filled(
                egui::Rect::from_min_max(egui::pos2(rect.min.x + 2.0 * w, rect.min.y), rect.max),
                round,
                egui::Color32::from_rgb(239, 65, 53),
            );
        }
        UiLanguage::De => {
            let h = rect.height() / 3.0;
            painter.rect_filled(
                egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.min.y + h)),
                round,
                egui::Color32::BLACK,
            );
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(rect.min.x, rect.min.y + h),
                    egui::pos2(rect.max.x, rect.min.y + 2.0 * h),
                ),
                egui::CornerRadius::ZERO,
                egui::Color32::from_rgb(221, 0, 0),
            );
            painter.rect_filled(
                egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y + 2.0 * h), rect.max),
                round,
                egui::Color32::from_rgb(255, 206, 0),
            );
        }
        UiLanguage::Ja => {
            painter.rect_filled(rect, round, egui::Color32::WHITE);
            painter.circle_filled(rect.center(), 3.0, egui::Color32::from_rgb(188, 0, 45));
        }
        UiLanguage::ZhHans => {
            painter.rect_filled(rect, round, egui::Color32::from_rgb(222, 41, 16));
            painter.circle_filled(
                egui::pos2(rect.min.x + 4.0, rect.min.y + 3.5),
                1.4,
                egui::Color32::from_rgb(255, 222, 0),
            );
        }
        UiLanguage::Ru => {
            let h = rect.height() / 3.0;
            painter.rect_filled(
                egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.min.y + h)),
                round,
                egui::Color32::WHITE,
            );
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(rect.min.x, rect.min.y + h),
                    egui::pos2(rect.max.x, rect.min.y + 2.0 * h),
                ),
                egui::CornerRadius::ZERO,
                egui::Color32::from_rgb(0, 57, 166),
            );
            painter.rect_filled(
                egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y + 2.0 * h), rect.max),
                round,
                egui::Color32::from_rgb(213, 43, 30),
            );
        }
        UiLanguage::Ar => {
            let h = rect.height() / 3.0;
            painter.rect_filled(
                egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.min.y + h)),
                round,
                egui::Color32::from_rgb(0, 108, 53),
            );
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(rect.min.x, rect.min.y + h),
                    egui::pos2(rect.max.x, rect.min.y + 2.0 * h),
                ),
                egui::CornerRadius::ZERO,
                egui::Color32::WHITE,
            );
            painter.rect_filled(
                egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y + 2.0 * h), rect.max),
                round,
                egui::Color32::from_rgb(0, 0, 0),
            );
        }
        UiLanguage::Hi => {
            let h = rect.height() / 3.0;
            painter.rect_filled(
                egui::Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.min.y + h)),
                round,
                egui::Color32::from_rgb(255, 153, 51),
            );
            painter.rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(rect.min.x, rect.min.y + h),
                    egui::pos2(rect.max.x, rect.min.y + 2.0 * h),
                ),
                egui::CornerRadius::ZERO,
                egui::Color32::WHITE,
            );
            painter.rect_filled(
                egui::Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y + 2.0 * h), rect.max),
                round,
                egui::Color32::from_rgb(19, 136, 8),
            );
            painter.circle_stroke(
                rect.center(),
                1.7,
                egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 0, 128)),
            );
        }
    }

    painter.rect_stroke(
        rect,
        round,
        egui::Stroke::new(1.0, BrandColors::line()),
        egui::StrokeKind::Outside,
    );
}

pub fn hero_logo(ui: &mut egui::Ui, texture: &egui::TextureHandle) {
    let desired = egui::vec2(56.0, 56.0);
    let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
    let painter = ui.painter_at(rect);
    let rounding = egui::CornerRadius::same(16);

    painter.rect_filled(rect, rounding, egui::Color32::from_rgb(10, 16, 30));
    painter.rect_stroke(
        rect,
        rounding,
        egui::Stroke::new(1.0, BrandColors::line()),
        egui::StrokeKind::Inside,
    );

    let glow_rect = rect.expand2(egui::vec2(2.0, 2.0));
    painter.rect_stroke(
        glow_rect,
        rounding,
        egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(
                BrandColors::VIOLET.r(),
                BrandColors::VIOLET.g(),
                BrandColors::VIOLET.b(),
                50,
            ),
        ),
        egui::StrokeKind::Outside,
    );

    let image_rect = rect.shrink2(egui::vec2(6.0, 6.0));
    ui.put(
        image_rect,
        egui::Image::new((texture.id(), image_rect.size()))
            .corner_radius(egui::CornerRadius::same(12)),
    );

    let badge_rect = egui::Rect::from_min_size(
        egui::pos2(rect.right() - 16.0, rect.top() - 4.0),
        egui::vec2(18.0, 18.0),
    );
    painter.rect_filled(
        badge_rect,
        egui::CornerRadius::same(9),
        egui::Color32::from_rgb(16, 34, 54),
    );
    painter.rect_stroke(
        badge_rect,
        egui::CornerRadius::same(9),
        egui::Stroke::new(
            1.0,
            egui::Color32::from_rgba_unmultiplied(
                BrandColors::CYAN.r(),
                BrandColors::CYAN.g(),
                BrandColors::CYAN.b(),
                80,
            ),
        ),
        egui::StrokeKind::Inside,
    );
    painter.text(
        badge_rect.center(),
        egui::Align2::CENTER_CENTER,
        "AI",
        egui::FontId::proportional(9.5),
        BrandColors::CYAN,
    );
}

pub fn output_style_name(index: usize, ui_language: UiLanguage) -> &'static str {
    match index {
        1 => ui_language.text("executive_format"),
        2 => ui_language.text("bullets_format"),
        _ => ui_language.text("chat_format"),
    }
}

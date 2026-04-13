use eframe::{App, Frame, egui};

use super::*;
use crate::ui::{
    components::top_bar_frame,
    theme::{BrandColors, LayoutSpace},
};

impl App for YoutubeNativeApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.handle_background_messages();

        if self.copy_feedback_started_at.is_some() && !self.copy_feedback_active() {
            self.copy_feedback_started_at = None;
        }

        if self.busy {
            ctx.request_repaint();
        }

        if self.copy_feedback_active() {
            ctx.request_repaint_after(Duration::from_millis(16));
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        let visuals = ui.visuals_mut();
        *visuals = egui::Visuals::dark();
        visuals.override_text_color = Some(BrandColors::TEXT);
        visuals.panel_fill = BrandColors::BG;
        visuals.extreme_bg_color = BrandColors::BG_SOFT;
        visuals.faint_bg_color = BrandColors::PANEL;
        visuals.code_bg_color = BrandColors::BG_SOFT;
        visuals.window_fill = BrandColors::BG;
        visuals.hyperlink_color = BrandColors::CYAN;
        visuals.selection.bg_fill = BrandColors::VIOLET;
        visuals.selection.stroke.color = BrandColors::TEXT;
        visuals.window_stroke = egui::Stroke::new(1.0, BrandColors::line());
        visuals.widgets.noninteractive.bg_fill = BrandColors::PANEL;
        visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, BrandColors::line());
        visuals.widgets.noninteractive.fg_stroke.color = BrandColors::TEXT;
        visuals.widgets.inactive.bg_fill = BrandColors::PANEL_STRONG;
        visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, BrandColors::line());
        visuals.widgets.inactive.fg_stroke.color = BrandColors::TEXT;
        visuals.widgets.hovered.bg_fill = BrandColors::VIOLET;
        visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, BrandColors::line_strong());
        visuals.widgets.hovered.fg_stroke.color = BrandColors::TEXT;
        visuals.widgets.active.bg_fill = BrandColors::PINK;
        visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, BrandColors::line_strong());
        visuals.widgets.active.fg_stroke.color = BrandColors::TEXT;
        visuals.widgets.open.bg_fill = BrandColors::PANEL;
        visuals.widgets.open.bg_stroke = egui::Stroke::new(1.0, BrandColors::line_strong());
        visuals.widgets.open.fg_stroke.color = BrandColors::TEXT;
        visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(14);
        visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(14);
        visuals.widgets.active.corner_radius = egui::CornerRadius::same(14);
        visuals.widgets.open.corner_radius = egui::CornerRadius::same(14);

        let style = ui.style_mut();
        style.spacing.button_padding = egui::vec2(LayoutSpace::MD, LayoutSpace::SM);
        style.spacing.item_spacing = egui::vec2(LayoutSpace::SM, LayoutSpace::SM);
        style.spacing.window_margin = egui::Margin::same(16);
        let mut scroll_style = egui::style::ScrollStyle::solid();
        scroll_style.bar_width = 6.0;
        scroll_style.bar_inner_margin = 2.0;
        scroll_style.handle_min_length = 28.0;
        style.spacing.scroll = scroll_style;

        let rect = ui.max_rect();
        let builder = egui::UiBuilder::new().max_rect(rect);
        let mut root_ui = ui.new_child(builder);

        egui::Panel::top("top_bar")
            .exact_size(76.0)
            .frame(top_bar_frame())
            .show_inside(&mut root_ui, |ui| self.render_top_bar(ui));

        egui::Panel::right("chat_panel")
            .resizable(false)
            .default_size(400.0)
            .min_size(400.0)
            .max_size(400.0)
            .frame(
                egui::Frame::default()
                    .fill(BrandColors::BG)
                    .inner_margin(egui::Margin {
                        left: 20,
                        right: 20,
                        top: 8,
                        bottom: 24,
                    }),
            )
            .show_inside(&mut root_ui, |ui| self.render_chat_panel(ui));

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(BrandColors::BG)
                    .inner_margin(egui::Margin {
                        left: 20,
                        right: 20,
                        top: 8,
                        bottom: 24,
                    }),
            )
            .show_inside(&mut root_ui, |ui| self.render_main_panel(ui));
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        super::actions::save_persisted_settings(storage, &self.persisted_settings());
    }
}

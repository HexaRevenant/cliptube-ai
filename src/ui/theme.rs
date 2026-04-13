use eframe::egui;

pub struct BrandColors;

impl BrandColors {
    pub const BG: egui::Color32 = egui::Color32::from_rgb(7, 11, 20);
    pub const BG_SOFT: egui::Color32 = egui::Color32::from_rgb(12, 18, 32);
    pub const PANEL: egui::Color32 = egui::Color32::from_rgb(17, 23, 40);
    pub const PANEL_STRONG: egui::Color32 = egui::Color32::from_rgb(13, 19, 34);
    pub const TEXT: egui::Color32 = egui::Color32::from_rgb(245, 247, 255);
    pub const MUTED: egui::Color32 = egui::Color32::from_rgb(152, 164, 195);
    pub const PINK: egui::Color32 = egui::Color32::from_rgb(255, 76, 118);
    pub const CYAN: egui::Color32 = egui::Color32::from_rgb(55, 212, 255);
    pub const VIOLET: egui::Color32 = egui::Color32::from_rgb(116, 92, 255);

    pub fn line() -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 26)
    }

    pub fn line_strong() -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 40)
    }
}

pub struct LayoutSpace;

impl LayoutSpace {
    pub const XS: f32 = 6.0;
    pub const SM: f32 = 10.0;
    pub const MD: f32 = 14.0;
    pub const LG: f32 = 18.0;
    pub const XL: f32 = 24.0;
    pub const CARD_RADIUS: u8 = 22;
    pub const INPUT_RADIUS: u8 = 14;
    pub const CHAT_MESSAGES_HEIGHT: f32 = 360.0;
    pub const CHAT_COMPOSER_HEIGHT: f32 = 152.0;
}

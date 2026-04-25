mod ai;
mod app;
mod db;
mod history;
mod transcript;
mod transcript_helpers;
mod ui;

use app::YoutubeNativeApp;
use eframe::{NativeOptions, egui};

fn main() -> Result<(), eframe::Error> {
    let icon = load_embedded_svg_icon();

    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("ClipTube AI")
            .with_app_id("io.github.cliptubeai.ClipTubeAI")
            .with_inner_size([960.0, 820.0])
            .with_min_inner_size([760.0, 620.0])
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "ClipTube AI",
        options,
        Box::new(|cc| Ok(Box::new(YoutubeNativeApp::new(cc)))),
    )
}

fn load_embedded_svg_icon() -> egui::viewport::IconData {
    let (rgba, width, height) = render_embedded_svg_rgba();

    egui::viewport::IconData {
        rgba,
        width,
        height,
    }
}

fn render_embedded_svg_rgba() -> (Vec<u8>, u32, u32) {
    let svg_data = include_bytes!("../assets/icon.svg");
    let tree = resvg::usvg::Tree::from_data(svg_data, &resvg::usvg::Options::default())
        .expect("embedded application icon must be a valid SVG");
    let size = tree.size().to_int_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(size.width(), size.height())
        .expect("application icon SVG must have a renderable size");

    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::default(),
        &mut pixmap.as_mut(),
    );
    (pixmap.data().to_vec(), size.width(), size.height())
}

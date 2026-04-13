use eframe::egui;

pub(super) fn load_embedded_svg_color_image() -> egui::ColorImage {
    let (rgba, width, height) = render_embedded_svg_rgba();
    egui::ColorImage::from_rgba_unmultiplied([width as usize, height as usize], &rgba)
}

fn render_embedded_svg_rgba() -> (Vec<u8>, u32, u32) {
    let svg_data = include_bytes!("../../assets/icon.svg");
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

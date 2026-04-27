mod app;
mod config;
mod engine;
mod i18n;
mod theme;
mod ui;

use app::LlamaLunchApp;
use theme::load_custom_fonts;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(1100.0, 700.0))
            .with_title("llama.cpp lunch"),
        ..Default::default()
    };

    eframe::run_native(
        "llama.cpp lunch",
        options,
        Box::new(|cc| {
            let fonts = load_custom_fonts();
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(LlamaLunchApp::new(&cc)))
        }),
    )
}

#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;
mod config;
mod engine;
mod i18n;
mod theme;
mod ui;

use app::LlamaLunchApp;
use theme::load_custom_fonts;
use std::sync::Arc;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(1100.0, 700.0))
            .with_title("llama.cpp lunch")
            .with_icon(load_icon("llama-blue.ico")),
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

/// 读取 ICO 文件并构造 IconData
fn load_icon(path: &str) -> Arc<egui::IconData> {
    if let Some(icon) = load_icon_inner(path) {
        icon
    } else {
        egui::IconData::default().into()
    }
}

/// 内部读取函数
fn load_icon_inner(path: &str) -> Option<Arc<egui::IconData>> {
    let file = std::fs::File::open(path).ok()?;
    let reader = std::io::BufReader::new(file);
    let img = image::ImageReader::new(reader)
        .decode().ok()?
        .into_rgba8();
    let (width, height) = img.dimensions();
    let rgba = img.into_raw();
    Some(Arc::new(egui::IconData {
        rgba,
        width,
        height,
    }))
}

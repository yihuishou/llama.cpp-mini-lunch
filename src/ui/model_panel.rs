use crate::config::settings::AppSettings;
use std::fs;

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings) {
    ui.heading("模型管理");
    ui.separator();

    // 当前模型
    ui.horizontal(|ui| {
        ui.label("GGUF 模型文件:");
        if ui.button("浏览...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_title("选择 GGUF 模型文件")
                .add_filter("GGUF 模型", &["gguf"])
                .pick_file()
            {
                settings.model_path = path;
            }
        }
    });

    let mut model_path_str = settings.model_path.to_string_lossy().to_string();
    let response = ui.text_edit_singleline(&mut model_path_str);
    if response.changed() {
        settings.model_path = std::path::PathBuf::from(&model_path_str);
    }

    ui.add_space(8.0);

    // 模型信息
    if !settings.model_path.as_os_str().is_empty() {
        if let Ok(metadata) = fs::metadata(&settings.model_path) {
            let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);
            ui.label(format!(
                "文件: {} ( {:.2} MB )",
                settings.model_path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default(),
                size_mb
            ));
        } else {
            ui.colored_label(egui::Color32::RED, "文件不存在或无法访问");
        }
    } else {
        ui.colored_label(egui::Color32::RED, "尚未选择模型文件");
    }
}

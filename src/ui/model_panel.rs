use crate::config::settings::AppSettings;
use crate::i18n;
use std::fs;

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings, lang: &i18n::Language) {
    ui.heading(i18n::t(i18n::Key::PanelModelTitle, lang));
    ui.separator();

    // 当前模型
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelModelPath, lang));
        if ui.button(i18n::t(i18n::Key::BtnBrowse, lang)).clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_title(i18n::t(i18n::Key::DialogSelectModel, lang))
                .add_filter(i18n::t(i18n::Key::FilterGguf, lang), &["gguf"])
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
            let filename = settings.model_path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            ui.label(format!("{}: {} ({:.2} MB)", i18n::t(i18n::Key::LabelFile, lang), filename, size_mb));
        } else {
            ui.colored_label(egui::Color32::RED, i18n::t(i18n::Key::ModelNotExist, lang));
        }
    } else {
        ui.colored_label(egui::Color32::RED, i18n::t(i18n::Key::ModelNotSelected, lang));
    }

    // 多模态投影文件
    ui.add_space(12.0);
    ui.heading(i18n::t(i18n::Key::SectionMultimodal, lang));
    ui.separator();

    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelMmprojPath, lang));
        if ui.button(i18n::t(i18n::Key::BtnBrowse, lang)).clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_title(i18n::t(i18n::Key::DialogSelectMmproj, lang))
                .add_filter(i18n::t(i18n::Key::FilterMmproj, lang), &["gguf", "bin"])
                .pick_file()
            {
                settings.mmproj_path = path;
            }
        }
    });

    let mut mmproj_path_str = settings.mmproj_path.to_string_lossy().to_string();
    let response = ui.text_edit_singleline(&mut mmproj_path_str);
    if response.changed() {
        settings.mmproj_path = std::path::PathBuf::from(&mmproj_path_str);
    }

}

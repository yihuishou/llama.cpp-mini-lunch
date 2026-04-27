use crate::config::settings::{AppSettings, SettingsManager};
use crate::i18n;

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings, settings_manager: &SettingsManager, lang: &i18n::Language) {
    ui.heading(i18n::t(i18n::Key::PanelServerTitle, lang));
    ui.separator();

    // 二进制路径
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelServerPath, lang));
        if ui.button(i18n::t(i18n::Key::BtnBrowse, lang)).clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_title(i18n::t(i18n::Key::DialogSelectServer, lang))
                .add_filter(i18n::t(i18n::Key::FilterExecutable, lang), &["exe"])
                .pick_file()
            {
                settings.server_path = path;
            }
        }
        if ui.button(i18n::t(i18n::Key::BtnAutoDetect, lang)).clicked() {
            if let Some(path) = settings_manager.auto_detect_server_path() {
                settings.server_path = path;
            } else {
                settings.server_path = std::path::PathBuf::from("");
            }
        }
    });
    let mut server_path_str = settings.server_path.to_string_lossy().to_string();
    let response = ui.text_edit_singleline(&mut server_path_str);
    if response.changed() {
        settings.server_path = std::path::PathBuf::from(&server_path_str);
    }

    ui.add_space(8.0);

    // 监听地址
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelHost, lang));
        ui.text_edit_singleline(&mut settings.host);
        ui.label(i18n::t(i18n::Key::LabelPort, lang));
        ui.add(egui::DragValue::new(&mut settings.port).range(1..=65535));
    });

    ui.add_space(8.0);

    // 并行槽位
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelParallelSlots, lang));
        ui.add(egui::DragValue::new(&mut settings.parallel_slots).range(1..=32));
    });

    // GPU 层数
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelGpuLayers, lang));
        ui.text_edit_singleline(&mut settings.gpu_layers_str);
        ui.small(i18n::t(i18n::Key::HintGpuLayers, lang));
    });

    ui.add_space(8.0);
    ui.checkbox(&mut settings.verbose, i18n::t(i18n::Key::CheckboxVerbose, lang));
}

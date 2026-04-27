use crate::config::settings::{AppSettings, SettingsManager};
use crate::i18n;

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings, settings_manager: &SettingsManager, lang: &i18n::Language) {
    ui.heading(i18n::t(i18n::Key::PanelRpcTitle, lang));
    ui.separator();

    // rpc-server.exe 路径
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelRpcPath, lang));
        if ui.button(i18n::t(i18n::Key::BtnBrowse, lang)).clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_title(i18n::t(i18n::Key::DialogSelectRpc, lang))
                .add_filter(i18n::t(i18n::Key::FilterExecutable, lang), &["exe"])
                .pick_file()
            {
                settings.rpc_server_path = path;
            }
        }
        if ui.button(i18n::t(i18n::Key::BtnAutoDetect, lang)).clicked() {
            if let Some(path) = settings_manager.auto_detect_rpc_path() {
                settings.rpc_server_path = path;
            } else {
                settings.rpc_server_path = std::path::PathBuf::from("");
            }
        }
    });
    let mut rpc_path_str = settings.rpc_server_path.to_string_lossy().to_string();
    let response = ui.text_edit_singleline(&mut rpc_path_str);
    if response.changed() {
        settings.rpc_server_path = std::path::PathBuf::from(&rpc_path_str);
    }

    // 路径验证提示
    if !settings.rpc_server_path.as_os_str().is_empty() {
        let exists = settings.rpc_server_path.exists();
        let (icon, color) = if exists { ("✓", egui::Color32::from_rgb(110, 255, 140)) }
            else { ("✗", egui::Color32::from_rgb(255, 100, 100)) };
        ui.colored_label(color, format!("{} {}", icon, if exists { i18n::t(i18n::Key::FileExists, lang) } else { i18n::t(i18n::Key::FileNotExists, lang) }));
    }

    ui.add_space(8.0);

    // 监听地址
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelHost, lang));
        ui.text_edit_singleline(&mut settings.rpc_host);
        ui.label(i18n::t(i18n::Key::LabelPort, lang));
        ui.add(egui::DragValue::new(&mut settings.rpc_port).range(1..=65535));
    });

    ui.add_space(8.0);

    ui.add_space(8.0);

    // 线程数
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelRpcThreads, lang));
        ui.add(egui::DragValue::new(&mut settings.rpc_threads).range(1..=128));
        ui.small(i18n::t(i18n::Key::HintRpcThreads, lang));
    });

    // 设备列表
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelRpcDevice, lang));
        ui.text_edit_singleline(&mut settings.rpc_device);
        ui.small(i18n::t(i18n::Key::HintRpcDevice, lang));
    });

    // 本地缓存
    ui.horizontal(|ui| {
        ui.checkbox(&mut settings.rpc_cache, i18n::t(i18n::Key::CheckboxRpcCache, lang));
    });

  }

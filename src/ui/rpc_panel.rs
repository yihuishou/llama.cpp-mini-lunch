use crate::config::settings::{AppSettings, SettingsManager};

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings, settings_manager: &SettingsManager) {
    ui.heading("RPC 配置");
    ui.separator();

    // rpc-server.exe 路径
    ui.horizontal(|ui| {
        ui.label("rpc-server 路径:");
        if ui.button("浏览...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_title("选择 rpc-server 可执行文件")
                .add_filter("可执行文件", &["exe"])
                .pick_file()
            {
                settings.rpc_server_path = path;
            }
        }
        if ui.button("自动检测").clicked() {
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
        ui.colored_label(color, format!("{} {}", icon, if exists { "文件存在" } else { "文件不存在" }));
    }

    ui.add_space(8.0);

    // 监听地址
    ui.horizontal(|ui| {
        ui.label("主机:");
        ui.text_edit_singleline(&mut settings.rpc_host);
        ui.label("端口:");
        ui.add(egui::DragValue::new(&mut settings.rpc_port).range(1..=65535));
    });

    ui.add_space(8.0);

    ui.add_space(8.0);

    // 线程数
    ui.horizontal(|ui| {
        ui.label("CPU 线程数:");
        ui.add(egui::DragValue::new(&mut settings.rpc_threads).range(1..=128));
        ui.small("(默认: 12)");
    });

    // 设备列表
    ui.horizontal(|ui| {
        ui.label("设备:");
        ui.text_edit_singleline(&mut settings.rpc_device);
        ui.small("逗号分隔，如: 0,1");
    });

    // 本地缓存
    ui.horizontal(|ui| {
        ui.checkbox(&mut settings.rpc_cache, "启用本地文件缓存");
    });

  }

use crate::config::settings::AppSettings;

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings) {
    ui.heading("Server 配置");
    ui.separator();

    // 二进制路径
    ui.horizontal(|ui| {
        ui.label("llama-server 路径:");
        if ui.button("浏览...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_title("选择 llama-server 可执行文件")
                .add_filter("可执行文件", &["exe"])
                .pick_file()
            {
                settings.server_path = path;
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
        ui.label("主机:");
        ui.text_edit_singleline(&mut settings.host);
        ui.label("端口:");
        ui.add(egui::DragValue::new(&mut settings.port).range(1..=65535));
    });

    ui.add_space(8.0);

    // 并行槽位
    ui.horizontal(|ui| {
        ui.label("并发数量:");
        ui.add(egui::DragValue::new(&mut settings.parallel_slots).range(1..=32));
    });

    // GPU 层数
    ui.horizontal(|ui| {
        ui.label("GPU 层数 (n-gpu-layers):");
        ui.text_edit_singleline(&mut settings.gpu_layers_str);
        ui.small("(数字/自动/全部)");
    });

    ui.add_space(8.0);
    ui.checkbox(&mut settings.verbose, "详细输出 (verbose)");
}

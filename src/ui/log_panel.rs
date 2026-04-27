use crate::engine::server::{LogLevel, ServerManager};

pub fn ui(ui: &mut egui::Ui, manager: &mut ServerManager) {
    ui.heading("运行日志");
    ui.separator();

    ui.horizontal(|ui| {
        if ui.small_button("清空日志").clicked() {
            manager.clear_logs();
        }
        ui.small("日志仅在当前会话中保留");
    });

    ui.add_space(8.0);

    egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
        let logs = manager.logs();
        if logs.is_empty() {
            ui.add_space(20.0);
            ui.horizontal_centered(|ui| {
                ui.colored_label(egui::Color32::GRAY, "暂无日志输出");
            });
        } else {
            for entry in &logs {
                let color = match entry.level {
                    LogLevel::Info => egui::Color32::LIGHT_GRAY,
                    LogLevel::Warn => egui::Color32::YELLOW,
                    LogLevel::Error => egui::Color32::RED,
                };

                let prefix = match entry.level {
                    LogLevel::Info => "",
                    LogLevel::Warn => "⚠ ",
                    LogLevel::Error => "✖ ",
                };

                ui.horizontal_wrapped(|ui| {
                    ui.colored_label(color, format!("{}{}", prefix, entry.text));
                });
            }
        }
    });
}

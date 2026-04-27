use crate::config::settings::{AppSettings, Preset, SettingsManager};

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings, manager: &mut SettingsManager) {
    ui.heading("配置预设");
    ui.separator();

    // 保存预设
    let mut name = String::new();
    ui.horizontal(|ui| {
        ui.label("预设名称:");
        ui.text_edit_singleline(&mut name);
        if ui.button("保存当前配置").clicked() {
            if !name.is_empty() {
                let preset = Preset {
                    name: name.clone(),
                    settings: settings.clone(),
                };
                if let Err(e) = manager.save_preset(&preset) {
                    log::error!("保存预设失败: {}", e);
                }
            }
        }
    });

    ui.add_space(12.0);
    ui.heading("预设列表");
    ui.separator();

    let presets = manager.list_presets();

    if presets.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.add_space(20.0);
            ui.label("暂无预设配置");
        });
        return;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        for preset in &presets {
            ui.horizontal(|ui| {
                ui.label(&preset.name);

                if ui.small_button("加载").clicked() {
                    *settings = preset.settings.clone();
                }

                if ui.small_button("删除").clicked() {
                    if let Err(e) = manager.delete_preset(&preset.name) {
                        log::error!("删除预设失败: {}", e);
                    }
                }
            });
        }
    });
}

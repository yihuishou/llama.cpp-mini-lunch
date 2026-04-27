use crate::config::settings::{AppSettings, Preset, SettingsManager};
use crate::i18n;

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings, manager: &mut SettingsManager, lang: &i18n::Language) {
    ui.heading(i18n::t(i18n::Key::PanelPresetTitle, lang));
    ui.separator();

    let mut name = String::new();
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelPresetName, lang));
        ui.text_edit_singleline(&mut name);
        if ui.button(i18n::t(i18n::Key::BtnSavePreset, lang)).clicked() {
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
    ui.heading(i18n::t(i18n::Key::SectionPresetList, lang));
    ui.separator();

    let presets = manager.list_presets();

    if presets.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.add_space(20.0);
            ui.label(i18n::t(i18n::Key::HintNoPresets, lang));
        });
        return;
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        for preset in &presets {
            ui.horizontal(|ui| {
                ui.label(&preset.name);

                if ui.small_button(i18n::t(i18n::Key::BtnLoad, lang)).clicked() {
                    *settings = preset.settings.clone();
                }

                if ui.small_button(i18n::t(i18n::Key::BtnDelete, lang)).clicked() {
                    if let Err(e) = manager.delete_preset(&preset.name) {
                        log::error!("删除预设失败: {}", e);
                    }
                }
            });
        }
    });
}

use crate::engine::server::{LogLevel, ServerManager};
use crate::i18n;

pub fn ui(ui: &mut egui::Ui, manager: &mut ServerManager, lang: &i18n::Language) {
    ui.heading(i18n::t(i18n::Key::PanelLogTitle, lang));
    ui.separator();

    ui.horizontal(|ui| {
        if ui.small_button(i18n::t(i18n::Key::BtnClearLogs, lang)).clicked() {
            manager.clear_logs();
        }
        ui.small(i18n::t(i18n::Key::HintLogSession, lang));
    });

    ui.add_space(8.0);

    egui::ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
        let logs = manager.logs();
        if logs.is_empty() {
            ui.add_space(20.0);
            ui.horizontal_centered(|ui| {
                ui.colored_label(egui::Color32::GRAY, i18n::t(i18n::Key::HintNoLogs, lang));
            });
        } else {
            for entry in &logs {
                let prefix = match entry.level {
                    LogLevel::Info => "",
                    LogLevel::Warn => "⚠ ",
                    LogLevel::Error => "✖ ",
                };

                let text = format!("{}{}", prefix, entry.text);

                ui.horizontal_wrapped(|ui| {
                    match entry.level {
                        LogLevel::Info => {
                            ui.colored_label(egui::Color32::LIGHT_GRAY, &text);
                        }
                        LogLevel::Warn => {
                            egui::Frame::default()
                                .fill(egui::Color32::from_rgb(80, 80, 80))
                                .inner_margin(egui::Margin::same(4.0))
                                .show(ui, |ui| {
                                    ui.colored_label(egui::Color32::YELLOW, &text);
                                });
                        }
                        LogLevel::Error => {
                            ui.colored_label(egui::Color32::RED, &text);
                        }
                    }
                });
            }
        }
    });
}

use crate::engine::rpc::RpcManager;
use crate::engine::server::ServerManager;
use crate::i18n;

pub fn ui(ui: &mut egui::Ui, server: &ServerManager, rpc: &RpcManager, lang: &i18n::Language) {
    ui.heading(i18n::t(i18n::Key::SectionLaunchCommands, lang));
    ui.add_space(4.0);

    // Server 启动命令
    ui.label(i18n::t(i18n::Key::LabelServerCommand, lang));
    if let Some(ref cmd) = server.launch_command() {
        ui.monospace(cmd);
    } else {
        ui.colored_label(egui::Color32::GRAY, i18n::t(i18n::Key::HintNoCommand, lang));
    }
    ui.add_space(8.0);

    // RPC 启动命令
    ui.label(i18n::t(i18n::Key::LabelRpcCommand, lang));
    if let Some(ref cmd) = rpc.launch_command() {
        ui.monospace(cmd);
    } else {
        ui.colored_label(egui::Color32::GRAY, i18n::t(i18n::Key::HintNoCommand, lang));
    }
}

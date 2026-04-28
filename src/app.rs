use crate::config::settings::{AppSettings, SettingsManager};
use crate::engine::rpc::{RpcManager, RpcState};
use crate::engine::server::{ServerManager, ServerState};
use crate::i18n::{self, Language};
use crate::theme::{ThemeManager, ThemeVariant};
use crate::ui::{log_panel, model_panel, params_panel, rpc_panel, server_panel};

pub struct LlamaLunchApp {
    settings: AppSettings,
    settings_manager: SettingsManager,
    server_manager: ServerManager,
    rpc_manager: RpcManager,
    tab_selected: String,
    show_about: bool,
    theme_variant: ThemeVariant,
    theme_manager: ThemeManager,
    lang: Language,
}

impl LlamaLunchApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings_manager = SettingsManager::new();
        let settings = settings_manager.load().unwrap_or_default();
        let locale = sys_locale::get_locale().unwrap_or_default();
        let lang = if locale.starts_with("zh") {
            Language::Zh
        } else {
            Language::En
        };

        let server_manager = ServerManager::new();
        let rpc_manager = RpcManager::new();
        let mut theme_manager = ThemeManager::new();
        theme_manager.init(&cc.egui_ctx);

        // 全局 UI 放大 1.5 倍
        cc.egui_ctx.set_zoom_factor(1.5);

        Self {
            settings,
            settings_manager,
            server_manager,
            rpc_manager,
            tab_selected: "Server".to_string(),
            show_about: false,
            theme_variant: ThemeVariant::Latte,
            theme_manager,
            lang,
        }
    }

    fn save(&mut self) {
        if let Err(e) = self.settings_manager.save(&self.settings) {
            log::error!("保存配置失败: {}", e);
        }
    }

    fn render_server_controls(&mut self, ui: &mut egui::Ui) {
        let server_state = self.server_manager.state();
        let start_fill = egui::Color32::from_rgb(40, 120, 40);
        let stop_fill = egui::Color32::from_rgb(180, 50, 50);
        match server_state {
            ServerState::Idle | ServerState::Error(_) => {
                let can_start = !self.settings.server_path.as_os_str().is_empty()
                    && !self.settings.model_path.as_os_str().is_empty();
                if ui
                    .add_enabled(can_start, egui::Button::new(i18n::t(i18n::Key::BtnStartServer, &self.lang)).fill(start_fill))
                    .clicked
                {
                    self.server_manager.start(&self.settings);
                }
            }
            ServerState::Running => {
                if ui.add(egui::Button::new(i18n::t(i18n::Key::BtnStopServer, &self.lang)).fill(stop_fill)).clicked {
                    self.server_manager.stop();
                }
            }
            ServerState::Starting | ServerState::Stopping => {
                ui.label(i18n::t(i18n::Key::StatusProcessing, &self.lang));
            }
        }
    }

    fn render_rpc_controls(&mut self, ui: &mut egui::Ui) {
        let rpc_state = self.rpc_manager.state();
        let rpc_start_fill = egui::Color32::from_rgb(40, 100, 140);
        let rpc_stop_fill = egui::Color32::from_rgb(180, 50, 50);
        match rpc_state {
            RpcState::Idle | RpcState::Error(_) => {
                if ui
                    .add_enabled(!self.settings.rpc_server_path.as_os_str().is_empty(), egui::Button::new(i18n::t(i18n::Key::BtnStartRpc, &self.lang)).fill(rpc_start_fill))
                    .clicked
                {
                    self.rpc_manager.start(&self.settings);
                }
            }
            RpcState::Running => {
                if ui.add(egui::Button::new(i18n::t(i18n::Key::BtnStopRpc, &self.lang)).fill(rpc_stop_fill)).clicked {
                    self.rpc_manager.stop();
                }
            }
            RpcState::Starting | RpcState::Stopping => {
                ui.label(i18n::t(i18n::Key::StatusProcessing, &self.lang));
            }
        }
    }
}

impl eframe::App for LlamaLunchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.server_manager.poll_logs();
        self.rpc_manager.poll();

        if self.show_about {
            egui::Window::new(i18n::t(i18n::Key::AboutTitle, &self.lang))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label(i18n::t(i18n::Key::AboutVersion, &self.lang));
                    ui.label(i18n::t(i18n::Key::AboutDescription, &self.lang));
                    if ui.button(i18n::t(i18n::Key::BtnClose, &self.lang)).clicked() {
                        self.show_about = false;
                    }
                });
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button(i18n::t(i18n::Key::MenuFile, &self.lang), |ui| {
                    if ui.button(i18n::t(i18n::Key::MenuItemSaveConfig, &self.lang)).clicked() {
                        self.save();
                    }
                    if ui.button(i18n::t(i18n::Key::MenuItemLoadConfig, &self.lang)).clicked() {
                        if let Ok(s) = self.settings_manager.load() {
                            self.settings = s;
                        }
                    }
                });

                // 标签页切换
                let tabs = [
                    i18n::t(i18n::Key::TabServer, &self.lang),
                    i18n::t(i18n::Key::TabRpc, &self.lang),
                    i18n::t(i18n::Key::TabModel, &self.lang),
                    i18n::t(i18n::Key::TabParams, &self.lang),
                    i18n::t(i18n::Key::TabLog, &self.lang),
                ];
                for tab in &tabs {
                    let selected = self.tab_selected == *tab;
                    if ui.selectable_label(selected, *tab).clicked() {
                        self.tab_selected = tab.to_string();
                    }
                }

                ui.separator();

                // 控制按钮
                self.render_server_controls(ui);
                self.render_rpc_controls(ui);

                ui.separator();

                ui.menu_button(i18n::t(i18n::Key::MenuTheme, &self.lang), |ui| {
                    for variant in ThemeVariant::all_variants() {
                        let selected = self.theme_variant == variant;
                        let label = if selected {
                            format!("◉ {}", variant.label())
                        } else {
                            format!("○ {}", variant.label())
                        };
                        if ui.button(label).clicked() {
                            self.theme_variant = variant;
                            self.theme_manager.apply(variant, ctx);
                        }
                    }
                });
                ui.menu_button(i18n::t(i18n::Key::MenuHelp, &self.lang), |ui| {
                    if ui.button(i18n::t(i18n::Key::MenuItemAbout, &self.lang)).clicked() {
                        self.show_about = true;
                    }
                });

                ui.separator();
                let status = self.server_manager.status_text(&self.lang);
                let color = if self.server_manager.is_running() {
                    egui::Color32::from_rgb(110, 255, 140)
                } else {
                    egui::Color32::GRAY
                };
                ui.colored_label(color, format!("[Server: {}]", status));
                let rpc_status = self.rpc_manager.status_text(&self.lang);
                let rpc_color = if self.rpc_manager.is_running() {
                    egui::Color32::from_rgb(110, 255, 140)
                } else {
                    egui::Color32::GRAY
                };
                ui.colored_label(rpc_color, format!("[RPC: {}]", rpc_status));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                match self.tab_selected.as_str() {
                    tab if tab == i18n::t(i18n::Key::TabServer, &self.lang) => server_panel::ui(ui, &mut self.settings, &self.settings_manager, &self.lang),
                    tab if tab == i18n::t(i18n::Key::TabRpc, &self.lang) => rpc_panel::ui(ui, &mut self.settings, &self.settings_manager, &self.lang),
                    tab if tab == i18n::t(i18n::Key::TabModel, &self.lang) => model_panel::ui(ui, &mut self.settings, &self.lang),
                  tab if tab == i18n::t(i18n::Key::TabParams, &self.lang) => params_panel::ui(ui, &mut self.settings, &self.lang),
                     tab if tab == i18n::t(i18n::Key::TabLog, &self.lang) => log_panel::ui(ui, &mut self.server_manager, &self.lang),

                    _ => { ui.label(i18n::t(i18n::Key::GenericSelectModule, &self.lang)); },
                }
            });
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }
}

impl Drop for LlamaLunchApp {
    fn drop(&mut self) {
        self.server_manager.stop();
        self.rpc_manager.stop();
        self.save();
    }
}

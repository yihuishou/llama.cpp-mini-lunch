use crate::config::settings::{AppSettings, SettingsManager};
use crate::engine::rpc::{RpcManager, RpcState};
use crate::engine::server::{ServerManager, ServerState};
use crate::theme::{ThemeManager, ThemeVariant};
use crate::ui::{log_panel, model_panel, params_panel, preset_panel, rpc_panel, server_panel};

pub struct LlamaLunchApp {
    settings: AppSettings,
    settings_manager: SettingsManager,
    server_manager: ServerManager,
    rpc_manager: RpcManager,
    tab_selected: String,
    show_about: bool,
    theme_variant: ThemeVariant,
    theme_manager: ThemeManager,
}

impl LlamaLunchApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let settings_manager = SettingsManager::new();
        let settings = settings_manager.load().unwrap_or_default();
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
                if ui
                    .add_enabled(!self.settings.model_path.as_os_str().is_empty(), egui::Button::new("启动 Server").fill(start_fill))
                    .clicked
                {
                    self.server_manager.start(&self.settings);
                }
            }
            ServerState::Running => {
                if ui.add(egui::Button::new("停止 Server").fill(stop_fill)).clicked {
                    self.server_manager.stop();
                }
            }
            ServerState::Starting | ServerState::Stopping => {
                ui.label("处理中...");
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
                    .add_enabled(!self.settings.rpc_server_path.as_os_str().is_empty(), egui::Button::new("启动 RPC").fill(rpc_start_fill))
                    .clicked
                {
                    self.rpc_manager.start(&self.settings);
                }
            }
            RpcState::Running => {
                if ui.add(egui::Button::new("停止 RPC").fill(rpc_stop_fill)).clicked {
                    self.rpc_manager.stop();
                }
            }
            RpcState::Starting | RpcState::Stopping => {
                ui.label("处理中...");
            }
        }
    }
}

impl eframe::App for LlamaLunchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.server_manager.poll_logs();
        self.rpc_manager.poll();

        if self.show_about {
            egui::Window::new("关于").collapsible(false).resizable(false).anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]).show(ctx, |ui| {
                ui.label("Llama Lunch v0.1.0");
                ui.label("llama-server 图形启动器");
                if ui.button("关闭").clicked() {
                    self.show_about = false;
                }
            });
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("文件", |ui| {
                    if ui.button("保存配置").clicked() {
                        self.save();
                    }
                    if ui.button("加载配置").clicked() {
                        if let Ok(s) = self.settings_manager.load() {
                            self.settings = s;
                        }
                    }
                });

                // 标签页切换
                let tabs = ["Server", "RPC", "模型", "参数", "预设", "日志"];
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

                ui.menu_button("主题", |ui| {
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
                ui.menu_button("帮助", |ui| {
                    if ui.button("关于").clicked() {
                        self.show_about = true;
                    }
                });

                ui.separator();
                let status = self.server_manager.status_text();
                let color = if self.server_manager.is_running() {
                    egui::Color32::from_rgb(110, 255, 140)
                } else {
                    egui::Color32::GRAY
                };
                ui.colored_label(color, format!("[Server: {}]", status));
                let rpc_status = self.rpc_manager.status_text();
                let rpc_color = if self.rpc_manager.is_running() {
                    egui::Color32::from_rgb(110, 255, 140)
                } else {
                    egui::Color32::GRAY
                };
                ui.colored_label(rpc_color, format!("[RPC: {}]", rpc_status));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.tab_selected.as_str() {
                "Server" => server_panel::ui(ui, &mut self.settings, &self.settings_manager),
                "RPC" => rpc_panel::ui(ui, &mut self.settings, &self.settings_manager),
                "模型" => model_panel::ui(ui, &mut self.settings),
                "参数" => params_panel::ui(ui, &mut self.settings),
                "预设" => preset_panel::ui(ui, &mut self.settings, &mut self.settings_manager),
                "日志" => log_panel::ui(ui, &mut self.server_manager),
                _ => { ui.label("请选择一个功能模块"); },
            }
        });

        ctx.request_repaint();
    }
}

impl Drop for LlamaLunchApp {
    fn drop(&mut self) {
        self.server_manager.stop();
        self.rpc_manager.stop();
        self.save();
    }
}

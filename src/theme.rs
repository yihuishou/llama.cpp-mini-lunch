use egui::FontData;
use egui_themes::{FRAPPE, LATTE, MACCHIATO, MOCHA, StateMachine, Theme};

/// Catppuccin 主题选项
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ThemeVariant {
    Mocha,
    Macchiato,
    Frappe,
    #[default]
    Latte,
}

impl ThemeVariant {
    /// 获取对应的 Theme 常量
    pub fn get_theme(&self) -> Theme {
        match self {
            ThemeVariant::Mocha => MOCHA,
            ThemeVariant::Macchiato => MACCHIATO,
            ThemeVariant::Frappe => FRAPPE,
            ThemeVariant::Latte => LATTE,
        }
    }

    /// 主题显示名称
    pub fn label(&self) -> &'static str {
        match self {
            ThemeVariant::Mocha => "Mocha",
            ThemeVariant::Macchiato => "Macchiato",
            ThemeVariant::Frappe => "Frappe",
            ThemeVariant::Latte => "Latte",
        }
    }

    /// 所有变体列表
    pub fn all_variants() -> [ThemeVariant; 4] {
        [
            ThemeVariant::Mocha,
            ThemeVariant::Macchiato,
            ThemeVariant::Frappe,
            ThemeVariant::Latte,
        ]
    }
}

/// 主题状态机，用于应用主题到 egui Context
pub struct ThemeManager {
    state_machine: StateMachine,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            state_machine: StateMachine::new(),
        }
    }

    /// 应用主题变体
    pub fn apply(&mut self, variant: ThemeVariant, ctx: &egui::Context) {
        let theme = variant.get_theme();
        self.state_machine.set_theme(ctx, &theme);
    }

    /// 初始化默认主题
    pub fn init(&mut self, ctx: &egui::Context) {
        self.apply(ThemeVariant::default(), ctx);
    }
}

/// 加载自定义字体
pub fn load_custom_fonts() -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();

    // 加载中文字体
    let cjk_font_path = if cfg!(target_os = "windows") {
        "C:\\Windows\\Fonts\\msyh.ttc"
    } else if cfg!(target_os = "macos") {
        "/System/Library/Fonts/PingFang.ttc"
    } else {
        "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc"
    };

    if let Ok(data) = std::fs::read(cjk_font_path) {
        let font_data = FontData::from_owned(data);
        let font_name = "CJK".to_string();
        fonts.font_data.insert(font_name.clone(), font_data);
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(1, font_name.clone());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(1, font_name);
    }

    fonts
}

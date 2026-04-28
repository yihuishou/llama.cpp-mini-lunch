use crate::config::settings::AppSettings;
use crate::i18n;

/// 文件名解析为彩色标签
fn parse_tags(filename: &str) -> Vec<(String, egui::Color32)> {
    let stem = filename
        .strip_suffix(".gguf")
        .unwrap_or(filename);

    let purple = egui::Color32::from_rgb(180, 120, 255);
    let orange = egui::Color32::from_rgb(255, 165, 0);
    let gray = egui::Color32::from_rgb(160, 160, 160);
    let green = egui::Color32::from_rgb(100, 200, 100);
    let blue = egui::Color32::from_rgb(100, 150, 255);

    let mut tags = Vec::new();
    for part in stem.split('-') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }

        let lower = trimmed.to_lowercase();
        let color = if lower.matches(|c: char| c.is_ascii_digit()).count() > 0
            && (lower.ends_with('b') || lower.ends_with('m') || lower.ends_with('k'))
        {
            purple
        } else if lower.starts_with('q') {
            orange
        } else if trimmed
            .chars()
            .all(|c| c.is_ascii_digit() || c == '.')
        {
            gray
        } else if lower.contains("instruct")
            || lower.contains("chat")
            || lower.contains("sft")
            || lower.contains("rlhf")
            || lower.contains("dpo")
            || lower.contains("orpo")
            || lower.contains("grpo")
        {
            green
        } else {
            blue
        };

        tags.push((trimmed.to_string(), color));
    }

    tags
}

/// 判断是否为 mmproj 文件
fn is_mmproj_file(filename: &str) -> bool {
    let lower = filename.to_lowercase();
    lower.contains("mmproj")
        || lower.contains("clip")
        || (lower.contains("proj") && lower.contains("vision"))
}

fn render_file_list(
    ui: &mut egui::Ui,
    dir: &std::path::Path,
    selected_path: std::path::PathBuf,
    on_select: &mut impl FnMut(std::path::PathBuf),
    lang: &i18n::Language,
    is_mmproj_mode: bool,
) {
    let entries: Vec<_> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy().to_string().to_lowercase();
                if !name.ends_with(".gguf") {
                    return false;
                }
                if is_mmproj_mode {
                    is_mmproj_file(&name)
                } else {
                    !is_mmproj_file(&name)
                }
            })
            .collect(),
        Err(_) => Vec::new(),
    };

    if entries.is_empty() {
        ui.colored_label(
            egui::Color32::GRAY,
            if is_mmproj_mode {
                i18n::t(i18n::Key::NoMmprojFiles, lang)
            } else {
                i18n::t(i18n::Key::NoGgufFiles, lang)
            },
        );
        return;
    }

    for entry in entries {
        let file_path = entry.path();
        let filename = entry.file_name().to_string_lossy().to_string();
        let selected = selected_path == file_path;

        ui.horizontal(|ui| {
            // 标签
            let tags = parse_tags(&filename);
            for (text, color) in &tags {
                ui.add(egui::Button::new(egui::RichText::new(text).color(egui::Color32::WHITE))
                    .fill(*color)
                    .rounding(4.0));
            }

            ui.separator();

            // 单选框
            if ui
                .add(egui::RadioButton::new(selected, ""))
                .clicked
            {
                on_select(file_path);
            }
        });
    }
}

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings, lang: &i18n::Language) {
    ui.heading(i18n::t(i18n::Key::PanelModelTitle, lang));
    ui.separator();

    // 文件夹选择
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelModelDir, lang));
        if ui.button(i18n::t(i18n::Key::BtnSelectFolder, lang)).clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .set_title(i18n::t(i18n::Key::DialogSelectFolder, lang))
                .pick_folder()
            {
                settings.model_dir = path;
            }
        }
    });

    let mut dir_str = settings.model_dir.to_string_lossy().to_string();
    let response = ui.text_edit_singleline(&mut dir_str);
    if response.changed() {
        settings.model_dir = std::path::PathBuf::from(&dir_str);
    }

    ui.add_space(8.0);

    // 文件夹为空时提示
    if settings.model_dir.as_os_str().is_empty() {
        ui.colored_label(egui::Color32::GRAY, i18n::t(i18n::Key::NoModelDir, lang));
        return;
    }

   // 模型文件列表
    ui.heading(i18n::t(i18n::Key::SectionModels, lang));
    ui.separator();
    let selected_model = settings.model_path.clone();
    render_file_list(
        ui,
        &settings.model_dir,
        selected_model,
        &mut |path| {
            settings.model_path = path;
        },
        lang,
        false,
    );

    // 分隔
    ui.add_space(12.0);
    ui.heading(i18n::t(i18n::Key::SectionMmproj, lang));
    ui.separator();
    let selected_mmproj = settings.mmproj_path.clone();
    render_file_list(
        ui,
        &settings.model_dir,
        selected_mmproj.clone(),
        &mut |path| {
            // 再次点击已选中的路径 → 取消选中
            settings.mmproj_path = if selected_mmproj == path {
                std::path::PathBuf::new()
            } else {
                path
            };
        },
        lang,
        true,
    );
}

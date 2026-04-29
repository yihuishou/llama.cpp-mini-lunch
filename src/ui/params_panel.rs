use crate::config::settings::AppSettings;
use crate::i18n;

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings, lang: &i18n::Language) {
    ui.heading(i18n::t(i18n::Key::PanelParamsTitle, lang));
    ui.separator();

    // 上下文
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelNCtx, lang));
        ui.add(
            egui::DragValue::new(&mut settings.n_ctx)
                .range(256..=262144)
                .speed(256),
        );
    });

    // 最大预测
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelNPredict, lang));
        ui.add(egui::DragValue::new(&mut settings.n_predict).range(-1..=8192));
        ui.small(i18n::t(i18n::Key::HintNPredict, lang));
    });

    ui.add_space(12.0);
    ui.heading(i18n::t(i18n::Key::SectionSampling, lang));
    ui.separator();

    // 温度
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelTemperature, lang));
        ui.add(egui::Slider::new(&mut settings.temperature, 0.0..=2.0));
        ui.label(format!("{:.2}", settings.temperature));
    });

    // top_p
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelTopP, lang));
        ui.add(egui::Slider::new(&mut settings.top_p, 0.0..=1.0));
        ui.label(format!("{:.2}", settings.top_p));
    });

    // top_k
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelTopK, lang));
        ui.add(egui::DragValue::new(&mut settings.top_k).range(0..=1000));
    });

    // 重复惩罚
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelRepeatPenalty, lang));
        ui.add(egui::Slider::new(&mut settings.repeat_penalty, 0.0..=2.0));
        ui.label(format!("{:.2}", settings.repeat_penalty));
    });

    ui.add_space(12.0);
    ui.heading(i18n::t(i18n::Key::SectionKvCache, lang));
    ui.separator();

    // K/V 缓存卸载
    ui.horizontal(|ui| {
        ui.checkbox(&mut settings.kv_offload, i18n::t(i18n::Key::CheckboxKvOffload, lang));
        ui.small(i18n::t(i18n::Key::HintKvOffload, lang));
    });

    // K 缓存类型
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelCacheTypeK, lang));
        let k_types = ["f16", "q8_0", "q4_0"];
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for k_type in &k_types {
                let selected = settings.cache_type_k == *k_type;
                if ui.selectable_label(selected, *k_type).clicked() {
                    settings.cache_type_k = k_type.to_string();
                }
            }
        });
    });

    // V 缓存类型
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelCacheTypeV, lang));
        let v_types = ["f16", "q8_0", "q4_0"];
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for v_type in &v_types {
                let selected = settings.cache_type_v == *v_type;
                if ui.selectable_label(selected, *v_type).clicked() {
                    settings.cache_type_v = v_type.to_string();
                }
            }
        });
    });

    ui.add_space(12.0);
    ui.heading(i18n::t(i18n::Key::SectionGpuDevice, lang));
    ui.separator();

    // GPU 层数
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelGpuDevice, lang));
        ui.text_edit_singleline(&mut settings.gpu_layers_str);
        ui.small(i18n::t(i18n::Key::HintGpuLayers, lang));
    });

    // 设备列表
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelRpcDevice, lang));
        ui.text_edit_singleline(&mut settings.gpu_device);
        ui.small(i18n::t(i18n::Key::HintRpcDevice, lang));
    });

    // 拆分模式
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelSplitMode, lang));
        let modes = ["layer", "none", "row", "tensor"];
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            for mode in &modes {
                let selected = settings.split_mode == *mode;
                if ui.selectable_label(selected, *mode).clicked() {
                    settings.split_mode = mode.to_string();
                }
            }
        });
        ui.small(i18n::t(i18n::Key::HintSplitMode, lang));
    });

    // 张量拆分比例
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelTensorSplit, lang));
        ui.text_edit_singleline(&mut settings.tensor_split);
        ui.small(i18n::t(i18n::Key::HintTensorSplit, lang));
    });

    // CPU MoE
    ui.horizontal(|ui| {
        ui.checkbox(&mut settings.cpu_moe, i18n::t(i18n::Key::CheckboxCpuMoe, lang));
    });

    // N CPU MoE
    ui.horizontal(|ui| {
        ui.label(i18n::t(i18n::Key::LabelNCpuMoe, lang));
        ui.add(egui::DragValue::new(&mut settings.n_cpu_moe).range(0..=256));
        ui.small(i18n::t(i18n::Key::HintNCpuMoe, lang));
    });

    ui.add_space(16.0);
    ui.heading(i18n::t(i18n::Key::SectionParamsHelp, lang));
    ui.separator();

    ui.label(egui::RichText::new(i18n::t(i18n::Key::ParamsHelpText, lang)).weak());
}

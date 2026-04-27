use crate::config::settings::AppSettings;

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings) {
    ui.heading("推理参数");
    ui.separator();

    // 上下文
    ui.horizontal(|ui| {
        ui.label("上下文长度 (n_ctx):");
        ui.add(
            egui::DragValue::new(&mut settings.n_ctx)
                .range(256..=131072)
                .speed(256),
        );
    });

    // 最大预测
    ui.horizontal(|ui| {
        ui.label("最大生成长度 (n_predict):");
        ui.add(egui::DragValue::new(&mut settings.n_predict).range(-1..=8192));
        ui.small("-1 = 无限");
    });

    ui.add_space(12.0);
    ui.heading("采样参数");
    ui.separator();

    // 温度
    ui.horizontal(|ui| {
        ui.label("温度:");
        ui.add(egui::Slider::new(&mut settings.temperature, 0.0..=2.0));
        ui.label(format!("{:.2}", settings.temperature));
    });

    // top_p
    ui.horizontal(|ui| {
        ui.label("Top P:");
        ui.add(egui::Slider::new(&mut settings.top_p, 0.0..=1.0));
        ui.label(format!("{:.2}", settings.top_p));
    });

    // top_k
    ui.horizontal(|ui| {
        ui.label("Top K:");
        ui.add(egui::DragValue::new(&mut settings.top_k).range(0..=1000));
    });

    // 重复惩罚
    ui.horizontal(|ui| {
        ui.label("重复惩罚:");
        ui.add(egui::Slider::new(&mut settings.repeat_penalty, 0.0..=2.0));
        ui.label(format!("{:.2}", settings.repeat_penalty));
    });

    ui.add_space(12.0);
    ui.heading("KV 缓存配置");
    ui.separator();

    // KV 缓存卸载
    ui.horizontal(|ui| {
        ui.checkbox(&mut settings.kv_offload, "KV 缓存卸载到 GPU");
        ui.small("(默认开启)");
    });

    // K 缓存类型
    ui.horizontal(|ui| {
        ui.label("K 缓存类型:");
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
        ui.label("V 缓存类型:");
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
    ui.heading("GPU 与设备分配");
    ui.separator();

    // GPU 层数
    ui.horizontal(|ui| {
        ui.label("GPU 层数 (n_gl):");
        ui.text_edit_singleline(&mut settings.gpu_layers_str);
        ui.small("(数字/自动/全部)");
    });

    // 设备列表
    ui.horizontal(|ui| {
        ui.label("设备 (dev):");
        ui.text_edit_singleline(&mut settings.gpu_device);
        ui.small("逗号分隔，如: 0,1");
    });

    // 拆分模式
    ui.horizontal(|ui| {
        ui.label("拆分模式 (sm):");
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
        ui.small("(默认: layer)");
    });

    // 张量拆分比例
    ui.horizontal(|ui| {
        ui.label("张量拆分 (ts):");
        ui.text_edit_singleline(&mut settings.tensor_split);
        ui.small("如: 3,1");
    });

    // CPU MoE
    ui.horizontal(|ui| {
        ui.checkbox(&mut settings.cpu_moe, "CPU MoE: 所有 MoE 权重保留在 CPU");
    });

    // N CPU MoE
    ui.horizontal(|ui| {
        ui.label("N CPU MoE:");
        ui.add(egui::DragValue::new(&mut settings.n_cpu_moe).range(0..=256));
        ui.small("前 N 层 MoE 权重保留在 CPU");
    });

    ui.add_space(16.0);
    ui.heading("参数说明");
    ui.separator();

    ui.label(egui::RichText::new(
        "温度: 控制随机性，越高越随机\n\
         Top P: 核采样阈值，只保留累积概率超过该值的token\n\
         Top K: 只保留概率最高的K个候选token\n\
         重复惩罚: 降低重复内容的概率\n\n\
         KV 缓存卸载: 允许将 KV 缓存卸载到 GPU\n\
         K/V 缓存类型: 缓存数据类型 (f16, q8_0, q4_0)，使用量化类型可节省显存\n\n\
         GPU 层数: 存储在显存中的模型层数\n\
         拆分模式: layer(按层), none(单GPU), row(按行), tensor(按张量)\n\
         张量拆分: 多 GPU 卸载比例\n\
         CPU MoE: 将 MoE 权重保留在 CPU",
    ).weak());
}

/// 语言枚举
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Language {
    #[default]
    Zh,
    En,
}

/// 检测系统语言
/// 翻译键枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    // 菜单
    MenuFile,
    MenuTheme,
    MenuHelp,
    MenuItemSaveConfig,
    MenuItemLoadConfig,
    MenuItemAbout,

    // 标签页
    TabServer,
    TabRpc,
    TabModel,
    TabParams,
    TabLog,

    // 按钮
    BtnStartServer,
    BtnStopServer,
    BtnStartRpc,
    BtnStopRpc,
    BtnBrowse,
    BtnAutoDetect,
    BtnClearLogs,
    BtnClose,

    // 状态
    StatusProcessing,
    StatusIdle,
    StatusStarting,
    StatusRunning,
    StatusStopping,
    StatusError,
    StatusServerExited,
    StatusServerCrashed,
    StatusRpcCrashed,

    // Server 面板
    PanelServerTitle,
    LabelServerPath,
    LabelHost,
    LabelPort,
    LabelParallelSlots,
    HintGpuLayers,
    CheckboxVerbose,
    CheckboxRpcMode,
    LabelRpcEndpoints,
    HintRpcEndpoints,
    DialogSelectServer,
    FilterExecutable,

    // RPC 面板
    PanelRpcTitle,
    LabelRpcPath,
    LabelRpcThreads,
    HintRpcThreads,
    LabelRpcDevice,
    HintRpcDevice,
    CheckboxRpcCache,
  DialogSelectRpc,

    // 模型面板
    PanelModelTitle,
    LabelModelPath,
    LabelMmprojPath,
    DialogSelectModel,
    DialogSelectMmproj,
    FilterGguf,
    FilterMmproj,
    SectionMultimodal,
    ModelNotExist,
    ModelNotSelected,
    LabelFile,

    // 参数面板
    PanelParamsTitle,
    LabelNCtx,
    LabelNPredict,
    HintNPredict,
    SectionSampling,
    LabelTemperature,
    LabelTopP,
    LabelTopK,
    LabelRepeatPenalty,
    SectionKvCache,
    CheckboxKvOffload,
    HintKvOffload,
    LabelCacheTypeK,
    LabelCacheTypeV,
    SectionGpuDevice,
    LabelGpuDevice,
    LabelSplitMode,
    HintSplitMode,
    LabelTensorSplit,
    HintTensorSplit,
    CheckboxCpuMoe,
    LabelNCpuMoe,
    HintNCpuMoe,
    SectionParamsHelp,
    ParamsHelpText,

    // 日志面板
    PanelLogTitle,
    HintLogSession,
    HintNoLogs,

    // 错误信息
    ErrServerModelMissing,
    ErrRpcPathMissing,
    ErrRpcFileNotFound,
    ErrStartFailed,

    // 关于
    AboutTitle,
    AboutVersion,
    AboutDescription,

    // 通用
    GenericSelectModule,
}

impl Key {
    pub fn translate(self, lang: &Language) -> &'static str {
        match (self, lang) {
            // 菜单
            (Key::MenuFile, &Language::Zh) => "文件",
            (Key::MenuFile, &Language::En) => "File",
            (Key::MenuTheme, &Language::Zh) => "主题",
            (Key::MenuTheme, &Language::En) => "Theme",
            (Key::MenuHelp, &Language::Zh) => "帮助",
            (Key::MenuHelp, &Language::En) => "Help",
            (Key::MenuItemSaveConfig, &Language::Zh) => "保存配置",
            (Key::MenuItemSaveConfig, &Language::En) => "Save Config",
            (Key::MenuItemLoadConfig, &Language::Zh) => "加载配置",
            (Key::MenuItemLoadConfig, &Language::En) => "Load Config",
            (Key::MenuItemAbout, &Language::Zh) => "关于",
            (Key::MenuItemAbout, &Language::En) => "About",

            // 标签页
            (Key::TabServer, &Language::Zh) => "Server",
            (Key::TabServer, &Language::En) => "Server",
            (Key::TabRpc, &Language::Zh) => "RPC",
            (Key::TabRpc, &Language::En) => "RPC",
            (Key::TabModel, &Language::Zh) => "模型",
            (Key::TabModel, &Language::En) => "Model",
            (Key::TabParams, &Language::Zh) => "参数",
            (Key::TabParams, &Language::En) => "Params",
 
            (Key::TabLog, &Language::Zh) => "日志",
            (Key::TabLog, &Language::En) => "Logs",

            // 按钮
            (Key::BtnStartServer, &Language::Zh) => "启动 Server",
            (Key::BtnStartServer, &Language::En) => "Start Server",
            (Key::BtnStopServer, &Language::Zh) => "停止 Server",
            (Key::BtnStopServer, &Language::En) => "Stop Server",
            (Key::BtnStartRpc, &Language::Zh) => "启动 RPC",
            (Key::BtnStartRpc, &Language::En) => "Start RPC",
            (Key::BtnStopRpc, &Language::Zh) => "停止 RPC",
            (Key::BtnStopRpc, &Language::En) => "Stop RPC",
            (Key::BtnBrowse, &Language::Zh) => "浏览...",
            (Key::BtnBrowse, &Language::En) => "Browse...",
            (Key::BtnAutoDetect, &Language::Zh) => "自动检测",
            (Key::BtnAutoDetect, &Language::En) => "Auto Detect",
            (Key::BtnClearLogs, &Language::Zh) => "清空日志",
            (Key::BtnClearLogs, &Language::En) => "Clear Logs",
          (Key::BtnClose, &Language::Zh) => "关闭",
            (Key::BtnClose, &Language::En) => "Close",

            // 状态
            (Key::StatusProcessing, &Language::Zh) => "处理中...",
            (Key::StatusProcessing, &Language::En) => "Processing...",
            (Key::StatusIdle, &Language::Zh) => "已停止",
            (Key::StatusIdle, &Language::En) => "Stopped",
            (Key::StatusStarting, &Language::Zh) => "启动中",
            (Key::StatusStarting, &Language::En) => "Starting",
            (Key::StatusRunning, &Language::Zh) => "运行中",
            (Key::StatusRunning, &Language::En) => "Running",
            (Key::StatusStopping, &Language::Zh) => "停止中",
            (Key::StatusStopping, &Language::En) => "Stopping",
            (Key::StatusError, &Language::Zh) => "错误",
            (Key::StatusError, &Language::En) => "Error",
            (Key::StatusServerExited, &Language::Zh) => "Server 进程已正常退出",
            (Key::StatusServerExited, &Language::En) => "Server process exited normally",
            (Key::StatusServerCrashed, &Language::Zh) => "Server 进程异常退出",
            (Key::StatusServerCrashed, &Language::En) => "Server process crashed",
            (Key::StatusRpcCrashed, &Language::Zh) => "RPC 进程异常退出",
            (Key::StatusRpcCrashed, &Language::En) => "RPC process crashed",

            // Server 面板
            (Key::PanelServerTitle, &Language::Zh) => "Server 配置",
            (Key::PanelServerTitle, &Language::En) => "Server Config",
            (Key::LabelServerPath, &Language::Zh) => "llama-server 路径:",
            (Key::LabelServerPath, &Language::En) => "llama-server path:",
            (Key::LabelHost, &Language::Zh) => "主机:",
            (Key::LabelHost, &Language::En) => "Host:",
            (Key::LabelPort, &Language::Zh) => "端口:",
            (Key::LabelPort, &Language::En) => "Port:",
            (Key::LabelParallelSlots, &Language::Zh) => "并发数量:",
            (Key::LabelParallelSlots, &Language::En) => "Parallel slots:",
            (Key::HintGpuLayers, &Language::Zh) => "(数字/自动/全部)",
            (Key::HintGpuLayers, &Language::En) => "(number/auto/all)",
            (Key::CheckboxVerbose, &Language::Zh) => "详细输出 (verbose)",
          (Key::CheckboxVerbose, &Language::En) => "Verbose output",
            (Key::CheckboxRpcMode, &Language::Zh) => "RPC 模式 (--rpc)",
            (Key::CheckboxRpcMode, &Language::En) => "RPC Mode (--rpc)",
            (Key::LabelRpcEndpoints, &Language::Zh) => "RPC 节点地址:",
            (Key::LabelRpcEndpoints, &Language::En) => "RPC Endpoints:",
            (Key::HintRpcEndpoints, &Language::Zh) => "(格式: host:port,host:port)",
            (Key::HintRpcEndpoints, &Language::En) => "(format: host:port,host:port)",
            (Key::DialogSelectServer, &Language::Zh) => "选择 llama-server 可执行文件",
            (Key::DialogSelectServer, &Language::En) => "Select llama-server executable",
            (Key::FilterExecutable, &Language::Zh) => "可执行文件",
            (Key::FilterExecutable, &Language::En) => "Executable",

            // RPC 面板
            (Key::PanelRpcTitle, &Language::Zh) => "RPC 配置",
            (Key::PanelRpcTitle, &Language::En) => "RPC Config",
            (Key::LabelRpcPath, &Language::Zh) => "rpc-server 路径:",
            (Key::LabelRpcPath, &Language::En) => "rpc-server path:",
            (Key::LabelRpcThreads, &Language::Zh) => "CPU 线程数:",
            (Key::LabelRpcThreads, &Language::En) => "CPU Threads:",
            (Key::HintRpcThreads, &Language::Zh) => "(默认: 12)",
            (Key::HintRpcThreads, &Language::En) => "(default: 12)",
            (Key::LabelRpcDevice, &Language::Zh) => "设备:",
            (Key::LabelRpcDevice, &Language::En) => "Device:",
            (Key::HintRpcDevice, &Language::Zh) => "逗号分隔，如: 0,1",
            (Key::HintRpcDevice, &Language::En) => "Comma separated, e.g. 0,1",
            (Key::CheckboxRpcCache, &Language::Zh) => "启用本地文件缓存",
            (Key::CheckboxRpcCache, &Language::En) => "Enable local file cache",
            (Key::DialogSelectRpc, &Language::Zh) => "选择 rpc-server 可执行文件",
(Key::DialogSelectRpc, &Language::En) => "Select rpc-server executable",

        // 模型面板
            (Key::PanelModelTitle, &Language::Zh) => "模型管理",
            (Key::PanelModelTitle, &Language::En) => "Model Management",
            (Key::LabelModelPath, &Language::Zh) => "GGUF 模型文件:",
            (Key::LabelModelPath, &Language::En) => "GGUF Model File:",
            (Key::LabelMmprojPath, &Language::Zh) => "mmproj 投影文件:",
            (Key::LabelMmprojPath, &Language::En) => "mmproj Projection File:",
            (Key::DialogSelectModel, &Language::Zh) => "选择 GGUF 模型文件",
            (Key::DialogSelectModel, &Language::En) => "Select GGUF model file",
            (Key::DialogSelectMmproj, &Language::Zh) => "选择 mmproj 投影文件",
            (Key::DialogSelectMmproj, &Language::En) => "Select mmproj projection file",
            (Key::FilterGguf, &Language::Zh) => "GGUF 模型",
            (Key::FilterGguf, &Language::En) => "GGUF Model",
            (Key::FilterMmproj, &Language::Zh) => "投影文件",
            (Key::FilterMmproj, &Language::En) => "Projection File",
            (Key::SectionMultimodal, &Language::Zh) => "多模态",
            (Key::SectionMultimodal, &Language::En) => "Multimodal",
            (Key::ModelNotExist, &Language::Zh) => "文件不存在或无法访问",
            (Key::ModelNotExist, &Language::En) => "File does not exist or is inaccessible",
            (Key::ModelNotSelected, &Language::Zh) => "尚未选择模型文件",
            (Key::ModelNotSelected, &Language::En) => "No model file selected",
            (Key::LabelFile, &Language::Zh) => "文件",
            (Key::LabelFile, &Language::En) => "File",

            // 参数面板
            (Key::PanelParamsTitle, &Language::Zh) => "推理参数",
            (Key::PanelParamsTitle, &Language::En) => "Inference Params",
            (Key::LabelNCtx, &Language::Zh) => "上下文长度 (n_ctx):",
            (Key::LabelNCtx, &Language::En) => "Context Length (n_ctx):",
            (Key::LabelNPredict, &Language::Zh) => "最大生成长度 (n_predict):",
            (Key::LabelNPredict, &Language::En) => "Max Predict Length (n_predict):",
            (Key::HintNPredict, &Language::Zh) => "-1 = 无限",
            (Key::HintNPredict, &Language::En) => "-1 = unlimited",
            (Key::SectionSampling, &Language::Zh) => "采样参数",
            (Key::SectionSampling, &Language::En) => "Sampling",
            (Key::LabelTemperature, &Language::Zh) => "温度:",
            (Key::LabelTemperature, &Language::En) => "Temperature:",
            (Key::LabelTopP, &Language::Zh) => "Top P:",
            (Key::LabelTopP, &Language::En) => "Top P:",
            (Key::LabelTopK, &Language::Zh) => "Top K:",
            (Key::LabelTopK, &Language::En) => "Top K:",
            (Key::LabelRepeatPenalty, &Language::Zh) => "重复惩罚:",
            (Key::LabelRepeatPenalty, &Language::En) => "Repeat Penalty:",
            (Key::SectionKvCache, &Language::Zh) => "KV 缓存配置",
            (Key::SectionKvCache, &Language::En) => "KV Cache Config",
            (Key::CheckboxKvOffload, &Language::Zh) => "KV 缓存卸载到 GPU",
            (Key::CheckboxKvOffload, &Language::En) => "KV Cache Offload to GPU",
            (Key::HintKvOffload, &Language::Zh) => "(默认开启)",
            (Key::HintKvOffload, &Language::En) => "(default: on)",
            (Key::LabelCacheTypeK, &Language::Zh) => "K 缓存类型:",
            (Key::LabelCacheTypeK, &Language::En) => "K Cache Type:",
            (Key::LabelCacheTypeV, &Language::Zh) => "V 缓存类型:",
            (Key::LabelCacheTypeV, &Language::En) => "V Cache Type:",
            (Key::SectionGpuDevice, &Language::Zh) => "GPU 与设备分配",
            (Key::SectionGpuDevice, &Language::En) => "GPU & Device Allocation",
            (Key::LabelGpuDevice, &Language::Zh) => "GPU 层数 (n_gl):",
            (Key::LabelGpuDevice, &Language::En) => "GPU Layers (n_gl):",
            (Key::LabelSplitMode, &Language::Zh) => "拆分模式 (sm):",
            (Key::LabelSplitMode, &Language::En) => "Split Mode (sm):",
            (Key::HintSplitMode, &Language::Zh) => "(默认: layer)",
            (Key::HintSplitMode, &Language::En) => "(default: layer)",
            (Key::LabelTensorSplit, &Language::Zh) => "张量拆分 (ts):",
            (Key::LabelTensorSplit, &Language::En) => "Tensor Split (ts):",
            (Key::HintTensorSplit, &Language::Zh) => "如: 3,1",
            (Key::HintTensorSplit, &Language::En) => "e.g. 3,1",
            (Key::CheckboxCpuMoe, &Language::Zh) => "CPU MoE: 所有 MoE 权重保留在 CPU",
            (Key::CheckboxCpuMoe, &Language::En) => "CPU MoE: Keep all MoE weights on CPU",
            (Key::LabelNCpuMoe, &Language::Zh) => "N CPU MoE:",
            (Key::LabelNCpuMoe, &Language::En) => "N CPU MoE:",
            (Key::HintNCpuMoe, &Language::Zh) => "前 N 层 MoE 权重保留在 CPU",
            (Key::HintNCpuMoe, &Language::En) => "Keep first N MoE layers on CPU",
            (Key::SectionParamsHelp, &Language::Zh) => "参数说明",
            (Key::SectionParamsHelp, &Language::En) => "Parameter Help",
            (Key::ParamsHelpText, &Language::Zh) => "温度: 控制随机性，越高越随机\n\
 Top P: 核采样阈值，只保留累积概率超过该值的token\n\
 Top K: 只保留概率最高的K个候选token\n\
 重复惩罚: 降低重复内容的概率\n\n\
 KV 缓存卸载: 允许将 KV 缓存卸载到 GPU\n\
 K/V 缓存类型: 缓存数据类型 (f16, q8_0, q4_0)，使用量化类型可节省显存\n\n\
 GPU 层数: 存储在显存中的模型层数\n\
 拆分模式: layer(按层), none(单GPU), row(按行), tensor(按张量)\n\
 张量拆分: 多 GPU 卸载比例\n\
 CPU MoE: 将 MoE 权重保留在 CPU",
            (Key::ParamsHelpText, &Language::En) => "Temperature: Controls randomness, higher = more random\n\
 Top P: Nucleus sampling threshold, only keep tokens with cumulative probability above this value\n\
 Top K: Only keep the top K most probable candidate tokens\n\
 Repeat Penalty: Reduces the probability of repetitive content\n\n\
 KV Cache Offload: Allows offloading KV cache to GPU\n\
 K/V Cache Type: Cache data type (f16, q8_0, q4_0), quantized types save VRAM\n\n\
 GPU Layers: Number of model layers stored in VRAM\n\
 Split Mode: layer, none (single GPU), row, tensor\n\
 Tensor Split: Multi-GPU offload ratio\n\
          CPU MoE: Keep MoE weights on CPU",

            // 日志面板
            (Key::PanelLogTitle, &Language::Zh) => "运行日志",
            (Key::PanelLogTitle, &Language::En) => "Runtime Logs",
            (Key::HintLogSession, &Language::Zh) => "日志仅在当前会话中保留",
            (Key::HintLogSession, &Language::En) => "Logs are only kept for the current session",
            (Key::HintNoLogs, &Language::Zh) => "暂无日志输出",
            (Key::HintNoLogs, &Language::En) => "No log output",

            // 错误信息
            (Key::ErrServerModelMissing, &Language::Zh) => "请先配置 Server 路径和模型路径",
            (Key::ErrServerModelMissing, &Language::En) => "Please configure Server path and model path first",
            (Key::ErrRpcPathMissing, &Language::Zh) => "请先配置 rpc-server 路径",
            (Key::ErrRpcPathMissing, &Language::En) => "Please configure rpc-server path first",
            (Key::ErrRpcFileNotFound, &Language::Zh) => "rpc-server.exe 文件不存在",
            (Key::ErrRpcFileNotFound, &Language::En) => "rpc-server.exe file not found",
            (Key::ErrStartFailed, &Language::Zh) => "启动失败",
            (Key::ErrStartFailed, &Language::En) => "Start failed",

            // 关于
            (Key::AboutTitle, &Language::Zh) => "关于",
            (Key::AboutTitle, &Language::En) => "About",
            (Key::AboutVersion, &Language::Zh) => "llama.cpp lunch v0.1.0",
            (Key::AboutVersion, &Language::En) => "llama.cpp lunch v0.1.0",
            (Key::AboutDescription, &Language::Zh) => "llama-server 图形启动器",
            (Key::AboutDescription, &Language::En) => "llama-server GUI launcher",

            // 通用
            (Key::GenericSelectModule, &Language::Zh) => "请选择一个功能模块",
            (Key::GenericSelectModule, &Language::En) => "Please select a module",
        }
    }
}

/// 翻译函数 - 类型安全地获取翻译字符串
pub fn t(key: Key, lang: &Language) -> &'static str {
    key.translate(lang)
}

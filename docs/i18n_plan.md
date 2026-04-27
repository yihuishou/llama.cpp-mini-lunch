# llama.cpp lunch 国际化 (i18n) 实现方案

## 目标
为 llama.cpp lunch 界面增加中英文双语支持，启动时自动跟随系统语言。

## 设计决策

| 决策项 | 选择 | 原因 |
|--------|------|------|
| 语言切换方式 | 仅启动时检测 | 项目为服务器启动器，语言偏好稳定 |
| 技术术语 | 双语保留 | f16, q8_0, layer, tensor 等不翻译 |
| 翻译存储 | Rust 枚举 match | 编译时类型安全，适合 2 种语言小项目 |
| 外部依赖 | 无 | 不引入 i18n 框架 |

## 实施计划

### 1. 新建 `src/i18n.rs`
- `Language` 枚举：`Zh` / `En`，实现 `Serialize` / `Deserialize`
- `detect_system_language()`：检测系统语言
  - Windows: 查 `LANG` 环境变量，回退到 `lcid` 注册表
  - Linux/macOS: 查 `LANG` 环境变量
  - 匹配规则：以 `zh` 开头 → `Zh`，否则 → `En`
- `Key` 枚举：约 80 个翻译键，覆盖所有 UI 文本
- `t!(key, lang)` 宏：编译时类型安全翻译

### 2. 修改 `src/config/settings.rs`
- `AppSettings` 增加 `language: String` 字段
- `Default` 设为 `""`（首次启动检测后写入）

### 3. 修改 `src/app.rs`
- `new()` 时检测系统语言（若 `settings.language` 为空）
- 增加 `lang: Language` 字段
- 传递 `&self.lang` 给所有面板和状态文本
- 硬编码字符串替换为 `t!(...)`

### 4. 修改 UI 面板（6 个文件）
每个面板函数签名增加 `lang: &Language` 参数，替换所有硬编码字符串：
- `server_panel.rs`
- `rpc_panel.rs`
- `model_panel.rs`
- `params_panel.rs`
- `preset_panel.rs`
- `log_panel.rs`

### 5. 修改引擎模块（2 个文件）
- `server.rs`：`status_text()` 增加 `lang` 参数
- `rpc.rs`：`status_text()` 增加 `lang` 参数

## 预估工作量

| 指标 | 数值 |
|------|------|
| 新增文件 | 1 (`src/i18n.rs`) |
| 修改文件 | 9 |
| 翻译键数 | ~80 |
| 新增行数 | ~350 |

## 翻译键命名约定

| 前缀 | 含义 | 示例 |
|------|------|------|
| `menu_` | 菜单项 | `menu_file`, `menu_theme`, `menu_help` |
| `tab_` | 标签页 | `tab_server`, `tab_model` |
| `btn_` | 按钮 | `btn_start_server`, `btn_browse` |
| `status_` | 状态文本 | `status_running`, `status_idle` |
| `label_` | 字段标签 | `label_model_path`, label_host` |
| `hint_` | 提示文本 | `hint_gpu_layers`, `hint_device` |
| `err_` | 错误文本 | `err_server_path_missing` |
| `dialog_` | 对话框标题 | `dialog_select_server` |

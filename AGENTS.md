# LLama Lunch — Root

## 项目概述
llama.cpp 桌面启动器。Rust + eframe/egui 0.29 构建的 GUI，管理 llama-server 进程和 RPC 后端。

## 技术栈
- **Rust** 2024 版, egui 0.29, eframe 0.29
- **序列化**: serde + serde_json
- **i18n**: 自研 (zh, ja, ko, en, fr, ru, de)
- **文件对话框**: rfd
- **日志**: log + env_logger
- **窗口**: winit, Windows 图标 (winres via build.rs)

## 架构
```
main.rs → App (app.rs) → 6个面板 (ui/) + SettingsManager (config/) + ServerManager/RpcManager (engine/)
```

## 启动流程
1. `main.rs`: 加载 CJK 字体 (Noto Sans SC/TC/JP/KR), 创建 eframe
2. `app.rs`: 加载 `native::locale()`, 读取 settings.json
3. 每帧: 路由到当前活动面板 (server/params/model/rpc/log/launch_commands)
4. 退出: `Drop` trait 自动停止 server/RPC 子进程

## 核心模块
| 模块 | 路径 | 职责 |
|------|------|------|
| ui | src/ui/ | 6个面板 (egui), 模型文件标签解析 |
| engine | src/engine/ | 进程管理 (server + RPC), 状态机, 日志聚合 |
| config | src/config/settings.rs | AppSettings 结构体, JSON 读写, 自动检测 exe |
| i18n | src/i18n.rs | Key 枚举 + 多语言字符串映射 |

## 关键数据结构
- `AppSettings`: 全局配置 (server/RPC/模型/推理/GPU), Default + Serialize/Deserialize
- `ServerState` / `RpcState`: 状态枚举 (Idle, Starting, Running, Stopping, Error)
- `AppState`: 当前语言、活动面板索引、settings 引用

## 约束
- **无外部依赖管理**: 不打包 Cargo workspace, 单 binary
- **Windows 优先**: CREATE_NO_WINDOW 标志, .exe 检测, winres 图标
- **进程管理**: `std::process::Child` + `Arc<Mutex<>>` 线程安全包装
- **日志**: stderr/stdout 通过 BufReader 行读取, 聚合到 `VecDeque<String>`
- **i18n Key**: 所有 UI 文本必须通过 `i18n::t(i18n::Key::Xxx, lang)`, 禁止硬编码中文

## 构建
```bash
cargo build --release
# Windows 输出: target/release/llama-lunch.exe
# 运行时在 exe 同级创建 llama_lunch/settings.json
```

## 代码风格
- 简体中文注释和 i18n Key 命名 (中文意思)
- 模块文件名英文
- egui 0.29 API (注意: 不是 0.28 或 0.30)
- 错误处理: `Result<T, String>` + `map_err`
- 无 clippy warn 容忍 (未使用的变量直接忽略)

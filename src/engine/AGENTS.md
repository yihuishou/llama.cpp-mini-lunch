# src/engine/ — 引擎目录

## 职责
llama-server 和 rpc-server 的进程管理、状态机、日志聚合。

## 文件清单
| 文件 | 核心类型 | 职责 |
|------|----------|------|
| server.rs | `ServerManager`, `ServerState`, `ServerLog` | llama-server 进程生命周期 |
| rpc.rs | `RpcManager`, `RpcState`, `RpcConnection` | rpc-server 进程生命周期 |
| mod.rs | `LogEntry`, `LogType` | 日志聚合、枚举定义 |

## ServerManager 状态机
```
Idle → Starting → Running → Idle          (正常停止)
Idle → Starting → Running → Error          (崩溃)
Running → Stopping → Idle                  (停止中)
```
- `start()`: 构建 Command → spawn → 启动 stdout/stderr 线程 → 捕获 launch_command
- `stop()`: `child.kill()` + `child.wait()` → 清理
- `poll()`: `try_wait()` 检测进程退出 → 更新状态
- `restart()`: `stop()` then `start()`

## RpcManager 状态机 (类似)
```
Idle → Starting → Running → Idle
            ↓
        Connected/Disconnected (连接状态)
```
- 额外 `connection: RpcConnection` 字段跟踪连接状态
- `start()` 成功后设置 `Connected`

## 进程管理通用模式
- `Arc<Mutex<InnerState>>` 包裹 `std::process::Child`
- stdout/stderr 各一个 `thread::spawn` 线程
- `BufReader::lines()` 行读取 → `LogEntry` → `VecDeque`
- Windows `CREATE_NO_WINDOW` (0x08000000) 隐藏控制台窗口
- `Drop` trait 自动 `stop()` — App 退出时清理子进程

## Log 聚合 (mod.rs)
- `LogEntry { source: LogType, timestamp, message }`
- `LogType::Server` / `LogType::Rpc`
- `poll_logs(&mut self, server: &mut ServerManager, rpc: &mut RpcManager)` — 合并两路日志
- 日志环形缓冲区: `VecDeque`, 容量限制 2000 行

## 约束
- `start()` 前置检查: path 非空 + 文件存在
- `start()` 幂等: 已运行则直接返回
- 错误消息走 i18n (`Key::ErrStartFailed` 等)
- 日志线程在 `stop()` 时通过 `child.take()` 自然终止

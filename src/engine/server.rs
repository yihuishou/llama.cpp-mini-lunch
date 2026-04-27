use crate::config::settings::AppSettings;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, PartialEq)]
pub enum ServerState {
    Idle,
    Starting,
    Running,
    Stopping,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub text: String,
    pub level: LogLevel,
}

struct InnerState {
    child: Option<Child>,
    logs: Vec<LogEntry>,
}

pub struct ServerManager {
    state: ServerState,
    inner: Arc<Mutex<InnerState>>,
    _threads: Vec<thread::JoinHandle<()>>,
}

impl ServerManager {
    pub fn new() -> Self {
        Self {
            state: ServerState::Idle,
            inner: Arc::new(Mutex::new(InnerState {
                child: None,
                logs: Vec::new(),
            })),
            _threads: Vec::new(),
        }
    }

    pub fn state(&self) -> ServerState {
        self.state.clone()
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, ServerState::Running)
    }

    pub fn status_text(&self) -> String {
        match &self.state {
            ServerState::Idle => "已停止".to_string(),
            ServerState::Starting => "启动中".to_string(),
            ServerState::Running => "运行中".to_string(),
            ServerState::Stopping => "停止中".to_string(),
            ServerState::Error(msg) => format!("错误: {}", msg),
        }
    }

    pub fn logs(&self) -> Vec<LogEntry> {
        self.inner.lock().unwrap().logs.clone()
    }

    pub fn clear_logs(&mut self) {
        self.inner.lock().unwrap().logs.clear();
    }

    pub fn start(&mut self, settings: &AppSettings) {
        if self.is_running() {
            return;
        }

        let server_path = settings.server_path.clone();
        let model_path = settings.model_path.clone();

        if server_path.as_os_str().is_empty() || model_path.as_os_str().is_empty() {
            self.state = ServerState::Error("请先配置 Server 路径和模型路径".to_string());
            return;
        }

        self.state = ServerState::Starting;
        self.clear_logs();
        self._threads.clear();

        let mut cmd = Command::new(&server_path);
        cmd.arg("-m").arg(&model_path)
            .arg("--host").arg(&settings.host)
            .arg("--port").arg(settings.port.to_string())
            .arg("-c").arg(settings.n_ctx.to_string())
            .arg("-np").arg(settings.n_predict.to_string())
            .arg("--parallel").arg(settings.parallel_slots.to_string())
            .arg("--n-gpu-layers").arg(&settings.gpu_layers_str)
            .arg("--temp").arg(settings.temperature.to_string())
            .arg("--top-p").arg(settings.top_p.to_string())
            .arg("--top-k").arg(settings.top_k.to_string())
            .arg("--repeat-penalty").arg(settings.repeat_penalty.to_string());

        // KV 缓存配置
        if settings.kv_offload {
            cmd.arg("-kvo");
        } else {
            cmd.arg("-nkvo");
        }
        if !settings.cache_type_k.is_empty() {
            cmd.arg("-ctk").arg(&settings.cache_type_k);
        }
        if !settings.cache_type_v.is_empty() {
            cmd.arg("-ctv").arg(&settings.cache_type_v);
        }

        // GPU 与设备分配
        if !settings.gpu_device.is_empty() {
            cmd.arg("--device").arg(&settings.gpu_device);
        }
        if !settings.split_mode.is_empty() && settings.split_mode != "layer" {
            cmd.arg("--split-mode").arg(&settings.split_mode);
        }
        if !settings.tensor_split.is_empty() {
            cmd.arg("--tensor-split").arg(&settings.tensor_split);
        }
        if settings.cpu_moe {
            cmd.arg("--cpu-moe");
        }
        if settings.n_cpu_moe > 0 {
            cmd.arg("--n-cpu-moe").arg(settings.n_cpu_moe.to_string());
        }

        if settings.verbose {
            cmd.arg("--verbose");
        }

        cmd.stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        match cmd.spawn() {
            Ok(child) => {
                {
                    let mut inner = self.inner.lock().unwrap();
                    inner.child = Some(child);
                }

                let inner_clone = Arc::clone(&self.inner);
                let stdout_thread = thread::spawn(move || {
                    let mut inner = inner_clone.lock().unwrap();
                    if let Some(ref mut child) = inner.child {
                        if let Some(stdout) = child.stdout.take() {
                            let reader = BufReader::new(stdout);
                            for line in reader.lines() {
                                match line {
                                    Ok(l) => {
                                        inner.logs.push(LogEntry {
                                            text: l,
                                            level: LogLevel::Info,
                                        });
                                    }
                                    Err(_) => break,
                                }
                            }
                        }
                    }
                });

                let inner_clone2 = Arc::clone(&self.inner);
                let stderr_thread = thread::spawn(move || {
                    let mut inner = inner_clone2.lock().unwrap();
                    if let Some(ref mut child) = inner.child {
                        if let Some(stderr) = child.stderr.take() {
                            let reader = BufReader::new(stderr);
                            for line in reader.lines() {
                                match line {
                                    Ok(l) => {
                                        let level = if l.contains("WARN") || l.contains("warn") {
                                            LogLevel::Warn
                                        } else if l.contains("ERROR") || l.contains("error") {
                                            LogLevel::Error
                                        } else {
                                            LogLevel::Warn
                                        };
                                        inner.logs.push(LogEntry {
                                            text: l,
                                            level,
                                        });
                                    }
                                    Err(_) => break,
                                }
                            }
                        }
                    }
                });

                self._threads.push(stdout_thread);
                self._threads.push(stderr_thread);
            }
            Err(e) => {
                self.state = ServerState::Error(format!("启动失败: {}", e));
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(mut child) = self.inner.lock().unwrap().child.take() {
            self.state = ServerState::Stopping;
            let _ = child.kill();
            let _ = child.wait();
            self.state = ServerState::Idle;
        }
        self._threads.clear();
    }

    pub fn poll_logs(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(ref mut child) = inner.child {
            if let Ok(Some(status)) = child.try_wait() {
                let exit_msg = if status.success() {
                    "Server 进程已正常退出".to_string()
                } else {
                    format!("Server 进程异常退出: {:?}", status.code())
                };
                inner.logs.push(LogEntry {
                    text: exit_msg,
                    level: LogLevel::Warn,
                });
                self.state = ServerState::Idle;
            }
        }
        drop(inner);

        if matches!(self.state, ServerState::Starting) {
            self.state = ServerState::Running;
        }
    }
}

impl Drop for ServerManager {
    fn drop(&mut self) {
        self.stop();
    }
}

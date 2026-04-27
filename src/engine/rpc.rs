use crate::config::settings::AppSettings;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;

/// RPC 服务器运行状态
#[derive(Debug, Clone, PartialEq)]
pub enum RpcState {
    Idle,
    Starting,
    Running,
    Stopping,
    Error(String),
}

/// RPC 连接状态
#[derive(Debug, Clone, PartialEq, Default)]
pub enum RpcConnection {
    #[default]
    Disconnected,
    Connected,
}

/// RPC 服务器内部状态
struct RpcInner {
    child: Option<Child>,
}

pub struct RpcManager {
    state: RpcState,
    connection: RpcConnection,
    inner: Arc<Mutex<RpcInner>>,
    _threads: Vec<thread::JoinHandle<()>>,
}

impl RpcManager {
    pub fn new() -> Self {
        Self {
            state: RpcState::Idle,
            connection: RpcConnection::Disconnected,
            inner: Arc::new(Mutex::new(RpcInner { child: None })),
            _threads: Vec::new(),
        }
    }

    pub fn state(&self) -> RpcState {
        self.state.clone()
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, RpcState::Running)
    }

    pub fn status_text(&self) -> String {
        match &self.state {
            RpcState::Idle => "未启动".to_string(),
            RpcState::Starting => "启动中".to_string(),
            RpcState::Running => "运行中".to_string(),
            RpcState::Stopping => "停止中".to_string(),
            RpcState::Error(msg) => format!("错误: {}", msg),
        }
    }

    /// 检查 rpc-server.exe 文件是否存在
    pub fn check_rpc_server(&self, path: &std::path::Path) -> bool {
        if path.as_os_str().is_empty() {
            return false;
        }
        std::path::Path::new(path).exists()
    }

    /// 启动 rpc-server
    pub fn start(&mut self, settings: &AppSettings) {
        if self.is_running() {
            return;
        }

        let rpc_path = settings.rpc_server_path.clone();

        if rpc_path.as_os_str().is_empty() {
            self.state = RpcState::Error("请先配置 rpc-server 路径".to_string());
            return;
        }

        if !self.check_rpc_server(&rpc_path) {
            self.state = RpcState::Error("rpc-server.exe 文件不存在".to_string());
            return;
        }

        self.state = RpcState::Starting;
        self._threads.clear();

        let mut cmd = Command::new(&rpc_path);
        cmd.arg("--host").arg(&settings.rpc_host)
            .arg("--port").arg(settings.rpc_port.to_string())
            .arg("--threads").arg(settings.rpc_threads.to_string());

        if !settings.rpc_device.is_empty() {
            cmd.arg("--device").arg(&settings.rpc_device);
        }

        if settings.rpc_cache {
            cmd.arg("--cache");
        }

        cmd.stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());

        match cmd.spawn() {
            Ok(child) => {
                {
                    let mut inner = self.inner.lock().unwrap();
                    inner.child = Some(child);
                }

                // 启动成功后更新连接状态
                self.connection = RpcConnection::Connected;

                let inner_clone = Arc::clone(&self.inner);
                let stdout_thread = thread::spawn(move || {
                    let mut inner = inner_clone.lock().unwrap();
                    if let Some(ref mut child) = inner.child {
                        if let Some(stdout) = child.stdout.take() {
                            let reader = BufReader::new(stdout);
                            for line in reader.lines() {
                                match line {
                                    Ok(_) => {}
                                    Err(_) => break,
                                }
                            }
                        }
                    }
                });

                self._threads.push(stdout_thread);
            }
            Err(e) => {
                self.state = RpcState::Error(format!("启动失败: {}", e));
                self.connection = RpcConnection::Disconnected;
            }
        }
    }

    /// 停止 rpc-server
    pub fn stop(&mut self) {
        if let Some(mut child) = self.inner.lock().unwrap().child.take() {
            self.state = RpcState::Stopping;
            let _ = child.kill();
            let _ = child.wait();
            self.state = RpcState::Idle;
            self.connection = RpcConnection::Disconnected;
        }
        self._threads.clear();
    }

    /// 检查 rpc-server 进程状态
    pub fn poll(&mut self) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(ref mut child) = inner.child {
            if let Ok(Some(status)) = child.try_wait() {
                if status.success() {
                    self.state = RpcState::Idle;
                } else {
                    self.state = RpcState::Error(format!("进程异常退出: {:?}", status.code()));
                }
                self.connection = RpcConnection::Disconnected;
            }
        }
        drop(inner);

        if matches!(self.state, RpcState::Starting) {
            self.state = RpcState::Running;
        }
    }
}

impl Drop for RpcManager {
    fn drop(&mut self) {
        self.stop();
    }
}

use crate::config::settings::AppSettings;
use crate::i18n;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

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
    launch_command: Option<String>,
    _threads: Vec<thread::JoinHandle<()>>,
}

impl RpcManager {
    pub fn new() -> Self {
        Self {
            state: RpcState::Idle,
            connection: RpcConnection::Disconnected,
            inner: Arc::new(Mutex::new(RpcInner { child: None })),
            launch_command: None,
            _threads: Vec::new(),
        }
    }

    pub fn state(&self) -> RpcState {
        self.state.clone()
    }

    pub fn is_running(&self) -> bool {
        matches!(self.state, RpcState::Running)
    }

    /// 获取 RPC 启动命令
    pub fn launch_command(&self) -> Option<String> {
        self.launch_command.clone()
    }

    pub fn status_text(&self, lang: &i18n::Language) -> String {
        match &self.state {
            RpcState::Idle => i18n::t(i18n::Key::StatusIdle, lang).to_string(),
            RpcState::Starting => i18n::t(i18n::Key::StatusStarting, lang).to_string(),
            RpcState::Running => i18n::t(i18n::Key::StatusRunning, lang).to_string(),
            RpcState::Stopping => i18n::t(i18n::Key::StatusStopping, lang).to_string(),
            RpcState::Error(msg) => format!("{}: {}", i18n::t(i18n::Key::StatusError, lang), msg),
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
            self.state = RpcState::Error(i18n::t(i18n::Key::ErrRpcPathMissing, &i18n::Language::En).to_string());
            return;
        }

        if !self.check_rpc_server(&rpc_path) {
            self.state = RpcState::Error(i18n::t(i18n::Key::ErrRpcFileNotFound, &i18n::Language::En).to_string());
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

        // Windows: 隐藏子进程的命令行窗口
        #[cfg(target_os = "windows")]
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW

        // 捕获启动命令（在 spawn 消费 cmd 之前）
        let rpc_path_cloned = rpc_path.clone();
        let cmd_line = {
            let args: Vec<String> = cmd.get_args().map(|a| a.to_string_lossy().to_string()).collect();
            format!("{} {}", rpc_path_cloned.display(), args.join(" "))
        };

        match cmd.spawn() {
            Ok(child) => {
                self.launch_command = Some(cmd_line);
                {
                    let mut inner = self.inner.lock().unwrap();
                    inner.child = Some(child);
                }

                // 启动成功后更新连接状态
                self.connection = RpcConnection::Connected;

                let inner_clone = Arc::clone(&self.inner);
                let stdout_thread = thread::spawn(move || {
                    let stdout = {
                        let mut inner = inner_clone.lock().unwrap();
                        if let Some(ref mut child) = inner.child {
                            child.stdout.take()
                        } else {
                            None
                        }
                    };
                    if let Some(stdout) = stdout {
                        let reader = BufReader::new(stdout);
                        for line in reader.lines() {
                            match line {
                                Ok(_) => {}
                                Err(_) => break,
                            }
                        }
                    }
                });

                self._threads.push(stdout_thread);
            }
            Err(e) => {
                self.state = RpcState::Error(format!("{}: {}", i18n::t(i18n::Key::ErrStartFailed, &i18n::Language::En), e));
                self.connection = RpcConnection::Disconnected;
                self.launch_command = None;
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
            self.launch_command = None;
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
                    self.state = RpcState::Error(format!("{}: {:?}", i18n::t(i18n::Key::StatusRpcCrashed, &i18n::Language::En), status.code()));
                    self.launch_command = None;
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

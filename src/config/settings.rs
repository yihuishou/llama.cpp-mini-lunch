use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_DIR: &str = "llama_lunch";
const CONFIG_FILE: &str = "settings.json";
const PRESETS_DIR: &str = "presets";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    // Server 配置
    pub server_path: PathBuf,
    pub host: String,
    pub port: u16,
    pub parallel_slots: usize,

    // 模型
    pub model_path: PathBuf,
    pub mmproj_path: PathBuf,

    // 推理参数
    pub n_ctx: usize,
    pub n_predict: i32,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub repeat_penalty: f32,

    // KV 缓存配置
    pub kv_offload: bool,
    pub cache_type_k: String,
    pub cache_type_v: String,

    // GPU 与设备分配
    pub gpu_device: String,
    pub gpu_layers_str: String,
    pub split_mode: String,
    pub tensor_split: String,
     pub cpu_moe: bool,
    pub n_cpu_moe: usize,

    // RPC 配置
    pub rpc_server_path: PathBuf,
    pub rpc_host: String,
    pub rpc_port: u16,
    pub rpc_threads: usize,
    pub rpc_device: String,
    pub rpc_cache: bool,

    // 高级
    pub verbose: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            server_path: PathBuf::new(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            parallel_slots: 4,
            model_path: PathBuf::new(),
    mmproj_path: PathBuf::new(),
            n_ctx: 4096,
            n_predict: 256,
            temperature: 0.8,
            top_p: 0.95,
            top_k: 40,
            repeat_penalty: 1.1,
            kv_offload: true,
            cache_type_k: "f16".to_string(),
            cache_type_v: "f16".to_string(),
            gpu_device: "".to_string(),
            gpu_layers_str: "99".to_string(),
            split_mode: "layer".to_string(),
            tensor_split: "".to_string(),
            cpu_moe: false,
            n_cpu_moe: 0,
            rpc_server_path: PathBuf::new(),
            rpc_host: "127.0.0.1".to_string(),
            rpc_port: 50052,
            rpc_threads: 12,
            rpc_device: "".to_string(),
            rpc_cache: false,
            verbose: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub settings: AppSettings,
}

pub struct SettingsManager {
    config_dir: PathBuf,
}

impl SettingsManager {
    pub fn new() -> Self {
        let config_dir = std::env::current_exe()
            .map(|p| p.parent().unwrap_or(Path::new("")).to_path_buf())
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(CONFIG_DIR);

        fs::create_dir_all(&config_dir).ok();
        fs::create_dir_all(config_dir.join(PRESETS_DIR)).ok();

        Self { config_dir }
    }

    pub fn load(&self) -> Result<AppSettings, String> {
        let path = self.config_dir.join(CONFIG_FILE);
        if !path.exists() {
            return Ok(AppSettings::default());
        }
        let content = fs::read_to_string(&path).map_err(|e| format!("读取配置失败: {}", e))?;
        serde_json::from_str(&content).map_err(|e| format!("解析配置失败: {}", e))
    }

    pub fn save(&self, settings: &AppSettings) -> Result<(), String> {
        let path = self.config_dir.join(CONFIG_FILE);
        let content = serde_json::to_string_pretty(settings)
            .map_err(|e| format!("序列化配置失败: {}", e))?;
        fs::write(&path, content).map_err(|e| format!("写入配置失败: {}", e))?;
        Ok(())
    }

    pub fn list_presets(&self) -> Vec<Preset> {
        let presets_dir = self.config_dir.join(PRESETS_DIR);
        let mut presets = Vec::new();

        if let Ok(entries) = fs::read_dir(&presets_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(preset) = serde_json::from_str::<Preset>(&content) {
                            presets.push(preset);
                        }
                    }
                }
            }
        }

        presets.sort_by(|a, b| a.name.cmp(&b.name));
        presets
    }

    pub fn save_preset(&self, preset: &Preset) -> Result<(), String> {
        let path = self.config_dir.join(PRESETS_DIR).join(format!("{}.json", preset.name));
        let content = serde_json::to_string_pretty(preset)
            .map_err(|e| format!("序列化预设失败: {}", e))?;
        fs::write(&path, content).map_err(|e| format!("写入预设失败: {}", e))?;
        Ok(())
    }

    pub fn delete_preset(&self, name: &str) -> Result<(), String> {
        let path = self.config_dir.join(PRESETS_DIR).join(format!("{}.json", name));
        fs::remove_file(&path).map_err(|e| format!("删除预设失败: {}", e))?;
        Ok(())
    }

  /// 在可执行文件所在目录查找指定名称的可执行文件
    pub fn locate_exe(&self, name: &str) -> Option<PathBuf> {
        let exe_dir = self.config_dir.parent()?;
        let filename = if cfg!(target_os = "windows") {
            format!("{}.exe", name)
        } else {
            name.to_string()
        };
        let path = exe_dir.join(&filename);
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    /// 自动检测 llama-server 路径
    pub fn auto_detect_server_path(&self) -> Option<PathBuf> {
        self.locate_exe("llama-server")
    }

    /// 自动检测 rpc-server 路径
    pub fn auto_detect_rpc_path(&self) -> Option<PathBuf> {
        self.locate_exe("rpc-server")
    }
}

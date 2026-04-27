# 实现计划：mmproj 多模态投影 + 路径自动检测

## 功能 1：mmproj 多模态模型投影

### 1.1 `settings.rs` - 添加字段
```
文件: src/config/settings.rs
位置: 第18行（model_path 之后）
变更:
  - 新增字段: pub mmproj_path: PathBuf
  - Default 中添加: mmproj_path: PathBuf::new()
```

### 1.2 `model_panel.rs` - 添加 mmproj UI
```
文件: src/ui/model_panel.rs
位置: 第47行（模型信息之后，函数结束前）
变更:
  - 添加 ui.add_space(12.0)
  - 添加 ui.heading("多模态")
  - 添加 ui.separator()
  - 添加 mmproj 文件选择器（浏览按钮 + 文本输入框）
  - 过滤器:  "*.gguf,*.bin"
  - 文件存在性验证提示（同 model_path 的验证逻辑）
```

### 1.3 `server.rs` - 传递 --mmproj 参数
```
文件: src/engine/server.rs
位置: 第106行（repeat-penalty 之后）
变更:
  - 添加条件判断: if !settings.mmproj_path.as_os_str().is_empty()
  - 传递参数: cmd.arg("--mmproj").arg(&settings.mmproj_path)
```

---

## 功能 2：Server + RPC 路径自动检测

### 2.1 `settings.rs` - 添加自动检测方法
```
文件: src/config/settings.rs
位置: SettingsManager impl 块末尾（第160行之后）
变更:
  - 新增方法: fn locate_exe(name: &str) -> Option<PathBuf>
    仅检查当前可执行文件所在目录（self.config_dir 的父目录）
    查找文件: {name}.exe (Windows) 或 {name} (其他平台)

  - 新增方法: fn auto_detect_server_path(&self) -> Option<PathBuf>
    调用 locate_exe("llama-server")

  - 新增方法: fn auto_detect_rpc_path(&self) -> Option<PathBuf>
    调用 locate_exe("rpc-server")
```

### 2.2 `server_panel.rs` - 添加自动检测按钮
```
文件: src/ui/server_panel.rs
位置: 第9-19行（"浏览..."按钮旁边）
变更:
  - 在水平布局中添加 "自动检测" 按钮
  - 点击时调用 SettingsManager::auto_detect_server_path()
  - 找到时设置 settings.server_path，未找到时弹提示
```

注意: server_panel.rs 的 ui 函数签名需要增加 SettingsManager 参数，
或者将自动检测逻辑直接内联在 UI 中。

方案 A: 修改函数签名，传入 &SettingsManager
方案 B: 在 server_panel 内部实例化 SettingsManager（简单但重复创建）
方案 C: 将自动检测逻辑提取为独立模块函数

推荐方案 A，需要同时修改 app.rs 中的调用处。

### 2.3 `rpc_panel.rs` - 添加自动检测按钮
```
文件: src/ui/rpc_panel.rs
位置: 第10-18行（"浏览..."按钮旁边）
变更:
  - 在水平布局中添加 "自动检测" 按钮
  - 点击时调用 SettingsManager::auto_detect_rpc_path()
  - 找到时设置 settings.rpc_server_path，未找到时弹提示
```

同样需要修改函数签名传入 &SettingsManager。

### 2.4 `app.rs` - 传递 SettingsManager
```
文件: src/app.rs
位置: 第182-183行
变更:
  - server_panel::ui(ui, &mut self.settings)
    → server_panel::ui(ui, &mut self.settings, &self.settings_manager)
  - rpc_panel::ui(ui, &mut self.settings)
    → rpc_panel::ui(ui, &mut self.settings, &self.settings_manager)
```

---

## 执行顺序

1. settings.rs - 添加 mmproj_path 字段 + 自动检测方法
2. model_panel.rs - 添加 mmproj UI
3. server.rs - 传递 --mmproj 参数
4. server_panel.rs - 添加自动检测按钮 + 修改签名
5. rpc_panel.rs - 添加自动检测按钮 + 修改签名
6. app.rs - 修改调用传递 SettingsManager
7. cargo build 验证编译

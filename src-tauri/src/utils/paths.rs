use std::path::PathBuf;
use std::fs;

pub struct AppPaths {
    pub storage: PathBuf,
    pub auth: PathBuf,
    pub db: PathBuf,
    pub cursor_exe: PathBuf,
}

impl AppPaths {
    pub fn new() -> Result<Self, String> {
        let base_dir = if cfg!(target_os = "windows") {
            // Windows: %APPDATA%\Cursor\User\globalStorage
            let app_data = std::env::var("APPDATA")
                .map_err(|e| format!("获取 APPDATA 路径失败: {}", e))?;
            PathBuf::from(app_data).join("Cursor")
        } else if cfg!(target_os = "macos") {
            // macOS: ~/Library/Application Support/Cursor/User/globalStorage
            let home = std::env::var("HOME")
                .map_err(|e| format!("获取 HOME 路径失败: {}", e))?;
            PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("Cursor")
        } else if cfg!(target_os = "linux") {
            // Linux: ~/.config/Cursor/User/globalStorage
            let home = std::env::var("HOME")
                .map_err(|e| format!("获取 HOME 路径失败: {}", e))?;
            PathBuf::from(home)
                .join(".config")
                .join("Cursor")
        } else {
            return Err(format!("不支持的操作系统: {}", std::env::consts::OS));
        };

        let global_storage = base_dir
            .join("User")
            .join("globalStorage");

        // 获取 Cursor 可执行文件路径
        let cursor_exe = if cfg!(target_os = "windows") {
            let local_app_data = std::env::var("LOCALAPPDATA")
                .map_err(|e| format!("获取 LOCALAPPDATA 路径失败: {}", e))?;
            PathBuf::from(local_app_data)
                .join("Programs")
                .join("cursor")
                .join("Cursor.exe")
        } else if cfg!(target_os = "macos") {
            PathBuf::from("/Applications")
                .join("Cursor.app")
                .join("Contents")
                .join("MacOS")
                .join("Cursor")
        } else {
            PathBuf::from("/usr/bin/cursor")  // Linux 默认路径
        };

        let paths = Self {
            storage: global_storage.join("storage.json"),
            auth: global_storage.join("cursor.auth.json"),
            db: global_storage.join("state.vscdb"),
            cursor_exe,
        };

        // 确保目录存在
        if let Some(parent) = paths.storage.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("创建目录失败: {}", e))?;
        }

        Ok(paths)
    }

    // 确保父目录存在
    pub fn ensure_parent_exists(&self, path: &PathBuf) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("创建目录失败: {}", e))?;
        }
        Ok(())
    }

    // 启动 Cursor
    pub fn launch_cursor(&self) -> Result<(), String> {
        if !self.cursor_exe.exists() {
            return Err("Cursor 可执行文件不存在".to_string());
        }

        std::process::Command::new(&self.cursor_exe)
            .spawn()
            .map_err(|e| format!("启动 Cursor 失败: {}", e))?;

        Ok(())
    }
}

use std::path::{ PathBuf };
use std::fs;

pub struct AppPaths {
    pub storage: PathBuf,
    pub auth: PathBuf,
    pub db: PathBuf,
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

        let paths = Self {
            storage: global_storage.join("storage.json"),
            auth: global_storage.join("cursor.auth.json"),
            db: global_storage.join("state.vscdb"),
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
}

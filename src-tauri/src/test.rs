use std::env;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_updater_path() {
        // 获取 LOCALAPPDATA 环境变量
        let local_app_data = match env::var("LOCALAPPDATA") {
            Ok(path) => path,
            Err(_) => {
                println!("无法获取 LOCALAPPDATA 环境变量");
                return;
            }
        };

        // 构建 cursor-updater 完整路径
        let cursor_updater_path = Path::new(&local_app_data).join("cursor-updater");
        
        // 检查路径是否存在
        if !cursor_updater_path.exists() {
            println!("cursor-updater 路径不存在: {:?}", cursor_updater_path);
            return;
        }

        // 检查是文件还是目录
        if cursor_updater_path.is_file() {
            println!("cursor-updater 是一个文件");
        } else if cursor_updater_path.is_dir() {
            println!("cursor-updater 是一个目录");
            
            // 可选:列出目录内容
            if let Ok(entries) = std::fs::read_dir(&cursor_updater_path) {
                println!("\n目录内容:");
                for entry in entries {
                    if let Ok(entry) = entry {
                        println!("- {:?}", entry.path());
                    }
                }
            }
        }
    }
}

// 公开函数用于在非测试环境中使用
pub fn check_cursor_updater_path() -> Result<String, String> {
    let local_app_data = env::var("LOCALAPPDATA")
        .map_err(|e| format!("无法获取 LOCALAPPDATA 环境变量: {}", e))?;

    let cursor_updater_path = Path::new(&local_app_data).join("cursor-updater");

    if !cursor_updater_path.exists() {
        return Err(format!("cursor-updater 路径不存在: {:?}", cursor_updater_path));
    }

    let path_type = if cursor_updater_path.is_file() {
        "文件"
    } else if cursor_updater_path.is_dir() {
        "目录"
    } else {
        "未知类型"
    };

    Ok(format!("cursor-updater 是一个 {}", path_type))
} 
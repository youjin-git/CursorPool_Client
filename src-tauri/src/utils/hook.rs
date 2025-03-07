use std::fs;
use lazy_static::lazy_static;
use regex::Regex;
use crate::utils::paths::AppPaths;
use crate::api::client::ApiClient;
use crate::utils::ErrorReporter;
use tauri::State;

lazy_static! {
    /// machineId 函数匹配模式
    static ref MACHINE_ID_REGEX: Regex = Regex::new(
        r#"async\s+(\w+)\s*\(\)\s*\{\s*return\s+this\.[\w.]+(?:\?\?|\?)\s*this\.([\w.]+)\.machineId\s*\}"#
    ).unwrap();

    /// macMachineId 函数匹配模式
    static ref MAC_MACHINE_ID_REGEX: Regex = Regex::new(
        r#"async\s+(\w+)\s*\(\)\s*\{\s*return\s+this\.[\w.]+(?:\?\?|\?)\s*this\.([\w.]+)\.macMachineId\s*\}"#
    ).unwrap();
}

pub struct Hook;

impl Hook {
    /// 获取 machineId 正则表达式
    pub fn machine_id_regex() -> &'static Regex {
        &MACHINE_ID_REGEX
    }

    /// 获取 macMachineId 正则表达式
    pub fn mac_machine_id_regex() -> &'static Regex {
        &MAC_MACHINE_ID_REGEX
    }

    /// 读取 main.js 文件内容
    fn get_main_js_content() -> Result<String, String> {
        let paths = AppPaths::new()?;
        fs::read_to_string(&paths.main_js)
            .map_err(|e| format!("读取 main.js 失败: {}", e))
    }

    /// 更新 main.js 文件内容
    pub async fn update_main_js_content(client: Option<State<'_, ApiClient>>) -> Result<(), String> {
        let paths = AppPaths::new()?;
        let file_path = &paths.main_js;
        
        // 读取文件内容
        let content = Self::get_main_js_content()?;
        
        // 创建备份
        let backup_path = file_path.with_extension("js.backup");
        if !backup_path.exists() {
            if let Err(e) = fs::write(&backup_path, &content) {
                let err_msg = format!("创建备份失败: {}", e);
                if let Some(ref client) = client {
                    ErrorReporter::report_error(
                        client.clone(),
                        "create_backup",
                        &err_msg,
                        None,
                        Some("high".to_string())
                    ).await;
                }
                return Err(err_msg);
            }
        }

        // 使用正则表达式进行替换
        let machine_id_matches = MACHINE_ID_REGEX.find_iter(&content).count();
        let mac_machine_id_matches = MAC_MACHINE_ID_REGEX.find_iter(&content).count();
        
        if machine_id_matches == 0 || mac_machine_id_matches == 0 {
            return Err("无法找到匹配的 machineId 或 macMachineId 函数".to_string());
        }

        // 生成新的随机 ID
        let new_id = uuid::Uuid::new_v4().to_string();
        
        // 替换 machineId
        let modified_content = MACHINE_ID_REGEX.replace_all(&content, |caps: &regex::Captures| {
            format!("async {}() {{ return \"{}\" }}", &caps[1], new_id)
        }).to_string();
        
        // 替换 macMachineId
        let modified_content = MAC_MACHINE_ID_REGEX.replace_all(&modified_content, |caps: &regex::Captures| {
            format!("async {}() {{ return \"{}\" }}", &caps[1], new_id)
        }).to_string();
        
        // 写入修改后的内容
        if let Err(e) = fs::write(file_path, &modified_content) {
            let err_msg = format!("写入修改后的内容失败: {}", e);
            if let Some(ref client) = client {
                ErrorReporter::report_error(
                    client.clone(),
                    "write_modified_file",
                    &err_msg,
                    None,
                    Some("high".to_string())
                ).await;
            }
            return Err(err_msg);
        }
        
        Ok(())
    }

    /// 从备份恢复 main.js
    pub async fn restore_from_backup(client: Option<State<'_, ApiClient>>) -> Result<(), String> {
        let paths = AppPaths::new()?;
        let file_path = &paths.main_js;
        let backup_path = file_path.with_extension("js.backup");

        if !backup_path.exists() {
            let err_msg = "备份文件不存在".to_string();
            if let Some(ref client) = client {
                ErrorReporter::report_error(
                    client.clone(),
                    "restore_from_backup",
                    &err_msg,
                    None,
                    Some("medium".to_string())
                ).await;
            }
            return Err(err_msg);
        }

        let backup_content = match fs::read_to_string(&backup_path) {
            Ok(content) => content,
            Err(e) => {
                let err_msg = format!("读取备份文件失败: {}", e);
                if let Some(ref client) = client {
                    ErrorReporter::report_error(
                        client.clone(),
                        "restore_from_backup",
                        &err_msg,
                        None,
                        Some("medium".to_string())
                    ).await;
                }
                return Err(err_msg);
            }
        };

        if let Err(e) = fs::write(file_path, &backup_content) {
            let err_msg = format!("恢复文件失败: {}", e);
            if let Some(ref client) = client {
                ErrorReporter::report_error(
                    client.clone(),
                    "restore_from_backup",
                    &err_msg,
                    None,
                    Some("medium".to_string())
                ).await;
            }
            return Err(err_msg);
        }

        if let Err(e) = fs::remove_file(backup_path) {
            let err_msg = format!("删除备份文件失败: {}", e);
            if let Some(ref client) = client {
                ErrorReporter::report_error(
                    client.clone(),
                    "restore_from_backup",
                    &err_msg,
                    None,
                    Some("low".to_string())
                ).await;
            }
        }

        Ok(())
    }
} 
use serde_json::{json, Value};
use std::fs;
use rusqlite::Connection;
use crate::utils::paths::AppPaths;
use crate::utils::id_generator::generate_new_ids;
use crate::utils::ProcessManager;
use crate::utils::hook::Hook;
use crate::utils::file_utils::safe_write;
use crate::utils::ErrorReporter;
use crate::api::client::ApiClient;
use tauri::State;

/// 终止 Cursor 进程
#[tauri::command]
pub async fn close_cursor() -> Result<bool, String> {
    let process_manager = ProcessManager::new();
    
    // 检查Cursor是否在运行
    if !process_manager.is_cursor_running() {
        return Ok(false); // Cursor未运行，无需关闭
    }
    
    // 关闭Cursor进程
    process_manager.kill_cursor_processes()?;
    
    // 等待进程完全关闭
    let mut attempts = 0;
    while process_manager.is_cursor_running() && attempts < 10 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        attempts += 1;
    }
    
    // 检查是否成功关闭
    if process_manager.is_cursor_running() {
        return Err("无法完全关闭Cursor进程".to_string());
    }
    
    Ok(true)
}

/// 启动 Cursor 应用
#[tauri::command]
pub async fn launch_cursor() -> Result<bool, String> {
    let paths = AppPaths::new()?;
    
    // 启动Cursor
    paths.launch_cursor()?;
    
    Ok(true)
}

/// 重置设备标识符
#[tauri::command]
pub async fn reset_machine_id(
    client: State<'_, ApiClient>,
    force_kill: bool,
    machine_id: Option<String>
) -> Result<bool, String> {
    let process_manager = ProcessManager::new();
    
    // 检查Cursor进程
    if !force_kill && process_manager.is_cursor_running() {
        return Err("Cursor进程正在运行, 请先关闭Cursor".to_string());
    }

    // 如果force_kill为true, 则强制终止Cursor进程
    if force_kill {
        match process_manager.kill_cursor_processes() {
            Ok(_) => {},
            Err(e) => {
                // 上报错误
                ErrorReporter::report_error(
                    client.clone(),
                    "reset_machine_id",
                    &e,
                    None,
                    Some("low".to_string())
                ).await;
                return Err(e);
            }
        }
    }

    let paths = match AppPaths::new() {
        Ok(p) => p,
        Err(e) => {
            // 上报错误
            ErrorReporter::report_error(
                client.clone(),
                "reset_machine_id",
                &e,
                None,
                Some("low".to_string())
            ).await;
            return Err(e);
        }
    };
    
    let new_ids = if let Some(id) = machine_id {
        // 生成随机 ID
        let mut ids = generate_new_ids();
        // 替换 devDeviceId
        ids.insert("telemetry.devDeviceId".to_string(), id);
        ids
    } else {
        // 否则随机生成
        generate_new_ids()
    };

    // 更新 storage.json
    let mut storage_content = if paths.storage.exists() {
        let content = match fs::read_to_string(&paths.storage) {
            Ok(c) => c,
            Err(e) => {
                let err = format!("读取 storage.json 失败: {}", e);
                ErrorReporter::report_error(
                    client.clone(),
                    "reset_machine_id",
                    &err,
                    None,
                    Some("low".to_string())
                ).await;
                return Err(err);
            }
        };
        
        match serde_json::from_str(&content) {
            Ok(c) => c,
            Err(e) => {
                let err = format!("解析 storage.json 失败: {}", e);
                ErrorReporter::report_error(
                    client.clone(),
                    "reset_machine_id",
                    &err,
                    None,
                    Some("low".to_string())
                ).await;
                return Err(err);
            }
        }
    } else {
        json!({})
    };

    if let Value::Object(ref mut map) = storage_content {
        map.insert("telemetry.devDeviceId".to_string(), Value::String(new_ids.get("telemetry.devDeviceId").unwrap().clone()));
        map.insert("telemetry.macMachineId".to_string(), Value::String(new_ids.get("telemetry.macMachineId").unwrap().clone()));
        map.insert("telemetry.machineId".to_string(), Value::String(new_ids.get("telemetry.machineId").unwrap().clone()));
        map.insert("telemetry.sqmId".to_string(), Value::String(new_ids.get("telemetry.sqmId").unwrap().clone()));
    }

    // 使用 safe_write 代替 fs::write
    let storage_content_str = match serde_json::to_string_pretty(&storage_content) {
        Ok(s) => s,
        Err(e) => {
            let err = format!("序列化 storage.json 失败: {}", e);
            ErrorReporter::report_error(
                client.clone(),
                "reset_machine_id",
                &err,
                None,
                Some("low".to_string())
            ).await;
            return Err(err);
        }
    };
    
    if let Err(e) = safe_write(&paths.storage, &storage_content_str) {
        let err = format!("写入 storage.json 失败: {}", e);
        ErrorReporter::report_error(
            client.clone(),
            "reset_machine_id",
            &err,
            None,
            Some("low".to_string())
        ).await;
        return Err(err);
    }

    // 更新数据库
    if paths.db.exists() {
        let updates = vec![
            ("device_id", new_ids.get("telemetry.devDeviceId").unwrap()),
            ("mac_id", new_ids.get("telemetry.macMachineId").unwrap()),
            ("machineId", new_ids.get("telemetry.machineId").unwrap()),
            ("sqm_id", new_ids.get("telemetry.sqmId").unwrap())
        ];
        
        if let Err(e) = update_database(&paths.db, &updates) {
            ErrorReporter::report_error(
                client.clone(),
                "reset_machine_id",
                &e,
                None,
                Some("low".to_string())
            ).await;
            return Err(e);
        }
    }

    // 移除自动启动Cursor的代码
    Ok(true)
}

/// 切换用户账号
#[tauri::command]
pub async fn switch_account(
    email: String,
    token: String,
    force_kill: bool
) -> Result<bool, String> {
    let process_manager = ProcessManager::new();
    
    // 检查Cursor进程
    if !force_kill && process_manager.is_cursor_running() {
        return Err("Cursor进程正在运行, 请先关闭Cursor".to_string());
    }

    // 如果force_kill为true, 则强制终止Cursor进程
    if force_kill {
        process_manager.kill_cursor_processes()?;
    }

    let paths = AppPaths::new()?;

    let account_updates = vec![
        ("cursor.email", email.clone()),
        ("cursor.accessToken", token.clone()),
        ("cursorAuth/refreshToken", token.clone()),
        ("cursorAuth/accessToken", token.clone()),
        ("cursorAuth/cachedEmail", email),
    ];

    update_database(&paths.db, &account_updates)?;

    // 等待一段时间确保数据库更新完成
    std::thread::sleep(std::time::Duration::from_millis(500));

    Ok(true)
}

/// 获取设备标识符和当前账号信息
#[tauri::command]
pub fn get_machine_ids() -> Result<Value, String> {
    let paths = AppPaths::new()?;
    let mut result = json!({
        "machineId": "",
        "currentAccount": ""
    });

    // 从数据库读取机器码和 Cursor 邮箱
    if paths.db.exists() {
        if let Ok(conn) = Connection::open(&paths.db) {
            // 读取机器码
            if let Ok(mut stmt) = conn.prepare("SELECT value FROM ItemTable WHERE key = 'telemetry.devDeviceId'") {
                if let Ok(mut rows) = stmt.query([]) {
                    if let Ok(Some(row)) = rows.next() {
                        if let Ok(device_id) = row.get::<_, String>(0) {
                            result["machineId"] = json!(device_id);
                        }
                    }
                }
            }

            // 读取 Cursor 邮箱
            if let Ok(mut stmt) = conn.prepare("SELECT value FROM ItemTable WHERE key = 'cursorAuth/cachedEmail'") {
                if let Ok(mut rows) = stmt.query([]) {
                    if let Ok(Some(row)) = rows.next() {
                        if let Ok(email) = row.get::<_, String>(0) {
                            result["currentAccount"] = json!(email);
                        }
                    }
                }
            }

            // 读取cursor token
            if let Ok(mut stmt) = conn.prepare("SELECT value FROM ItemTable WHERE key = 'cursorAuth/refreshToken'") {
                if let Ok(mut rows) = stmt.query([]) {
                    if let Ok(Some(row)) = rows.next() {
                        if let Ok(token) = row.get::<_, String>(0) {
                            result["cursorToken"] = json!(token);
                        }
                    }
                }
            }
        }
    }

    Ok(result)
}

/// 检查 Cursor 进程状态
#[tauri::command]
pub fn check_cursor_running() -> Result<bool, String> {
    let process_manager = ProcessManager::new();
    Ok(process_manager.is_cursor_running())
}

/// 检查管理员权限
#[tauri::command]
pub fn check_admin_privileges() -> Result<bool, String> {
    crate::utils::check_admin_privileges()
}

/// 请求管理员权限
#[tauri::command]
pub fn request_admin_privileges(exe_path: String) -> Result<bool, String> {
    crate::utils::privileges::request_admin_privileges(&exe_path)
}

/// 更新数据库键值对
fn update_database(db_path: &std::path::Path, updates: &[(impl AsRef<str>, impl AsRef<str>)]) -> Result<(), String> {
    let conn = Connection::open(db_path)
        .map_err(|e| format!("打开数据库失败: {}", e))?;

    for (key, value) in updates {
        let key = match key.as_ref() {
            "device_id" => "telemetry.devDeviceId",
            "mac_id" => "telemetry.macMachineId",
            "machineId" => "telemetry.machineId",
            "sqm_id" => "telemetry.sqmId",
            _ => key.as_ref(),
        };

        // 先尝试更新已存在的记录
        let result = conn.execute(
            "UPDATE ItemTable SET value = ?1 WHERE key = ?2",
            [value.as_ref(), key]
        );

        // 如果记录不存在（没有更新任何行）, 则插入新记录
        if let Ok(0) = result {
            conn.execute(
                "INSERT INTO ItemTable (key, value) VALUES (?1, ?2)",
                [key, value.as_ref()]
            ).map_err(|e| format!("插入数据失败: {}", e))?;
        } else {
            result.map_err(|e| format!("更新数据失败: {}", e))?;
        }
    }

    Ok(())
}

/// 检查 main.js 注入状态
#[tauri::command]
pub async fn is_hook() -> Result<bool, String> {
    let paths = AppPaths::new()?;
    let content = fs::read_to_string(&paths.main_js)
        .map_err(|e| format!("读取 main.js 失败: {}", e))?;

    // 检查正则匹配
    let machine_id_matches = Hook::machine_id_regex().find_iter(&content).count();
    let mac_machine_id_matches = Hook::mac_machine_id_regex().find_iter(&content).count();

    // 如果找不到匹配，说明已经被 hook 了
    if machine_id_matches == 0 || mac_machine_id_matches == 0 {
        return Ok(true);
    }

    Ok(false)
}

/// 注入 main.js 文件
#[tauri::command]
pub async fn hook_main_js(
    client: State<'_, ApiClient>,
    force_kill: bool
) -> Result<(), String> {
    let process_manager = ProcessManager::new();
    
    // 检查 Cursor 进程
    if !force_kill && process_manager.is_cursor_running() {
        return Err("Cursor进程正在运行, 请先关闭Cursor".to_string());
    }

    // 如果 force_kill 为 true, 则强制终止 Cursor 进程
    if force_kill {
        match process_manager.kill_cursor_processes() {
            Ok(_) => {},
            Err(e) => {
                // 上报错误
                ErrorReporter::report_error(
                    client.clone(),
                    "hook_main_js",
                    &e,
                    None,
                    Some("medium".to_string())
                ).await;
                return Err(e);
            }
        }
    }

    // 执行 hook 操作，传递 client 用于错误上报
    Hook::update_main_js_content(Some(client)).await
}

/// 恢复 main.js 原始内容
#[tauri::command]
pub async fn restore_hook(
    client: State<'_, ApiClient>,
    force_kill: bool
) -> Result<(), String> {
    let process_manager = ProcessManager::new();
    
    // 检查 Cursor 进程
    if !force_kill && process_manager.is_cursor_running() {
        return Err("Cursor进程正在运行, 请先关闭Cursor".to_string());
    }

    // 如果 force_kill 为 true, 则强制终止 Cursor 进程
    if force_kill {
        match process_manager.kill_cursor_processes() {
            Ok(_) => {},
            Err(e) => {
                // 上报错误
                ErrorReporter::report_error(
                    client.clone(),
                    "restore_hook",
                    &e,
                    None,
                    Some("medium".to_string())
                ).await;
                return Err(e);
            }
        }
    }

    // 执行恢复操作，传递 client 用于错误上报
    Hook::restore_from_backup(Some(client)).await
}

/// 检查操作系统是否为 Windows
#[tauri::command]
pub fn check_is_windows() -> bool {
    crate::utils::privileges::is_windows()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_reset_machine_id() {
        println!("开始测试重置机器码...");
        
        let old_ids = match get_machine_ids() {
            Ok(ids) => {
                println!("重置前机器ID: {:?}", ids);
                ids
            },
            Err(e) => {
                println!("获取机器ID失败: {}", e);
                return;
            }
        };
        
        let paths = match AppPaths::new() {
            Ok(p) => p,
            Err(e) => {
                println!("获取应用路径失败: {}", e);
                return;
            }
        };
        
        let mut new_ids = HashMap::new();
        let new_device_id = format!("test-device-id-{}", chrono::Local::now().timestamp());
        let new_machine_id = format!("test-machine-id-{}", chrono::Local::now().timestamp());
        let new_mac_id = format!("test-mac-id-{}", chrono::Local::now().timestamp());
        let new_sqm_id = format!("test-sqm-id-{}", chrono::Local::now().timestamp());
        
        new_ids.insert("telemetry.devDeviceId".to_string(), new_device_id.clone());
        new_ids.insert("telemetry.machineId".to_string(), new_machine_id);
        new_ids.insert("telemetry.macMachineId".to_string(), new_mac_id);
        new_ids.insert("telemetry.sqmId".to_string(), new_sqm_id);
        
        println!("生成的新机器ID: {:?}", new_ids);
        
        let mut storage_content = if paths.storage.exists() {
            match fs::read_to_string(&paths.storage) {
                Ok(content) => match serde_json::from_str(&content) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        println!("解析storage.json失败: {}", e);
                        json!({})
                    }
                },
                Err(e) => {
                    println!("读取storage.json失败: {}", e);
                    json!({})
                }
            }
        } else {
            json!({})
        };
        
        if let Value::Object(ref mut map) = storage_content {
            for (key, value) in &new_ids {
                map.insert(key.clone(), Value::String(value.clone()));
            }
        }
        
        match fs::write(&paths.storage, serde_json::to_string_pretty(&storage_content).unwrap()) {
            Ok(_) => println!("成功写入新机器ID"),
            Err(e) => println!("写入storage.json失败: {}", e)
        }
        
        match get_machine_ids() {
            Ok(ids) => {
                println!("重置后机器ID: {:?}", ids);
                if let Some(old_id) = old_ids["machineId"].as_str() {
                    if let Some(new_id) = ids["machineId"].as_str() {
                        if old_id != new_id {
                            println!("✅ 机器ID已成功更新!");
                        } else {
                            println!("❌ 机器ID未更新!");
                        }
                    }
                }
            },
            Err(e) => println!("获取更新后的机器ID失败: {}", e)
        }
        
        println!("机器码重置测试完成");
    }
    
    #[tokio::test]
    async fn test_switch_account() {
        println!("开始测试切换账户...");
        
        let test_email = "test@example.com";
        let test_token = "your_actual_token";
        
        println!("测试账户: {}", test_email);
        println!("测试Token: {}", test_token);
        
        println!("开始调用switch_account...");
        let force_kill = true;
        
        match switch_account(test_email.to_string(), test_token.to_string(), force_kill).await {
            Ok(_) => println!("✅ 成功切换到账户: {}", test_email),
            Err(e) => println!("❌ 切换账户失败: {}", e)
        }
        
        match get_machine_ids() {
            Ok(info) => {
                println!("切换后账户信息: {:?}", info);
                if let Some(current_account) = info["currentAccount"].as_str() {
                    if current_account == test_email {
                        println!("✅ 账户已成功切换为: {}", test_email);
                    } else {
                        println!("❌ 账户未切换为预期的值! 当前: {}", current_account);
                    }
                }
            },
            Err(e) => println!("获取更新后的账户信息失败: {}", e)
        }
        
        println!("切换账户测试完成");
    }
}
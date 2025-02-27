use serde_json::{json, Value};
use std::fs;
use rusqlite::Connection;
use crate::utils::paths::AppPaths;
use crate::utils::id_generator::generate_new_ids;
use crate::utils::ProcessManager;
use std::thread;
use crate::utils::UpdateBlocker;
use crate::utils::hook::Hook;
use crate::utils::file_utils::safe_write;
use std::path::PathBuf;
use crate::utils::ErrorReporter;
use crate::api::client::ApiClient;
use tauri::State;

#[tauri::command]
pub async fn reset_machine_id(
    client: State<'_, ApiClient>,
    force_kill: bool
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
    
    let new_ids = generate_new_ids();

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

    Ok(true)
}

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

    Ok(true)
}

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

#[tauri::command]
pub async fn kill_cursor_process() -> Result<(), String> {
    let process_manager = ProcessManager::new();
    
    // 启动新线程执行关闭和重启操作
    thread::spawn(move || {
        process_manager.kill_and_restart_cursor()
    });
    
    Ok(())
}

#[tauri::command]
pub fn check_cursor_running() -> Result<bool, String> {
    let process_manager = ProcessManager::new();
    Ok(process_manager.is_cursor_running())
}

#[tauri::command]
pub fn check_admin_privileges() -> Result<bool, String> {
    crate::utils::check_admin_privileges()
}

#[tauri::command]
pub fn request_admin_privileges(exe_path: String) -> Result<bool, String> {
    crate::utils::privileges::request_admin_privileges(&exe_path)
}

/// 禁用 Cursor 自动更新
#[tauri::command]
pub async fn disable_cursor_update(force_kill: bool) -> Result<(), String> {
    let process_manager = ProcessManager::new();
    
    // 检查 Cursor 进程
    if !force_kill && process_manager.is_cursor_running() {
        return Err("Cursor进程正在运行, 请先关闭Cursor".to_string());
    }

    // 如果 force_kill 为 true, 则强制终止 Cursor 进程
    if force_kill {
        process_manager.kill_cursor_processes()?;
    }

    let paths = AppPaths::new()?;
    let blocker = UpdateBlocker::new();
    
    match blocker.disable_auto_update(&paths.cursor_updater) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("禁用自动更新失败: {}", e))
    }
}

/// 恢复 Cursor 自动更新
#[tauri::command]
pub async fn restore_cursor_update(force_kill: bool) -> Result<(), String> {
    let process_manager = ProcessManager::new();
    
    // 检查 Cursor 进程
    if !force_kill && process_manager.is_cursor_running() {
        return Err("Cursor进程正在运行, 请先关闭Cursor".to_string());
    }

    // 如果 force_kill 为 true, 则强制终止 Cursor 进程
    if force_kill {
        process_manager.kill_cursor_processes()?;
    }

    let paths = AppPaths::new()?;
    let blocker = UpdateBlocker::new();
    
    match blocker.restore_auto_update(&paths.cursor_updater) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("恢复自动更新失败: {}", e))
    }
}

/// 检查 Cursor 是否被禁止更新
#[tauri::command]
pub fn check_update_disabled() -> Result<bool, String> {
    let paths = AppPaths::new()?;
    
    match std::env::consts::OS {
        "macos" => {
            // macOS 下检查 app-update.yml 文件状态和 main.js.backup 是否存在
            let app_update_path = PathBuf::from("/Applications/Cursor.app/Contents/Resources/app-update.yml");
            let main_js_backup = paths.main_js.with_extension("js.backup");
            
            if !app_update_path.exists() {
                return Ok(false);
            }

            // 检查 app-update.yml 是否为只读
            let is_readonly = crate::utils::file_utils::is_read_only(&app_update_path)
                .map_err(|e| format!("检查文件权限失败: {}", e))?;

            // 同时检查备份文件是否存在和文件是否只读
            Ok(is_readonly && main_js_backup.exists())
        },
        "windows" | "linux" => {
            // Windows 和 Linux 下检查更新器路径
            if !paths.cursor_updater.exists() {
                return Ok(false);
            }
            // 如果是文件而不是目录, 说明已被禁用
            Ok(!paths.cursor_updater.is_dir())
        },
        _ => Err("不支持的操作系统".to_string()),
    }
}

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

/// 检查 main.js 是否已被 hook
#[tauri::command]
pub async fn is_hook() -> Result<bool, String> {
    let paths = AppPaths::new()?;
    let content = fs::read_to_string(&paths.main_js)
        .map_err(|e| format!("读取 main.js 失败: {}", e))?;

    // 检查正则匹配
    let machine_id_matches = Hook::machine_id_regex().find_iter(&content).count();
    let mac_machine_id_matches = Hook::mac_machine_id_regex().find_iter(&content).count();

    // 如果找不到匹配, 说明已经被 hook 了
    if machine_id_matches == 0 || mac_machine_id_matches == 0 {
        return Ok(true);
    }

    // 从远程获取所有可能的行数
    let line_counts = Hook::get_all_line_counts_with_remote().await?;
    
    for &count in &line_counts {
        let hash = Hook::calculate_md5_without_last_lines(&content, count);
        if Hook::main_js_md5().contains_key(hash.as_str()) {
            return Ok(false);
        }
    }
    
    // 如果没有匹配的哈希，说明版本不兼容
    Ok(false)
}

/// Hook main.js 文件
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

/// 从备份恢复 main.js 文件
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

#[tauri::command]
pub fn check_is_windows() -> bool {
    crate::utils::privileges::is_windows()
}
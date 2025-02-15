use serde_json::{json, Value};
use std::fs;
use rusqlite::Connection;
use crate::utils::paths::AppPaths;
use crate::utils::id_generator::generate_new_ids;

#[tauri::command]
pub async fn reset_machine_id_only(
    device_id: String,
    mac_id: String,
    machine_id: String,
    sqm_id: String
) -> Result<bool, String> {
    let paths = AppPaths::new()?;

    // 更新 storage.json
    let mut storage_content = if paths.storage.exists() {
        let content = fs::read_to_string(&paths.storage)
            .map_err(|e| format!("读取 storage.json 失败: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("解析 storage.json 失败: {}", e))?
    } else {
        json!({})
    };

    if let Value::Object(ref mut map) = storage_content {
        map.insert("telemetry.devDeviceId".to_string(), json!(device_id));
        map.insert("telemetry.macMachineId".to_string(), json!(mac_id));
        map.insert("telemetry.machineId".to_string(), json!(machine_id));
        map.insert("telemetry.sqmId".to_string(), json!(sqm_id));
    }

    // 写入文件
    fs::write(
        &paths.storage,
        serde_json::to_string_pretty(&storage_content)
            .map_err(|e| format!("序列化 JSON 失败: {}", e))?,
    )
    .map_err(|e| format!("写入 storage.json 失败: {}", e))?;

    // 更新数据库中的机器码
    let machine_updates = vec![
        ("telemetry.devDeviceId", device_id.clone()),
        ("telemetry.macMachineId", mac_id.clone()),
        ("telemetry.machineId", machine_id.clone()),
        ("telemetry.sqmId", sqm_id.clone()),
    ];

    update_database(&paths.db, &machine_updates)?;

    Ok(true)
}

#[tauri::command]
pub async fn switch_account(
    email: String,
    token: String
) -> Result<bool, String> {
    let paths = AppPaths::new()?;

    // 更新 storage.json
    let mut storage_content = if paths.storage.exists() {
        let content = fs::read_to_string(&paths.storage)
            .map_err(|e| format!("读取 storage.json 失败: {}", e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("解析 storage.json 失败: {}", e))?
    } else {
        json!({})
    };

    if let Value::Object(ref mut map) = storage_content {
        map.insert("email".to_string(), json!(email));
        map.insert("access_token".to_string(), json!(token));
    }

    // 写入文件
    fs::write(
        &paths.storage,
        serde_json::to_string_pretty(&storage_content)
            .map_err(|e| format!("序列化 JSON 失败: {}", e))?,
    )
    .map_err(|e| format!("写入 storage.json 失败: {}", e))?;

    // 更新数据库中的账号信息
    let account_updates = vec![
        ("email", email),
        ("access_token", token),
    ];

    update_database(&paths.db, &account_updates)?;

    Ok(true)
}

#[tauri::command]
pub fn get_current_account() -> Result<Value, String> {
    let paths = AppPaths::new()?;

    // 从 storage.json 读取账号信息
    if paths.storage.exists() {
        let content = fs::read_to_string(&paths.storage)
            .map_err(|e| format!("读取 storage.json 失败: {}", e))?;
        let storage_content: Value = serde_json::from_str(&content)
            .map_err(|e| format!("解析 storage.json 失败: {}", e))?;
        Ok(storage_content)
    } else {
        Ok(json!({}))
    }
}

#[tauri::command]
pub fn get_machine_ids() -> Result<String, String> {
    let paths = AppPaths::new()?;

    // 从 storage.json 读取机器码
    if paths.storage.exists() {
        let content = fs::read_to_string(&paths.storage)
            .map_err(|e| format!("读取 storage.json 失败: {}", e))?;
        let storage_content: Value = serde_json::from_str(&content)
            .map_err(|e| format!("解析 storage.json 失败: {}", e))?;
        
        // 只返回 devDeviceId
        if let Some(device_id) = storage_content.get("telemetry.devDeviceId") {
            Ok(device_id.as_str().unwrap_or("").to_string())
        } else {
            Ok("".to_string())
        }
    } else {
        Ok("".to_string())
    }
}

fn update_database(db_path: &std::path::Path, updates: &[(impl AsRef<str>, impl AsRef<str>)]) -> Result<(), String> {
    let conn = Connection::open(db_path)
        .map_err(|e| format!("打开数据库失败: {}", e))?;

    for (key, value) in updates {
        conn.execute(
            "INSERT OR REPLACE INTO ItemTable (key, value) VALUES (?1, ?2)",
            [key.as_ref(), value.as_ref()],
        )
        .map_err(|e| format!("更新数据库失败: {}", e))?;
    }

    Ok(())
}
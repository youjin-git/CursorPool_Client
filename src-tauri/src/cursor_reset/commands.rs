use serde_json::{json, Value};
use std::fs;
use rusqlite::Connection;
use crate::utils::paths::AppPaths;
use crate::utils::id_generator::generate_new_ids;

#[tauri::command]
pub async fn reset_machine_id_only() -> Result<bool, String> {
    let paths = AppPaths::new()?;
    let new_ids = generate_new_ids();

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
        map.insert("telemetry.devDeviceId".to_string(), Value::String(new_ids.get("telemetry.devDeviceId").unwrap().clone()));
        map.insert("telemetry.macMachineId".to_string(), Value::String(new_ids.get("telemetry.macMachineId").unwrap().clone()));
        map.insert("telemetry.machineId".to_string(), Value::String(new_ids.get("telemetry.machineId").unwrap().clone()));
        map.insert("telemetry.sqmId".to_string(), Value::String(new_ids.get("telemetry.sqmId").unwrap().clone()));
    }

    fs::write(&paths.storage, serde_json::to_string_pretty(&storage_content)
        .map_err(|e| format!("序列化 storage.json 失败: {}", e))?)
        .map_err(|e| format!("写入 storage.json 失败: {}", e))?;

    // 更新数据库
    if paths.db.exists() {
        let updates = vec![
            ("device_id", new_ids.get("telemetry.devDeviceId").unwrap()),
            ("mac_id", new_ids.get("telemetry.macMachineId").unwrap()),
            ("machine_id", new_ids.get("telemetry.machineId").unwrap()),
            ("sqm_id", new_ids.get("telemetry.sqmId").unwrap())
        ];
        update_database(&paths.db, &updates)?;
    }

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
        map.insert("cursor.email".to_string(), json!(email));
        map.insert("cursor.accessToken".to_string(), json!(token));
        map.insert("cursorAuth/refreshToken".to_string(), json!(token));
        map.insert("cursorAuth/accessToken".to_string(), json!(token));
        map.insert("cursorAuth/cachedEmail".to_string(), json!(email));
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
pub fn get_machine_ids() -> Result<Value, String> {
    let paths = AppPaths::new()?;
    let mut result = json!({
        "machine_id": "",
        "current_account": ""
    });

    // 从数据库读取机器码和 Cursor 邮箱
    if paths.db.exists() {
        if let Ok(conn) = Connection::open(&paths.db) {
            // 读取机器码
            if let Ok(mut stmt) = conn.prepare("SELECT value FROM ItemTable WHERE key = 'telemetry.devDeviceId'") {
                if let Ok(mut rows) = stmt.query([]) {
                    if let Ok(Some(row)) = rows.next() {
                        if let Ok(device_id) = row.get::<_, String>(0) {
                            result["machine_id"] = json!(device_id);
                        }
                    }
                }
            }

            // 读取 Cursor 邮箱
            if let Ok(mut stmt) = conn.prepare("SELECT value FROM ItemTable WHERE key = 'cursorAuth/cachedEmail'") {
                if let Ok(mut rows) = stmt.query([]) {
                    if let Ok(Some(row)) = rows.next() {
                        if let Ok(email) = row.get::<_, String>(0) {
                            result["current_account"] = json!(email);
                        }
                    }
                }
            }
        }
    }

    Ok(result)
}

fn update_database(db_path: &std::path::Path, updates: &[(impl AsRef<str>, impl AsRef<str>)]) -> Result<(), String> {
    let conn = Connection::open(db_path)
        .map_err(|e| format!("打开数据库失败: {}", e))?;

    for (key, value) in updates {
        let key = match key.as_ref() {
            "device_id" => "telemetry.devDeviceId",
            "mac_id" => "telemetry.macMachineId",
            "machine_id" => "telemetry.machineId",
            "sqm_id" => "telemetry.sqmId",
            _ => key.as_ref(),
        };

        // 先尝试更新已存在的记录
        let result = conn.execute(
            "UPDATE ItemTable SET value = ?1 WHERE key = ?2",
            [value.as_ref(), key]
        );

        // 如果记录不存在（没有更新任何行），则插入新记录
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
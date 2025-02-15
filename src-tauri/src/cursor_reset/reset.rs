use std::fs;
use serde_json::Value;
use crate::utils::{AppPaths, generate_new_ids, update_sqlite_db, kill_cursor_process};
use crate::auth::{AuthInfo, update_auth};

pub fn reset_machine_id(paths: &AppPaths) -> Result<(), String> {
    // 生成新ID
    let new_ids = generate_new_ids();
    
    // 更新 storage.json
    let storage_content = fs::read_to_string(&paths.storage)
        .map_err(|e| format!("读取storage.json失败: {}", e))?;
    
    let mut config: Value = serde_json::from_str(&storage_content)
        .map_err(|e| format!("解析storage.json失败: {}", e))?;
    
    if let Value::Object(ref mut map) = config {
        for (key, value) in &new_ids {
            map.insert(key.clone(), Value::String(value.clone()));
        }
    }
    
    fs::write(
        &paths.storage,
        serde_json::to_string_pretty(&config).map_err(|e| format!("序列化JSON失败: {}", e))?,
    ).map_err(|e| format!("写入storage.json失败: {}", e))?;
    
    // 更新数据库
    update_sqlite_db(&paths.db, &new_ids)?;
    
    Ok(())
}

pub fn perform_reset(email: String, token: String) -> Result<(), String> {
    let paths = AppPaths::new()?;
    
    // 重置机器码
    reset_machine_id(&paths)?;
    
    // 更新认证信息
    update_auth(&paths, &AuthInfo { email, token })?;
    
    // 关闭Cursor进程
    kill_cursor_process();
    
    Ok(())
}

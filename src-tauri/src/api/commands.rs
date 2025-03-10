use tauri::State;
use crate::database::Database;
use serde::{Deserialize, Serialize};

/// 历史记录条目
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub id: i64,
    pub type_name: String,
    pub detail: String,
    pub timestamp: String,
    pub operator: String,
}

/// 历史账户记录
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryAccount {
    pub email: String,
    pub token: String,
    pub machine_code: String,
    pub gpt4_count: i32,
    pub gpt35_count: i32,
    pub last_used: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpt4_max_usage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpt35_max_usage: Option<i32>,
}

/// 保存历史记录
#[tauri::command]
pub async fn save_history_record(
    db: State<'_, Database>,
    record: HistoryRecord,
) -> Result<(), String> {
    // 获取现有历史记录
    let history_key = "user.history";
    let history_json = match db.get_item(history_key) {
        Ok(Some(json)) => json,
        Ok(None) => "[]".to_string(),
        Err(e) => return Err(format!("获取历史记录失败: {}", e)),
    };

    // 解析现有历史记录
    let mut history: Vec<HistoryRecord> = match serde_json::from_str(&history_json) {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };

    // 添加新记录到开头
    history.insert(0, record);

    // 限制历史记录数量，最多保留1000条
    if history.len() > 1000 {
        history.truncate(1000);
    }

    // 保存更新后的历史记录
    let updated_json = serde_json::to_string(&history)
        .map_err(|e| format!("序列化历史记录失败: {}", e))?;
    
    db.set_item(history_key, &updated_json)
        .map_err(|e| format!("保存历史记录失败: {}", e))?;

    Ok(())
}

/// 批量保存历史记录
#[tauri::command]
pub async fn save_history_records(
    db: State<'_, Database>,
    records: Vec<HistoryRecord>,
) -> Result<(), String> {
    if records.is_empty() {
        return Ok(());
    }

    // 获取现有历史记录
    let history_key = "user.history";
    let history_json = match db.get_item(history_key) {
        Ok(Some(json)) => json,
        Ok(None) => "[]".to_string(),
        Err(e) => return Err(format!("获取历史记录失败: {}", e)),
    };

    // 解析现有历史记录
    let mut history: Vec<HistoryRecord> = match serde_json::from_str(&history_json) {
        Ok(records) => records,
        Err(_) => Vec::new(),
    };

    // 添加新记录到开头（保持原有顺序）
    for record in records.into_iter().rev() {
        history.insert(0, record);
    }

    // 限制历史记录数量，最多保留1000条
    if history.len() > 1000 {
        history.truncate(1000);
    }

    // 保存更新后的历史记录
    let updated_json = serde_json::to_string(&history)
        .map_err(|e| format!("序列化历史记录失败: {}", e))?;
    
    db.set_item(history_key, &updated_json)
        .map_err(|e| format!("保存历史记录失败: {}", e))?;

    Ok(())
}

/// 获取历史记录
#[tauri::command]
pub async fn get_history_records(
    db: State<'_, Database>,
) -> Result<Vec<HistoryRecord>, String> {
    let history_key = "user.history";
    let history_json = match db.get_item(history_key) {
        Ok(Some(json)) => json,
        Ok(None) => "[]".to_string(),
        Err(e) => return Err(format!("获取历史记录失败: {}", e)),
    };

    // 解析历史记录
    let history: Vec<HistoryRecord> = match serde_json::from_str(&history_json) {
        Ok(records) => records,
        Err(e) => return Err(format!("解析历史记录失败: {}", e)),
    };

    Ok(history)
}

/// 保存历史账户
#[tauri::command]
pub async fn save_history_account(
    db: State<'_, Database>,
    account: HistoryAccount,
) -> Result<(), String> {
    // 获取现有历史账户
    let history_key = "user.history.accounts";
    let history_json = match db.get_item(history_key) {
        Ok(Some(json)) => json,
        Ok(None) => "[]".to_string(),
        Err(e) => return Err(format!("获取历史账户失败: {}", e)),
    };

    // 解析现有历史账户
    let mut accounts: Vec<HistoryAccount> = match serde_json::from_str(&history_json) {
        Ok(accounts) => accounts,
        Err(_) => Vec::new(),
    };

    // 检查是否已存在该账户
    let index = accounts.iter().position(|a| a.email == account.email);
    if let Some(pos) = index {
        // 更新现有账户
        accounts[pos] = account;
    } else {
        // 添加新账户
        accounts.push(account);
    }

    // 限制历史账户数量，最多保留10个
    if accounts.len() > 10 {
        // 按最后使用时间排序
        accounts.sort_by(|a, b| b.last_used.cmp(&a.last_used));
        accounts.truncate(10);
    }

    // 保存更新后的历史账户
    let updated_json = serde_json::to_string(&accounts)
        .map_err(|e| format!("序列化历史账户失败: {}", e))?;
    
    db.set_item(history_key, &updated_json)
        .map_err(|e| format!("保存历史账户失败: {}", e))?;

    Ok(())
}

/// 获取历史账户
#[tauri::command]
pub async fn get_history_accounts(
    db: State<'_, Database>,
) -> Result<Vec<HistoryAccount>, String> {
    let history_key = "user.history.accounts";
    let history_json = match db.get_item(history_key) {
        Ok(Some(json)) => json,
        Ok(None) => "[]".to_string(),
        Err(e) => return Err(format!("获取历史账户失败: {}", e)),
    };

    // 解析历史账户
    let accounts: Vec<HistoryAccount> = match serde_json::from_str(&history_json) {
        Ok(accounts) => accounts,
        Err(e) => return Err(format!("解析历史账户失败: {}", e)),
    };

    Ok(accounts)
}

/// 删除历史账户
#[tauri::command]
pub async fn remove_history_account(
    db: State<'_, Database>,
    email: String,
) -> Result<(), String> {
    // 获取现有历史账户
    let history_key = "user.history.accounts";
    let history_json = match db.get_item(history_key) {
        Ok(Some(json)) => json,
        Ok(None) => "[]".to_string(),
        Err(e) => return Err(format!("获取历史账户失败: {}", e)),
    };

    // 解析现有历史账户
    let mut accounts: Vec<HistoryAccount> = match serde_json::from_str(&history_json) {
        Ok(accounts) => accounts,
        Err(_) => Vec::new(),
    };

    // 移除指定账户
    accounts.retain(|a| a.email != email);

    // 保存更新后的历史账户
    let updated_json = serde_json::to_string(&accounts)
        .map_err(|e| format!("序列化历史账户失败: {}", e))?;
    
    db.set_item(history_key, &updated_json)
        .map_err(|e| format!("保存历史账户失败: {}", e))?;

    Ok(())
}

/// 清除所有历史账户
#[tauri::command]
pub async fn clear_history_accounts(
    db: State<'_, Database>,
) -> Result<(), String> {
    let history_key = "user.history.accounts";
    
    // 保存空数组
    db.set_item(history_key, "[]")
        .map_err(|e| format!("清除历史账户失败: {}", e))?;

    Ok(())
} 
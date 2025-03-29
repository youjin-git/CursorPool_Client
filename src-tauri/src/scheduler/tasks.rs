use crate::api::client::ApiClient;
use crate::config;
use crate::cursor_reset::commands;
use crate::database::Database;
use crate::utils::ErrorReporter;
use crate::utils::retry;
use serde_json::Value;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};
use tracing::error;

/// 检查账户使用限制
pub async fn check_account_limit(app_handle: &AppHandle) -> Result<(), String> {
    // 获取当前机器码和用户信息
    let db = app_handle.state::<Database>();
    
    // 使用 retry 函数获取机器码信息
    let info = retry::retry(
        || commands::get_machine_ids(db.clone()),
        3,
        Duration::from_millis(500),
        "获取机器码信息"
    ).await?;
    
    // 提取当前账户信息
    let current_account = match info.get("currentAccount") {
        Some(Value::String(account)) => account.clone(),
        _ => {
            let err = "无法获取当前账户信息".to_string();
            error!("{}", err);
            return Err(err);
        }
    };
    
    let token = match info.get("cursorToken") {
        Some(Value::String(token)) => token.clone(),
        _ => {
            let err = "无法获取当前账户Token".to_string();
            error!("{}", err);
            return Err(err);
        }
    };
    
    // 获取API客户端
    let api_client = app_handle.state::<ApiClient>();
    
    // 使用endpoints提供的函数获取使用情况，而不是直接请求API
    let usage_result = crate::api::get_usage(
        app_handle.state::<ApiClient>(),
        token
    ).await;
    
    // 处理获取的使用情况
    let usage_data = match usage_result {
        Ok(response) => {
            if let Some(data) = response.data {
                Value::Object(serde_json::to_value(data).unwrap_or_default().as_object().unwrap_or(&serde_json::Map::new()).clone())
            } else {
                let err_msg = format!("获取使用情况失败: {}", response.msg);
                error!("{}", err_msg);
                return Err(err_msg);
            }
        },
        Err(e) => {
            let err_msg = format!("获取使用情况失败: {}", e);
            error!("{}", err_msg);
            // 上报错误，但是不中断流程
            let _ = ErrorReporter::report_error(
                api_client.clone(),
                "check_account_limit",
                &err_msg,
                None,
                Some("low".to_string()),
            )
            .await;
            return Err(err_msg);
        }
    };
    
    // 从数据库获取警告阈值
    let account_threshold_key = config::get_db_key("account_usage_threshold");
    let account_usage_threshold = match db.inner().get_item(&account_threshold_key) {
        Ok(Some(val)) => match val.parse::<f64>() {
            Ok(v) => v,
            Err(_) => {
                // 使用默认值
                config::get_account_usage_threshold()
            }
        },
        _ => {
            // 使用默认值
            config::get_account_usage_threshold()
        }
    };
    
    // 检查GPT-4使用情况
    if let Some(gpt4_usage) = usage_data.get("gpt-4") {
        if let (Some(Value::Number(used)), Some(Value::Number(total))) = 
            (gpt4_usage.get("numRequests"), gpt4_usage.get("maxRequestUsage")) 
        {
            let used_f = used.as_f64().unwrap_or(0.0);
            let mut total_f = total.as_f64().unwrap_or(1.0);
            
            if total_f == 150.0 {
                if used_f < 51.0 {
                    // 如果使用量小于52，将最大数量调整为50
                    total_f = 50.0;
                } else if used_f < 101.0 {
                    // 如果使用量小于102，将最大数量调整为100
                    total_f = 100.0;
                }
            }
            
            // 只有当total不为0或接近无限大的值时才进行计算
            if total_f > 0.0 && total_f < 9990.0 {
                let remaining_ratio = (total_f - used_f) / total_f;
                
                if remaining_ratio <= account_usage_threshold {
                    // 确保百分比不小于0
                    let remaining_percentage = (remaining_ratio * 100.0).round().max(0.0);
                    
                    // 发送通知到前端，直接传递账户名和剩余百分比
                    send_notification(app_handle, &current_account, remaining_percentage as i64).await?;
                }
            }
        }
    }
    Ok(())
}

/// 发送通知到前端
async fn send_notification(app_handle: &AppHandle, account: &str, remaining_percentage: i64) -> Result<(), String> {
    // 只向前端发送账户数据和剩余使用量百分比
    if let Some(window) = app_handle.get_webview_window("main") {
        if let Err(e) = window.emit("account-usage-warning", 
            serde_json::json!({
                "data": {
                    "account": account,
                    "remaining_percentage": remaining_percentage
                }
            })
        ) {
            let err_msg = format!("发送前端通知事件失败: {}", e);
            error!("{}", err_msg);
            return Err(err_msg);
        }
        Ok(())
    } else {
        let err_msg = "无法获取应用窗口，通知发送失败".to_string();
        error!("{}", err_msg);
        Err(err_msg)
    }
} 
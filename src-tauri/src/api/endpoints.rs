use super::client::get_base_url;
use super::types::*;
use tauri::State;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::env;
use crate::database::Database;

// Bug报告请求结构
#[derive(Serialize, Deserialize)]
pub struct BugReportRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    pub app_version: String,
    pub os_version: String,
    pub device_model: String,
    pub cursor_version: String,
    pub bug_description: String,
    pub occurrence_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screenshot_urls: Option<Vec<String>>,
    pub severity: String,
}

/// 检查用户是否存在
#[tauri::command]
pub async fn check_user(
    client: State<'_, super::client::ApiClient>,
    email: String,
) -> Result<ApiResponse<()>, String> {
    let response = client
        .post(format!("{}/checkUser", get_base_url()))
        .form(&[("email", email)])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// 发送验证码
#[tauri::command]
pub async fn send_code(
    client: State<'_, super::client::ApiClient>,
    email: String,
    r#type: String,
) -> Result<ApiResponse<()>, String> {
    let response = client
        .post(format!("{}/register/sendEmailCode", get_base_url()))
        .form(&[("email", email), ("type", r#type)])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// 注册用户
#[tauri::command]
pub async fn register(
    client: State<'_, super::client::ApiClient>,
    email: String,
    code: String,
    password: String,
    spread: String,
) -> Result<ApiResponse<()>, String> {
    let response = client
        .post(format!("{}/emailRegister", get_base_url()))
        .multipart([
            ("email".to_string(), email),
            ("code".to_string(), code),
            ("password".to_string(), password),
            ("spread".to_string(), "0".to_string()),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_text = response.text().await.map_err(|e| e.to_string())?;
    serde_json::from_str(&response_text).map_err(|e| e.to_string())
}

/// 用户登录
#[tauri::command]
pub async fn login(
    client: State<'_, super::client::ApiClient>,
    account: String,
    password: String,
    spread: String,
) -> Result<ApiResponse<LoginResponse>, String> {
    let response = client
        .post(format!("{}/login", get_base_url()))
        .form(&[
            ("account", account),
            ("password", password),
            ("spread", spread),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// 获取用户信息
#[tauri::command]
pub async fn get_user_info(
    client: State<'_, super::client::ApiClient>,
) -> Result<ApiResponse<UserInfo>, String> {
    let response = client
        .get(format!("{}/user", get_base_url()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// 激活账户
#[tauri::command]
pub async fn activate(
    client: State<'_, super::client::ApiClient>,
    code: String,
) -> Result<ApiResponse<()>, String> {
    let response = client
        .post(format!("{}/user/activate", get_base_url()))
        .form(&[("code", code)])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// 修改密码
#[tauri::command]
pub async fn change_password(
    client: State<'_, super::client::ApiClient>,
    old_password: String,
    new_password: String,
) -> Result<ApiResponse<()>, String> {
    let response = client
        .post(format!("{}/user/updatePassword", get_base_url()))
        .form(&[
            ("old_password", old_password.clone()),
            ("new_password", new_password.clone()),
            ("confirm_password", new_password.clone()),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;
    response.json().await.map_err(|e| e.to_string())
}

/// 获取账户信息
#[tauri::command]
pub async fn get_account(
    client: State<'_, super::client::ApiClient>,
    account: Option<String>,
    usage_count: Option<String>,
) -> Result<ApiResponse<AccountPoolInfo>, String> {
    let mut url = format!("{}/accountpool/get", get_base_url());
    
    let mut query_params = Vec::new();
    if let Some(acc) = account {
        query_params.push(format!("account={}", acc));
    }
    if let Some(count) = usage_count {
        query_params.push(format!("usage_count={}", count));
    }
    
    if !query_params.is_empty() {
        url = format!("{}?{}", url, query_params.join("&"));
    }
    
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// 获取 Cursor 使用情况
#[tauri::command]
pub async fn get_usage(
    client: State<'_, super::client::ApiClient>,
    token: String,
) -> Result<ApiResponse<CursorUsageInfo>, String> {
    let user_id = "user_01000000000000000000000000";
    let response = client
        .get("https://www.cursor.com/api/usage")
        .header("Cookie", format!("WorkosCursorSessionToken={}%3A%3A{}", user_id, token).as_str())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_text = response.text().await.map_err(|e| e.to_string())?;
    
    match serde_json::from_str::<CursorUsageInfo>(&response_text) {
        Ok(usage_info) => {
            Ok(ApiResponse {
                status: 200,
                msg: "获取使用情况成功".to_string(),
                data: Some(usage_info),
                code: Some("460001".to_string()),
            })
        },
        Err(e) => Err(format!("Failed to parse Cursor usage info: {}", e))
    }
}

/// 获取版本信息
#[tauri::command]
pub async fn get_version(
    client: State<'_, super::client::ApiClient>,
) -> Result<serde_json::Value, String> {
    let response = client
        .get(format!("{}/version", get_base_url()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_text = response.text().await.map_err(|e| e.to_string())?;
    
    let json_value: serde_json::Value = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse response as JSON: {}", e))?;
    
    if let Some(status) = json_value.get("status") {
        if status.is_string() && status.as_str().unwrap() == "success" {
            if let Some(data) = json_value.get("data") {
                return Ok(serde_json::json!({
                    "status": 200,
                    "msg": "获取版本信息成功",
                    "data": data,
                    "code": "460001"
                }));
            }
        }
    }
    
    Ok(serde_json::json!({
        "status": 200,
        "msg": "获取版本信息成功",
        "data": json_value,
        "code": "460001"
    }))
}

/// 获取公告信息
#[tauri::command]
pub async fn get_public_info(
    client: State<'_, super::client::ApiClient>,
) -> Result<ApiResponse<PublicInfo>, String> {
    let response = client
        .get(format!("{}/public/info", get_base_url()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// 重置密码
#[tauri::command]
pub async fn reset_password(
    client: State<'_, super::client::ApiClient>,
    email: String,
    code: String,
    password: String,
) -> Result<ApiResponse<()>, String> {
    let response = client
        .post(format!("{}/emailResetPassword", get_base_url()))
        .form(&[
            ("email", email),
            ("code", code),
            ("password", password),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// 报告错误
#[tauri::command]
pub async fn report_bug(
    client: State<'_, super::client::ApiClient>,
    severity: String,
    bug_description: String,
    api_key: Option<String>,
    screenshot_urls: Option<Vec<String>>,
    cursor_version: Option<String>,
) -> Result<ApiResponse<()>, String> {
    let app_version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
    let os_version = os_info::get().to_string();
    let device_model = "PC".to_string();
    let occurrence_time = Utc::now().to_rfc3339();

    let request = BugReportRequest {
        api_key,
        app_version,
        os_version,
        device_model,
        cursor_version: cursor_version.unwrap_or_else(|| "unknown".to_string()),
        bug_description,
        occurrence_time,
        screenshot_urls,
        severity,
    };

    let response = client
        .post(format!("{}/bug/report", get_base_url()))
        .json(&request)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// 获取免责声明
#[tauri::command]
pub async fn get_disclaimer(
    client: State<'_, super::client::ApiClient>,
) -> Result<ApiResponse<DisclaimerResponse>, String> {
    let response = client
        .get(format!("{}/disclaimer", get_base_url()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

/// 用户登出
#[tauri::command]
pub async fn logout(
    db: State<'_, Database>,
) -> Result<ApiResponse<()>, String> {
    db.delete_item("user.info.token")
        .map_err(|e| e.to_string())?;
    
    Ok(ApiResponse {
        status: 200,
        msg: "登出成功".to_string(),
        data: None,
        code: Some("460001".to_string()),
    })
}

use super::client::get_base_url;
use super::types::*;
use crate::database::Database;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use tauri::State;

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
) -> Result<ApiResponse<serde_json::Value>, String> {
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
    db: State<'_, Database>,
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

    let response = client.get(&url).send().await.map_err(|e| e.to_string())?;

    let response: ApiResponse<AccountPoolInfo> =
        response.json().await.map_err(|e| e.to_string())?;

    // 如果获取成功且有账户信息，将token保存到历史记录
    if response.status == 200 && response.data.is_some() {
        let account_info = &response.data.as_ref().unwrap().account_info;
        if !account_info.account.is_empty() && !account_info.token.is_empty() {
            // 获取当前机器码
            use crate::cursor_reset::get_machine_ids;
            let machine_info = get_machine_ids()?;
            let machine_id = machine_info["machineId"]
                .as_str()
                .unwrap_or_default()
                .to_string();

            // 保存到历史记录
            if let Err(e) = super::interceptor::save_cursor_token_to_history(
                &db,
                &account_info.account,
                &account_info.token,
                &machine_id,
            )
            .await
            {
                eprintln!("保存Cursor token到历史记录失败: {}", e);
            }
        }
    }

    Ok(response)
}

/// 获取 Cursor 使用情况
#[tauri::command]
pub async fn get_usage(
    client: State<'_, super::client::ApiClient>,
    token: String,
) -> Result<ApiResponse<CursorUsageInfo>, String> {
    let user_id = "user_01000000000000000000000000";
    
    // token可能包含了用户ID部分，需要分割并只使用token部分
    let actual_token = if token.contains("%3A%3A") {
        // 如果token包含分隔符，取第二部分
        token.split("%3A%3A").nth(1).unwrap_or(&token).to_string()
    } else {
        // 否则使用原始token
        token
    };
    
    let response = client
        .get("https://www.cursor.com/api/usage")
        .header(
            "Cookie",
            format!("WorkosCursorSessionToken={}%3A%3A{}", user_id, actual_token).as_str(),
        )
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let response_text = response.text().await.map_err(|e| e.to_string())?;

    match serde_json::from_str::<CursorUsageInfo>(&response_text) {
        Ok(usage_info) => Ok(ApiResponse {
            status: 200,
            msg: "获取使用情况成功".to_string(),
            data: Some(usage_info),
            code: Some("460001".to_string()),
        }),
        Err(e) => {
            println!("Cursor API 响应: {}", response_text);
            Err(format!("Failed to parse Cursor usage info: {}", e))
        }
    }
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
        .form(&[("email", email), ("code", code), ("password", password)])
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

/// 用户登出
#[tauri::command]
pub async fn logout(db: State<'_, Database>) -> Result<ApiResponse<()>, String> {
    db.delete_item("user.info.token")
        .map_err(|e| e.to_string())?;

    Ok(ApiResponse {
        status: 200,
        msg: "登出成功".to_string(),
        data: None,
        code: Some("460001".to_string()),
    })
}

/// 设置用户数据
#[tauri::command]
pub async fn set_user_data(
    db: State<'_, Database>,
    key: String,
    value: String,
) -> Result<ApiResponse<()>, String> {
    match db.set_item(&key, &value) {
        Ok(_) => Ok(ApiResponse {
            status: 200,
            msg: "成功设置用户数据".to_string(),
            data: None,
            code: Some("SUCCESS".to_string()),
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// 获取用户数据
#[tauri::command]
pub async fn get_user_data(
    db: State<'_, Database>,
    key: String,
) -> Result<ApiResponse<serde_json::Value>, String> {
    match db.get_item(&key) {
        Ok(value) => Ok(ApiResponse {
            status: 200,
            msg: "成功获取用户数据".to_string(),
            data: Some(json!({ "value": value })),
            code: Some("SUCCESS".to_string()),
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// 删除用户数据
#[tauri::command]
pub async fn del_user_data(
    db: State<'_, Database>,
    key: String,
) -> Result<ApiResponse<()>, String> {
    match db.delete_item(&key) {
        Ok(_) => Ok(ApiResponse {
            status: 200,
            msg: "成功删除用户数据".to_string(),
            data: None,
            code: Some("SUCCESS".to_string()),
        }),
        Err(e) => Err(e.to_string()),
    }
}

/// 获取公告列表
#[tauri::command]
pub async fn get_article_list(
    client: State<'_, super::client::ApiClient>,
) -> Result<ApiResponse<Vec<Article>>, String> {
    // 获取公告数据
    let result = fetch_article_list(&client).await;
    
    match result {
        Ok(articles) => {
            Ok(ApiResponse {
                status: 200,
                msg: "获取公告成功".to_string(),
                data: Some(articles),
                code: Some("SUCCESS".to_string()),
            })
        },
        Err(_) => {
            // 接口错误时，返回空列表而不是错误
            Ok(ApiResponse {
                status: 200,
                msg: "获取公告成功".to_string(),
                data: Some(Vec::new()),
                code: Some("SUCCESS".to_string()),
            })
        }
    }
}

/// 内部函数：获取公告列表数据
async fn fetch_article_list(client: &super::client::ApiClient) -> Result<Vec<Article>, String> {
    let response = client
        .get(format!("{}/article/list/1", get_base_url()))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let response_json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    
    // 检查状态码
    let status = response_json["status"].as_i64().unwrap_or(0);
    if status != 200 {
        return Err("获取公告失败".to_string());
    }
    
    // 提取所需字段
    let empty_vec = Vec::new();
    let data = response_json["data"].as_array().unwrap_or(&empty_vec);
    let mut articles = Vec::new();
    
    for item in data {
        let id = item["id"].as_i64().unwrap_or(0) as i32;
        let title = item["title"].as_str().unwrap_or("").to_string();
        let content = item["content"].as_str().unwrap_or("").to_string();
        
        articles.push(Article {
            id,
            title,
            content,
        });
    }
    
    Ok(articles)
}

/// 标记文章为已读
#[tauri::command]
pub async fn mark_article_read(
    db: State<'_, Database>,
    article_id: i32,
) -> Result<ApiResponse<()>, String> {
    // 获取已读ID集合
    let read_ids = match db.get_item("system.articles") {
        Ok(Some(data)) => {
            serde_json::from_str::<Vec<i32>>(&data).unwrap_or_default()
        },
        _ => Vec::new(),
    };
    
    // 检查文章ID是否已在已读列表中
    let mut updated_ids = read_ids.clone();
    if !updated_ids.contains(&article_id) {
        updated_ids.push(article_id);
        
        // 保存更新后的已读ID列表
        let json_data = serde_json::to_string(&updated_ids).map_err(|e| e.to_string())?;
        db.set_item("system.articles", &json_data).map_err(|e| e.to_string())?;
    }
    
    Ok(ApiResponse {
        status: 200,
        msg: "文章已标记为已读".to_string(),
        data: None,
        code: Some("SUCCESS".to_string()),
    })
}

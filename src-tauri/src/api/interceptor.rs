use crate::config;
use crate::database::Database;
use reqwest::Request;
use std::ops::Not;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Manager;

/// HTTP 请求拦截器特征
pub trait Interceptor: Send + Sync {
    /// 拦截并处理请求
    fn intercept(&self, request: &mut Request) -> Result<(), String>;
}

/// JWT 认证拦截器
pub struct AuthInterceptor {
    app_handle: Arc<AppHandle>,
}

impl AuthInterceptor {
    /// 创建认证拦截器实例
    pub fn new(app_handle: Arc<AppHandle>) -> Self {
        Self { app_handle }
    }
}

impl Interceptor for AuthInterceptor {
    fn intercept(&self, request: &mut Request) -> Result<(), String> {
        let db = self.app_handle.state::<Database>();

        let token_key = config::get_db_key("token");
        
        //header 添加X-API-Key = token 
        let token = match db.get_item(&token_key) {
            Ok(Some(token)) => token,
            _ => return Ok(()),
        };

        // 添加 X-API-Key 到请求头
        request.headers_mut().insert(
            "X-API-Key",
            token.parse().unwrap(),
        );

        request.headers_mut().insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );

        let lang_key = config::get_db_key("lang");
        let lang = match db.get_item(&lang_key) {
            Ok(Some(lang)) if lang != "zh-CN" => lang,
            _ => "zh-CN".to_string(),
        };

        request
            .headers_mut()
            .insert("cb-lang", lang.parse().unwrap());

        Ok(())
    }
}

/// 检查 URL 是否需要认证
pub fn is_auth_required_url(url: &str) -> bool {
    config::is_public_endpoint(url).not()
}

/// 保存认证令牌
pub async fn save_auth_token(
    db: &tauri::State<'_, Database>,
    url: &str,
    response_text: &str,
) -> Result<(), String> {
    if !url.contains("/login") && !url.contains("/emailRegister") {
        return Ok(());
    }

    let api_response: crate::api::types::ApiResponse<crate::api::types::LoginResponse> =
        match serde_json::from_str(response_text) {
            Ok(response) => response,
            Err(_) => return Ok(()),
        };

    if api_response.code == 200 && api_response.data.is_some() {
        let data = api_response.data.unwrap();
        if let Some(token) = data.token {
            let token_key = config::get_db_key("token");
            db.set_item(&token_key, &token).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

/// 清除认证令牌
pub async fn clear_auth_token(db: &tauri::State<'_, Database>) -> Result<(), String> {
    let token_key = config::get_db_key("token");
    db.delete_item(&token_key).map_err(|e| e.to_string())
}

/// 保存Cursor token到历史记录
pub async fn save_cursor_token_to_history(
    db: &tauri::State<'_, Database>,
    email: &str,
    token: &str,
    machine_id: &str,
) -> Result<(), String> {
    // 处理token，分割并只取第二部分
    let processed_token = if token.contains("%3A%3A") {
        token.split("%3A%3A").nth(1).unwrap_or(token).to_string()
    } else {
        token.to_string()
    };
    // 1. 获取当前历史记录
    let accounts = match db.get_item("user.history.accounts") {
        Ok(Some(data)) => {
            serde_json::from_str::<Vec<crate::api::types::HistoryAccountRecord>>(&data)
                .unwrap_or_default()
        }
        _ => Vec::new(),
    };

    // 2. 准备新的账户记录
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64;

    let new_account = crate::api::types::HistoryAccountRecord {
        email: email.to_string(),
        token: processed_token,
        machine_code: machine_id.to_string(),
        gpt4_count: 0,
        gpt35_count: 0,
        last_used: now,
        gpt4_max_usage: None,
        gpt35_max_usage: None,
    };

    // 3. 更新记录列表（替换或添加）
    let mut updated_accounts = accounts
        .into_iter()
        .filter(|a| a.email != email)
        .collect::<Vec<_>>();

    updated_accounts.push(new_account);

    // 4. 保存回数据库
    let json_data = serde_json::to_string(&updated_accounts).map_err(|e| e.to_string())?;

    db.set_item("user.history.accounts", &json_data)
        .map_err(|e| e.to_string())?;

    Ok(())
}

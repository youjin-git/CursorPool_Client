use super::client::get_base_url;
use super::types::*;
use tauri::State;

#[tauri::command]
pub async fn check_user(
    client: State<'_, super::client::ApiClient>,
    username: String,
) -> Result<ApiResponse<CheckUserResponse>, String> {
    let response = client
        .0
        .post(format!("{}/user/check", get_base_url()))
        .json(&CheckUserRequest { username })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_code(
    client: State<'_, super::client::ApiClient>,
    username: String,
) -> Result<ApiResponse<SendCodeResponse>, String> {
    let response = client
        .0
        .post(format!("{}/user/send_code", get_base_url()))
        .json(&SendCodeRequest { username })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn login(
    client: State<'_, super::client::ApiClient>,
    username: String,
    password: String,
    device_id: String,
    sms_code: Option<String>,
) -> Result<LoginResponse, String> {
    let response = client
        .0
        .post(format!("{}/user/login", get_base_url()))
        .json(&LoginRequest {
            username,
            password,
            device_id,
            sms_code,
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    // 先解析为 ApiResponse<LoginResponse>
    let api_response: ApiResponse<LoginResponse> = response.json().await.map_err(|e| e.to_string())?;
    
    // 如果状态不是成功，返回错误
    if api_response.status != "success" {
        return Err(api_response.message);
    }
    
    // 从 ApiResponse 中提取 LoginResponse
    api_response.data.ok_or_else(|| "No login data received".to_string())
}

#[tauri::command]
pub async fn get_user_info(
    client: State<'_, super::client::ApiClient>,
    api_key: String,
) -> Result<ApiResponse<UserInfo>, String> {
    let response = client
        .0
        .get(format!("{}/user/info", get_base_url()))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn activate(
    client: State<'_, super::client::ApiClient>,
    api_key: String,
    code: String,
) -> Result<ApiResponse<ActivateResponse>, String> {
    let response = client
        .0
        .post(format!("{}/user/activate", get_base_url()))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&ActivateRequest { code })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn change_password(
    client: State<'_, super::client::ApiClient>,
    api_key: String,
    old_password: String,
    new_password: String,
) -> Result<ApiResponse<LoginResponse>, String> {
    let response = client
        .0
        .post(format!("{}/user/change_password", get_base_url()))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&ChangePasswordRequest {
            old_password,
            new_password,
        })
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_account(
    client: State<'_, super::client::ApiClient>,
    api_key: String,
) -> Result<ApiResponse<AccountDetail>, String> {
    let response = client
        .0
        .get(format!("{}/account/get", get_base_url()))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let account_response: ApiResponse<AccountInfo> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;
    
    // 只返回需要的字段
    Ok(ApiResponse {
        status: account_response.status,
        message: account_response.message,
        data: account_response.data.map(|account_info| {
            let parts: Vec<&str> = account_info.token.split("%3A%3A").collect();
            AccountDetail {
                email: account_info.email,
                user_id: parts[0].to_string(),
                token: parts[1].to_string(),
            }
        }),
    })
}

#[tauri::command]
pub async fn get_usage(
    client: State<'_, super::client::ApiClient>,
    token: String,
) -> Result<ApiResponse<CursorUsageInfo>, String> {
    // 固定 user_id 不检测
    let user_id = "user_01000000000000000000000000";
    
    let response = client
        .0
        .get("https://www.cursor.com/api/usage")
        .header("Cookie", format!("WorkosCursorSessionToken={}%3A%3A{}", user_id, token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let usage_info = response.json().await.map_err(|e| e.to_string())?;
    
    Ok(ApiResponse {
        status: "success".to_string(),
        message: "获取使用情况成功".to_string(),
        data: Some(usage_info),
    })
}

#[tauri::command]
pub async fn get_user_info_cursor(
    client: State<'_, super::client::ApiClient>,
    user_id: String,
    token: String,
) -> Result<ApiResponse<CursorUserInfo>, String> {
    let response = client
        .0
        .get("https://www.cursor.com/api/auth/me")
        .header("Cookie", format!("WorkosCursorSessionToken={}%3A%3A{}", user_id, token))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let user_info = response.json().await.map_err(|e| e.to_string())?;
    
    Ok(ApiResponse {
        status: "success".to_string(),
        message: "获取用户信息成功".to_string(),
        data: Some(user_info),
    })
}

#[tauri::command]
pub async fn get_version(
    client: State<'_, super::client::ApiClient>,
) -> Result<ApiResponse<VersionInfo>, String> {
    let response = client
        .0
        .get(format!("{}/version", get_base_url()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_public_info(
    client: State<'_, super::client::ApiClient>,
) -> Result<ApiResponse<PublicInfo>, String> {
    let response = client
        .0
        .get(format!("{}/public/info", get_base_url()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response.json().await.map_err(|e| e.to_string())
}

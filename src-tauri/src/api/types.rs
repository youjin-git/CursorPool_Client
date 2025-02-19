use serde::{Deserialize, Serialize};

// 通用 API 响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

// 用户信息
// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserInfo {
//     pub username: String,
//     pub email: Option<String>,
//     pub activated: bool,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub total_count: i32,
    pub used_count: i32,
    pub expire_time: i64,
    pub level: i32,
    pub is_expired: bool,
    pub username: String,
}

// 账户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub email: String,
    pub token: String,
    pub used_count: i32,
    pub total_limit: i32,
}

// 账户详细信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDetail {
    pub email: String,
    pub user_id: String,
    pub token: String,
}

// 登录请求
#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub device_id: String,
    pub sms_code: Option<String>,
}

// 登录响应
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub api_key: Option<String>,
}

// 检查用户请求
#[derive(Debug, Serialize)]
pub struct CheckUserRequest {
    pub username: String,
}

// 检查用户响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckUserResponse {
    pub exists: bool,
    pub need_code: bool,
}

// 发送验证码请求
#[derive(Debug, Serialize)]
pub struct SendCodeRequest {
    pub username: String,
}

// 发送验证码响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SendCodeResponse {
    pub expire_in: i32,
}

// 激活请求
#[derive(Debug, Serialize)]
pub struct ActivateRequest {
    pub code: String,
}

// 激活响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateResponse {
    pub expire_time: i64,
    pub level: i32,
}

// 修改密码请求
#[derive(Debug, Serialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

// 版本信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub force_update: bool,
    pub download_url: String,
    pub change_log: String,
}

// 公告信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct PublicInfo {
    pub r#type: String,
    pub closeable: bool,
    pub props: PublicInfoProps,
    pub actions: Vec<PublicInfoAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicInfoProps {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicInfoAction {
    pub r#type: String,
    pub text: String,
    pub url: String,
}

// GPT 模型使用情况
#[derive(Debug, Serialize, Deserialize)]
pub struct GptModelUsage {
    pub num_requests: i32,
    pub num_requests_total: i32,
    pub num_tokens: i32,
    pub max_request_usage: Option<i32>,
    pub max_token_usage: Option<i32>,
}

// 使用情况响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UsageInfo {
    pub models: Vec<GptModelUsage>,
}

// 用户信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub models: Vec<GptModelUsage>,
}

// Cursor 用户信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CursorUserInfo {
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub sub: String,
    pub updated_at: String,
    pub picture: Option<String>,
}

// Cursor 模型使用情况
#[derive(Debug, Serialize, Deserialize)]
pub struct CursorModelUsage {
    #[serde(rename = "numRequests")]
    pub num_requests: i32,
    #[serde(rename = "numRequestsTotal")]
    pub num_requests_total: i32,
    #[serde(rename = "numTokens")]
    pub num_tokens: i32,
    #[serde(rename = "maxRequestUsage")]
    pub max_request_usage: Option<i32>,
    #[serde(rename = "maxTokenUsage")]
    pub max_token_usage: Option<i32>,
}

// Cursor 使用情况响应
#[derive(Debug, Serialize, Deserialize)]
pub struct CursorUsageInfo {
    #[serde(rename = "gpt-4")]
    pub gpt4: CursorModelUsage,
    #[serde(rename = "gpt-3.5-turbo")]
    pub gpt35: CursorModelUsage,
    #[serde(rename = "gpt-4-32k")]
    pub gpt4_32k: CursorModelUsage,
    #[serde(rename = "startOfMonth")]
    pub start_of_month: String,
}

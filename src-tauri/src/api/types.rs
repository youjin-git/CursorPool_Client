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
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub email: Option<String>,
    pub activated: bool,
}

// 账户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    pub email: String,
    pub token: String,
    pub daily_used: i32,
    pub daily_limit: i32,
}

// 账户详细信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountDetail {
    pub email: String,
    pub user_id: String,
    pub token: String,
    pub daily_used: i32,
    pub daily_limit: i32,
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

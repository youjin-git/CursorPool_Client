use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

// 共享的 HTTP 客户端
#[derive(Clone)]
pub struct ApiClient(pub(crate) Arc<Client>);

impl Default for ApiClient {
    fn default() -> Self {
        Self(Arc::new(
            Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        ))
    }
}

// 从环境变量获取基础 URL
pub fn get_base_url() -> String {
    std::env::var("CURSOR_POOL_API_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:11451/api".to_string())
}

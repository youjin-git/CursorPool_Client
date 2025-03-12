use reqwest::{Client, Request, Response};
use reqwest::header::HeaderValue;
use std::sync::Arc;
use std::time::Duration;
use tauri::AppHandle;
use tauri::Manager;
use crate::api::interceptor::{Interceptor, AuthInterceptor, is_auth_required_url, save_auth_token};
use crate::database::Database;

/// HTTP 请求客户端，支持拦截器机制
pub struct ApiClient {
    client: Arc<Client>,
    interceptors: Vec<Box<dyn Interceptor>>,
    app_handle: Option<Arc<AppHandle>>,
}

impl ApiClient {
    /// 创建 API 客户端实例
    pub fn new(app_handle: Option<AppHandle>) -> Self {
        let client = Arc::new(
            Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to create HTTP client"),
        );
        
        let mut interceptors = Vec::new();
        if let Some(handle) = &app_handle {
            interceptors.push(Box::new(AuthInterceptor::new(Arc::new(handle.clone()))) as Box<dyn Interceptor>);
        }
        
        Self {
            client,
            interceptors,
            app_handle: app_handle.map(Arc::new),
        }
    }
    
    /// 发送 HTTP 请求
    pub async fn send(&self, mut request: Request) -> Result<Response, reqwest::Error> {
        let url: String = request.url().to_string();
        
        if is_auth_required_url(&url) {
            for interceptor in &self.interceptors {
                if let Err(_) = interceptor.intercept(&mut request) {
                    continue;
                }
            }
        }
        
        let response = self.client.execute(request).await?;
        
        if self.app_handle.is_none() {
            return Ok(response);
        }
        
        let handle = self.app_handle.as_ref().unwrap();
        let db = handle.state::<Database>();
        let status = response.status();
        let url_str = url.clone();
        
        let response_text = response.text().await?;
        
        if url_str.contains("/user/updatePassword") {
            if let Ok(response_json) = serde_json::from_str::<serde_json::Value>(&response_text) {
                if response_json["status"] == 200 {
                    let _ = clear_auth_token(&db).await;
                }
            }
        } else {
            let _ = save_auth_token(&db, &url_str, &response_text).await;
        }
        
        Ok(Response::from(
            http::Response::builder()
                .status(status)
                .body(response_text)
                .unwrap()
        ))
    }
    
    /// 创建 GET 请求
    pub fn get(&self, url: impl AsRef<str>) -> RequestBuilder {
        RequestBuilder {
            inner: self.client.get(url.as_ref()),
            client: self,
        }
    }
    
    /// 创建 POST 请求
    pub fn post(&self, url: impl AsRef<str>) -> RequestBuilder {
        RequestBuilder {
            inner: self.client.post(url.as_ref()),
            client: self,
        }
    }
    
    /// 创建 PUT 请求
    pub fn put(&self, url: impl AsRef<str>) -> RequestBuilder {
        RequestBuilder {
            inner: self.client.put(url.as_ref()),
            client: self,
        }
    }
    
    /// 创建 DELETE 请求
    pub fn delete(&self, url: impl AsRef<str>) -> RequestBuilder {
        RequestBuilder {
            inner: self.client.delete(url.as_ref()),
            client: self,
        }
    }
}

/// HTTP 请求构建器
pub struct RequestBuilder<'a> {
    inner: reqwest::RequestBuilder,
    client: &'a ApiClient,
}

impl<'a> RequestBuilder<'a> {
    /// 发送请求
    pub async fn send(self) -> Result<Response, reqwest::Error> {
        let request = self.inner.build()?;
        self.client.send(request).await
    }

    /// 添加表单数据
    pub fn form<T: serde::Serialize + ?Sized>(self, form: &T) -> Self {
        Self {
            inner: self.inner.form(form),
            client: self.client,
        }
    }

    /// 添加 JSON 数据
    pub fn json<T: serde::Serialize + ?Sized>(self, json: &T) -> Self {
        Self {
            inner: self.inner.json(json),
            client: self.client,
        }
    }

    /// 添加请求头
    pub fn header(self, key: &str, value: &str) -> Self {
        Self {
            inner: self.inner.header(key, HeaderValue::from_str(value).unwrap()),
            client: self.client,
        }
    }

    /// 添加 multipart 表单数据
    pub fn multipart<T: IntoIterator<Item = (String, String)>>(self, form: T) -> Self {
        let mut form_builder = reqwest::multipart::Form::new();
        for (key, value) in form {
            form_builder = form_builder.text(key, value);
        }
        
        Self {
            inner: self.inner
                .header("Accept", "*/*")
                .multipart(form_builder),
            client: self.client,
        }
    }
}

/// 获取 API 基础 URL
pub fn get_base_url() -> String {
    "http://103.108.66.226:555/api".to_string()
}

/// 清除认证令牌
async fn clear_auth_token(db: &tauri::State<'_, Database>) -> Result<(), String> {
    db.delete_item("user.info.token")
        .map_err(|e| e.to_string())?;
    Ok(())
}

use crate::api::client::ApiClient;
use crate::api::endpoints::report_bug;
use tauri::State;

pub struct ErrorReporter;

impl ErrorReporter {
    /// 上报错误信息
    pub async fn report_error(
        client: State<'_, ApiClient>,
        function_name: &str,
        error: &str,
        api_key: Option<String>,
        severity: Option<String>,
    ) {
        // 构建错误描述
        let bug_description = format!(
            "函数: {}\n错误: {}", 
            function_name, 
            error
        );
        
        // 使用 report_bug 函数上报错误
        let _ = report_bug(
            client,
            severity.unwrap_or("low".to_string()),
            bug_description,
            api_key,
            None,
            None,
        ).await;
    }
} 
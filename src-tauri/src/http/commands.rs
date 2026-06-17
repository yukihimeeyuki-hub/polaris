use crate::http::interceptor::{RequestConfig, ResponseData};
use std::sync::Arc;
use tauri::Manager;

/// 全局 HTTP 客户端状态
pub struct HttpState {
    pub client: crate::http::HttpClient,
}

/// Tauri command: 创建 HTTP 客户端
#[tauri::command]
pub fn create_http_client() -> Result<(), String> {
    // 客户端会在应用启动时初始化
    Ok(())
}

/// Tauri command: 通用请求
#[tauri::command]
pub async fn http_request(
    app: tauri::AppHandle,
    config: RequestConfig,
) -> Result<ResponseData, String> {
    let state = app.state::<HttpState>();
    state.client.request(config).await
}

/// Tauri command: GET 请求
#[tauri::command]
pub async fn http_get(app: tauri::AppHandle, url: String) -> Result<ResponseData, String> {
    let state = app.state::<HttpState>();
    state.client.get(url).await
}

/// Tauri command: POST 请求
#[tauri::command]
pub async fn http_post(
    app: tauri::AppHandle,
    url: String,
    body: String,
) -> Result<ResponseData, String> {
    let state = app.state::<HttpState>();
    state.client.post(url, body).await
}

/// Tauri command: PUT 请求
#[tauri::command]
pub async fn http_put(
    app: tauri::AppHandle,
    url: String,
    body: String,
) -> Result<ResponseData, String> {
    let state = app.state::<HttpState>();
    state.client.put(url, body).await
}

/// Tauri command: DELETE 请求
#[tauri::command]
pub async fn http_delete(app: tauri::AppHandle, url: String) -> Result<ResponseData, String> {
    let state = app.state::<HttpState>();
    state.client.delete(url).await
}

/// Tauri command: HEAD 请求
#[tauri::command]
pub async fn http_head(app: tauri::AppHandle, url: String) -> Result<ResponseData, String> {
    let state = app.state::<HttpState>();
    state.client.head(url).await
}

/// Tauri command: OPTIONS 请求
#[tauri::command]
pub async fn http_options(app: tauri::AppHandle, url: String) -> Result<ResponseData, String> {
    let state = app.state::<HttpState>();
    state.client.options(url).await
}

/// 示例：自定义请求拦截器 - 添加 Authorization header
pub struct AuthInterceptor {
    pub token: String,
}

impl crate::http::RequestInterceptor for AuthInterceptor {
    fn before_request(&self, config: &mut RequestConfig) -> Result<(), String> {
        config
            .headers
            .insert("Authorization".to_string(), format!("Bearer {}", self.token));
        Ok(())
    }
}

/// 示例：自定义响应拦截器 - 统一错误处理
pub struct ErrorHandlerInterceptor;

impl crate::http::ResponseInterceptor for ErrorHandlerInterceptor {
    fn after_response(&self, response: &mut ResponseData) -> Result<(), String> {
        if !response.is_success() {
            log::error!(
                "HTTP Error: {} {} - {}",
                response.status,
                response.status_text,
                response.body
            );
        }
        Ok(())
    }
}

/// 示例：自定义响应拦截器 - 数据转换
pub struct DataTransformInterceptor;

impl crate::http::ResponseInterceptor for DataTransformInterceptor {
    fn after_response(&self, _response: &mut ResponseData) -> Result<(), String> {
        // 可以在这里对响应数据进行统一处理
        // 例如：解压、解密、格式化等
        Ok(())
    }
}

/// 初始化 HTTP 客户端并添加到 Tauri 状态
pub fn init_http_client() -> HttpState {
    let mut client = crate::http::HttpClient::new();

    // 添加默认的拦截器
    // 示例：添加错误处理拦截器
    client.add_response_interceptor(Arc::new(ErrorHandlerInterceptor));

    // 示例：添加数据转换拦截器
    client.add_response_interceptor(Arc::new(DataTransformInterceptor));

    // 如果需要认证，可以添加认证拦截器
    // client.add_request_interceptor(Arc::new(AuthInterceptor {
    //     token: "your-token".to_string(),
    // }));

    HttpState { client }
}

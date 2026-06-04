use reqwest::{Request, Response};
use reqwest::header::{HeaderName, HeaderValue};
use std::collections::HashMap;

/// 请求拦截器 trait
pub trait RequestInterceptor: Send + Sync {
    fn intercept(&self, request: Request) -> Result<Request, Box<dyn std::error::Error + Send + Sync>>;
}

/// 响应拦截器 trait
pub trait ResponseInterceptor: Send + Sync {
    fn intercept(&self, response: Response) -> Result<Response, Box<dyn std::error::Error + Send + Sync>>;
}

/// 默认的请求拦截器实现
pub struct DefaultRequestInterceptor {
    pub headers: HashMap<String, String>,
}

impl DefaultRequestInterceptor {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn with_token(token: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        Self { headers }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }
}

impl RequestInterceptor for DefaultRequestInterceptor {
    fn intercept(&self, mut request: Request) -> Result<Request, Box<dyn std::error::Error + Send + Sync>> {
        for (key, value) in &self.headers {
            let header_name = HeaderName::from_bytes(key.as_bytes())?;
            let header_value = HeaderValue::from_str(value)?;
            request.headers_mut().insert(header_name, header_value);
        }
        Ok(request)
    }
}

/// 默认的响应拦截器实现
pub struct DefaultResponseInterceptor;

impl DefaultResponseInterceptor {
    pub fn new() -> Self {
        Self
    }
}

impl ResponseInterceptor for DefaultResponseInterceptor {
    fn intercept(&self, response: Response) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        // 可以在这里添加统一的响应处理逻辑
        // 例如：检查状态码、解析错误码等
        if !response.status().is_success() {
            return Err(format!("HTTP request failed with status: {}", response.status()).into());
        }
        Ok(response)
    }
}
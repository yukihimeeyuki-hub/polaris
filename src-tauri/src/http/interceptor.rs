use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 请求拦截器 trait
pub trait RequestInterceptor: Send + Sync {
    /// 请求发送前的拦截处理
    /// 可以修改请求配置、添加 headers 等
    fn before_request(&self, config: &mut RequestConfig) -> Result<(), String>;
}

/// 响应拦截器 trait
pub trait ResponseInterceptor: Send + Sync {
    /// 响应接收后的拦截处理
    /// 可以修改响应数据、处理错误等
    fn after_response(&self, response: &mut ResponseData) -> Result<(), String>;
}

/// 拦截器上下文，用于在拦截器链中传递数据
#[derive(Debug, Clone, Default)]
pub struct InterceptorContext {
    pub data: HashMap<String, String>,
}

impl InterceptorContext {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}

/// 请求配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestConfig {
    pub url: String,
    pub method: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub query: HashMap<String, String>,
    #[serde(default)]
    pub timeout: Option<u64>,
}

impl RequestConfig {
    pub fn new(method: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            method: method.into(),
            headers: HashMap::new(),
            body: None,
            query: HashMap::new(),
            timeout: None,
        }
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

/// 响应数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseData {
    pub status: u16,
    pub status_text: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: String,
}

impl ResponseData {
    pub fn new(status: u16) -> Self {
        Self {
            status,
            status_text: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

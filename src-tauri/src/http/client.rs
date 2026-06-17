use crate::http::interceptor::{
    RequestConfig, RequestInterceptor, ResponseData, ResponseInterceptor,
};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::sync::Arc;
use std::time::Duration;

/// HTTP 客户端
pub struct HttpClient {
    client: reqwest::Client,
    request_interceptors: Vec<Arc<dyn RequestInterceptor>>,
    response_interceptors: Vec<Arc<dyn ResponseInterceptor>>,
}

impl HttpClient {
    /// 创建新的 HTTP 客户端
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            request_interceptors: Vec::new(),
            response_interceptors: Vec::new(),
        }
    }

    /// 创建带配置的 HTTP 客户端
    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(timeout)
                .build()
                .unwrap_or_default(),
            request_interceptors: Vec::new(),
            response_interceptors: Vec::new(),
        }
    }

    /// 添加请求拦截器
    pub fn add_request_interceptor(&mut self, interceptor: Arc<dyn RequestInterceptor>) {
        self.request_interceptors.push(interceptor);
    }

    /// 添加响应拦截器
    pub fn add_response_interceptor(&mut self, interceptor: Arc<dyn ResponseInterceptor>) {
        self.response_interceptors.push(interceptor);
    }

    /// 通用请求方法
    pub async fn request(&self, mut config: RequestConfig) -> Result<ResponseData, String> {
        // 执行请求拦截器
        for interceptor in &self.request_interceptors {
            interceptor.before_request(&mut config)?;
        }

        // 构建请求
        let request = self.build_request(&config)?;

        // 发送请求
        let response = request
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        // 解析响应
        let mut response_data = self.parse_response(response).await?;

        // 执行响应拦截器
        for interceptor in &self.response_interceptors {
            interceptor.after_response(&mut response_data)?;
        }

        Ok(response_data)
    }

    /// GET 请求
    pub async fn get(&self, url: impl Into<String>) -> Result<ResponseData, String> {
        let config = RequestConfig::new("GET", url);
        self.request(config).await
    }

    /// GET 请求带配置
    pub async fn get_with_config(&self, config: RequestConfig) -> Result<ResponseData, String> {
        let mut config = config;
        config.method = "GET".to_string();
        self.request(config).await
    }

    /// POST 请求
    pub async fn post(
        &self,
        url: impl Into<String>,
        body: impl Into<String>,
    ) -> Result<ResponseData, String> {
        let config = RequestConfig::new("POST", url).body(body);
        self.request(config).await
    }

    /// POST 请求带配置
    pub async fn post_with_config(&self, config: RequestConfig) -> Result<ResponseData, String> {
        let mut config = config;
        config.method = "POST".to_string();
        self.request(config).await
    }

    /// PUT 请求
    pub async fn put(
        &self,
        url: impl Into<String>,
        body: impl Into<String>,
    ) -> Result<ResponseData, String> {
        let config = RequestConfig::new("PUT", url).body(body);
        self.request(config).await
    }

    /// PUT 请求带配置
    pub async fn put_with_config(&self, config: RequestConfig) -> Result<ResponseData, String> {
        let mut config = config;
        config.method = "PUT".to_string();
        self.request(config).await
    }

    /// DELETE 请求
    pub async fn delete(&self, url: impl Into<String>) -> Result<ResponseData, String> {
        let config = RequestConfig::new("DELETE", url);
        self.request(config).await
    }

    /// DELETE 请求带配置
    pub async fn delete_with_config(&self, config: RequestConfig) -> Result<ResponseData, String> {
        let mut config = config;
        config.method = "DELETE".to_string();
        self.request(config).await
    }

    /// HEAD 请求
    pub async fn head(&self, url: impl Into<String>) -> Result<ResponseData, String> {
        let config = RequestConfig::new("HEAD", url);
        self.request(config).await
    }

    /// HEAD 请求带配置
    pub async fn head_with_config(&self, config: RequestConfig) -> Result<ResponseData, String> {
        let mut config = config;
        config.method = "HEAD".to_string();
        self.request(config).await
    }

    /// OPTIONS 请求
    pub async fn options(&self, url: impl Into<String>) -> Result<ResponseData, String> {
        let config = RequestConfig::new("OPTIONS", url);
        self.request(config).await
    }

    /// OPTIONS 请求带配置
    pub async fn options_with_config(
        &self,
        config: RequestConfig,
    ) -> Result<ResponseData, String> {
        let mut config = config;
        config.method = "OPTIONS".to_string();
        self.request(config).await
    }

    /// 构建 reqwest 请求
    fn build_request(&self, config: &RequestConfig) -> Result<reqwest::RequestBuilder, String> {
        let method = config
            .method
            .parse::<reqwest::Method>()
            .map_err(|e| format!("Invalid method: {}", e))?;

        let mut builder = self.client.request(method, &config.url);

        // 添加 headers
        let mut headers = HeaderMap::new();
        for (key, value) in &config.headers {
            if let (Ok(name), Ok(val)) = (
                key.parse::<HeaderName>(),
                HeaderValue::from_str(value),
            ) {
                headers.insert(name, val);
            }
        }
        builder = builder.headers(headers);

        // 添加 query 参数
        if !config.query.is_empty() {
            let query_string = serde_urlencoded::to_string(&config.query)
                .map_err(|e| format!("Failed to encode query: {}", e))?;
            if !query_string.is_empty() {
                let url_with_query = if config.url.contains('?') {
                    format!("{}&{}", config.url, query_string)
                } else {
                    format!("{}?{}", config.url, query_string)
                };
                builder = self.client.request(
                    config.method.parse::<reqwest::Method>().map_err(|e| format!("Invalid method: {}", e))?,
                    &url_with_query
                );
            }
        }

        // 添加 body
        if let Some(body) = &config.body {
            builder = builder.body(body.clone());
        }

        // 设置超时
        if let Some(timeout) = config.timeout {
            builder = builder.timeout(Duration::from_secs(timeout));
        }

        Ok(builder)
    }

    /// 解析响应
    async fn parse_response(
        &self,
        response: reqwest::Response,
    ) -> Result<ResponseData, String> {
        let status = response.status().as_u16();
        let status_text = response
            .status()
            .canonical_reason()
            .unwrap_or("")
            .to_string();

        let mut headers = std::collections::HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(v) = value.to_str() {
                headers.insert(key.as_str().to_string(), v.to_string());
            }
        }

        let body = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        Ok(ResponseData {
            status,
            status_text,
            headers,
            body,
        })
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

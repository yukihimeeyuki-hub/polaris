use std::time::Duration;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::module::interceptor::{RequestInterceptor, ResponseInterceptor, DefaultRequestInterceptor, DefaultResponseInterceptor};

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse<T> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

pub struct HttpServer {
    client: Client,
    base_url: String,
    request_interceptor: Box<dyn RequestInterceptor>,
    response_interceptor: Box<dyn ResponseInterceptor>,
}

impl HttpServer {
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .pool_max_idle_per_host(10)
            .build()
            .expect("http client error");

        Self {
            client,
            base_url: base_url.to_string(),
            request_interceptor: Box::new(DefaultRequestInterceptor::new()),
            response_interceptor: Box::new(DefaultResponseInterceptor::new()),
        }
    }

    /// 设置请求拦截器
    pub fn with_request_interceptor(mut self, interceptor: impl RequestInterceptor + 'static) -> Self {
        self.request_interceptor = Box::new(interceptor);
        self
    }

    /// 设置响应拦截器
    pub fn with_response_interceptor(mut self, interceptor: impl ResponseInterceptor + 'static) -> Self {
        self.response_interceptor = Box::new(interceptor);
        self
    }

    /// 设置认证 token
    pub fn with_token(mut self, token: &str) -> Self {
        self.request_interceptor = Box::new(DefaultRequestInterceptor::with_token(token));
        self
    }

    /// 获取 base_url
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// 拼接完整 URL
    fn build_url(&self, path: &str) -> String {
        if path.starts_with("http://") || path.starts_with("https://") {
            path.to_string()
        } else {
            format!("{}{}", self.base_url, path)
        }
    }

    /// GET 请求
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<HttpResponse<T>, Box<dyn std::error::Error + Send + Sync>> {
        let url = self.build_url(path);
        let request = self.client.get(&url).build()?;
        let intercepted_request = self.request_interceptor.intercept(request)?;
        let response = self.client.execute(intercepted_request).await?;
        let intercepted_response = self.response_interceptor.intercept(response)?;
        let body = intercepted_response.text().await?;
        let result: HttpResponse<T> = serde_json::from_str(&body)?;
        Ok(result)
    }

    /// POST 请求
    pub async fn post<T: for<'de> Deserialize<'de>, B: Serialize>(&self, path: &str, body: &B) -> Result<HttpResponse<T>, Box<dyn std::error::Error + Send + Sync>> {
        let url = self.build_url(path);
        let request = self.client.post(&url).json(body).build()?;
        let intercepted_request = self.request_interceptor.intercept(request)?;
        let response = self.client.execute(intercepted_request).await?;
        let intercepted_response = self.response_interceptor.intercept(response)?;
        let response_body = intercepted_response.text().await?;
        let result: HttpResponse<T> = serde_json::from_str(&response_body)?;
        Ok(result)
    }

    /// PUT 请求
    pub async fn put<T: for<'de> Deserialize<'de>, B: Serialize>(&self, path: &str, body: &B) -> Result<HttpResponse<T>, Box<dyn std::error::Error + Send + Sync>> {
        let url = self.build_url(path);
        let request = self.client.put(&url).json(body).build()?;
        let intercepted_request = self.request_interceptor.intercept(request)?;
        let response = self.client.execute(intercepted_request).await?;
        let intercepted_response = self.response_interceptor.intercept(response)?;
        let response_body = intercepted_response.text().await?;
        let result: HttpResponse<T> = serde_json::from_str(&response_body)?;
        Ok(result)
    }

    /// DELETE 请求
    pub async fn delete<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<HttpResponse<T>, Box<dyn std::error::Error + Send + Sync>> {
        let url = self.build_url(path);
        let request = self.client.delete(&url).build()?;
        let intercepted_request = self.request_interceptor.intercept(request)?;
        let response = self.client.execute(intercepted_request).await?;
        let intercepted_response = self.response_interceptor.intercept(response)?;
        let body = intercepted_response.text().await?;
        let result: HttpResponse<T> = serde_json::from_str(&body)?;
        Ok(result)
    }

    /// PATCH 请求
    pub async fn patch<T: for<'de> Deserialize<'de>, B: Serialize>(&self, path: &str, body: &B) -> Result<HttpResponse<T>, Box<dyn std::error::Error + Send + Sync>> {
        let url = self.build_url(path);
        let request = self.client.patch(&url).json(body).build()?;
        let intercepted_request = self.request_interceptor.intercept(request)?;
        let response = self.client.execute(intercepted_request).await?;
        let intercepted_response = self.response_interceptor.intercept(response)?;
        let response_body = intercepted_response.text().await?;
        let result: HttpResponse<T> = serde_json::from_str(&response_body)?;
        Ok(result)
    }
}
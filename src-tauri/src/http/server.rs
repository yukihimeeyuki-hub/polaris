use std::time::Duration;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct HttpResponse<T>{
    pub code:u16,
    pub msg:String,
    pub data:Option<T>,
}
pub struct HttpServer{
    client: Client,
    base_url: String,

}
impl HttpServer {
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .pool_max_idle_per_host(10)
            .build().expect("http client error");
        Self{
            client,
            base_url: base_url.to_string(),
        }
    }
}
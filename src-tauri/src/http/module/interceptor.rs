use reqwest::{Request, Response};
use reqwest::header::{HeaderName, HeaderValue};

struct Interceptor;

impl Interceptor {
    pub fn request_interceptor(
        &self,
        mut request: Request,
    ) -> Result<Request, Box<dyn std::error::Error>> {
        request.headers_mut().insert(
            HeaderName::from_static("token"),
            HeaderValue::from_static("123456"),
        );

        Ok(request)
    }

    pub fn response_interceptor(
        &self,
        response: Response,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        Ok(response)
    }
}
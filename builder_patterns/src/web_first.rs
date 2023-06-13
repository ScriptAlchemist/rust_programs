use crate::prelude::*;
use std::fmt;
use serde::Serialize;
use serde_json;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum HttpVersion {
    Http1_1,
    Http2,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HttpVersion::Http1_1 => write!(f, "HTTP/1.1"),
            HttpVersion::Http2 => write!(f, "HTTP/2"),
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub struct HttpHeader {
    pub name: String,
    pub value: String,
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct HttpRequest {
    method: HttpMethod,
    url: String,
    http_version: HttpVersion,
    headers: Vec<HttpHeader>,
    body: Option<String>,
}

#[allow(dead_code)]
impl HttpRequest {
    pub fn new() -> HttpRequestBuilder {
        HttpRequestBuilder::new()
    }
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct HttpRequestBuilder {
    method: Option<HttpMethod>,
    url: Option<String>,
    http_version: Option<HttpVersion>,
    headers: Vec<HttpHeader>,
    body: Option<String>,
}

#[allow(dead_code)]
impl HttpRequestBuilder {
    pub fn new() -> Self {
        HttpRequestBuilder::default()
    }

    pub fn method(&mut self, method: HttpMethod) -> &mut Self {
        self.method = Some(method);
        self
    }
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }
    pub fn http_version(&mut self, version: HttpVersion) -> &mut Self {
        self.http_version = Some(version);
        self
    }
    pub fn header(&mut self, header: HttpHeader) -> &mut Self {
        self.headers.push(header);
        self
    }
    pub fn body(&mut self, body: impl Serialize) -> &mut Self {
        let json = serde_json::to_string(&body).unwrap();
        self.body = Some(json);
        self
    }
    pub fn build(mut self) -> Result<HttpRequest> {
        let method = self.method.take().ok_or(Error::Static("No METHOD"))?;
        let url = self.url.take().ok_or(Error::Static("No URL"))?;
        let http_version = self.http_version.take().ok_or(Error::Static("No HTTP VERSION"))?;

        Ok(HttpRequest {
            method,
            url,
            http_version,
            headers: self.headers,
            body: self.body.take(),
        })
    }
}



#[cfg(test)]
mod request_builder {
    use super::*;

    #[derive(Serialize)]
    struct ExampleBody {
        name: String,
        age: u32,
    }

    #[test]
    fn setup_new_builder() {
        let req: HttpRequestBuilder = HttpRequest::new();
        assert_eq!(req.method, None);
        assert_eq!(req.url, None);
        assert_eq!(req.http_version, None);
        assert_eq!(req.headers.len(), 0);
        assert_eq!(req.body, None);
    }
    #[test]
    fn build_request() {
        let body = ExampleBody {
            name: "Justin".to_string(),
            age: 38,
        };

        let mut req: HttpRequestBuilder = HttpRequest::new();
        req.method(HttpMethod::Put)
            .url("https://google.com".to_string())
            .http_version(HttpVersion::Http1_1)
            .header(
                HttpHeader {
                    name: "cookie".to_string(),
                    value: "value".to_string()
                })
            .body(&body);
        let build = req.build().unwrap();

        assert_eq!(build.method, HttpMethod::Put);
        assert_eq!(build.url, "https://google.com".to_string());
        assert_eq!(build.http_version, HttpVersion::Http1_1);
        assert_eq!(build.headers.len(), 1);
        assert_eq!(
            build.body,
            Some(r#"{"name":"Justin","age":38}"#.to_string())
        );
    }
    #[test]
    fn build_with_no_method() {
        let mut req: HttpRequestBuilder = HttpRequest::new();
        req.url("https://google.com".to_string())
            .http_version(HttpVersion::Http1_1);

        let result = req.build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Static error: No METHOD");
    }
    #[test]
    fn build_with_no_url() {
        let mut req: HttpRequestBuilder = HttpRequest::new();
        req.method(HttpMethod::Put)
            .http_version(HttpVersion::Http1_1);

        let result = req.build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Static error: No URL");
    }
    #[test]
    fn build_with_no_http_version() {
        let mut req: HttpRequestBuilder = HttpRequest::new();
        req.method(HttpMethod::Put)
            .url("https://google.com".to_string());

        let result = req.build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Static error: No HTTP VERSION");
    }
}

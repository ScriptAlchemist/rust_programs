use crate::prelude::*;
use std::fmt;
use serde::Serialize;
use serde_json;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
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
#[derive(PartialEq, Debug, Clone)]
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
    pub key: String,
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
    pub fn build(&self) -> Result<HttpRequest> {
        let method = self.method.clone()
            .unwrap_or_else(|| HttpMethod::Get);

        let http_version = self.http_version.clone()
            .unwrap_or_else(|| HttpVersion::Http2);

        let Some(url) = self.url.as_ref() else {
            return Err(Error::Static("No URL"));
        };

        Ok(HttpRequest {
            method,
            url: url.to_string(),
            http_version,
            headers: self.headers.clone(),
            body: self.body.clone(),
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
        let body = ExampleBody {
            name: "Justin".to_string(),
            age: 38,
        };
        let req = HttpRequestBuilder::new()
            .url("https://google.com")
            .method(HttpMethod::Put)
            .http_version(HttpVersion::Http2)
            .header(
                HttpHeader {
                    key: "key".to_string(),
                    value: "value".to_string()
                })
            .body(&body)
            .build();
        let result = req.unwrap();

        assert_eq!(result.method, HttpMethod::Put);
        assert_eq!(result.url, "https://google.com".to_string());
        assert_eq!(result.http_version, HttpVersion::Http2);
        assert_eq!(result.headers.len(), 1);
        assert_eq!(result.body, Some(r#"{"name":"Justin","age":38}"#.to_string()));
    }
    // #[test]
    // fn build_request() {
        // let body = ExampleBody {
            // name: "Justin".to_string(),
            // age: 38,
        // };
// 
        // let mut req: HttpRequestBuilder = HttpRequest::new();
        // req.method(HttpMethod::Put)
            // .url("https://google.com".to_string())
            // .http_version(HttpVersion::Http1_1)
            // .header(
                // HttpHeader {
                    // name: "cookie".to_string(),
                    // value: "value".to_string()
                // })
            // .body(&body);
        // let build = req.build().unwrap();
// 
        // assert_eq!(build.method, HttpMethod::Put);
        // assert_eq!(build.url, "https://google.com".to_string());
        // assert_eq!(build.http_version, HttpVersion::Http1_1);
        // assert_eq!(build.headers.len(), 1);
        // assert_eq!(
            // build.body,
            // Some(r#"{"name":"Justin","age":38}"#.to_string())
        // );
    // }
}

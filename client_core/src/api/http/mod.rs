use std::fmt;

pub mod traits;

pub mod constants {
    pub static GET: &'static str = "Get";
    pub static POST: &'static str = "Post";
    pub static PUT: &'static str = "Put";
    pub static DELETE: &'static str = "Delete";
    pub static PATCH: &'static str = "Patch";

    pub static ACCEPT_K: &'static str = "Accept";
    pub static PROTOBUF_V: &'static str = "application/x-protobuf";
    pub static CONTENT_TYPE_K: &'static str = "Content-Type";
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ResponseType {
    Ok = 0,
    Timeout = 1,
    NetworkError = 2,
    HttpError = 3,
    Other = 4,
}

#[derive(Clone, Debug)]
pub struct HttpRequestArgs {
    pub method: String,
    pub endpoint: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub timeout_secs: u64,
}

impl HttpRequestArgs {
    pub fn new(
        method: String,
        endpoint: String,
        headers: Vec<(String, String)>,
        body: Vec<u8>,
        timeout_secs: u64,
    ) -> Self {
        Self {
            method,
            endpoint,
            headers,
            body,
            timeout_secs,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HttpResponse {
    pub status: u8, // ResponseType
    pub response_code: u16,
    pub payload: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status: u8, response_code: u16, payload: Vec<u8>) -> Self {
        Self {
            status,
            response_code,
            payload,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl HttpMethod {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Get => constants::GET,
            Self::Post => constants::POST,
            Self::Put => constants::PUT,
            Self::Patch => constants::PATCH,
            Self::Delete => constants::DELETE,
        }
    }
}

pub struct ProtobufRequestBuilder {
    url: String,
    body: Vec<u8>,
    timeout_secs: u64,
    method: HttpMethod,
}

impl ProtobufRequestBuilder {
    pub fn new(url: String, body: Vec<u8>) -> Self {
        Self {
            url,
            body,
            timeout_secs: 10,
            method: HttpMethod::Post,
        }
    }

    pub fn method(mut self, method: HttpMethod) -> Self {
        self.method = method;
        self
    }

    pub fn build(self) -> HttpRequestArgs {
        HttpRequestArgs {
            method: self.method.as_str().to_string(),
            endpoint: self.url,
            headers: protobuf_headers(),
            body: self.body,
            timeout_secs: self.timeout_secs,
        }
    }
}

pub fn protobuf_headers() -> Vec<(String, String)> {
    vec![
        (
            constants::ACCEPT_K.to_owned(),
            constants::PROTOBUF_V.to_owned(),
        ),
        (
            constants::CONTENT_TYPE_K.to_owned(),
            "application/x-protobuf".to_string(),
        ),
    ]
}

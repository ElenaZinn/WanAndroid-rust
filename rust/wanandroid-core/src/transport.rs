use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: BTreeMap<String, String>,
    pub form: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
}

impl HttpRequest {
    pub fn get(path: impl Into<String>, headers: BTreeMap<String, String>) -> Self {
        Self {
            method: HttpMethod::Get,
            path: path.into(),
            headers,
            form: BTreeMap::new(),
        }
    }

    pub fn post_form(
        path: impl Into<String>,
        headers: BTreeMap<String, String>,
        form: BTreeMap<String, String>,
    ) -> Self {
        Self {
            method: HttpMethod::Post,
            path: path.into(),
            headers,
            form,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub body: String,
    pub headers: BTreeMap<String, String>,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TransportError {
    #[error("transport error: {0}")]
    Failed(String),
}

pub trait HttpClient: Send + Sync {
    fn execute(&self, request: &HttpRequest) -> Result<HttpResponse, String>;
}

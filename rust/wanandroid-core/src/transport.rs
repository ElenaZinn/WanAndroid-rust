use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest {
    pub path: String,
    pub headers: BTreeMap<String, String>,
}

impl HttpRequest {
    pub fn get(path: impl Into<String>, headers: BTreeMap<String, String>) -> Self {
        Self {
            path: path.into(),
            headers,
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TransportError {
    #[error("transport error: {0}")]
    Failed(String),
}

pub trait HttpClient: Send + Sync {
    fn get(&self, request: &HttpRequest) -> Result<String, String>;
}

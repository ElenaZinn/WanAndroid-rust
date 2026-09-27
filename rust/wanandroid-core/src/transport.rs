use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum TransportError {
    #[error("transport error: {0}")]
    Failed(String),
}

pub trait HttpClient: Send + Sync {
    fn get(&self, path: &str) -> Result<String, String>;
}

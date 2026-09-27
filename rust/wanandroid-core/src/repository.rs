use crate::domain::{ApiArticle, ApiBanner, ApiEnvelope, ApiPage, ArticlePage, Banner};
use crate::transport::HttpClient;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RepositoryError {
    #[error("transport error: {0}")]
    Transport(String),
    #[error("invalid response: {0}")]
    Decode(String),
    #[error("api error {code}: {message}")]
    Api { code: i32, message: String },
}

pub trait HomeRepository: Send + Sync {
    fn get_banners(&self) -> Result<Vec<Banner>, RepositoryError>;
    fn get_articles(&self, page: u32) -> Result<ArticlePage, RepositoryError>;
}

pub struct WanAndroidRepository<C> {
    client: Arc<C>,
}

impl<C> WanAndroidRepository<C> {
    pub fn new(client: Arc<C>) -> Self {
        Self { client }
    }
}

impl<C: HttpClient> HomeRepository for WanAndroidRepository<C> {
    fn get_banners(&self) -> Result<Vec<Banner>, RepositoryError> {
        let body = self
            .client
            .get("/banner/json")
            .map_err(RepositoryError::Transport)?;
        let envelope: ApiEnvelope<Vec<ApiBanner>> =
            serde_json::from_str(&body).map_err(|error| RepositoryError::Decode(error.to_string()))?;
        if envelope.error_code != 0 {
            return Err(RepositoryError::Api {
                code: envelope.error_code,
                message: envelope.error_msg,
            });
        }
        Ok(envelope.data.into_iter().map(Into::into).collect())
    }

    fn get_articles(&self, page: u32) -> Result<ArticlePage, RepositoryError> {
        let path = format!("/article/list/{page}/json");
        let body = self
            .client
            .get(&path)
            .map_err(RepositoryError::Transport)?;
        let envelope: ApiEnvelope<ApiPage<ApiArticle>> =
            serde_json::from_str(&body).map_err(|error| RepositoryError::Decode(error.to_string()))?;
        if envelope.error_code != 0 {
            return Err(RepositoryError::Api {
                code: envelope.error_code,
                message: envelope.error_msg,
            });
        }
        Ok(envelope.data.into())
    }
}

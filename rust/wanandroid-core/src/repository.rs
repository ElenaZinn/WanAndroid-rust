use crate::domain::{ApiArticle, ApiBanner, ApiEnvelope, ApiPage, ArticlePage, Banner};
use crate::session::{cookie_headers, SessionStore};
use crate::transport::{HttpClient, HttpRequest};
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

pub struct WanAndroidRepository<C, S = crate::session::MemorySessionStore> {
    client: Arc<C>,
    session: Arc<S>,
}

impl<C> WanAndroidRepository<C> {
    pub fn new(client: Arc<C>) -> Self {
        Self::with_session(client, Arc::new(crate::session::MemorySessionStore::default()))
    }
}

impl<C, S> WanAndroidRepository<C, S> {
    pub fn with_session(client: Arc<C>, session: Arc<S>) -> Self {
        Self { client, session }
    }

    fn request(&self, path: impl Into<String>) -> Result<String, RepositoryError>
    where
        C: HttpClient,
        S: SessionStore,
    {
        let request = HttpRequest::get(path, cookie_headers(&self.session.load_cookies()));
        self.client
            .get(&request)
            .map_err(RepositoryError::Transport)
    }
}

impl<C: HttpClient, S: SessionStore> HomeRepository for WanAndroidRepository<C, S> {
    fn get_banners(&self) -> Result<Vec<Banner>, RepositoryError> {
        let body = self.request("/banner/json")?;
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
        let body = self.request(format!("/article/list/{page}/json"))?;
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

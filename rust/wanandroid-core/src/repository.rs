use crate::domain::{ApiArticle, ApiBanner, ApiEnvelope, ApiPage, ArticlePage, Banner};
use crate::session::{cookie_headers, SessionStore};
use crate::transport::{HttpClient, HttpRequest, HttpResponse};
use std::collections::BTreeMap;
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedUser {
    pub id: u64,
    pub username: String,
}

pub trait AuthRepository: Send + Sync {
    fn login(&self, username: &str, password: &str) -> Result<AuthenticatedUser, RepositoryError>;
    fn logout(&self) -> Result<(), RepositoryError>;
    fn restore_session(&self) -> Option<AuthenticatedUser>;
}

pub struct WanAndroidRepository<C, S = crate::session::MemorySessionStore> {
    client: Arc<C>,
    session: Arc<S>,
}

impl<C, S> Clone for WanAndroidRepository<C, S> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            session: self.session.clone(),
        }
    }
}

impl<C> WanAndroidRepository<C> {
    pub fn new(client: Arc<C>) -> Self {
        Self::with_session(
            client,
            Arc::new(crate::session::MemorySessionStore::default()),
        )
    }
}

impl<C, S> WanAndroidRepository<C, S> {
    pub fn with_session(client: Arc<C>, session: Arc<S>) -> Self {
        Self { client, session }
    }

    pub(crate) fn get(&self, path: impl Into<String>) -> Result<HttpResponse, RepositoryError>
    where
        C: HttpClient,
        S: SessionStore,
    {
        self.client
            .execute(&HttpRequest::get(
                path,
                cookie_headers(&self.session.load_cookies()),
            ))
            .map_err(RepositoryError::Transport)
    }

    pub(crate) fn post_form(
        &self,
        path: impl Into<String>,
        form: BTreeMap<String, String>,
    ) -> Result<HttpResponse, RepositoryError>
    where
        C: HttpClient,
        S: SessionStore,
    {
        self.client
            .execute(&HttpRequest::post_form(
                path,
                cookie_headers(&self.session.load_cookies()),
                form,
            ))
            .map_err(RepositoryError::Transport)
    }

    pub(crate) fn decode<T: serde::de::DeserializeOwned>(
        &self,
        body: &str,
    ) -> Result<T, RepositoryError> {
        let envelope: ApiEnvelope<serde_json::Value> = serde_json::from_str(body)
            .map_err(|error| RepositoryError::Decode(error.to_string()))?;
        if envelope.error_code != 0 {
            return Err(RepositoryError::Api {
                code: envelope.error_code,
                message: envelope.error_msg,
            });
        }
        serde_json::from_value(envelope.data)
            .map_err(|error| RepositoryError::Decode(error.to_string()))
    }
}

impl<C: HttpClient, S: SessionStore> HomeRepository for WanAndroidRepository<C, S> {
    fn get_banners(&self) -> Result<Vec<Banner>, RepositoryError> {
        let response = self.get("/banner/json")?;
        let banners: Vec<ApiBanner> = self.decode(&response.body)?;
        Ok(banners.into_iter().map(Into::into).collect())
    }

    fn get_articles(&self, page: u32) -> Result<ArticlePage, RepositoryError> {
        let response = self.get(format!("/article/list/{page}/json"))?;
        let page: ApiPage<ApiArticle> = self.decode(&response.body)?;
        Ok(page.into())
    }
}

impl<C: HttpClient, S: SessionStore> AuthRepository for WanAndroidRepository<C, S> {
    fn login(&self, username: &str, password: &str) -> Result<AuthenticatedUser, RepositoryError> {
        let form = BTreeMap::from([
            ("username".into(), username.into()),
            ("password".into(), password.into()),
        ]);
        let response = self.post_form("/user/login", form)?;
        let user: ApiLogin = self.decode(&response.body)?;
        self.session
            .save_cookies(extract_cookies(&response.headers));
        let username = user.username.clone();
        self.session.save_user(AuthenticatedUser {
            id: user.id,
            username: username.clone(),
        });
        Ok(AuthenticatedUser {
            id: user.id,
            username,
        })
    }

    fn logout(&self) -> Result<(), RepositoryError> {
        let response = self.get("/user/logout/json")?;
        let _: serde_json::Value = self.decode(&response.body)?;
        self.session.clear();
        Ok(())
    }

    fn restore_session(&self) -> Option<AuthenticatedUser> {
        self.session.load_user()
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiLogin {
    id: u64,
    username: String,
}

fn extract_cookies(headers: &BTreeMap<String, String>) -> Vec<String> {
    headers
        .get("Set-Cookie")
        .map(|value| {
            value
                .split('\n')
                .filter_map(|cookie| cookie.split(';').next())
                .filter(|cookie| !cookie.trim().is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

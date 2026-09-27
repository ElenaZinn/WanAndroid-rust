use crate::domain::{Article, Banner};
use crate::interactor::auth::{AuthAction, AuthInteractor, AuthState};
use crate::interactor::home::{HomeAction, HomeInteractor, HomeState};
use crate::repository::{AuthRepository, HomeRepository};
use crate::state::LoadState;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreSnapshot {
    pub home: HomeSnapshot,
    pub auth: AuthSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HomeSnapshot {
    pub banners: Vec<Banner>,
    pub articles: Vec<Article>,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub can_load_more: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthSnapshot {
    pub username: Option<String>,
    pub is_loading: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CoreAction {
    HomeLoad,
    HomeRefresh,
    HomeLoadNextPage,
    HomeToggleCollect { article_id: u64 },
    AuthRestore,
    AuthLogin { username: String, password: String },
    AuthLogout,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingError {
    InvalidAction(String),
    Serialization(String),
}

impl std::fmt::Display for BindingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAction(message) => write!(formatter, "invalid action: {message}"),
            Self::Serialization(message) => write!(formatter, "serialization error: {message}"),
        }
    }
}

impl std::error::Error for BindingError {}

/// Opaque core ownership for a generated JNI/UniFFI/C-ABI adapter.
/// Adapters exchange JSON only, so Android cannot reach repositories or state-machine details.
pub struct CoreHandle<H, A> {
    home: Mutex<HomeInteractor<H>>,
    auth: Mutex<AuthInteractor<A>>,
}

impl<H: HomeRepository, A: AuthRepository> CoreHandle<H, A> {
    pub fn new(home_repository: H, auth_repository: A) -> Self {
        Self {
            home: Mutex::new(HomeInteractor::new(home_repository)),
            auth: Mutex::new(AuthInteractor::new(auth_repository)),
        }
    }

    pub fn snapshot(&self) -> CoreSnapshot {
        let home = self.home.lock().expect("core handle mutex poisoned");
        let auth = self.auth.lock().expect("core handle mutex poisoned");
        CoreSnapshot {
            home: HomeSnapshot::from(&home.state),
            auth: AuthSnapshot::from(&auth.state),
        }
    }

    pub fn snapshot_json(&self) -> Result<String, BindingError> {
        serde_json::to_string(&self.snapshot())
            .map_err(|error| BindingError::Serialization(error.to_string()))
    }

    pub fn dispatch_json(&self, action_json: &str) -> Result<String, BindingError> {
        let action: CoreAction = serde_json::from_str(action_json)
            .map_err(|error| BindingError::InvalidAction(error.to_string()))?;
        self.dispatch(action)?;
        self.snapshot_json()
    }

    pub fn dispatch(&self, action: CoreAction) -> Result<(), BindingError> {
        match action {
            CoreAction::HomeLoad => self.run_home(HomeAction::Load),
            CoreAction::HomeRefresh => self.run_home(HomeAction::Refresh),
            CoreAction::HomeLoadNextPage => self.run_home(HomeAction::LoadNextPage),
            CoreAction::HomeToggleCollect { article_id } => {
                self.run_home(HomeAction::ToggleCollect { article_id })
            }
            CoreAction::AuthRestore => self.run_auth(AuthAction::Restore),
            CoreAction::AuthLogin { username, password } => {
                self.run_auth(AuthAction::Login { username, password })
            }
            CoreAction::AuthLogout => self.run_auth(AuthAction::Logout),
        }
    }

    fn run_home(&self, action: HomeAction) -> Result<(), BindingError> {
        let mut home = self.home.lock().expect("core handle mutex poisoned");
        let effects = home.dispatch(action).effects;
        for effect in effects {
            let result = home.run(effect);
            home.dispatch(result);
        }
        Ok(())
    }

    fn run_auth(&self, action: AuthAction) -> Result<(), BindingError> {
        let mut auth = self.auth.lock().expect("core handle mutex poisoned");
        let effects = auth.dispatch(action);
        for effect in effects {
            let result = auth.run(effect);
            auth.dispatch(result);
        }
        Ok(())
    }
}

impl From<&HomeState> for HomeSnapshot {
    fn from(state: &HomeState) -> Self {
        let (banners, banners_loading, banners_error) = state_content(&state.banners);
        let (articles, articles_loading, articles_error) = state_content(&state.articles);
        Self {
            banners,
            articles,
            is_loading: banners_loading || articles_loading,
            error_message: banners_error.or(articles_error),
            can_load_more: state.has_more,
        }
    }
}

impl From<&AuthState> for AuthSnapshot {
    fn from(state: &AuthState) -> Self {
        Self {
            username: state.user.as_ref().map(|user| user.username.clone()),
            is_loading: state.is_loading,
            error_message: state.error_message.clone(),
        }
    }
}

fn state_content<T: Clone + Default>(state: &LoadState<T>) -> (T, bool, Option<String>) {
    match state {
        LoadState::Idle => (T::default(), false, None),
        LoadState::Loading => (T::default(), true, None),
        LoadState::Refreshing(value) => (value.clone(), true, None),
        LoadState::Ready(value) => (value.clone(), false, None),
        LoadState::Error(error) => (T::default(), false, Some(error.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::{BindingError, CoreHandle};
    use crate::domain::{ArticlePage, Banner};
    use crate::repository::{AuthenticatedUser, AuthRepository, HomeRepository, RepositoryError};

    #[derive(Clone)]
    struct FakeRepository;

    impl HomeRepository for FakeRepository {
        fn get_banners(&self) -> Result<Vec<Banner>, RepositoryError> { Ok(Vec::new()) }
        fn get_articles(&self, _page: u32) -> Result<ArticlePage, RepositoryError> {
            Err(RepositoryError::Transport("offline".into()))
        }
    }

    impl AuthRepository for FakeRepository {
        fn login(&self, _username: &str, _password: &str) -> Result<AuthenticatedUser, RepositoryError> {
            Ok(AuthenticatedUser { id: 1, username: "Elena".into() })
        }
        fn logout(&self) -> Result<(), RepositoryError> { Ok(()) }
        fn restore_session(&self) -> Option<AuthenticatedUser> { None }
    }

    #[test]
    fn handle_serializes_snapshot_after_dispatching_into_the_interactors() {
        let handle = CoreHandle::new(FakeRepository, FakeRepository);
        let snapshot = handle
            .dispatch_json(r#"{"type":"home_load"}"#)
            .expect("action should dispatch");
        assert!(snapshot.contains("\"error_message\":\"transport error: offline\""));
        assert!(snapshot.contains("\"auth\""));
    }

    #[test]
    fn handle_rejects_malformed_actions_without_panicking() {
        let handle = CoreHandle::new(FakeRepository, FakeRepository);
        assert!(matches!(
            handle.dispatch_json("not-json"),
            Err(BindingError::InvalidAction(_))
        ));
    }
}

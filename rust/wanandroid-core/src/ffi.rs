use crate::collection::CollectionRepository;
use crate::domain::{Article, Banner};
use crate::interactor::auth::{AuthAction, AuthInteractor, AuthState};
use crate::interactor::collection::{CollectionAction, CollectionInteractor, CollectionState};
use crate::interactor::home::{HomeAction, HomeInteractor, HomeState};
use crate::interactor::project::{ProjectAction, ProjectInteractor, ProjectState};
use crate::interactor::search::{SearchAction, SearchInteractor, SearchState};
use crate::project::ProjectRepository;
use crate::repository::{AuthRepository, HomeRepository, WanAndroidRepository};
use crate::reqwest_client::ReqwestHttpClient;
use crate::search::{HotKey, SearchRepository};
use crate::session::MemorySessionStore;
use crate::state::LoadState;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreSnapshot {
    pub home: HomeSnapshot,
    pub auth: AuthSnapshot,
    pub search: SearchSnapshot,
    pub project: ProjectSnapshot,
    pub collection: CollectionSnapshot,
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
pub struct SearchSnapshot {
    pub hot_keys: Vec<HotKey>,
    pub articles: Vec<Article>,
    pub keyword: String,
    pub author: Option<String>,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub can_load_more: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectSnapshot {
    pub categories: Vec<crate::project::ProjectCategory>,
    pub articles: Vec<crate::project::ProjectArticle>,
    pub selected_category: Option<u64>,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub can_load_more: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionSnapshot {
    pub articles: Vec<Article>,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub can_load_more: bool,
    pub pending_change: bool,
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
    SearchLoadHotKeys,
    SearchSubmit { keyword: String },
    SearchAuthor { author: String },
    SearchLoadNextPage,
    ProjectLoadCategories,
    ProjectSelectCategory { id: u64 },
    ProjectLoadNextPage,
    CollectionLoad,
    CollectionRefresh,
    CollectionLoadNextPage,
    CollectionCollect { article_id: u64 },
    CollectionUncollect { origin_id: u64 },
}

/// Generic business runtime used by host tests and the generated UniFFI facade.
pub struct CoreRuntime<R> {
    home: Mutex<HomeInteractor<R>>,
    auth: Mutex<AuthInteractor<R>>,
    search: Mutex<SearchInteractor<R>>,
    project: Mutex<ProjectInteractor<R>>,
    collection: Mutex<CollectionInteractor<R>>,
}

impl<R> CoreRuntime<R>
where
    R: Clone
        + HomeRepository
        + AuthRepository
        + SearchRepository
        + ProjectRepository
        + CollectionRepository,
{
    pub fn new(repository: R) -> Self {
        Self {
            home: Mutex::new(HomeInteractor::new(repository.clone())),
            auth: Mutex::new(AuthInteractor::new(repository.clone())),
            search: Mutex::new(SearchInteractor::new(repository.clone())),
            project: Mutex::new(ProjectInteractor::new(repository.clone())),
            collection: Mutex::new(CollectionInteractor::new(repository)),
        }
    }

    pub fn snapshot(&self) -> CoreSnapshot {
        let home = self.home.lock().expect("core runtime mutex poisoned");
        let auth = self.auth.lock().expect("core runtime mutex poisoned");
        let search = self.search.lock().expect("core runtime mutex poisoned");
        let project = self.project.lock().expect("core runtime mutex poisoned");
        let collection = self.collection.lock().expect("core runtime mutex poisoned");
        CoreSnapshot {
            home: HomeSnapshot::from(&home.state),
            auth: AuthSnapshot::from(&auth.state),
            search: SearchSnapshot::from(&search.state),
            project: ProjectSnapshot::from(&project.state),
            collection: CollectionSnapshot::from(&collection.state),
        }
    }

    pub fn dispatch(&self, action: CoreAction) {
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
            CoreAction::SearchLoadHotKeys => self.run_search(SearchAction::LoadHotKeys),
            CoreAction::SearchSubmit { keyword } => {
                self.run_search(SearchAction::Submit { keyword })
            }
            CoreAction::SearchAuthor { author } => {
                self.run_search(SearchAction::SearchAuthor { author })
            }
            CoreAction::SearchLoadNextPage => self.run_search(SearchAction::LoadNextPage),
            CoreAction::ProjectLoadCategories => self.run_project(ProjectAction::LoadCategories),
            CoreAction::ProjectSelectCategory { id } => {
                self.run_project(ProjectAction::SelectCategory { id })
            }
            CoreAction::ProjectLoadNextPage => self.run_project(ProjectAction::LoadNextPage),
            CoreAction::CollectionLoad => self.run_collection(CollectionAction::Load),
            CoreAction::CollectionRefresh => self.run_collection(CollectionAction::Refresh),
            CoreAction::CollectionLoadNextPage => {
                self.run_collection(CollectionAction::LoadNextPage)
            }
            CoreAction::CollectionCollect { article_id } => {
                self.run_collection(CollectionAction::Collect { article_id })
            }
            CoreAction::CollectionUncollect { origin_id } => {
                self.run_collection(CollectionAction::Uncollect { origin_id })
            }
        }
    }

    fn run_home(&self, action: HomeAction) {
        let mut feature = self.home.lock().expect("core runtime mutex poisoned");
        let mut pending = vec![action];
        while let Some(next) = pending.pop() {
            for effect in feature.dispatch(next).effects {
                // Running an effect yields the action that reports its outcome. Queue it so any
                // effects that action triggers also run in the same pass.
                pending.push(feature.run(effect));
            }
        }
    }

    fn run_auth(&self, action: AuthAction) {
        let mut feature = self.auth.lock().expect("core runtime mutex poisoned");
        let mut pending = vec![action];
        while let Some(next) = pending.pop() {
            for effect in feature.dispatch(next) {
                pending.push(feature.run(effect));
            }
        }
    }

    fn run_search(&self, action: SearchAction) {
        let mut feature = self.search.lock().expect("core runtime mutex poisoned");
        let mut pending = vec![action];
        while let Some(next) = pending.pop() {
            for effect in feature.dispatch(next).effects {
                pending.push(feature.run(effect));
            }
        }
    }

    fn run_project(&self, action: ProjectAction) {
        let mut feature = self.project.lock().expect("core runtime mutex poisoned");
        let mut pending = vec![action];
        while let Some(next) = pending.pop() {
            for effect in feature.dispatch(next).effects {
                pending.push(feature.run(effect));
            }
        }
    }

    fn run_collection(&self, action: CollectionAction) {
        let mut feature = self.collection.lock().expect("core runtime mutex poisoned");
        let mut pending = vec![action];
        while let Some(next) = pending.pop() {
            for effect in feature.dispatch(next).effects {
                pending.push(feature.run(effect));
            }
        }
    }
}

type NativeRepository = WanAndroidRepository<ReqwestHttpClient, MemorySessionStore>;

/// UniFFI object consumed by generated Kotlin today and generated Swift in a later iOS app.
/// It intentionally exposes JSON snapshots/actions so every platform follows the same business
/// contract while Rust retains ownership of transport, session and state transitions.
pub struct CoreHandle {
    runtime: CoreRuntime<NativeRepository>,
}

impl CoreHandle {
    /// `base_url` overrides the production endpoint. Hosts pass an explicit value to point
    /// the shared core at a staging deployment or a local demo proxy; `None` uses production.
    pub fn new(timeout_ms: u64, base_url: Option<String>) -> Self {
        let timeout = Duration::from_millis(timeout_ms.max(1));
        let client = match base_url {
            Some(url) => ReqwestHttpClient::new(url, timeout),
            None => ReqwestHttpClient::wanandroid(timeout),
        }
        .expect("reqwest client configuration must be valid");
        let repository = WanAndroidRepository::with_session(
            Arc::new(client),
            Arc::new(MemorySessionStore::default()),
        );
        Self {
            runtime: CoreRuntime::new(repository),
        }
    }

    pub fn snapshot_json(&self) -> String {
        serde_json::to_string(&self.runtime.snapshot()).expect("core snapshot must serialize")
    }

    pub fn dispatch_json(&self, action_json: String) -> String {
        match serde_json::from_str::<CoreAction>(&action_json) {
            Ok(action) => self.runtime.dispatch(action),
            Err(error) => return self.error_snapshot(format!("invalid action: {error}")),
        }
        self.snapshot_json()
    }

    fn error_snapshot(&self, message: String) -> String {
        let mut snapshot = self.runtime.snapshot();
        snapshot.home.error_message = Some(message);
        serde_json::to_string(&snapshot).expect("core error snapshot must serialize")
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
            error_message: state
                .last_error
                .clone()
                .or(banners_error)
                .or(articles_error),
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

impl From<&SearchState> for SearchSnapshot {
    fn from(state: &SearchState) -> Self {
        let (hot_keys, hot_loading, hot_error) = state_content(&state.hot_keys);
        let (articles, article_loading, article_error) = state_content(&state.articles);
        Self {
            hot_keys,
            articles,
            keyword: state.keyword.clone(),
            author: state.author.clone(),
            is_loading: hot_loading || article_loading,
            error_message: hot_error.or(article_error),
            can_load_more: state.has_more,
        }
    }
}

impl From<&ProjectState> for ProjectSnapshot {
    fn from(state: &ProjectState) -> Self {
        let (categories, category_loading, category_error) = state_content(&state.categories);
        let (articles, article_loading, article_error) = state_content(&state.articles);
        Self {
            categories,
            articles,
            selected_category: state.selected_category,
            is_loading: category_loading || article_loading,
            error_message: category_error.or(article_error),
            can_load_more: state.has_more,
        }
    }
}

impl From<&CollectionState> for CollectionSnapshot {
    fn from(state: &CollectionState) -> Self {
        let (articles, is_loading, error_message) = state_content(&state.articles);
        Self {
            articles,
            is_loading,
            error_message,
            can_load_more: state.has_more,
            pending_change: state.pending_change.is_some(),
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

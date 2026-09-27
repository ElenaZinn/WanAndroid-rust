use crate::domain::{Article, Banner};
use crate::interactor::home::HomeState;
use crate::state::LoadState;
use serde::{Deserialize, Serialize};

/// JSON-friendly contract for the Android native binding.
/// The binding owns lifecycle/thread delivery; Rust owns the feature state semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreSnapshot {
    pub home: HomeSnapshot,
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

impl From<&HomeState> for HomeSnapshot {
    fn from(state: &HomeState) -> Self {
        let (banners, banner_loading, banner_error) = content(&state.banners);
        let (articles, article_loading, article_error) = content(&state.articles);
        Self {
            banners,
            articles,
            is_loading: banner_loading || article_loading,
            error_message: banner_error.or(article_error),
            can_load_more: state.has_more,
        }
    }
}

fn content<T: Clone>(state: &LoadState<T>) -> (T, bool, Option<String>)
where
    T: Default,
{
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
    use super::{CoreAction, HomeSnapshot};

    #[test]
    fn core_actions_use_stable_tagged_json() {
        let value = serde_json::to_string(&CoreAction::AuthLogin {
            username: "Elena".into(),
            password: "secret".into(),
        })
        .expect("action serializes");

        assert_eq!(
            value,
            r#"{"type":"auth_login","username":"Elena","password":"secret"}"#
        );
    }

    #[test]
    fn snapshot_is_deserializable_by_a_platform_binding() {
        let snapshot: HomeSnapshot = serde_json::from_str(
            r#"{"banners":[],"articles":[],"is_loading":false,"error_message":null,"can_load_more":true}"#,
        )
        .expect("snapshot deserializes");
        assert!(snapshot.can_load_more);
    }
}

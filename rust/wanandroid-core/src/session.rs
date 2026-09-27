use crate::repository::AuthenticatedUser;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub trait SessionStore: Send + Sync {
    fn load_cookies(&self) -> Vec<String>;
    fn save_cookies(&self, cookies: Vec<String>);
    fn load_user(&self) -> Option<AuthenticatedUser>;
    fn save_user(&self, user: AuthenticatedUser);
    fn clear(&self);
}

#[derive(Clone, Default)]
pub struct MemorySessionStore {
    state: Arc<Mutex<SessionState>>,
}

#[derive(Default)]
struct SessionState {
    cookies: Vec<String>,
    user: Option<AuthenticatedUser>,
}

impl SessionStore for MemorySessionStore {
    fn load_cookies(&self) -> Vec<String> {
        self.state.lock().expect("session mutex poisoned").cookies.clone()
    }

    fn save_cookies(&self, cookies: Vec<String>) {
        self.state.lock().expect("session mutex poisoned").cookies = cookies;
    }

    fn load_user(&self) -> Option<AuthenticatedUser> {
        self.state.lock().expect("session mutex poisoned").user.clone()
    }

    fn save_user(&self, user: AuthenticatedUser) {
        self.state.lock().expect("session mutex poisoned").user = Some(user);
    }

    fn clear(&self) {
        let mut state = self.state.lock().expect("session mutex poisoned");
        state.cookies.clear();
        state.user = None;
    }
}

pub fn cookie_headers(cookies: &[String]) -> BTreeMap<String, String> {
    let mut headers = BTreeMap::new();
    if !cookies.is_empty() {
        headers.insert("Cookie".into(), cookies.join("; "));
    }
    headers
}

#[cfg(test)]
mod tests {
    use super::{cookie_headers, MemorySessionStore, SessionStore};
    use crate::repository::AuthenticatedUser;

    #[test]
    fn session_store_round_trip_and_clear() {
        let store = MemorySessionStore::default();
        store.save_cookies(vec!["loginUserName=Elena".into()]);
        store.save_user(AuthenticatedUser {
            id: 7,
            username: "Elena".into(),
        });
        assert_eq!(store.load_cookies(), vec!["loginUserName=Elena"]);
        assert_eq!(store.load_user().expect("user").username, "Elena");
        assert_eq!(cookie_headers(&store.load_cookies())["Cookie"], "loginUserName=Elena");
        store.clear();
        assert!(store.load_cookies().is_empty());
        assert!(store.load_user().is_none());
    }
}

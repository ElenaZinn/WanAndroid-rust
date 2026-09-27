use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub trait SessionStore: Send + Sync {
    fn load_cookies(&self) -> Vec<String>;
    fn save_cookies(&self, cookies: Vec<String>);
    fn clear(&self);
}

#[derive(Clone, Default)]
pub struct MemorySessionStore {
    cookies: Arc<Mutex<Vec<String>>>,
}

impl SessionStore for MemorySessionStore {
    fn load_cookies(&self) -> Vec<String> {
        self.cookies.lock().expect("session mutex poisoned").clone()
    }

    fn save_cookies(&self, cookies: Vec<String>) {
        *self.cookies.lock().expect("session mutex poisoned") = cookies;
    }

    fn clear(&self) {
        self.cookies.lock().expect("session mutex poisoned").clear();
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

    #[test]
    fn session_store_round_trip_and_clear() {
        let store = MemorySessionStore::default();
        store.save_cookies(vec!["loginUserName=Elena".into()]);
        assert_eq!(store.load_cookies(), vec!["loginUserName=Elena"]);
        assert_eq!(cookie_headers(&store.load_cookies())["Cookie"], "loginUserName=Elena");
        store.clear();
        assert!(store.load_cookies().is_empty());
    }
}

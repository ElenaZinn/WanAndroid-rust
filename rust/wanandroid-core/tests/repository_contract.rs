use std::sync::{Arc, Mutex};
use wanandroid_core::{
    HomeRepository, MemorySessionStore, RepositoryError, SessionStore, WanAndroidRepository,
};
use wanandroid_core::transport::{HttpClient, HttpRequest};

struct FixtureClient {
    body: String,
    requests: Arc<Mutex<Vec<HttpRequest>>>,
}

impl HttpClient for FixtureClient {
    fn get(&self, request: &HttpRequest) -> Result<String, String> {
        self.requests.lock().expect("request mutex poisoned").push(request.clone());
        Ok(self.body.clone())
    }
}

fn fixture(body: &str) -> (Arc<FixtureClient>, WanAndroidRepository<FixtureClient>) {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient {
        body: body.into(),
        requests: requests.clone(),
    });
    (client, WanAndroidRepository::new(client.clone()))
}

#[test]
fn decodes_wanandroid_page_envelope_into_domain_model() {
    let body = r#"{
      "errorCode": 0,
      "errorMsg": "",
      "data": {
        "curPage": 0,
        "datas": [{
          "id": 42,
          "title": "Rust on Android",
          "link": "https://example.test/rust",
          "author": "Elena",
          "niceDate": "today",
          "collect": false,
          "chapterName": "Rust",
          "tags": []
        }],
        "over": true,
        "pageCount": 1,
        "total": 1
      }
    }"#;
    let (client, repository) = fixture(body);

    let page = repository.get_articles(0).expect("fixture should decode");

    assert_eq!(page.page.datas[0].id, 42);
    assert_eq!(page.page.datas[0].title, "Rust on Android");
    assert!(page.page.over);
    let requests = client.requests.lock().expect("request mutex poisoned");
    assert_eq!(requests[0].path, "/article/list/0/json");
    assert!(requests[0].headers.is_empty());
}

#[test]
fn forwards_session_cookie_at_the_transport_boundary() {
    let body = r#"{"errorCode":0,"errorMsg":"","data":[]}"#;
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient {
        body: body.into(),
        requests: requests.clone(),
    });
    let session = Arc::new(MemorySessionStore::default());
    session.save_cookies(vec!["loginUserName=Elena".into(), "token=abc".into()]);
    let repository = WanAndroidRepository::with_session(client, session);

    repository.get_banners().expect("fixture should decode");

    let requests = requests.lock().expect("request mutex poisoned");
    assert_eq!(
        requests[0].headers.get("Cookie"),
        Some(&"loginUserName=Elena; token=abc".to_string())
    );
}

#[test]
fn maps_non_zero_api_code_to_repository_error() {
    let body = r#"{"errorCode":-1001,"errorMsg":"not logged in","data":null}"#;
    let (_, repository) = fixture(body);

    let error = repository.get_articles(0).expect_err("API error expected");

    assert_eq!(
        error,
        RepositoryError::Api {
            code: -1001,
            message: "not logged in".into()
        }
    );
}

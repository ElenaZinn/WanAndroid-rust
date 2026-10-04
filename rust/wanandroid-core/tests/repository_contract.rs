use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use wanandroid_core::transport::{HttpClient, HttpMethod, HttpRequest, HttpResponse};
use wanandroid_core::{
    AuthRepository, CoreAction, CoreRuntime, HomeRepository, MemorySessionStore, RepositoryError,
    SearchRepository, SessionStore, WanAndroidRepository,
};

struct FixtureClient {
    responses: Mutex<Vec<HttpResponse>>,
    requests: Arc<Mutex<Vec<HttpRequest>>>,
}

impl FixtureClient {
    fn once(body: &str, requests: Arc<Mutex<Vec<HttpRequest>>>) -> Self {
        Self::with_response(
            HttpResponse {
                body: body.into(),
                headers: BTreeMap::new(),
            },
            requests,
        )
    }

    fn with_response(response: HttpResponse, requests: Arc<Mutex<Vec<HttpRequest>>>) -> Self {
        Self {
            responses: Mutex::new(vec![response]),
            requests,
        }
    }
}

impl HttpClient for FixtureClient {
    fn execute(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        self.requests
            .lock()
            .expect("request mutex poisoned")
            .push(request.clone());
        self.responses
            .lock()
            .expect("response mutex poisoned")
            .pop()
            .ok_or_else(|| "missing fixture response".into())
    }
}

fn fixture(body: &str) -> (Arc<FixtureClient>, WanAndroidRepository<FixtureClient>) {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient::once(body, requests));
    (client.clone(), WanAndroidRepository::new(client))
}

fn page_body() -> &'static str {
    r#"{
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
    }"#
}

#[test]
fn decodes_wanandroid_page_envelope_into_domain_model() {
    let (client, repository) = fixture(page_body());
    let page = repository.get_articles(0).expect("fixture should decode");
    assert_eq!(page.page.datas[0].id, 42);
    assert_eq!(page.page.datas[0].title, "Rust on Android");
    assert!(page.page.over);
    let requests = client.requests.lock().expect("request mutex poisoned");
    assert_eq!(requests[0].path, "/article/list/0/json");
    assert_eq!(requests[0].method, HttpMethod::Get);
    assert!(requests[0].headers.is_empty());
}

#[test]
fn forwards_session_cookie_at_the_transport_boundary() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient::once(
        r#"{"errorCode":0,"errorMsg":"","data":[]}"#,
        requests.clone(),
    ));
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
fn login_posts_credentials_persists_user_and_session_cookie() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient::with_response(
        HttpResponse {
            body: r#"{"errorCode":0,"errorMsg":"","data":{"id":7,"username":"Elena"}}"#.into(),
            headers: BTreeMap::from([(
                "Set-Cookie".into(),
                "loginUserName=Elena; Path=/\ntoken=abc; Path=/".into(),
            )]),
        },
        requests.clone(),
    ));
    let session = Arc::new(MemorySessionStore::default());
    let repository = WanAndroidRepository::with_session(client, session.clone());
    let user = repository
        .login("Elena", "safe-password")
        .expect("login should decode");
    assert_eq!(user.username, "Elena");
    assert_eq!(session.load_user(), Some(user));
    assert_eq!(
        session.load_cookies(),
        vec!["loginUserName=Elena", "token=abc"]
    );
    let requests = requests.lock().expect("request mutex poisoned");
    assert_eq!(requests[0].method, HttpMethod::Post);
    assert_eq!(requests[0].path, "/user/login");
    assert_eq!(requests[0].form.get("username"), Some(&"Elena".to_string()));
    assert_eq!(
        requests[0].form.get("password"),
        Some(&"safe-password".to_string())
    );
}

#[test]
fn search_posts_keyword_form_and_maps_hot_keys() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient::with_response(
        HttpResponse {
            body: r#"{"errorCode":0,"errorMsg":"","data":[{"id":9,"name":"Rust","link":"/rust"}]}"#
                .into(),
            headers: BTreeMap::new(),
        },
        requests.clone(),
    ));
    let repository = WanAndroidRepository::new(client);
    let hot_keys = repository.hot_keys().expect("hot keys should decode");
    assert_eq!(hot_keys[0].name, "Rust");
    let requests = requests.lock().expect("request mutex poisoned");
    assert_eq!(requests[0].path, "/hotkey/json");
    assert_eq!(requests[0].method, HttpMethod::Get);
}

#[test]
fn search_posts_keyword_form() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient::with_response(
        HttpResponse {
            body: page_body().into(),
            headers: BTreeMap::new(),
        },
        requests.clone(),
    ));
    let repository = WanAndroidRepository::new(client);
    repository
        .search(0, "rust android")
        .expect("search should decode");
    let requests = requests.lock().expect("request mutex poisoned");
    assert_eq!(requests[0].path, "/article/query/0/json");
    assert_eq!(requests[0].method, HttpMethod::Post);
    assert_eq!(requests[0].form.get("k"), Some(&"rust android".to_string()));
}

#[test]
fn search_author_encodes_query_value() {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient::with_response(
        HttpResponse {
            body: page_body().into(),
            headers: BTreeMap::new(),
        },
        requests.clone(),
    ));
    let repository = WanAndroidRepository::new(client);
    repository
        .search_author(0, "Elena Z+inn")
        .expect("author search should decode");
    let requests = requests.lock().expect("request mutex poisoned");
    assert_eq!(
        requests[0].path,
        "/article/list/0/json?author=Elena%20Z%2Binn"
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
            message: "not logged in".into(),
        }
    );
}

#[test]
fn maps_error_response_without_data_field_to_api_error() {
    // The server omits `data` entirely for failures such as a missing login, so the API error
    // must be detected before any attempt to deserialize a payload.
    let body = r#"{"errorCode":-1001,"errorMsg":"请先登录！"}"#;
    let (_, repository) = fixture(body);
    let error = repository.get_articles(0).expect_err("API error expected");
    assert_eq!(
        error,
        RepositoryError::Api {
            code: -1001,
            message: "请先登录！".into(),
        }
    );
}

#[test]
fn project_contract_uses_category_query_parameter() {
    use wanandroid_core::ProjectRepository;
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient::with_response(
        HttpResponse {
            body: r#"{"errorCode":0,"errorMsg":"","data":{"curPage":0,"datas":[{"id":3,"title":"Compose","link":"https://example.test/project","author":"Elena","niceDate":"today","collect":false,"chapterName":"Project","envelopePic":"","thumbnail":""}],"over":true,"pageCount":1,"total":1}}"#.into(),
            headers: BTreeMap::new(),
        },
        requests.clone(),
    ));
    let repository = WanAndroidRepository::new(client);
    let page = repository.articles(0, 77).expect("project should decode");
    assert_eq!(page.datas[0].id, 3);
    let requests = requests.lock().expect("request mutex poisoned");
    assert_eq!(requests[0].path, "/project/list/0/json?cid=77");
    assert_eq!(requests[0].method, HttpMethod::Get);
}

#[test]
fn core_runtime_runs_effects_produced_by_earlier_effects() {
    // Loading categories auto-selects the first category, so the runtime has to keep driving: the
    // result of `GetCategories` produces the article request. Fixture responses are consumed LIFO.
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient {
        responses: Mutex::new(vec![
            HttpResponse {
                body: r#"{"errorCode":0,"errorMsg":"","data":{"curPage":0,"datas":[],"over":true,"pageCount":0,"total":0}}"#.into(),
                headers: BTreeMap::new(),
            },
            HttpResponse {
                body: r#"{"errorCode":0,"errorMsg":"","data":[{"id":77,"name":"Compose","order":1,"visible":1}]}"#.into(),
                headers: BTreeMap::new(),
            },
        ]),
        requests: requests.clone(),
    });

    let runtime = CoreRuntime::new(WanAndroidRepository::new(client));
    runtime.dispatch(CoreAction::ProjectLoadCategories);

    let paths: Vec<String> = requests
        .lock()
        .expect("request mutex poisoned")
        .iter()
        .map(|request| request.path.clone())
        .collect();
    assert!(
        paths
            .iter()
            .any(|path| path.starts_with("/project/tree/json")),
        "categories request missing: {paths:?}"
    );
    assert!(
        paths
            .iter()
            .any(|path| path.starts_with("/project/list/0/json")),
        "auto-selected category articles request missing: {paths:?}"
    );
}

#[test]
fn collection_contract_routes_collect_and_uncollect_as_post() {
    use wanandroid_core::CollectionRepository;
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient {
        responses: Mutex::new(vec![
            HttpResponse {
                body: r#"{"errorCode":0,"errorMsg":"","data":null}"#.into(),
                headers: BTreeMap::new(),
            },
            HttpResponse {
                body: r#"{"errorCode":0,"errorMsg":"","data":null}"#.into(),
                headers: BTreeMap::new(),
            },
        ]),
        requests: requests.clone(),
    });
    let repository = WanAndroidRepository::new(client);
    repository.collect(42).expect("collect should decode");
    repository.uncollect(42).expect("uncollect should decode");
    let requests = requests.lock().expect("request mutex poisoned");
    assert_eq!(requests[0].path, "/lg/collect/42/json");
    assert_eq!(requests[0].method, HttpMethod::Post);
    assert_eq!(requests[1].path, "/lg/uncollect_originId/42/json");
    assert_eq!(requests[1].method, HttpMethod::Post);
}

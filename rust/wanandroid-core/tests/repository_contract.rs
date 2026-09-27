use std::sync::Arc;
use wanandroid_core::{HomeRepository, RepositoryError, WanAndroidRepository};
use wanandroid_core::transport::HttpClient;

struct FixtureClient {
    body: String,
}

impl HttpClient for FixtureClient {
    fn get(&self, _path: &str) -> Result<String, String> {
        Ok(self.body.clone())
    }
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
    let repository = WanAndroidRepository::new(Arc::new(FixtureClient { body: body.into() }));

    let page = repository.get_articles(0).expect("fixture should decode");

    assert_eq!(page.page.datas[0].id, 42);
    assert_eq!(page.page.datas[0].title, "Rust on Android");
    assert!(page.page.over);
}

#[test]
fn maps_non_zero_api_code_to_repository_error() {
    let body = r#"{"errorCode":-1001,"errorMsg":"not logged in","data":null}"#;
    let repository = WanAndroidRepository::new(Arc::new(FixtureClient { body: body.into() }));

    let error = repository.get_articles(0).expect_err("API error expected");

    assert_eq!(
        error,
        RepositoryError::Api {
            code: -1001,
            message: "not logged in".into()
        }
    );
}

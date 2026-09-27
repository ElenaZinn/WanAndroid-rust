
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
fn collection_contract_routes_list_collect_and_uncollect() {
    use wanandroid_core::CollectionRepository;
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient::with_response(
        HttpResponse {
            body: r#"{"errorCode":0,"errorMsg":"","data":null}"#.into(),
            headers: BTreeMap::new(),
        },
        requests.clone(),
    ));
    let repository = WanAndroidRepository::new(client);
    repository.collect(42).expect("collect should decode");
    let requests = requests.lock().expect("request mutex poisoned");
    assert_eq!(requests[0].path, "/lg/collect/42/json");
    assert_eq!(requests[0].method, HttpMethod::Post);
    assert!(requests[0].form.is_empty());
}

#[test]
fn collection_uncollect_uses_post_contract() {
    use wanandroid_core::CollectionRepository;
    let requests = Arc::new(Mutex::new(Vec::new()));
    let client = Arc::new(FixtureClient::with_response(
        HttpResponse {
            body: r#"{"errorCode":0,"errorMsg":"","data":null}"#.into(),
            headers: BTreeMap::new(),
        },
        requests.clone(),
    ));
    let repository = WanAndroidRepository::new(client);
    repository.uncollect(42).expect("uncollect should decode");
    let requests = requests.lock().expect("request mutex poisoned");
    assert_eq!(requests[0].path, "/lg/uncollect_originId/42/json");
    assert_eq!(requests[0].method, HttpMethod::Post);

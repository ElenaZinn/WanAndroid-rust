use wanandroid_core::{
    Article, ArticlePage, HotKey, LoadState, Page, RepositoryError, SearchAction, SearchEffect,
    SearchInteractor, SearchRepository,
};

struct FakeSearch;

impl SearchRepository for FakeSearch {
    fn hot_keys(&self) -> Result<Vec<HotKey>, RepositoryError> {
        Ok(vec![HotKey {
            id: 1,
            name: "Rust".into(),
            link: "/article/query/0/json?k=Rust".into(),
        }])
    }

    fn search(&self, page: u32, _keyword: &str) -> Result<ArticlePage, RepositoryError> {
        Ok(page(page, page >= 1))
    }

    fn search_author(&self, page: u32, _author: &str) -> Result<ArticlePage, RepositoryError> {
        Ok(page(page, true))
    }
}

fn page(number: u32, over: bool) -> ArticlePage {
    ArticlePage {
        page: Page {
            cur_page: number,
            datas: vec![Article {
                id: number as u64 + 1,
                title: format!("Result {number}"),
                link: "https://example.test/result".into(),
                author: "Elena".into(),
                nice_date: "today".into(),
                collect: false,
                chapter_name: "Rust".into(),
                tags: vec![],
            }],
            over,
            page_count: 2,
            total: 2,
        },
    }
}

#[test]
fn hot_keys_load_through_effect_and_update_state() {
    let mut interactor = SearchInteractor::new(FakeSearch);
    let effects = interactor.dispatch(SearchAction::LoadHotKeys);
    assert_eq!(effects.effects, vec![SearchEffect::GetHotKeys]);
    assert_eq!(interactor.state.hot_keys, LoadState::Loading);

    let action = interactor.run(effects.effects[0].clone());
    interactor.dispatch(action);
    assert_eq!(interactor.state.hot_keys, LoadState::Ready(vec![HotKey {
        id: 1,
        name: "Rust".into(),
        link: "/article/query/0/json?k=Rust".into(),
    }]));
}

#[test]
fn keyword_search_accumulates_pages_and_stops_after_last_page() {
    let mut interactor = SearchInteractor::new(FakeSearch);
    let first = interactor.dispatch(SearchAction::Submit {
        keyword: " rust ".into(),
    });
    assert_eq!(
        first.effects,
        vec![SearchEffect::Search {
            page: 0,
            keyword: "rust".into()
        }]
    );
    let action = interactor.run(first.effects[0].clone());
    interactor.dispatch(action);

    let next = interactor.dispatch(SearchAction::LoadNextPage);
    assert_eq!(
        next.effects,
        vec![SearchEffect::Search {
            page: 1,
            keyword: "rust".into()
        }]
    );
    let action = interactor.run(next.effects[0].clone());
    interactor.dispatch(action);

    assert_eq!(interactor.state.articles, LoadState::Ready(vec![
        Article {
            id: 1,
            title: "Result 0".into(),
            link: "https://example.test/result".into(),
            author: "Elena".into(),
            nice_date: "today".into(),
            collect: false,
            chapter_name: "Rust".into(),
            tags: vec![],
        },
        Article {
            id: 2,
            title: "Result 1".into(),
            link: "https://example.test/result".into(),
            author: "Elena".into(),
            nice_date: "today".into(),
            collect: false,
            chapter_name: "Rust".into(),
            tags: vec![],
        },
    ]));
    assert!(!interactor.state.has_more);
    assert!(interactor.dispatch(SearchAction::LoadNextPage).effects.is_empty());
}

#[test]
fn empty_keyword_is_a_renderable_error_without_network_effect() {
    let mut interactor = SearchInteractor::new(FakeSearch);
    let result = interactor.dispatch(SearchAction::Submit {
        keyword: "  ".into(),
    });
    assert!(result.effects.is_empty());
    assert_eq!(interactor.state.articles, LoadState::Error("search keyword is empty".into()));
}

#[test]
fn author_search_uses_author_effect_and_clears_keyword() {
    let mut interactor = SearchInteractor::new(FakeSearch);
    interactor.dispatch(SearchAction::Submit {
        keyword: "old".into(),
    });
    let result = interactor.dispatch(SearchAction::SearchAuthor {
        author: " Elena ".into(),
    });
    assert_eq!(
        result.effects,
        vec![SearchEffect::SearchAuthor {
            page: 0,
            author: "Elena".into()
        }]
    );
    assert_eq!(interactor.state.author.as_deref(), Some("Elena"));
    assert!(interactor.state.keyword.is_empty());
}

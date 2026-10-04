use std::sync::{Arc, Mutex};
use wanandroid_core::{
    Article, ArticlePage, Banner, CollectionPage, CollectionRepository, HomeAction, HomeEffect,
    HomeInteractor, HomeRepository, LoadState, Page, RepositoryError,
};

/// Records collect/uncollect calls so the toggle can be asserted.
#[derive(Clone, Default)]
struct FakeCollected {
    calls: Arc<Mutex<Vec<(u64, bool)>>>,
}

struct FakeRepository {
    collected: FakeCollected,
}

impl FakeRepository {
    fn new() -> Self {
        Self {
            collected: FakeCollected::default(),
        }
    }
}

impl CollectionRepository for FakeRepository {
    fn list(&self, page: u32) -> Result<CollectionPage, RepositoryError> {
        Ok(CollectionPage {
            cur_page: page,
            datas: Vec::new(),
            over: true,
            page_count: 1,
            total: 0,
        })
    }

    fn collect(&self, article_id: u64) -> Result<(), RepositoryError> {
        self.collected
            .calls
            .lock()
            .expect("call mutex poisoned")
            .push((article_id, true));
        Ok(())
    }

    fn uncollect(&self, origin_id: u64) -> Result<(), RepositoryError> {
        self.collected
            .calls
            .lock()
            .expect("call mutex poisoned")
            .push((origin_id, false));
        Ok(())
    }
}

impl HomeRepository for FakeRepository {
    fn get_banners(&self) -> Result<Vec<Banner>, RepositoryError> {
        Ok(vec![Banner {
            id: 1,
            title: "Banner".into(),
            image_path: "https://example.test/banner.png".into(),
            url: "https://example.test/article".into(),
        }])
    }

    fn get_articles(&self, page: u32) -> Result<ArticlePage, RepositoryError> {
        Ok(ArticlePage {
            page: Page {
                cur_page: page,
                datas: vec![Article {
                    id: page as u64 + 1,
                    title: format!("Article {page}"),
                    link: "https://example.test/article".into(),
                    author: "Elena".into(),
                    nice_date: "today".into(),
                    collect: false,
                    chapter_name: "Android".into(),
                    tags: vec![],
                }],
                over: page >= 1,
                page_count: 2,
                total: 2,
            },
        })
    }
}

/// Repository whose collection writes always fail, to exercise the error path.
struct FailingCollectRepository;

impl HomeRepository for FailingCollectRepository {
    fn get_banners(&self) -> Result<Vec<Banner>, RepositoryError> {
        Ok(Vec::new())
    }

    fn get_articles(&self, page: u32) -> Result<ArticlePage, RepositoryError> {
        Ok(ArticlePage {
            page: Page {
                cur_page: page,
                datas: vec![Article {
                    id: 1,
                    title: "Article".into(),
                    link: "https://example.test/article".into(),
                    author: "Elena".into(),
                    nice_date: "today".into(),
                    collect: false,
                    chapter_name: "Android".into(),
                    tags: vec![],
                }],
                over: true,
                page_count: 1,
                total: 1,
            },
        })
    }
}

impl CollectionRepository for FailingCollectRepository {
    fn list(&self, page: u32) -> Result<CollectionPage, RepositoryError> {
        Ok(CollectionPage {
            cur_page: page,
            datas: Vec::new(),
            over: true,
            page_count: 1,
            total: 0,
        })
    }

    fn collect(&self, _article_id: u64) -> Result<(), RepositoryError> {
        Err(RepositoryError::Api {
            code: -1001,
            message: "请先登录！".into(),
        })
    }

    fn uncollect(&self, _origin_id: u64) -> Result<(), RepositoryError> {
        Err(RepositoryError::Api {
            code: -1001,
            message: "请先登录！".into(),
        })
    }
}

#[test]
fn failed_collect_reports_an_error_and_keeps_the_list() {
    let mut interactor = HomeInteractor::new(FailingCollectRepository);
    let load = interactor.dispatch(HomeAction::Load);
    let articles = interactor.run(load.effects[1].clone());
    interactor.dispatch(articles);
    assert!(interactor.state.articles.value().is_some());

    let toggle = interactor.dispatch(HomeAction::ToggleCollect { article_id: 1 });
    let finished = interactor.run(toggle.effects[0].clone());
    interactor.dispatch(finished);

    assert_eq!(
        interactor.state.last_error.as_deref(),
        Some("api error -1001: 请先登录！")
    );
    assert!(
        interactor.state.articles.value().is_some(),
        "a failed collect must not discard the loaded list"
    );
}

#[test]
fn toggling_collect_calls_the_opposite_collection_endpoint() {
    let repository = FakeRepository::new();
    let calls = repository.collected.calls.clone();
    let mut interactor = HomeInteractor::new(repository);

    // Load the first page so the article lands in state with `collect = false`.
    let load = interactor.dispatch(HomeAction::Load);
    let articles = interactor.run(load.effects[1].clone());
    interactor.dispatch(articles);
    assert_eq!(
        interactor
            .state
            .articles
            .value()
            .map(|items| items[0].collect),
        Some(false)
    );

    // Starring an uncollected article must call collect, then flip the state flag.
    let toggle = interactor.dispatch(HomeAction::ToggleCollect { article_id: 1 });
    assert_eq!(
        toggle.effects,
        vec![HomeEffect::SetCollected {
            article_id: 1,
            collected: true,
        }]
    );
    let finished = interactor.run(toggle.effects[0].clone());
    interactor.dispatch(finished);
    assert_eq!(
        interactor
            .state
            .articles
            .value()
            .map(|items| items[0].collect),
        Some(true)
    );
    assert_eq!(*calls.lock().expect("call mutex poisoned"), vec![(1, true)]);

    // Toggling again must uncollect instead.
    let toggle = interactor.dispatch(HomeAction::ToggleCollect { article_id: 1 });
    assert_eq!(
        toggle.effects,
        vec![HomeEffect::SetCollected {
            article_id: 1,
            collected: false,
        }]
    );
    let finished = interactor.run(toggle.effects[0].clone());
    interactor.dispatch(finished);
    assert_eq!(
        interactor
            .state
            .articles
            .value()
            .map(|items| items[0].collect),
        Some(false)
    );
    assert_eq!(
        *calls.lock().expect("call mutex poisoned"),
        vec![(1, true), (1, false)]
    );
}

#[test]
fn load_emits_banner_and_first_page_effects() {
    let mut interactor = HomeInteractor::new(FakeRepository::new());

    let result = interactor.dispatch(HomeAction::Load);

    assert_eq!(interactor.state.banners, LoadState::Loading);
    assert_eq!(interactor.state.articles, LoadState::Loading);
    assert_eq!(
        result.effects,
        vec![HomeEffect::GetBanners, HomeEffect::GetArticles { page: 0 }]
    );
}

#[test]
fn paging_accumulates_articles_and_stops_at_last_page() {
    let mut interactor = HomeInteractor::new(FakeRepository::new());
    let first = interactor.dispatch(HomeAction::Load);
    for effect in first.effects {
        let action = interactor.run(effect);
        interactor.dispatch(action);
    }

    let second = interactor.dispatch(HomeAction::LoadNextPage);
    assert_eq!(second.effects, vec![HomeEffect::GetArticles { page: 1 }]);
    for effect in second.effects {
        let action = interactor.run(effect);
        interactor.dispatch(action);
    }

    assert_eq!(
        interactor.state.articles,
        LoadState::Ready(vec![article(1, "Article 0"), article(2, "Article 1"),])
    );
    assert!(!interactor.state.has_more);
    assert!(interactor
        .dispatch(HomeAction::LoadNextPage)
        .effects
        .is_empty());
}

fn article(id: u64, title: &str) -> Article {
    Article {
        id,
        title: title.into(),
        link: "https://example.test/article".into(),
        author: "Elena".into(),
        nice_date: "today".into(),
        collect: false,
        chapter_name: "Android".into(),
        tags: vec![],
    }
}

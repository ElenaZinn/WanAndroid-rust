use wanandroid_core::{
    Article, ArticlePage, Banner, HomeAction, HomeEffect, HomeInteractor, HomeRepository, LoadState,
    Page, RepositoryError,
};

struct FakeRepository;

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

#[test]
fn load_emits_banner_and_first_page_effects() {
    let mut interactor = HomeInteractor::new(FakeRepository);

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
    let mut interactor = HomeInteractor::new(FakeRepository);
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

    assert_eq!(interactor.state.articles, LoadState::Ready(vec![
        article(1, "Article 0"),
        article(2, "Article 1"),
    ]));
    assert!(!interactor.state.has_more);
    assert!(interactor.dispatch(HomeAction::LoadNextPage).effects.is_empty());
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

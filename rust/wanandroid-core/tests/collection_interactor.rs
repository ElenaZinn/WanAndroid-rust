use wanandroid_core::{
    Article, CollectionAction, CollectionEffect, CollectionInteractor, CollectionPage,
    CollectionRepository, LoadState, RepositoryError,
};

struct FakeCollection;

impl CollectionRepository for FakeCollection {
    fn list(&self, page: u32) -> Result<CollectionPage, RepositoryError> {
        Ok(CollectionPage {
            cur_page: page,
            datas: vec![Article {
                id: page as u64 + 1,
                title: format!("Favorite {page}"),
                link: "https://example.test/favorite".into(),
                author: "Elena".into(),
                nice_date: "today".into(),
                collect: true,
                chapter_name: "Rust".into(),
                tags: vec![],
            }],
            over: page > 0,
            page_count: 2,
            total: 2,
        })
    }

    fn collect(&self, _article_id: u64) -> Result<(), RepositoryError> {
        Ok(())
    }
    fn uncollect(&self, _origin_id: u64) -> Result<(), RepositoryError> {
        Ok(())
    }
}

#[test]
fn collection_paging_and_uncollect_are_effect_driven() {
    let mut interactor = CollectionInteractor::new(FakeCollection);
    let first = interactor.dispatch(CollectionAction::Load);
    assert_eq!(first.effects, vec![CollectionEffect::GetPage { page: 0 }]);
    let action = interactor.run(first.effects[0].clone());
    interactor.dispatch(action);
    let next = interactor.dispatch(CollectionAction::LoadNextPage);
    assert_eq!(next.effects, vec![CollectionEffect::GetPage { page: 1 }]);
    let action = interactor.run(next.effects[0].clone());
    interactor.dispatch(action);
    assert_eq!(
        interactor.state.articles,
        LoadState::Ready(vec![
            Article {
                id: 1,
                title: "Favorite 0".into(),
                link: "https://example.test/favorite".into(),
                author: "Elena".into(),
                nice_date: "today".into(),
                collect: true,
                chapter_name: "Rust".into(),
                tags: vec![]
            },
            Article {
                id: 2,
                title: "Favorite 1".into(),
                link: "https://example.test/favorite".into(),
                author: "Elena".into(),
                nice_date: "today".into(),
                collect: true,
                chapter_name: "Rust".into(),
                tags: vec![]
            },
        ])
    );
    assert!(!interactor.state.has_more);

    let change = interactor.dispatch(CollectionAction::Uncollect { origin_id: 2 });
    assert_eq!(
        change.effects,
        vec![CollectionEffect::Uncollect { origin_id: 2 }]
    );
    let action = interactor.run(change.effects[0].clone());
    interactor.dispatch(action);
    assert!(interactor.state.pending_change.is_none());
}

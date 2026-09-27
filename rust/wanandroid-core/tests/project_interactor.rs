use wanandroid_core::{
    LoadState, Page, ProjectAction, ProjectArticle, ProjectCategory, ProjectEffect, ProjectInteractor,
    ProjectRepository, RepositoryError,
};

struct FakeProjects;

impl ProjectRepository for FakeProjects {
    fn categories(&self) -> Result<Vec<ProjectCategory>, RepositoryError> {
        Ok(vec![ProjectCategory { id: 3, name: "Compose".into(), order: 1, visible: 1 }])
    }

    fn articles(&self, page: u32, category_id: u64) -> Result<Page<ProjectArticle>, RepositoryError> {
        Ok(Page {
            cur_page: page,
            datas: vec![ProjectArticle {
                id: page as u64 + 1,
                title: format!("Project {category_id}/{page}"),
                link: "https://example.test/project".into(),
                author: "Elena".into(),
                nice_date: "today".into(),
                collect: false,
                chapter_name: "Project".into(),
                envelope_pic: String::new(),
                thumbnail: String::new(),
            }],
            over: page >= 1,
            page_count: 2,
            total: 2,
        })
    }
}

#[test]
fn selecting_category_resets_and_pages_project_results() {
    let mut interactor = ProjectInteractor::new(FakeProjects);
    let categories = interactor.dispatch(ProjectAction::LoadCategories);
    assert_eq!(categories.effects, vec![ProjectEffect::GetCategories]);
    let action = interactor.run(categories.effects[0].clone());
    interactor.dispatch(action);
    assert_eq!(interactor.state.categories, LoadState::Ready(vec![ProjectCategory {
        id: 3, name: "Compose".into(), order: 1, visible: 1,
    }]));

    let first = interactor.dispatch(ProjectAction::SelectCategory { id: 3 });
    assert_eq!(first.effects, vec![ProjectEffect::GetArticles { page: 0, category_id: 3 }]);
    let action = interactor.run(first.effects[0].clone());
    interactor.dispatch(action);
    let next = interactor.dispatch(ProjectAction::LoadNextPage);
    assert_eq!(next.effects, vec![ProjectEffect::GetArticles { page: 1, category_id: 3 }]);
    let action = interactor.run(next.effects[0].clone());
    interactor.dispatch(action);

    assert_eq!(interactor.state.articles, LoadState::Ready(vec![
        ProjectArticle { id: 1, title: "Project 3/0".into(), link: "https://example.test/project".into(), author: "Elena".into(), nice_date: "today".into(), collect: false, chapter_name: "Project".into(), envelope_pic: String::new(), thumbnail: String::new() },
        ProjectArticle { id: 2, title: "Project 3/1".into(), link: "https://example.test/project".into(), author: "Elena".into(), nice_date: "today".into(), collect: false, chapter_name: "Project".into(), envelope_pic: String::new(), thumbnail: String::new() },
    ]));
    assert!(!interactor.state.has_more);
}

use crate::project::{ProjectArticle, ProjectCategory, ProjectRepository};
use crate::repository::RepositoryError;
use crate::state::{LoadState, ReduceResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectState {
    pub categories: LoadState<Vec<ProjectCategory>>,
    pub articles: LoadState<Vec<ProjectArticle>>,
    pub selected_category: Option<u64>,
    pub next_page: u32,
    pub has_more: bool,
}

impl Default for ProjectState {
    fn default() -> Self {
        Self {
            categories: LoadState::Idle,
            articles: LoadState::Idle,
            selected_category: None,
            next_page: 0,
            has_more: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectAction {
    LoadCategories,
    CategoriesLoaded(Result<Vec<ProjectCategory>, RepositoryError>),
    SelectCategory { id: u64 },
    ArticlesLoaded(Result<crate::domain::Page<ProjectArticle>, RepositoryError>),
    LoadNextPage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectEffect {
    GetCategories,
    GetArticles { page: u32, category_id: u64 },
}

pub struct ProjectInteractor<R> {
    repository: R,
    pub state: ProjectState,
}

impl<R: ProjectRepository> ProjectInteractor<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            state: ProjectState::default(),
        }
    }

    pub fn dispatch(&mut self, action: ProjectAction) -> ReduceResult<ProjectEffect> {
        match action {
            ProjectAction::LoadCategories => {
                self.state.categories = LoadState::Loading;
                ReduceResult::effects([ProjectEffect::GetCategories])
            }
            ProjectAction::CategoriesLoaded(result) => {
                self.state.categories = result.into();
                // Auto-select the first category so the tab shows content without an extra tap.
                let first_category = match &self.state.categories {
                    LoadState::Ready(categories) => categories.first().map(|category| category.id),
                    _ => None,
                };
                match first_category {
                    Some(id) => {
                        self.state.selected_category = Some(id);
                        self.state.articles = LoadState::Loading;
                        self.state.next_page = 0;
                        self.state.has_more = true;
                        ReduceResult::effects([ProjectEffect::GetArticles {
                            page: 0,
                            category_id: id,
                        }])
                    }
                    None => ReduceResult::none(),
                }
            }
            ProjectAction::SelectCategory { id } => {
                self.state.selected_category = Some(id);
                self.state.articles = LoadState::Loading;
                self.state.next_page = 0;
                self.state.has_more = true;
                ReduceResult::effects([ProjectEffect::GetArticles {
                    page: 0,
                    category_id: id,
                }])
            }
            ProjectAction::LoadNextPage if self.state.has_more => {
                let Some(category_id) = self.state.selected_category else {
                    return ReduceResult::none();
                };
                self.state.articles = match &self.state.articles {
                    LoadState::Ready(items) => LoadState::Refreshing(items.clone()),
                    other => other.clone(),
                };
                ReduceResult::effects([ProjectEffect::GetArticles {
                    page: self.state.next_page,
                    category_id,
                }])
            }
            ProjectAction::LoadNextPage => ReduceResult::none(),
            ProjectAction::ArticlesLoaded(result) => {
                match result {
                    Ok(page) => {
                        self.state.has_more = !page.over;
                        self.state.next_page = page.cur_page + 1;
                        let mut articles = match &self.state.articles {
                            LoadState::Ready(items) | LoadState::Refreshing(items) => items.clone(),
                            _ => Vec::new(),
                        };
                        articles.extend(page.datas);
                        self.state.articles = LoadState::Ready(articles);
                    }
                    Err(error) => self.state.articles = LoadState::Error(error.to_string()),
                }
                ReduceResult::none()
            }
        }
    }

    pub fn run(&self, effect: ProjectEffect) -> ProjectAction {
        match effect {
            ProjectEffect::GetCategories => {
                ProjectAction::CategoriesLoaded(self.repository.categories())
            }
            ProjectEffect::GetArticles { page, category_id } => {
                ProjectAction::ArticlesLoaded(self.repository.articles(page, category_id))
            }
        }
    }
}

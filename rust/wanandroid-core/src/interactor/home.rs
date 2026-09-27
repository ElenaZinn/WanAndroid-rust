use crate::domain::{Article, Banner};
use crate::repository::{HomeRepository, RepositoryError};
use crate::state::{LoadState, ReduceResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomeState {
    pub banners: LoadState<Vec<Banner>>,
    pub articles: LoadState<Vec<Article>>,
    pub next_page: u32,
    pub has_more: bool,
}

impl Default for HomeState {
    fn default() -> Self {
        Self {
            banners: LoadState::Idle,
            articles: LoadState::Idle,
            next_page: 0,
            has_more: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeAction {
    Load,
    Refresh,
    LoadNextPage,
    BannersLoaded(Result<Vec<Banner>, RepositoryError>),
    ArticlesLoaded(Result<crate::domain::ArticlePage, RepositoryError>),
    ToggleCollect { article_id: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeEffect {
    GetBanners,
    GetArticles { page: u32 },
    Collect { article_id: u64 },
}

pub struct HomeInteractor<R> {
    repository: R,
    pub state: HomeState,
}

impl<R: HomeRepository> HomeInteractor<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            state: HomeState::default(),
        }
    }

    pub fn dispatch(&mut self, action: HomeAction) -> ReduceResult<HomeEffect> {
        match action {
            HomeAction::Load | HomeAction::Refresh => {
                self.state.banners = LoadState::Loading;
                self.state.articles = LoadState::Loading;
                self.state.next_page = 0;
                self.state.has_more = true;
                ReduceResult::effects([HomeEffect::GetBanners, HomeEffect::GetArticles { page: 0 }])
            }
            HomeAction::LoadNextPage if self.state.has_more => {
                self.state.articles = match &self.state.articles {
                    LoadState::Ready(items) => LoadState::Refreshing(items.clone()),
                    other => other.clone(),
                };
                ReduceResult::effects([HomeEffect::GetArticles {
                    page: self.state.next_page,
                }])
            }
            HomeAction::LoadNextPage => ReduceResult::none(),
            HomeAction::BannersLoaded(result) => {
                self.state.banners = result.into();
                ReduceResult::none()
            }
            HomeAction::ArticlesLoaded(result) => {
                match result {
                    Ok(page) => {
                        self.state.has_more = !page.page.over;
                        self.state.next_page = page.page.cur_page + 1;
                        let mut articles = match &self.state.articles {
                            LoadState::Ready(items) | LoadState::Refreshing(items) => items.clone(),
                            _ => Vec::new(),
                        };
                        articles.extend(page.page.datas);
                        self.state.articles = LoadState::Ready(articles);
                    }
                    Err(error) => self.state.articles = LoadState::Error(error.to_string()),
                }
                ReduceResult::none()
            }
            HomeAction::ToggleCollect { article_id } => {
                ReduceResult::effects([HomeEffect::Collect { article_id }])
            }
        }
    }

    pub fn run(&self, effect: HomeEffect) -> HomeAction {
        match effect {
            HomeEffect::GetBanners => HomeAction::BannersLoaded(self.repository.get_banners()),
            HomeEffect::GetArticles { page } => {
                HomeAction::ArticlesLoaded(self.repository.get_articles(page))
            }
            HomeEffect::Collect { article_id: _ } => HomeAction::ArticlesLoaded(Err(
                RepositoryError::Transport("collect endpoint not wired yet".into()),
            )),
        }
    }
}

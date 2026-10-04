use crate::collection::CollectionRepository;
use crate::domain::{Article, Banner};
use crate::repository::{HomeRepository, RepositoryError};
use crate::state::{LoadState, ReduceResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HomeState {
    pub banners: LoadState<Vec<Banner>>,
    pub articles: LoadState<Vec<Article>>,
    pub next_page: u32,
    pub has_more: bool,
    /// Non-fatal failure that should be surfaced without discarding the loaded list.
    pub last_error: Option<String>,
}

impl Default for HomeState {
    fn default() -> Self {
        Self {
            banners: LoadState::Idle,
            articles: LoadState::Idle,
            next_page: 0,
            has_more: true,
            last_error: None,
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
    ToggleCollect {
        article_id: u64,
    },
    CollectFinished {
        article_id: u64,
        collected: bool,
        result: Result<(), RepositoryError>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeEffect {
    GetBanners,
    GetArticles { page: u32 },
    SetCollected { article_id: u64, collected: bool },
}

pub struct HomeInteractor<R> {
    repository: R,
    pub state: HomeState,
}

impl<R: HomeRepository + CollectionRepository> HomeInteractor<R> {
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
                self.state.last_error = None;
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
                    Err(error) => {
                        // Keep an already loaded list visible instead of replacing it with an error.
                        if matches!(self.state.articles, LoadState::Refreshing(_)) {
                            if let LoadState::Refreshing(items) = self.state.articles.clone() {
                                self.state.articles = LoadState::Ready(items);
                            }
                            self.state.last_error = Some(error.to_string());
                        } else {
                            self.state.articles = LoadState::Error(error.to_string());
                        }
                    }
                }
                ReduceResult::none()
            }
            HomeAction::ToggleCollect { article_id } => {
                // Flip the star: collecting an uncollected article and vice versa.
                let collected = self
                    .state
                    .articles
                    .value()
                    .and_then(|articles| articles.iter().find(|article| article.id == article_id))
                    .map(|article| !article.collect)
                    .unwrap_or(true);
                self.state.last_error = None;
                ReduceResult::effects([HomeEffect::SetCollected {
                    article_id,
                    collected,
                }])
            }
            HomeAction::CollectFinished {
                article_id,
                collected,
                result,
            } => {
                match result {
                    Ok(()) => {
                        if let Some(articles) = self.state.articles.value_mut() {
                            if let Some(article) =
                                articles.iter_mut().find(|article| article.id == article_id)
                            {
                                article.collect = collected;
                            }
                        }
                        self.state.last_error = None;
                    }
                    Err(error) => self.state.last_error = Some(error.to_string()),
                }
                ReduceResult::none()
            }
        }
    }

    pub fn run(&self, effect: HomeEffect) -> HomeAction {
        match effect {
            HomeEffect::GetBanners => HomeAction::BannersLoaded(self.repository.get_banners()),
            HomeEffect::GetArticles { page } => {
                HomeAction::ArticlesLoaded(self.repository.get_articles(page))
            }
            HomeEffect::SetCollected {
                article_id,
                collected,
            } => {
                let result = if collected {
                    self.repository.collect(article_id)
                } else {
                    self.repository.uncollect(article_id)
                };
                HomeAction::CollectFinished {
                    article_id,
                    collected,
                    result,
                }
            }
        }
    }
}

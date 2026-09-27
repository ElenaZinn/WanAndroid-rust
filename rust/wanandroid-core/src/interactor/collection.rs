use crate::collection::{CollectionPage, CollectionRepository};
use crate::domain::Article;
use crate::repository::RepositoryError;
use crate::state::{LoadState, ReduceResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionState {
    pub articles: LoadState<Vec<Article>>,
    pub next_page: u32,
    pub has_more: bool,
    pub pending_change: Option<CollectionChange>,
}

impl Default for CollectionState {
    fn default() -> Self {
        Self { articles: LoadState::Idle, next_page: 0, has_more: true, pending_change: None }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionChange {
    Collect { article_id: u64 },
    Uncollect { origin_id: u64 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionAction {
    Load,
    Refresh,
    LoadNextPage,
    PageLoaded(Result<CollectionPage, RepositoryError>),
    Collect { article_id: u64 },
    Uncollect { origin_id: u64 },
    ChangeFinished(Result<(), RepositoryError>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionEffect {
    GetPage { page: u32 },
    Collect { article_id: u64 },
    Uncollect { origin_id: u64 },
}

pub struct CollectionInteractor<R> { repository: R, pub state: CollectionState }

impl<R: CollectionRepository> CollectionInteractor<R> {
    pub fn new(repository: R) -> Self { Self { repository, state: CollectionState::default() } }

    pub fn dispatch(&mut self, action: CollectionAction) -> ReduceResult<CollectionEffect> {
        match action {
            CollectionAction::Load | CollectionAction::Refresh => {
                self.state.articles = LoadState::Loading;
                self.state.next_page = 0;
                self.state.has_more = true;
                ReduceResult::effects([CollectionEffect::GetPage { page: 0 }])
            }
            CollectionAction::LoadNextPage if self.state.has_more => {
                self.state.articles = match &self.state.articles {
                    LoadState::Ready(items) => LoadState::Refreshing(items.clone()),
                    other => other.clone(),
                };
                ReduceResult::effects([CollectionEffect::GetPage { page: self.state.next_page }])
            }
            CollectionAction::LoadNextPage => ReduceResult::none(),
            CollectionAction::PageLoaded(result) => {
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
            CollectionAction::Collect { article_id } => {
                self.state.pending_change = Some(CollectionChange::Collect { article_id });
                ReduceResult::effects([CollectionEffect::Collect { article_id }])
            }
            CollectionAction::Uncollect { origin_id } => {
                self.state.pending_change = Some(CollectionChange::Uncollect { origin_id });
                ReduceResult::effects([CollectionEffect::Uncollect { origin_id }])
            }
            CollectionAction::ChangeFinished(result) => {
                match result {
                    Ok(()) => self.state.pending_change = None,
                    Err(error) => self.state.articles = LoadState::Error(error.to_string()),
                }
                ReduceResult::none()
            }
        }
    }

    pub fn run(&self, effect: CollectionEffect) -> CollectionAction {
        match effect {
            CollectionEffect::GetPage { page } => CollectionAction::PageLoaded(self.repository.list(page)),
            CollectionEffect::Collect { article_id } => CollectionAction::ChangeFinished(self.repository.collect(article_id)),
            CollectionEffect::Uncollect { origin_id } => CollectionAction::ChangeFinished(self.repository.uncollect(origin_id)),
        }
    }
}

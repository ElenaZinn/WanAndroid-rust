use crate::domain::{Article, ArticlePage};
use crate::repository::{RepositoryError, SearchRepository};
use crate::search::HotKey;
use crate::state::{LoadState, ReduceResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchState {
    pub hot_keys: LoadState<Vec<HotKey>>,
    pub articles: LoadState<Vec<Article>>,
    pub keyword: String,
    pub author: Option<String>,
    pub next_page: u32,
    pub has_more: bool,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            hot_keys: LoadState::Idle,
            articles: LoadState::Idle,
            keyword: String::new(),
            author: None,
            next_page: 0,
            has_more: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchAction {
    LoadHotKeys,
    HotKeysLoaded(Result<Vec<HotKey>, RepositoryError>),
    Submit { keyword: String },
    SearchLoaded(Result<ArticlePage, RepositoryError>),
    SearchAuthor { author: String },
    LoadNextPage,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchEffect {
    GetHotKeys,
    Search { page: u32, keyword: String },
    SearchAuthor { page: u32, author: String },
}

pub struct SearchInteractor<R> {
    repository: R,
    pub state: SearchState,
}

impl<R: SearchRepository> SearchInteractor<R> {
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            state: SearchState::default(),
        }
    }

    pub fn dispatch(&mut self, action: SearchAction) -> ReduceResult<SearchEffect> {
        match action {
            SearchAction::LoadHotKeys => {
                self.state.hot_keys = LoadState::Loading;
                ReduceResult::effects([SearchEffect::GetHotKeys])
            }
            SearchAction::HotKeysLoaded(result) => {
                self.state.hot_keys = result.into();
                ReduceResult::none()
            }
            SearchAction::Submit { keyword } => {
                let keyword = keyword.trim().to_owned();
                if keyword.is_empty() {
                    self.state.articles = LoadState::Error("search keyword is empty".into());
                    self.state.has_more = false;
                    return ReduceResult::none();
                }
                self.state.keyword = keyword.clone();
                self.state.author = None;
                self.state.next_page = 0;
                self.state.has_more = true;
                self.state.articles = LoadState::Loading;
                ReduceResult::effects([SearchEffect::Search { page: 0, keyword }])
            }
            SearchAction::SearchAuthor { author } => {
                let author = author.trim().to_owned();
                if author.is_empty() {
                    self.state.articles = LoadState::Error("author is empty".into());
                    self.state.has_more = false;
                    return ReduceResult::none();
                }
                self.state.author = Some(author.clone());
                self.state.keyword.clear();
                self.state.next_page = 0;
                self.state.has_more = true;
                self.state.articles = LoadState::Loading;
                ReduceResult::effects([SearchEffect::SearchAuthor { page: 0, author }])
            }
            SearchAction::LoadNextPage if self.state.has_more => {
                self.state.articles = match &self.state.articles {
                    LoadState::Ready(items) => LoadState::Refreshing(items.clone()),
                    other => other.clone(),
                };
                if let Some(author) = self.state.author.clone() {
                    ReduceResult::effects([SearchEffect::SearchAuthor {
                        page: self.state.next_page,
                        author,
                    }])
                } else {
                    ReduceResult::effects([SearchEffect::Search {
                        page: self.state.next_page,
                        keyword: self.state.keyword.clone(),
                    }])
                }
            }
            SearchAction::LoadNextPage => ReduceResult::none(),
            SearchAction::SearchLoaded(result) => {
                match result {
                    Ok(page) => self.apply_page(page),
                    Err(error) => self.state.articles = LoadState::Error(error.to_string()),
                }
                ReduceResult::none()
            }
        }
    }

    fn apply_page(&mut self, page: ArticlePage) {
        self.state.has_more = !page.page.over;
        self.state.next_page = page.page.cur_page + 1;
        let mut articles = match &self.state.articles {
            LoadState::Ready(items) | LoadState::Refreshing(items) => items.clone(),
            _ => Vec::new(),
        };
        articles.extend(page.page.datas);
        self.state.articles = LoadState::Ready(articles);
    }

    pub fn run(&self, effect: SearchEffect) -> SearchAction {
        match effect {
            SearchEffect::GetHotKeys => SearchAction::HotKeysLoaded(self.repository.hot_keys()),
            SearchEffect::Search { page, keyword } => {
                SearchAction::SearchLoaded(self.repository.search(page, &keyword))
            }
            SearchEffect::SearchAuthor { page, author } => {
                SearchAction::SearchLoaded(self.repository.search_author(page, &author))
            }
        }
    }
}

use crate::collection::CollectionRepository;
use crate::repository::HomeRepository;

pub mod auth;
pub mod collection;
pub mod home;
pub mod project;
pub mod search;

pub struct CoreInteractor<R> {
    pub home: home::HomeInteractor<R>,
}

// Home owns the collect toggle, so it needs the collection endpoints as well as the feed.
impl<R: HomeRepository + CollectionRepository> CoreInteractor<R> {
    pub fn new(repository: R) -> Self {
        Self {
            home: home::HomeInteractor::new(repository),
        }
    }
}

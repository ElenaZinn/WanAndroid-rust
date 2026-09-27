use crate::repository::HomeRepository;

pub mod auth;
pub mod home;

pub struct CoreInteractor<R> {
    pub home: home::HomeInteractor<R>,
}

impl<R: HomeRepository> CoreInteractor<R> {
    pub fn new(repository: R) -> Self {
        Self {
            home: home::HomeInteractor::new(repository),
        }
    }
}

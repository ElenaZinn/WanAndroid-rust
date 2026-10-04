use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadState<T> {
    Idle,
    Loading,
    Refreshing(T),
    Ready(T),
    Error(String),
}

impl<T> LoadState<T> {
    /// Borrows the loaded value when the state currently holds one.
    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Refreshing(value) | Self::Ready(value) => Some(value),
            Self::Idle | Self::Loading | Self::Error(_) => None,
        }
    }

    /// Mutably borrows the loaded value when the state currently holds one.
    pub fn value_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Refreshing(value) | Self::Ready(value) => Some(value),
            Self::Idle | Self::Loading | Self::Error(_) => None,
        }
    }
}

impl<T> From<Result<T, crate::repository::RepositoryError>> for LoadState<T> {
    fn from(value: Result<T, crate::repository::RepositoryError>) -> Self {
        match value {
            Ok(value) => Self::Ready(value),
            Err(error) => Self::Error(error.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReduceResult<E> {
    pub effects: Vec<E>,
}

impl<E> ReduceResult<E> {
    pub fn none() -> Self {
        Self {
            effects: Vec::new(),
        }
    }

    pub fn effects<const N: usize>(effects: [E; N]) -> Self {
        Self {
            effects: effects.into_iter().collect(),
        }
    }
}

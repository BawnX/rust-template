use crate::config::models::enums::{CqrsError, RepositoryError};
use std::sync::PoisonError;

pub mod enums;

pub type CqrsResult<T> = Result<T, CqrsError>;

impl<T> From<PoisonError<T>> for CqrsError {
    fn from(poison_error: PoisonError<T>) -> Self {
        CqrsError::LockError(poison_error.to_string())
    }
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

impl<T> From<PoisonError<T>> for RepositoryError {
    fn from(poison_error: PoisonError<T>) -> Self {
        RepositoryError::LockError(poison_error.to_string())
    }
}

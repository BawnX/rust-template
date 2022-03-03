use std::sync::PoisonError;
use crate::config::models::enums::RepositoryError;
use serde::{Deserialize, Serialize};

pub mod enums;

#[allow(dead_code)]
pub type RepositoryResult<T> = Result<T, RepositoryError>;

impl<T> From<PoisonError<T>> for RepositoryError {
    fn from(poison_error: PoisonError<T>) -> Self {
        RepositoryError::LockError(poison_error.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}
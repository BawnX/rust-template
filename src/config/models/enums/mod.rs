use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Poison Error: `{0}`")]
    LockError(String),
    #[error("This entity already exists")]
    AlreadyExists,
    #[error("This entity does not exists")]
    DoesNotExist,
    #[error("This id format is not valid")]
    InvalidId,
}
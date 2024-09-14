use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum DaoError {
    #[error("insert error: {0}")]
    InsertError(String),
    #[error("delete error: {0}")]
    DeleteError(String),
    #[error("fetch error: {0}")]
    FetchError(String),
    #[error("update error: {0}")]
    UpdateError(String),
}

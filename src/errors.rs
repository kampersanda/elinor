use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmirError {
    #[error("Error: {0}, {1}")]
    DuplicateEntry(String, String),

    #[error("Error: {0}")]
    MissingQueryId(String),
}

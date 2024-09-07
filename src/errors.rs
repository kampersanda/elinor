use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmirError {
    #[error("Error: QueryID {0}, DocID {1}")]
    DuplicateQueryDoc(String, String),

    #[error("Error: {0}")]
    MissingQueryId(String),
}

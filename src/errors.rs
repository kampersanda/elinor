//! Error handling for Emir.
use thiserror::Error;

/// Error types for Emir.
#[derive(Error, Debug)]
pub enum EmirError<K>
where
    K: std::fmt::Display,
{
    /// Error when query-document pairs are duplicated.
    #[error("Error: QueryID {0}, DocID {1}")]
    DuplicateDocId(K, K),

    /// Error when a query is missing.
    #[error("Error: {0}")]
    MissingQueryId(K),

    /// Error when a document is missing.
    #[error("Empty input")]
    EmptyLines,

    /// Error when a document is missing.
    #[error("Invalid format")]
    InvalidFormat,
}

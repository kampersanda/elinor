//! Error handling for Emir.
use thiserror::Error;

/// Error types for Emir.
#[derive(Error, Debug)]
pub enum EmirError {
    /// Error when query-document pairs are duplicated.
    #[error("{0}")]
    DuplicateEntry(String),

    /// Error when a query is missing.
    #[error("{0}")]
    MissingEntry(String),

    /// Error when a document is missing.
    #[error("{0}")]
    InvalidFormat(String),
}

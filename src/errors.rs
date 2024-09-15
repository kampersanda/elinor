//! Error handling for Ireval.
use thiserror::Error;

/// Error types for Ireval.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum EmirError {
    /// Error when an entry is duplicated.
    #[error("{0}")]
    DuplicateEntry(String),

    /// Error when an entry is missing.
    #[error("{0}")]
    MissingEntry(String),

    /// Error when the format is invalid.
    #[error("{0}")]
    InvalidFormat(String),
}

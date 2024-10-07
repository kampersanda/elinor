//! Error handling for Elinor.
use thiserror::Error;

/// Error types for Elinor.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum ElinorError {
    /// Error when an entry is duplicated.
    #[error("{0}")]
    DuplicateEntry(String),

    /// Error when an entry is missing.
    #[error("{0}")]
    MissingEntry(String),

    /// Error when the computation is uncomputable.
    #[error("{0}")]
    Uncomputable(String),

    /// Error when the argument is invalid.
    #[error("{0}")]
    InvalidArgument(String),

    /// Error when the format is invalid.
    #[error("{0}")]
    InvalidFormat(String),
}

/// Specialized result type for Elinor.
pub type Result<T> = std::result::Result<T, ElinorError>;

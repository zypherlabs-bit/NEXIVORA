//! Central error type for Nexivora.

use thiserror::Error;

/// The unified error type used across all Nexivora crates.
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("already exists: {0}")]
    AlreadyExists(String),

    #[error("out of bounds: {0}")]
    OutOfBounds(String),

    #[error("parse error: {0}")]
    Parse(String),

    #[error("formula error: {0}")]
    Formula(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("unsupported: {0}")]
    Unsupported(String),

    #[error("invalid data: {0}")]
    InvalidData(String),

    #[error("security error: {0}")]
    Security(String),

    #[error("internal error: {0}")]
    Internal(String),
}

/// Convenience alias.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Create an invalid-argument error.
    pub fn invalid_argument(msg: impl Into<String>) -> Self {
        Error::InvalidArgument(msg.into())
    }

    /// Create a not-found error.
    pub fn not_found(msg: impl Into<String>) -> Self {
        Error::NotFound(msg.into())
    }

    /// Create an unsupported error.
    pub fn unsupported(msg: impl Into<String>) -> Self {
        Error::Unsupported(msg.into())
    }
}
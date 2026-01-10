//! Error types and result aliases for Riwaq Arch.
//!
//! This module provides a unified error handling approach using `thiserror`
//! for defining error types and `anyhow` for error context propagation.

use std::path::PathBuf;
use thiserror::Error;

/// Result type alias using `RiwaqError` as the error type.
pub type Result<T> = std::result::Result<T, RiwaqError>;

/// Main error type for Riwaq Arch operations.
#[derive(Error, Debug)]
pub enum RiwaqError {
    /// File system related errors
    #[error("File system error at '{path}': {message}")]
    FileSystem { path: PathBuf, message: String },

    /// File not found
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    /// Directory not found
    #[error("Directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    /// File too large to process
    #[error("File too large to process: {path} ({size} bytes, max: {max_size} bytes)")]
    FileTooLarge {
        path: PathBuf,
        size: u64,
        max_size: u64,
    },

    /// Parsing related errors
    #[error("Failed to parse file '{path}': {message}")]
    ParseError { path: PathBuf, message: String },

    /// Unsupported language
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    /// Git repository errors
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    /// Not a git repository
    #[error("Not a git repository: {0}")]
    NotAGitRepository(PathBuf),

    /// LLM client errors
    #[error("LLM error: {message}")]
    LlmError { message: String },

    /// LLM rate limit exceeded
    #[error("LLM rate limit exceeded. Retry after {retry_after_secs} seconds")]
    LlmRateLimited { retry_after_secs: u64 },

    /// LLM context too large
    #[error("Context exceeds maximum token limit ({tokens} > {max_tokens})")]
    ContextTooLarge { tokens: usize, max_tokens: usize },

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Missing environment variable
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// HTTP request errors
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Index/search errors
    #[error("Index error: {0}")]
    Index(String),

    /// Analysis timeout
    #[error("Analysis timed out after {seconds} seconds")]
    Timeout { seconds: u64 },

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Multiple errors occurred
    #[error("Multiple errors occurred: {0:?}")]
    Multiple(Vec<RiwaqError>),
}

impl RiwaqError {
    /// Create a new file system error.
    pub fn file_system(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::FileSystem {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create a new parse error.
    pub fn parse_error(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::ParseError {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create a new LLM error.
    pub fn llm_error(message: impl Into<String>) -> Self {
        Self::LlmError {
            message: message.into(),
        }
    }

    /// Create a new configuration error.
    pub fn config(message: impl Into<String>) -> Self {
        Self::Configuration(message.into())
    }

    /// Create an internal error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    /// Check if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::LlmRateLimited { .. } | Self::Http(_) | Self::Timeout { .. }
        )
    }

    /// Get retry delay in seconds if this is a rate limit error.
    pub fn retry_after(&self) -> Option<u64> {
        match self {
            Self::LlmRateLimited { retry_after_secs } => Some(*retry_after_secs),
            _ => None,
        }
    }
}

/// Extension trait for adding context to errors.
pub trait ResultExt<T> {
    /// Add context to an error.
    fn with_context<F, S>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> S,
        S: Into<String>;
}

impl<T, E: Into<RiwaqError>> ResultExt<T> for std::result::Result<T, E> {
    fn with_context<F, S>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> S,
        S: Into<String>,
    {
        self.map_err(|e| {
            let inner = e.into();
            RiwaqError::Internal(format!("{}: {}", f().into(), inner))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = RiwaqError::FileNotFound(PathBuf::from("/test/file.rs"));
        assert_eq!(err.to_string(), "File not found: /test/file.rs");
    }

    #[test]
    fn test_is_retryable() {
        let rate_limit = RiwaqError::LlmRateLimited {
            retry_after_secs: 30,
        };
        assert!(rate_limit.is_retryable());

        let parse_err = RiwaqError::parse_error("/test.rs", "syntax error");
        assert!(!parse_err.is_retryable());
    }

    #[test]
    fn test_retry_after() {
        let err = RiwaqError::LlmRateLimited {
            retry_after_secs: 60,
        };
        assert_eq!(err.retry_after(), Some(60));

        let err = RiwaqError::FileNotFound(PathBuf::from("/test"));
        assert_eq!(err.retry_after(), None);
    }
}

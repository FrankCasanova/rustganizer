//! Core error types and handling for Rustganizer

use thiserror::Error;
use std::path::PathBuf;

/// Main error type for the application
#[derive(Error, Debug)]
pub enum Error {
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("User not found: {username}")]
    UserNotFound { username: String },
    
    #[error("Empty username provided")]
    EmptyUsername,
    
    #[error("File operation failed: {operation} on {path:?} - {source}")]
    FileOperation { 
        operation: String, 
        path: PathBuf, 
        source: std::io::Error 
    },
    
    #[error("Directory operation failed: {operation} on {path:?} - {source}")]
    DirectoryOperation { 
        operation: String, 
        path: PathBuf, 
        source: std::io::Error 
    },
    
    #[error("Permission denied: {path:?}")]
    PermissionDenied { path: PathBuf },
    
    #[error("Invalid file extension: {extension}")]
    InvalidExtension { extension: String },
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Thread operation failed: {0}")]
    Thread(#[from] std::thread::AccessError),
    
    #[error("Invalid language code: {language}")]
    InvalidLanguage { language: String },
    
    #[error("Cancelled operation")]
    Cancelled,
    
    #[error("Invalid configuration: {message}")]
    InvalidConfig { message: String },
    
    #[error("Other error: {0}")]
    Other(#[from] anyhow::Error),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Error context for better debugging
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub path: Option<PathBuf>,
    pub user_info: Option<String>,
    pub timestamp: std::time::SystemTime,
}

impl ErrorContext {
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            path: None,
            user_info: None,
            timestamp: std::time::SystemTime::now(),
        }
    }
    
    pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }
    
    pub fn with_user_info(mut self, user_info: impl Into<String>) -> Self {
        self.user_info = Some(user_info.into());
        self
    }
}

/// Error handling utilities
pub struct ErrorHandler;

impl ErrorHandler {
    pub fn handle_file_operation(
        error: std::io::Error,
        operation: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Error {
        Error::FileOperation {
            operation: operation.into(),
            path: path.into(),
            source: error,
        }
    }
    
    pub fn handle_directory_operation(
        error: std::io::Error,
        operation: impl Into<String>,
        path: impl Into<PathBuf>,
    ) -> Error {
        Error::DirectoryOperation {
            operation: operation.into(),
            path: path.into(),
            source: error,
        }
    }
    
    pub fn is_retryable_error(error: &Error) -> bool {
        matches!(
            error,
            Error::FileSystem(ref e) if e.kind() == std::io::ErrorKind::WouldBlock
                || e.kind() == std::io::ErrorKind::Interrupted
        )
    }
    
    pub fn is_fatal_error(error: &Error) -> bool {
        matches!(
            error,
            Error::PermissionDenied { .. }
                | Error::InvalidConfig { .. }
                | Error::InvalidLanguage { .. }
                | Error::EmptyUsername
        )
    }
}
//! Error types for enhanced PDF operations

use thiserror::Error;

/// Enhanced module result type
pub type Result<T> = std::result::Result<T, EnhancedError>;

/// Enhanced module errors
#[derive(Error, Debug)]
pub enum EnhancedError {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// PDF error from core library
    #[error("PDF error: {0}")]
    Pdf(#[from] crate::fitz::error::Error),

    /// Invalid dimensions
    #[error("Invalid dimensions: width={0}, height={1}")]
    InvalidDimensions(f32, f32),

    /// Page not found
    #[error("Page {0} not found")]
    PageNotFound(usize),

    /// Invalid page range
    #[error("Invalid page range: {0}..{1}")]
    InvalidRange(usize, usize),

    /// Unsupported operation
    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    /// Invalid parameter
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Document is encrypted
    #[error("Document is encrypted")]
    Encrypted,

    /// Document is not writable
    #[error("Document is not writable")]
    NotWritable,

    /// Feature not implemented
    #[error("Feature not implemented: {0}")]
    NotImplemented(String),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

impl From<String> for EnhancedError {
    fn from(s: String) -> Self {
        Self::Generic(s)
    }
}

impl From<&str> for EnhancedError {
    fn from(s: &str) -> Self {
        Self::Generic(s.to_string())
    }
}


//! Content Addition - Text, images, watermarks

use super::error::{EnhancedError, Result};

/// Watermark builder
pub struct Watermark {
    text: String,
}

impl Watermark {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn apply(&self) -> Result<()> {
        Err(EnhancedError::NotImplemented("Watermark::apply".into()))
    }
}


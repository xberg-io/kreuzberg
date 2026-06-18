//! Error types for heuristics operations.

use thiserror::Error;

/// Errors that can occur during heuristics analysis.
#[derive(Debug, Error)]
pub enum HeuristicsError {
    /// Invalid configuration value.
    #[error("Invalid heuristics configuration: {0}")]
    ConfigError(String),

    /// PDF analysis step failed (only when `heuristics-pdf` feature is active).
    #[error("PDF analysis failed: {0}")]
    PdfAnalysisError(String),
}

/// Result alias for heuristics operations.
pub type Result<T> = std::result::Result<T, HeuristicsError>;

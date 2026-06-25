//! Error types for Xberg.
//!
//! This module defines all error types used throughout the library. All errors
//! inherit from `XbergError` and follow Rust error handling best practices:
//!
//! - Use `thiserror` for automatic `Error` trait implementation
//! - Preserve error chains with `#[source]` attributes
//! - Include context in error messages (file paths, config values, etc.)
//!
//! # Error Handling Philosophy
//!
//! **System errors MUST always bubble up unchanged:**
//! - `XbergError::Io` (from `std::io::Error`) - File system errors, permission errors
//! - These indicate real system problems that users need to know about
//! - Never wrap or suppress these - they must surface to enable bug reports
//!
//! **Application errors are wrapped with context:**
//! - `Parsing` - Document format errors, corrupt files
//! - `Validation` - Invalid configuration or parameters
//! - `Ocr` - OCR processing failures
//! - `MissingDependency` - Missing optional system dependencies
//!
//! # Example
//!
//! ```rust
//! use xberg::{XbergError, Result};
//!
//! fn process_image_file(path: &str) -> Result<String> {
//!     // IO errors bubble up automatically via ?
//!     let content = std::fs::read_to_string(path)?;
//!
//!     // Application errors include context
//!     if content.is_empty() {
//!         return Err(XbergError::validation(
//!             format!("File is empty: {}", path)
//!         ));
//!     }
//!
//!     Ok(content)
//! }
//! ```
use thiserror::Error;

/// Result type alias using `XbergError`.
///
/// This is the standard return type for all fallible operations in Xberg.
pub type Result<T> = std::result::Result<T, XbergError>;

/// Main error type for all Xberg operations.
///
/// All errors in Xberg use this enum, which preserves error chains
/// and provides context for debugging.
///
/// # Variants
///
/// - `Io` - File system and I/O errors (always bubble up)
/// - `Parsing` - Document parsing errors (corrupt files, unsupported features)
/// - `Ocr` - OCR processing errors
/// - `Validation` - Input validation errors (invalid paths, config, parameters)
/// - `Cache` - Cache operation errors (non-fatal, can be ignored)
/// - `ImageProcessing` - Image manipulation errors
/// - `Serialization` - JSON/MessagePack serialization errors
/// - `MissingDependency` - Missing optional dependencies (tesseract, etc.)
/// - `Plugin` - Plugin-specific errors
/// - `LockPoisoned` - Mutex/RwLock poisoning (should not happen in normal operation)
/// - `UnsupportedFormat` - Unsupported MIME type or file format
/// - `Other` - Catch-all for uncommon errors
#[derive(Debug, Error)]
pub enum XbergError {
    /// A file system or I/O operation failed. These errors always bubble up unchanged.
    #[error("IO error: {0}")]
    Io(
        #[from]
        #[cfg_attr(alef, alef(skip))]
        std::io::Error,
    ), // Excluded from bindings; IO errors bubble up via Result

    /// Document parsing failed (e.g. corrupt file, unsupported format feature).
    #[error("Parsing error: {message}")]
    Parsing {
        /// Human-readable description of what failed during parsing.
        message: String,
        #[source]
        /// Underlying error that caused the parsing failure, if available.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// An OCR engine returned an error or produced unusable output.
    #[error("OCR error: {message}")]
    Ocr {
        /// Human-readable description of the OCR failure.
        message: String,
        #[source]
        /// Underlying error from the OCR backend, if available.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Invalid configuration or input parameters were supplied.
    #[error("Validation error: {message}")]
    Validation {
        /// Human-readable description of the validation failure.
        message: String,
        #[source]
        /// Underlying error that triggered validation failure, if available.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// A cache read or write operation failed.
    #[error("Cache error: {message}")]
    Cache {
        /// Human-readable description of the cache failure.
        message: String,
        #[source]
        /// Underlying error from the cache layer, if available.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// An image manipulation operation (resize, decode, DPI conversion) failed.
    #[error("Image processing error: {message}")]
    ImageProcessing {
        /// Human-readable description of the image processing failure.
        message: String,
        #[source]
        /// Underlying error from the image library, if available.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// JSON or MessagePack serialization/deserialization failed.
    #[error("Serialization error: {message}")]
    Serialization {
        /// Human-readable description of the serialization failure.
        message: String,
        #[source]
        /// Underlying serde error, if available.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// A required optional system dependency (e.g. `tesseract`) was not found.
    #[error("Missing dependency: {0}")]
    MissingDependency(String),

    /// A registered plugin returned an error during extraction.
    #[error("Plugin error in '{plugin_name}': {message}")]
    Plugin {
        /// Human-readable description of what the plugin failed to do.
        message: String,
        /// Name of the plugin that reported the error.
        plugin_name: String,
    },

    /// An internal `Mutex` or `RwLock` was found in a poisoned state.
    #[error("Lock poisoned: {0}")]
    LockPoisoned(String),

    /// The document's MIME type is not supported by any registered extractor.
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),

    /// The embedding model or embedding pipeline returned an error.
    #[error("Embedding error: {message}")]
    Embedding {
        /// Human-readable description of the embedding failure.
        message: String,
        #[source]
        /// Underlying error from the embedding backend, if available.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// The reranker model or reranking pipeline returned an error.
    ///
    /// Since v5.0.0.
    #[error("Reranking error: {message}")]
    Reranking {
        /// Human-readable description of the reranking failure.
        message: String,
        #[source]
        /// Underlying error from the reranker backend, if available.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// Audio/video transcription failed.
    #[error("Transcription error: {message}")]
    Transcription {
        /// Human-readable description of the transcription failure.
        message: String,
        #[source]
        /// Underlying error from the transcription backend, if available.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// The extraction operation exceeded the configured time limit.
    #[error("Extraction timed out after {elapsed_ms}ms (limit: {limit_ms}ms)")]
    Timeout {
        /// Wall-clock milliseconds elapsed before the timeout was detected.
        elapsed_ms: u64,
        /// Configured timeout limit in milliseconds.
        limit_ms: u64,
    },

    /// The extraction was cancelled via a [`crate::cancellation::CancellationToken`].
    #[error("Extraction cancelled")]
    Cancelled,

    /// A security policy was violated (e.g. zip bomb, oversized archive).
    #[error("Security violation: {message}")]
    Security {
        /// Human-readable description of the security violation.
        message: String,
        #[source]
        /// Underlying security error, if available.
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// A catch-all for uncommon errors that do not fit another variant.
    #[error("{0}")]
    Other(String),
}

impl From<crate::extractors::security::SecurityError> for XbergError {
    fn from(err: crate::extractors::security::SecurityError) -> Self {
        let message = err.to_string();
        XbergError::Security {
            message,
            source: Some(Box::new(err)),
        }
    }
}

#[cfg(any(feature = "excel", feature = "excel-wasm"))]
impl From<calamine::Error> for XbergError {
    fn from(err: calamine::Error) -> Self {
        XbergError::Parsing {
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<serde_json::Error> for XbergError {
    fn from(err: serde_json::Error) -> Self {
        XbergError::Serialization {
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<rmp_serde::encode::Error> for XbergError {
    fn from(err: rmp_serde::encode::Error) -> Self {
        XbergError::Serialization {
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

impl From<rmp_serde::decode::Error> for XbergError {
    fn from(err: rmp_serde::decode::Error) -> Self {
        XbergError::Serialization {
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

#[cfg(feature = "pdf")]
impl From<crate::pdf::error::PdfError> for XbergError {
    fn from(err: crate::pdf::error::PdfError) -> Self {
        if matches!(err, crate::pdf::error::PdfError::Cancelled) {
            return XbergError::Cancelled;
        }
        XbergError::Parsing {
            message: err.to_string(),
            source: Some(Box::new(err)),
        }
    }
}

macro_rules! error_constructor {
    ($name:ident, $variant:ident) => {
        pastey::paste! {
            #[doc = "Create a " $variant " error"]
            pub fn $name<S: Into<String>>(message: S) -> Self {
                Self::$variant {
                    message: message.into(),
                    source: None,
                }
            }

            #[doc = "Create a " $variant " error with source"]
            pub fn [<$name _with_source>]<S: Into<String>, E: std::error::Error + Send + Sync + 'static>(
                message: S,
                source: E,
            ) -> Self {
                Self::$variant {
                    message: message.into(),
                    source: Some(Box::new(source)),
                }
            }
        }
    };
}

impl XbergError {
    error_constructor!(parsing, Parsing);
    error_constructor!(ocr, Ocr);
    error_constructor!(validation, Validation);
    error_constructor!(cache, Cache);
    error_constructor!(image_processing, ImageProcessing);
    error_constructor!(serialization, Serialization);
    error_constructor!(embedding, Embedding);
    error_constructor!(reranking, Reranking);
    error_constructor!(security, Security);
    error_constructor!(transcription, Transcription);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_from() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let krz_err: XbergError = io_err.into();
        assert!(matches!(krz_err, XbergError::Io(_)));
        assert!(krz_err.to_string().contains("IO error"));
    }

    #[test]
    fn test_parsing_error() {
        let err = XbergError::parsing("invalid format");
        assert_eq!(err.to_string(), "Parsing error: invalid format");
    }

    #[test]
    fn test_parsing_error_with_source() {
        let source = std::io::Error::new(std::io::ErrorKind::InvalidData, "bad data");
        let err = XbergError::parsing_with_source("invalid format", source);
        assert_eq!(err.to_string(), "Parsing error: invalid format");
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn test_ocr_error() {
        let err = XbergError::ocr("OCR failed");
        assert_eq!(err.to_string(), "OCR error: OCR failed");
    }

    #[test]
    fn test_ocr_error_with_source() {
        let source = std::io::Error::other("tesseract failed");
        let err = XbergError::ocr_with_source("OCR failed", source);
        assert_eq!(err.to_string(), "OCR error: OCR failed");
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn test_validation_error() {
        let err = XbergError::validation("invalid input");
        assert_eq!(err.to_string(), "Validation error: invalid input");
    }

    #[test]
    fn test_validation_error_with_source() {
        let source = std::io::Error::new(std::io::ErrorKind::InvalidInput, "bad param");
        let err = XbergError::validation_with_source("invalid input", source);
        assert_eq!(err.to_string(), "Validation error: invalid input");
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn test_cache_error() {
        let err = XbergError::cache("cache write failed");
        assert_eq!(err.to_string(), "Cache error: cache write failed");
    }

    #[test]
    fn test_cache_error_with_source() {
        let source = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "cannot write");
        let err = XbergError::cache_with_source("cache write failed", source);
        assert_eq!(err.to_string(), "Cache error: cache write failed");
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn test_image_processing_error() {
        let err = XbergError::image_processing("resize failed");
        assert_eq!(err.to_string(), "Image processing error: resize failed");
    }

    #[test]
    fn test_image_processing_error_with_source() {
        let source = std::io::Error::other("image decode failed");
        let err = XbergError::image_processing_with_source("resize failed", source);
        assert_eq!(err.to_string(), "Image processing error: resize failed");
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn test_serialization_error() {
        let err = XbergError::serialization("JSON parse error");
        assert_eq!(err.to_string(), "Serialization error: JSON parse error");
    }

    #[test]
    fn test_serialization_error_with_source() {
        let source = std::io::Error::new(std::io::ErrorKind::InvalidData, "bad format");
        let err = XbergError::serialization_with_source("JSON parse error", source);
        assert_eq!(err.to_string(), "Serialization error: JSON parse error");
        assert!(std::error::Error::source(&err).is_some());
    }

    #[test]
    fn test_missing_dependency_error() {
        let err = XbergError::MissingDependency("tesseract not found".to_string());
        assert_eq!(err.to_string(), "Missing dependency: tesseract not found");
    }

    #[test]
    fn test_plugin_error() {
        let err = XbergError::Plugin {
            message: "extraction failed".to_string(),
            plugin_name: "pdf-extractor".to_string(),
        };
        assert_eq!(err.to_string(), "Plugin error in 'pdf-extractor': extraction failed");
    }

    #[test]
    fn test_unsupported_format_error() {
        let err = XbergError::UnsupportedFormat("application/unknown".to_string());
        assert_eq!(err.to_string(), "Unsupported format: application/unknown");
    }

    #[test]
    fn test_other_error() {
        let err = XbergError::Other("unexpected error".to_string());
        assert_eq!(err.to_string(), "unexpected error");
    }

    #[test]
    #[cfg(any(feature = "excel", feature = "excel-wasm"))]
    fn test_calamine_error_conversion() {
        let cal_err = calamine::Error::Msg("invalid Excel file");
        let krz_err: XbergError = cal_err.into();
        assert!(matches!(krz_err, XbergError::Parsing { .. }));
        assert!(krz_err.to_string().contains("Parsing error"));
    }

    #[test]
    fn test_serde_json_error_conversion() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let krz_err: XbergError = json_err.into();
        assert!(matches!(krz_err, XbergError::Serialization { .. }));
        assert!(krz_err.to_string().contains("Serialization error"));
    }

    #[test]
    fn test_rmp_encode_error_conversion() {
        use std::collections::HashMap;
        let mut map: HashMap<Vec<u8>, String> = HashMap::new();
        map.insert(vec![255, 255], "test".to_string());

        let result = rmp_serde::to_vec(&map);
        if let Err(rmp_err) = result {
            let krz_err: XbergError = rmp_err.into();
            assert!(matches!(krz_err, XbergError::Serialization { .. }));
        }
    }

    #[test]
    fn test_rmp_decode_error_conversion() {
        let invalid_msgpack = vec![0xFF, 0xFF, 0xFF];
        let rmp_err = rmp_serde::from_slice::<String>(&invalid_msgpack).unwrap_err();
        let krz_err: XbergError = rmp_err.into();
        assert!(matches!(krz_err, XbergError::Serialization { .. }));
    }

    #[test]
    #[cfg(feature = "pdf")]
    fn test_pdf_error_conversion() {
        let pdf_err = crate::pdf::error::PdfError::InvalidPdf("corrupt PDF".to_string());
        let krz_err: XbergError = pdf_err.into();
        assert!(matches!(krz_err, XbergError::Parsing { .. }));
    }

    #[test]
    fn test_error_debug() {
        let err = XbergError::validation("test");
        let debug_str = format!("{:?}", err);
        assert!(debug_str.contains("Validation"));
    }

    #[test]
    fn test_lock_poisoned_error() {
        let err = XbergError::LockPoisoned("Registry lock poisoned".to_string());
        assert_eq!(err.to_string(), "Lock poisoned: Registry lock poisoned");
    }

    #[test]
    fn test_io_error_bubbles_unchanged() {
        fn read_file() -> Result<String> {
            let content = std::fs::read_to_string("/nonexistent/file.txt")?;
            Ok(content)
        }

        let result = read_file();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), XbergError::Io(_)));
    }

    #[test]
    fn test_io_error_not_found() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let krz_err: XbergError = io_err.into();
        assert!(matches!(krz_err, XbergError::Io(_)));
        assert!(krz_err.to_string().contains("file not found"));
    }

    #[test]
    fn test_io_error_permission_denied() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "permission denied");
        let krz_err: XbergError = io_err.into();
        assert!(matches!(krz_err, XbergError::Io(_)));
        assert!(krz_err.to_string().contains("permission denied"));
    }

    #[test]
    fn test_io_error_invalid_data_vs_parsing() {
        let io_err = std::io::Error::new(std::io::ErrorKind::InvalidData, "corrupted data");
        let krz_err: XbergError = io_err.into();
        assert!(matches!(krz_err, XbergError::Io(_)));

        let parse_err = XbergError::parsing("corrupted format");
        assert!(matches!(parse_err, XbergError::Parsing { .. }));
    }
}

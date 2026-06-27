//! Request and response types for the extraction service.

use crate::core::config::{ExtractionConfig, FileExtractionConfig};
use bytes::Bytes;
use std::path::PathBuf;
#[cfg_attr(alef, alef(skip))]
/// The source of a document to extract.
#[derive(Debug, Clone)]
pub enum ExtractionSource {
    /// Extract from a filesystem path with an optional MIME type hint.
    File {
        /// Filesystem path to the document.
        path: PathBuf,
        /// Optional MIME type hint to skip content-based detection.
        mime_hint: Option<String>,
    },
    /// Extract from in-memory bytes with a known MIME type.
    Bytes {
        /// Raw document bytes.
        data: Bytes,
        /// MIME type of the in-memory document.
        mime_type: String,
    },
}
#[cfg_attr(alef, alef(skip))]
/// A request to extract content from a single document.
#[derive(Debug, Clone)]
pub struct ExtractionRequest {
    /// Where to read the document from.
    pub source: ExtractionSource,
    /// Base extraction configuration.
    pub config: ExtractionConfig,
    /// Optional per-file overrides (merged on top of `config`).
    pub file_overrides: Option<FileExtractionConfig>,
}

impl ExtractionRequest {
    /// Create a file-based extraction request.
    #[cfg(test)]
    pub(crate) fn file(path: impl Into<PathBuf>, config: ExtractionConfig) -> Self {
        Self {
            source: ExtractionSource::File {
                path: path.into(),
                mime_hint: None,
            },
            config,
            file_overrides: None,
        }
    }

    /// Create a bytes-based extraction request.
    pub(crate) fn bytes(data: impl Into<Bytes>, mime_type: impl Into<String>, config: ExtractionConfig) -> Self {
        Self {
            source: ExtractionSource::Bytes {
                data: data.into(),
                mime_type: mime_type.into(),
            },
            config,
            file_overrides: None,
        }
    }

    /// Set per-file overrides on this request.
    #[cfg(test)]
    pub(crate) fn with_overrides(mut self, overrides: FileExtractionConfig) -> Self {
        self.file_overrides = Some(overrides);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_creates_file_source() {
        let req = ExtractionRequest::file("/tmp/doc.pdf", ExtractionConfig::default());
        match &req.source {
            ExtractionSource::File { path, mime_hint } => {
                assert_eq!(path, &PathBuf::from("/tmp/doc.pdf"));
                assert!(mime_hint.is_none());
            }
            _ => panic!("expected File source"),
        }
        assert!(req.file_overrides.is_none());
    }

    #[test]
    fn bytes_creates_bytes_source() {
        let req = ExtractionRequest::bytes(b"hello".as_slice(), "text/plain", ExtractionConfig::default());
        match &req.source {
            ExtractionSource::Bytes { data, mime_type } => {
                assert_eq!(data.as_ref(), b"hello");
                assert_eq!(mime_type, "text/plain");
            }
            _ => panic!("expected Bytes source"),
        }
    }

    #[cfg(feature = "mcp")]
    #[test]
    fn with_overrides_sets_file_overrides() {
        let overrides = FileExtractionConfig::default();
        let req = ExtractionRequest::file("/tmp/doc.pdf", ExtractionConfig::default()).with_overrides(overrides);
        assert!(req.file_overrides.is_some());
    }
}

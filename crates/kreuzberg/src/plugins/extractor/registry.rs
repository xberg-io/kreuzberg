//! Document extractor registry management.
//!
//! This module provides functions for managing the global extractor registry.

use super::r#trait::DocumentExtractor;
use std::sync::Arc;

/// Register a document extractor with the global registry.
///
/// The extractor will be registered for all MIME types it supports and will be
/// available for document extraction. The extractor's `name()` method is used as
/// the registration name.
///
/// # Arguments
///
/// * `extractor` - The extractor implementation wrapped in Arc
///
/// # Returns
///
/// - `Ok(())` if registration succeeded
/// - `Err(...)` if validation failed or initialization failed
///
/// # Errors
///
/// - `KreuzbergError::Validation` - Invalid extractor name (empty or contains whitespace)
/// - Any error from the extractor's `initialize()` method
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::{Plugin, DocumentExtractor, register_extractor};
/// use kreuzberg::{Result, ExtractionConfig};
/// use kreuzberg::types::{ExtractionResult, Metadata};
/// use async_trait::async_trait;
/// use std::sync::Arc;
/// use std::path::Path;
///
/// struct CustomExtractor;
///
/// impl Plugin for CustomExtractor {
///     fn name(&self) -> &str { "custom-extractor" }
///     fn version(&self) -> String { "1.0.0".to_string() }
///     fn initialize(&self) -> Result<()> { Ok(()) }
///     fn shutdown(&self) -> Result<()> { Ok(()) }
/// }
///
/// #[async_trait]
/// impl DocumentExtractor for CustomExtractor {
///     async fn extract_bytes(&self, content: &[u8], mime_type: &str, _: &ExtractionConfig)
///         -> Result<ExtractionResult> {
///         Ok(ExtractionResult {
///             content: String::from_utf8_lossy(content).to_string(),
///             mime_type: mime_type.to_string().into(),
///             metadata: Metadata::default(),
///             tables: vec![],
///             detected_languages: None,
///             chunks: None,
///             images: None,
///             djot_content: None,
///             pages: None,
///             elements: None,
///             ocr_elements: None,
///             document: None,
///             quality_score: None,
///             processing_warnings: vec![],
///         })
///     }
///
///     fn supported_mime_types(&self) -> &[&str] {
///         &["text/custom"]
///     }
/// }
///
/// # tokio_test::block_on(async {
/// let extractor = Arc::new(CustomExtractor);
/// register_extractor(extractor)?;
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// # });
/// ```
pub fn register_extractor(extractor: Arc<dyn DocumentExtractor>) -> crate::Result<()> {
    use crate::plugins::registry::get_document_extractor_registry;

    let registry = get_document_extractor_registry();
    let mut registry = registry
        .write()
        .expect("~keep Failed to acquire write lock on extractor registry"); // ~keep

    registry.register(extractor)
}

/// Unregister a document extractor by name.
///
/// Removes the extractor from the global registry and calls its `shutdown()` method.
///
/// # Arguments
///
/// * `name` - Name of the extractor to unregister
///
/// # Returns
///
/// - `Ok(())` if the extractor was unregistered or didn't exist
/// - `Err(...)` if the shutdown method failed
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::unregister_extractor;
///
/// # tokio_test::block_on(async {
/// unregister_extractor("custom-extractor")?;
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// # });
/// ```
pub fn unregister_extractor(name: &str) -> crate::Result<()> {
    use crate::plugins::registry::get_document_extractor_registry;

    let registry = get_document_extractor_registry();
    let mut registry = registry
        .write()
        .expect("~keep Failed to acquire write lock on extractor registry"); // ~keep

    registry.remove(name)
}

/// List all registered extractors.
///
/// Returns the names of all extractors currently registered in the global registry.
///
/// # Returns
///
/// A vector of extractor names.
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::list_extractors;
///
/// # tokio_test::block_on(async {
/// let extractors = list_extractors()?;
/// for name in extractors {
///     println!("Registered extractor: {}", name);
/// }
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// # });
/// ```
pub fn list_extractors() -> crate::Result<Vec<String>> {
    use crate::plugins::registry::get_document_extractor_registry;

    let registry = get_document_extractor_registry();
    let registry = registry
        .read()
        .expect("~keep Failed to acquire read lock on extractor registry"); // ~keep

    Ok(registry.list())
}

/// Clear all extractors from the global registry.
///
/// Removes all extractors and calls their `shutdown()` methods.
///
/// # Returns
///
/// - `Ok(())` if all extractors were cleared successfully
/// - `Err(...)` if any shutdown method failed
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::clear_extractors;
///
/// # tokio_test::block_on(async {
/// clear_extractors()?;
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// # });
/// ```
pub fn clear_extractors() -> crate::Result<()> {
    use crate::plugins::registry::get_document_extractor_registry;

    let registry = get_document_extractor_registry();
    let mut registry = registry
        .write()
        .expect("~keep Failed to acquire write lock on extractor registry"); // ~keep

    registry.shutdown_all()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use crate::core::config::ExtractionConfig;
    use crate::plugins::Plugin;
    use crate::types::ExtractionResult;
    use async_trait::async_trait;
    use serial_test::serial;
    use std::borrow::Cow;

    struct MockExtractor {
        mime_types: Vec<&'static str>,
        priority: i32,
    }

    impl Plugin for MockExtractor {
        fn name(&self) -> &str {
            "mock-extractor"
        }

        fn version(&self) -> String {
            "1.0.0".to_string()
        }

        fn initialize(&self) -> Result<()> {
            Ok(())
        }

        fn shutdown(&self) -> Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl DocumentExtractor for MockExtractor {
        async fn extract_bytes(
            &self,
            content: &[u8],
            mime_type: &str,
            _config: &ExtractionConfig,
        ) -> Result<ExtractionResult> {
            Ok(ExtractionResult {
                content: String::from_utf8_lossy(content).to_string(),
                mime_type: mime_type.to_string().into(),
                metadata: crate::types::Metadata::default(),
                tables: vec![],
                detected_languages: None,
                chunks: None,
                images: None,
                djot_content: None,
                pages: None,
                elements: None,
                ocr_elements: None,
                document: None,
                #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
                extracted_keywords: None,
                quality_score: None,
                processing_warnings: Vec::new(),
                annotations: None,
            })
        }

        fn supported_mime_types(&self) -> &[&str] {
            &self.mime_types
        }

        fn priority(&self) -> i32 {
            self.priority
        }
    }

    #[test]
    #[serial]
    fn test_register_extractor() {
        use std::sync::Arc;

        let extractor = Arc::new(MockExtractor {
            mime_types: vec!["text/test-register"],
            priority: 50,
        });
        let result = super::register_extractor(extractor);
        assert!(result.is_ok());

        let _ = super::unregister_extractor("mock-extractor");
    }

    #[test]
    #[serial]
    fn test_unregister_extractor() {
        use std::sync::Arc;

        let extractor = Arc::new(MockExtractor {
            mime_types: vec!["text/test-unregister"],
            priority: 50,
        });
        super::register_extractor(extractor).unwrap();

        let result = super::unregister_extractor("mock-extractor");
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_unregister_nonexistent_extractor() {
        let result = super::unregister_extractor("nonexistent-extractor-xyz");
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn test_list_extractors() {
        use std::sync::Arc;

        super::clear_extractors().unwrap();

        let extractor1 = Arc::new(MockExtractor {
            mime_types: vec!["text/test-list-1"],
            priority: 50,
        });
        let extractor2 = Arc::new(MockExtractor {
            mime_types: vec!["text/test-list-2"],
            priority: 51,
        });

        let list_before = super::list_extractors().unwrap();
        assert_eq!(list_before.len(), 0);

        super::register_extractor(extractor1).unwrap();
        super::register_extractor(extractor2).unwrap();

        let list = super::list_extractors().unwrap();
        assert_eq!(list.len(), 1);
        assert!(list.contains(&"mock-extractor".to_string()));

        super::unregister_extractor("mock-extractor").unwrap();
    }

    #[test]
    #[serial]
    fn test_clear_extractors() {
        use std::sync::Arc;

        super::clear_extractors().unwrap();

        let extractor1 = Arc::new(MockExtractor {
            mime_types: vec!["text/test-clear-1"],
            priority: 50,
        });
        let extractor2 = Arc::new(MockExtractor {
            mime_types: vec!["text/test-clear-2"],
            priority: 51,
        });

        super::register_extractor(extractor1).unwrap();
        super::register_extractor(extractor2).unwrap();

        let result = super::clear_extractors();
        assert!(result.is_ok());

        let list = super::list_extractors().unwrap();
        assert_eq!(list.len(), 0);
    }

    #[test]
    #[serial]
    fn test_register_extractor_with_invalid_name() {
        use std::sync::Arc;

        struct InvalidNameExtractor;
        impl Plugin for InvalidNameExtractor {
            fn name(&self) -> &str {
                "invalid name with spaces"
            }
            fn version(&self) -> String {
                "1.0.0".to_string()
            }
            fn initialize(&self) -> Result<()> {
                Ok(())
            }
            fn shutdown(&self) -> Result<()> {
                Ok(())
            }
        }

        #[async_trait]
        impl DocumentExtractor for InvalidNameExtractor {
            async fn extract_bytes(&self, _: &[u8], _: &str, _: &ExtractionConfig) -> Result<ExtractionResult> {
                Ok(ExtractionResult {
                    content: String::new(),
                    mime_type: Cow::Borrowed(""),
                    metadata: crate::types::Metadata::default(),
                    tables: vec![],
                    detected_languages: None,
                    chunks: None,
                    images: None,
                    djot_content: None,
                    pages: None,
                    elements: None,
                    ocr_elements: None,
                    document: None,
                    #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
                    extracted_keywords: None,
                    quality_score: None,
                    processing_warnings: Vec::new(),
                    annotations: None,
                })
            }

            fn supported_mime_types(&self) -> &[&str] {
                &["text/plain"]
            }
        }

        let extractor = Arc::new(InvalidNameExtractor);
        let result = super::register_extractor(extractor);
        assert!(matches!(result, Err(crate::KreuzbergError::Validation { .. })));
    }

    #[test]
    #[serial]
    fn test_register_extractor_with_empty_name() {
        use std::sync::Arc;

        struct EmptyNameExtractor;
        impl Plugin for EmptyNameExtractor {
            fn name(&self) -> &str {
                ""
            }
            fn version(&self) -> String {
                "1.0.0".to_string()
            }
            fn initialize(&self) -> Result<()> {
                Ok(())
            }
            fn shutdown(&self) -> Result<()> {
                Ok(())
            }
        }

        #[async_trait]
        impl DocumentExtractor for EmptyNameExtractor {
            async fn extract_bytes(&self, _: &[u8], _: &str, _: &ExtractionConfig) -> Result<ExtractionResult> {
                Ok(ExtractionResult {
                    content: String::new(),
                    mime_type: Cow::Borrowed(""),
                    metadata: crate::types::Metadata::default(),
                    tables: vec![],
                    detected_languages: None,
                    chunks: None,
                    images: None,
                    djot_content: None,
                    pages: None,
                    elements: None,
                    ocr_elements: None,
                    document: None,
                    #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
                    extracted_keywords: None,
                    quality_score: None,
                    processing_warnings: Vec::new(),
                    annotations: None,
                })
            }

            fn supported_mime_types(&self) -> &[&str] {
                &["text/plain"]
            }
        }

        let extractor = Arc::new(EmptyNameExtractor);
        let result = super::register_extractor(extractor);
        assert!(matches!(result, Err(crate::KreuzbergError::Validation { .. })));
    }
}

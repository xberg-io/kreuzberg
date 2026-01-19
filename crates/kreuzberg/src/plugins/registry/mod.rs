//! Plugin registration and discovery.
//!
//! This module provides registries for managing plugins of different types.
//! Each plugin type (OcrBackend, DocumentExtractor, etc.) has its own registry
//! with type-safe registration and lookup.

mod extractor;
mod ocr;
mod processor;
mod validator;

pub use extractor::DocumentExtractorRegistry;
pub use ocr::OcrBackendRegistry;
pub use processor::PostProcessorRegistry;
pub use validator::ValidatorRegistry;

use crate::{KreuzbergError, Result};
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

/// Validate a plugin name before registration.
///
/// # Rules
///
/// - Name cannot be empty
/// - Name cannot contain whitespace
/// - Name should follow kebab-case convention (lowercase with hyphens)
///
/// # Errors
///
/// Returns `ValidationError` if the name is invalid.
pub(super) fn validate_plugin_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(KreuzbergError::Validation {
            message: "Plugin name cannot be empty".to_string(),
            source: None,
        });
    }

    if name.contains(char::is_whitespace) {
        return Err(KreuzbergError::Validation {
            message: format!("Plugin name '{}' cannot contain whitespace", name),
            source: None,
        });
    }

    Ok(())
}

/// Global OCR backend registry singleton.
pub static OCR_BACKEND_REGISTRY: Lazy<Arc<RwLock<OcrBackendRegistry>>> =
    Lazy::new(|| Arc::new(RwLock::new(OcrBackendRegistry::new())));

/// Global document extractor registry singleton.
pub static DOCUMENT_EXTRACTOR_REGISTRY: Lazy<Arc<RwLock<DocumentExtractorRegistry>>> =
    Lazy::new(|| Arc::new(RwLock::new(DocumentExtractorRegistry::new())));

/// Global post-processor registry singleton.
pub static POST_PROCESSOR_REGISTRY: Lazy<Arc<RwLock<PostProcessorRegistry>>> =
    Lazy::new(|| Arc::new(RwLock::new(PostProcessorRegistry::new())));

/// Global validator registry singleton.
pub static VALIDATOR_REGISTRY: Lazy<Arc<RwLock<ValidatorRegistry>>> =
    Lazy::new(|| Arc::new(RwLock::new(ValidatorRegistry::new())));

/// Get the global OCR backend registry.
pub fn get_ocr_backend_registry() -> Arc<RwLock<OcrBackendRegistry>> {
    OCR_BACKEND_REGISTRY.clone()
}

/// Get the global document extractor registry.
pub fn get_document_extractor_registry() -> Arc<RwLock<DocumentExtractorRegistry>> {
    DOCUMENT_EXTRACTOR_REGISTRY.clone()
}

/// Get the global post-processor registry.
pub fn get_post_processor_registry() -> Arc<RwLock<PostProcessorRegistry>> {
    POST_PROCESSOR_REGISTRY.clone()
}

/// Get the global validator registry.
pub fn get_validator_registry() -> Arc<RwLock<ValidatorRegistry>> {
    VALIDATOR_REGISTRY.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_registry_access() {
        let ocr_registry = get_ocr_backend_registry();
        let _ = ocr_registry
            .read()
            .expect("Failed to acquire read lock on OCR registry in test")
            .list();

        let extractor_registry = get_document_extractor_registry();
        let _ = extractor_registry
            .read()
            .expect("Failed to acquire read lock on extractor registry in test")
            .list();

        let processor_registry = get_post_processor_registry();
        let _ = processor_registry
            .read()
            .expect("Failed to acquire read lock on processor registry in test")
            .list();

        let validator_registry = get_validator_registry();
        let _ = validator_registry
            .read()
            .expect("Failed to acquire read lock on validator registry in test")
            .list();
    }
}

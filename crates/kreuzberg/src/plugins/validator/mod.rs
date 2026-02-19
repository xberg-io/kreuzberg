//! Validator plugin system.
//!
//! This module provides the trait and registry for implementing custom validators.

mod registry;
mod r#trait;

// Re-export trait for backward compatibility
pub use r#trait::Validator;

// Re-export registry functions for backward compatibility
pub use registry::{clear_validators, list_validators, register_validator, unregister_validator};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KreuzbergError;
    use crate::Result;
    use crate::core::config::ExtractionConfig;
    use crate::plugins::Plugin;
    use crate::types::ExtractionResult;
    use ahash::AHashMap;
    use async_trait::async_trait;
    use std::borrow::Cow;

    struct MockValidator {
        should_fail: bool,
    }

    impl Plugin for MockValidator {
        fn name(&self) -> &str {
            "mock-validator"
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
    impl Validator for MockValidator {
        async fn validate(&self, _result: &ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
            if self.should_fail {
                Err(KreuzbergError::validation("Validation failed".to_string()))
            } else {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn test_validator_success() {
        let validator = MockValidator { should_fail: false };

        let result = ExtractionResult {
            content: "test content".to_string(),
            mime_type: Cow::Borrowed("text/plain"),
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
        };

        let config = ExtractionConfig::default();
        assert!(validator.validate(&result, &config).await.is_ok());
    }

    #[tokio::test]
    async fn test_validator_failure() {
        let validator = MockValidator { should_fail: true };

        let result = ExtractionResult {
            content: "test content".to_string(),
            mime_type: Cow::Borrowed("text/plain"),
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
        };

        let config = ExtractionConfig::default();
        let validation_result = validator.validate(&result, &config).await;

        assert!(matches!(validation_result, Err(KreuzbergError::Validation { .. })));
    }

    #[test]
    fn test_validator_should_validate_default() {
        let validator = MockValidator { should_fail: false };

        let result = ExtractionResult {
            content: "test".to_string(),
            mime_type: Cow::Borrowed("text/plain"),
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
        };

        let config = ExtractionConfig::default();

        assert!(validator.should_validate(&result, &config));
    }

    #[test]
    fn test_validator_priority_default() {
        let validator = MockValidator { should_fail: false };
        assert_eq!(validator.priority(), 50);
    }

    #[tokio::test]
    async fn test_validator_plugin_interface() {
        let validator = MockValidator { should_fail: false };

        assert_eq!(validator.name(), "mock-validator");
        assert_eq!(validator.version(), "1.0.0");
        assert!(validator.initialize().is_ok());
        assert!(validator.shutdown().is_ok());
    }

    #[tokio::test]
    async fn test_validator_empty_content() {
        let validator = MockValidator { should_fail: false };

        let result = ExtractionResult {
            content: String::new(),
            mime_type: Cow::Borrowed("text/plain"),
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
        };

        let config = ExtractionConfig::default();
        assert!(validator.validate(&result, &config).await.is_ok());
    }

    #[test]
    fn test_validator_should_validate_conditional() {
        struct PdfOnlyValidator;

        impl Plugin for PdfOnlyValidator {
            fn name(&self) -> &str {
                "pdf-only"
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
        impl Validator for PdfOnlyValidator {
            async fn validate(&self, _result: &ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
                Ok(())
            }

            fn should_validate(&self, result: &ExtractionResult, _config: &ExtractionConfig) -> bool {
                result.mime_type == "application/pdf"
            }
        }

        let validator = PdfOnlyValidator;
        let config = ExtractionConfig::default();

        let pdf_result = ExtractionResult {
            content: "test".to_string(),
            mime_type: Cow::Borrowed("application/pdf"),
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
        };

        let txt_result = ExtractionResult {
            content: "test".to_string(),
            mime_type: Cow::Borrowed("text/plain"),
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
        };

        assert!(validator.should_validate(&pdf_result, &config));
        assert!(!validator.should_validate(&txt_result, &config));
    }

    #[test]
    fn test_validator_priority_ranges() {
        struct HighPriorityValidator;
        struct LowPriorityValidator;

        impl Plugin for HighPriorityValidator {
            fn name(&self) -> &str {
                "high-priority"
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

        impl Plugin for LowPriorityValidator {
            fn name(&self) -> &str {
                "low-priority"
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
        impl Validator for HighPriorityValidator {
            async fn validate(&self, _result: &ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
                Ok(())
            }

            fn priority(&self) -> i32 {
                100
            }
        }

        #[async_trait]
        impl Validator for LowPriorityValidator {
            async fn validate(&self, _result: &ExtractionResult, _config: &ExtractionConfig) -> Result<()> {
                Ok(())
            }

            fn priority(&self) -> i32 {
                10
            }
        }

        let high = HighPriorityValidator;
        let low = LowPriorityValidator;

        assert_eq!(high.priority(), 100);
        assert_eq!(low.priority(), 10);
        assert!(high.priority() > low.priority());
    }

    #[tokio::test]
    async fn test_validator_error_message() {
        let validator = MockValidator { should_fail: true };

        let result = ExtractionResult {
            content: "test".to_string(),
            mime_type: Cow::Borrowed("text/plain"),
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
        };

        let config = ExtractionConfig::default();
        let err = validator.validate(&result, &config).await.unwrap_err();

        match err {
            KreuzbergError::Validation { message: msg, .. } => {
                assert_eq!(msg, "Validation failed");
            }
            _ => panic!("Expected Validation error"),
        }
    }

    #[tokio::test]
    async fn test_validator_with_metadata() {
        let validator = MockValidator { should_fail: false };

        let mut additional = AHashMap::new();
        additional.insert(Cow::Borrowed("quality_score"), serde_json::json!(0.95));

        let result = ExtractionResult {
            content: "test".to_string(),
            mime_type: Cow::Borrowed("text/plain"),
            metadata: crate::types::Metadata {
                additional,
                ..Default::default()
            },
            pages: None,
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            elements: None,
            ocr_elements: None,
            document: None,
            #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
            extracted_keywords: None,
            quality_score: None,
            processing_warnings: Vec::new(),
            annotations: None,
        };

        let config = ExtractionConfig::default();
        assert!(validator.validate(&result, &config).await.is_ok());
    }

    #[tokio::test]
    async fn test_validator_with_tables() {
        use crate::types::Table;

        let validator = MockValidator { should_fail: false };

        let table = Table {
            cells: vec![vec!["A".to_string(), "B".to_string()]],
            markdown: "| A | B |".to_string(),
            page_number: 0,
            bounding_box: None,
        };

        let result = ExtractionResult {
            content: "test".to_string(),
            mime_type: Cow::Borrowed("text/plain"),
            metadata: crate::types::Metadata::default(),
            tables: vec![table],
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
        };

        let config = ExtractionConfig::default();
        assert!(validator.validate(&result, &config).await.is_ok());
    }

    #[tokio::test]
    async fn test_validator_different_mime_types() {
        let validator = MockValidator { should_fail: false };
        let config = ExtractionConfig::default();

        let mime_types = vec![
            "text/plain",
            "application/pdf",
            "application/json",
            "text/html",
            "image/png",
        ];

        for mime_type in mime_types {
            let result = ExtractionResult {
                content: "test".to_string(),
                mime_type: Cow::Borrowed(mime_type),
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
            };

            assert!(validator.validate(&result, &config).await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_validator_long_content() {
        let validator = MockValidator { should_fail: false };

        let result = ExtractionResult {
            content: "test content ".repeat(10000),
            mime_type: Cow::Borrowed("text/plain"),
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
        };

        let config = ExtractionConfig::default();
        assert!(validator.validate(&result, &config).await.is_ok());
    }
}

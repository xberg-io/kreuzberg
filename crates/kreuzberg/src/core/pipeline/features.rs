//! Feature processing logic.
//!
//! This module handles feature-specific processing like chunking,
//! embedding generation, and language detection.

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::types::{ExtractionResult, ProcessingWarning};
use std::borrow::Cow;

/// Execute chunking if configured.
pub(super) fn execute_chunking(result: &mut ExtractionResult, config: &ExtractionConfig) -> Result<()> {
    #[cfg(feature = "chunking")]
    if let Some(ref chunking_config) = config.chunking {
        let page_boundaries = result.metadata.pages.as_ref().and_then(|ps| ps.boundaries.as_deref());

        match crate::chunking::chunk_text(&result.content, chunking_config, page_boundaries) {
            Ok(chunking_result) => {
                result.chunks = Some(chunking_result.chunks);

                if let Some(ref chunks) = result.chunks {
                    // DEPRECATED: kept for backward compatibility; will be removed in next major version.
                    // chunk_count is derivable from result.chunks.len().
                    result.metadata.additional.insert(
                        Cow::Borrowed("chunk_count"),
                        serde_json::Value::Number(serde_json::Number::from(chunks.len())),
                    );
                }

                #[cfg(feature = "embeddings")]
                if let Some(ref embedding_config) = chunking_config.embedding
                    && let Some(ref mut chunks) = result.chunks
                {
                    match crate::embeddings::generate_embeddings_for_chunks(chunks, embedding_config) {
                        Ok(()) => {
                            // DEPRECATED: kept for backward compatibility; will be removed in next major version.
                            // embeddings_generated is derivable from result.chunks having non-None embeddings.
                            result
                                .metadata
                                .additional
                                .insert(Cow::Borrowed("embeddings_generated"), serde_json::Value::Bool(true));
                        }
                        Err(e) => {
                            tracing::warn!("Embedding generation failed: {e}. Check that ONNX Runtime is installed.");
                            let error_msg = e.to_string();
                            result.processing_warnings.push(ProcessingWarning {
                                source: "embedding".to_string(),
                                message: error_msg.clone(),
                            });
                            // DEPRECATED: kept for backward compatibility; will be removed in next major version.
                            result
                                .metadata
                                .additional
                                .insert(Cow::Borrowed("embedding_error"), serde_json::Value::String(error_msg));
                        }
                    }
                }

                #[cfg(not(feature = "embeddings"))]
                if chunking_config.embedding.is_some() {
                    tracing::warn!(
                        "Embedding config provided but embeddings feature is not enabled. Recompile with --features embeddings."
                    );
                    let error_msg = "Embeddings feature not enabled".to_string();
                    result.processing_warnings.push(ProcessingWarning {
                        source: "embedding".to_string(),
                        message: error_msg.clone(),
                    });
                    // DEPRECATED: kept for backward compatibility; will be removed in next major version.
                    result
                        .metadata
                        .additional
                        .insert(Cow::Borrowed("embedding_error"), serde_json::Value::String(error_msg));
                }
            }
            Err(e) => {
                let error_msg = e.to_string();
                result.processing_warnings.push(ProcessingWarning {
                    source: "chunking".to_string(),
                    message: error_msg.clone(),
                });
                // DEPRECATED: kept for backward compatibility; will be removed in next major version.
                result
                    .metadata
                    .additional
                    .insert(Cow::Borrowed("chunking_error"), serde_json::Value::String(error_msg));
            }
        }
    }

    #[cfg(not(feature = "chunking"))]
    if config.chunking.is_some() {
        let error_msg = "Chunking feature not enabled".to_string();
        result.processing_warnings.push(ProcessingWarning {
            source: "chunking".to_string(),
            message: error_msg.clone(),
        });
        // DEPRECATED: kept for backward compatibility; will be removed in next major version.
        result
            .metadata
            .additional
            .insert(Cow::Borrowed("chunking_error"), serde_json::Value::String(error_msg));
    }

    Ok(())
}

/// Execute language detection if configured.
pub(super) fn execute_language_detection(result: &mut ExtractionResult, config: &ExtractionConfig) -> Result<()> {
    #[cfg(feature = "language-detection")]
    if let Some(ref lang_config) = config.language_detection {
        match crate::language_detection::detect_languages(&result.content, lang_config) {
            Ok(detected) => {
                result.detected_languages = detected;
            }
            Err(e) => {
                let error_msg = e.to_string();
                result.processing_warnings.push(ProcessingWarning {
                    source: "language_detection".to_string(),
                    message: error_msg.clone(),
                });
                // DEPRECATED: kept for backward compatibility; will be removed in next major version.
                result.metadata.additional.insert(
                    Cow::Borrowed("language_detection_error"),
                    serde_json::Value::String(error_msg),
                );
            }
        }
    }

    #[cfg(not(feature = "language-detection"))]
    if config.language_detection.is_some() {
        let error_msg = "Language detection feature not enabled".to_string();
        result.processing_warnings.push(ProcessingWarning {
            source: "language_detection".to_string(),
            message: error_msg.clone(),
        });
        // DEPRECATED: kept for backward compatibility; will be removed in next major version.
        result.metadata.additional.insert(
            Cow::Borrowed("language_detection_error"),
            serde_json::Value::String(error_msg),
        );
    }

    Ok(())
}

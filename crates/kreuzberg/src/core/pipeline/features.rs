//! Feature processing logic.
//!
//! This module handles feature-specific processing like chunking,
//! embedding generation, and language detection.

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::types::ExtractionResult;

/// Execute chunking if configured.
pub(super) fn execute_chunking(result: &mut ExtractionResult, config: &ExtractionConfig) -> Result<()> {
    #[cfg(feature = "chunking")]
    if let Some(ref chunking_config) = config.chunking {
        let chunk_config = crate::chunking::ChunkingConfig {
            max_characters: chunking_config.max_chars,
            overlap: chunking_config.max_overlap,
            trim: true,
            chunker_type: crate::chunking::ChunkerType::Text,
        };

        let page_boundaries = result.metadata.pages.as_ref().and_then(|ps| ps.boundaries.as_deref());

        match crate::chunking::chunk_text(&result.content, &chunk_config, page_boundaries) {
            Ok(chunking_result) => {
                result.chunks = Some(chunking_result.chunks);

                if let Some(ref chunks) = result.chunks {
                    result.metadata.additional.insert(
                        "chunk_count".to_string(),
                        serde_json::Value::Number(serde_json::Number::from(chunks.len())),
                    );
                }

                #[cfg(feature = "embeddings")]
                if let Some(ref embedding_config) = chunking_config.embedding
                    && let Some(ref mut chunks) = result.chunks
                {
                    match crate::embeddings::generate_embeddings_for_chunks(chunks, embedding_config) {
                        Ok(()) => {
                            result
                                .metadata
                                .additional
                                .insert("embeddings_generated".to_string(), serde_json::Value::Bool(true));
                        }
                        Err(e) => {
                            result
                                .metadata
                                .additional
                                .insert("embedding_error".to_string(), serde_json::Value::String(e.to_string()));
                        }
                    }
                }

                #[cfg(not(feature = "embeddings"))]
                if chunking_config.embedding.is_some() {
                    result.metadata.additional.insert(
                        "embedding_error".to_string(),
                        serde_json::Value::String("Embeddings feature not enabled".to_string()),
                    );
                }
            }
            Err(e) => {
                result
                    .metadata
                    .additional
                    .insert("chunking_error".to_string(), serde_json::Value::String(e.to_string()));
            }
        }
    }

    #[cfg(not(feature = "chunking"))]
    if config.chunking.is_some() {
        result.metadata.additional.insert(
            "chunking_error".to_string(),
            serde_json::Value::String("Chunking feature not enabled".to_string()),
        );
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
                result.metadata.additional.insert(
                    "language_detection_error".to_string(),
                    serde_json::Value::String(e.to_string()),
                );
            }
        }
    }

    #[cfg(not(feature = "language-detection"))]
    if config.language_detection.is_some() {
        result.metadata.additional.insert(
            "language_detection_error".to_string(),
            serde_json::Value::String("Language detection feature not enabled".to_string()),
        );
    }

    Ok(())
}

//! Post-processing and chunking configuration.
//!
//! Defines configuration for post-processing pipelines, text chunking,
//! and embedding generation.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;

/// Post-processor configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostProcessorConfig {
    /// Enable post-processors
    #[serde(default = "default_true")]
    pub enabled: bool,

    /// Whitelist of processor names to run (None = all enabled)
    #[serde(default)]
    pub enabled_processors: Option<Vec<String>>,

    /// Blacklist of processor names to skip (None = none disabled)
    #[serde(default)]
    pub disabled_processors: Option<Vec<String>>,

    /// Pre-computed HashSet for O(1) enabled processor lookup
    #[serde(skip)]
    pub enabled_set: Option<HashSet<String>>,

    /// Pre-computed HashSet for O(1) disabled processor lookup
    #[serde(skip)]
    pub disabled_set: Option<HashSet<String>>,
}

impl PostProcessorConfig {
    /// Pre-compute HashSets for O(1) processor name lookups.
    ///
    /// This method converts the enabled/disabled processor Vec to HashSet
    /// for constant-time lookups in the pipeline.
    pub fn build_lookup_sets(&mut self) {
        if let Some(ref enabled) = self.enabled_processors {
            self.enabled_set = Some(enabled.iter().cloned().collect());
        }
        if let Some(ref disabled) = self.disabled_processors {
            self.disabled_set = Some(disabled.iter().cloned().collect());
        }
    }
}

impl Default for PostProcessorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            enabled_processors: None,
            disabled_processors: None,
            enabled_set: None,
            disabled_set: None,
        }
    }
}

/// Chunking configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkingConfig {
    /// Maximum characters per chunk
    #[serde(default = "default_chunk_size")]
    pub max_chars: usize,

    /// Overlap between chunks in characters
    #[serde(default = "default_chunk_overlap")]
    pub max_overlap: usize,

    /// Optional embedding configuration for chunk embeddings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<EmbeddingConfig>,

    /// Use a preset configuration (overrides individual settings if provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
}

/// Embedding configuration for text chunks.
///
/// Configures embedding generation using ONNX models via fastembed-rs.
/// Requires the `embeddings` feature to be enabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// The embedding model to use
    pub model: EmbeddingModelType,

    /// Whether to normalize embedding vectors (recommended for cosine similarity)
    #[serde(default = "default_normalize")]
    pub normalize: bool,

    /// Batch size for embedding generation
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,

    /// Show model download progress
    #[serde(default)]
    pub show_download_progress: bool,

    /// Custom cache directory for model files
    ///
    /// Defaults to `~/.cache/kreuzberg/embeddings/` if not specified.
    /// Allows full customization of model download location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_dir: Option<PathBuf>,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            model: EmbeddingModelType::Preset {
                name: "balanced".to_string(),
            },
            normalize: true,
            batch_size: 32,
            show_download_progress: false,
            cache_dir: None,
        }
    }
}

/// Embedding model types supported by Kreuzberg.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EmbeddingModelType {
    /// Use a preset model configuration (recommended)
    Preset { name: String },

    /// Use a specific fastembed model by name
    #[cfg(feature = "embeddings")]
    FastEmbed { model: String, dimensions: usize },

    /// Use a custom ONNX model from HuggingFace
    Custom { model_id: String, dimensions: usize },
}

fn default_true() -> bool {
    true
}

fn default_chunk_size() -> usize {
    1000
}

fn default_chunk_overlap() -> usize {
    200
}

fn default_normalize() -> bool {
    true
}

fn default_batch_size() -> usize {
    32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postprocessor_config_default() {
        let config = PostProcessorConfig::default();
        assert!(config.enabled);
        assert!(config.enabled_processors.is_none());
        assert!(config.disabled_processors.is_none());
    }

    #[test]
    fn test_postprocessor_config_build_lookup_sets() {
        let mut config = PostProcessorConfig {
            enabled: true,
            enabled_processors: Some(vec!["a".to_string(), "b".to_string()]),
            disabled_processors: Some(vec!["c".to_string()]),
            enabled_set: None,
            disabled_set: None,
        };

        config.build_lookup_sets();

        assert!(config.enabled_set.is_some());
        assert!(config.disabled_set.is_some());
        assert!(config.enabled_set.unwrap().contains("a"));
        assert!(config.disabled_set.unwrap().contains("c"));
    }

    #[test]
    fn test_chunking_config_defaults() {
        let config = ChunkingConfig {
            max_chars: 1000,
            max_overlap: 200,
            embedding: None,
            preset: None,
        };
        assert_eq!(config.max_chars, 1000);
        assert_eq!(config.max_overlap, 200);
    }

    #[test]
    fn test_embedding_config_default() {
        let config = EmbeddingConfig::default();
        assert!(config.normalize);
        assert_eq!(config.batch_size, 32);
        assert!(config.cache_dir.is_none());
    }
}

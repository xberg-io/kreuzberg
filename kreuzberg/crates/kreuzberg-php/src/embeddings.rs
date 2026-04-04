//! Embedding preset functions for PHP bindings
//!
//! Provides functions to list and retrieve embedding model presets.

use ext_php_rs::prelude::*;

/// Embedding preset configuration.
///
/// Contains all settings for a specific embedding model preset including chunk size,
/// overlap, model name, embedding dimensions, and description.
///
/// # Properties
///
/// - `name` (string): Name of the preset
/// - `chunk_size` (int): Recommended chunk size in characters
/// - `overlap` (int): Recommended overlap in characters
/// - `model_name` (string): Model identifier
/// - `dimensions` (int): Embedding vector dimensions
/// - `description` (string): Human-readable description
///
/// # Example
///
/// ```php
/// $preset = kreuzberg_get_embedding_preset("balanced");
/// echo "Model: {$preset->model_name}, Dims: {$preset->dimensions}\n";
/// ```
#[php_class]
#[php(name = "Kreuzberg\\Embeddings\\EmbeddingPreset")]
#[derive(Clone)]
pub struct EmbeddingPreset {
    #[php(prop)]
    pub name: String,
    #[php(prop)]
    pub chunk_size: i64,
    #[php(prop)]
    pub overlap: i64,
    #[php(prop)]
    pub model_name: String,
    #[php(prop)]
    pub dimensions: i64,
    #[php(prop)]
    pub description: String,
}

#[php_impl]
impl EmbeddingPreset {}

/// List all available embedding preset names.
///
/// Returns an array of preset names that can be used with kreuzberg_get_embedding_preset().
///
/// # Returns
///
/// Array of preset names
///
/// # Available Presets
///
/// - "fast": AllMiniLML6V2Q (384 dimensions) - Quick prototyping, low-latency
/// - "balanced": BGEBaseENV15 (768 dimensions) - General-purpose RAG
/// - "quality": BGELargeENV15 (1024 dimensions) - High-quality embeddings
/// - "multilingual": MultilingualE5Base (768 dimensions) - Multi-language support
///
/// # Example
///
/// ```php
/// $presets = kreuzberg_list_embedding_presets();
/// print_r($presets); // ["fast", "balanced", "quality", "multilingual"]
/// ```
#[php_function]
pub fn kreuzberg_list_embedding_presets() -> Vec<String> {
    kreuzberg::embeddings::list_presets()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

/// Get a specific embedding preset by name.
///
/// Returns a preset configuration object, or NULL if the preset name is not found.
///
/// # Parameters
///
/// - `name` (string): The preset name (case-sensitive)
///
/// # Returns
///
/// EmbeddingPreset object or NULL if not found
///
/// # Example
///
/// ```php
/// $preset = kreuzberg_get_embedding_preset("balanced");
/// if ($preset !== null) {
///     echo "Model: {$preset->model_name}\n";
///     echo "Dimensions: {$preset->dimensions}\n";
///     echo "Chunk size: {$preset->chunk_size}\n";
/// }
/// ```
#[php_function]
pub fn kreuzberg_get_embedding_preset(name: String) -> Option<EmbeddingPreset> {
    let preset = kreuzberg::embeddings::get_preset(&name)?;

    let model_name = preset.model_repo.to_string();

    Some(EmbeddingPreset {
        name: preset.name.to_string(),
        chunk_size: preset.chunk_size as i64,
        overlap: preset.overlap as i64,
        model_name,
        dimensions: preset.dimensions as i64,
        description: preset.description.to_string(),
    })
}

/// Returns all function builders for the embeddings module.
pub fn get_function_builders() -> Vec<ext_php_rs::builders::FunctionBuilder<'static>> {
    vec![
        wrap_function!(kreuzberg_list_embedding_presets),
        wrap_function!(kreuzberg_get_embedding_preset),
    ]
}

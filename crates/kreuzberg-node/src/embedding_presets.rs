use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct EmbeddingPreset {
    /// Name of the preset (e.g., "fast", "balanced", "quality", "multilingual")
    pub name: String,
    /// Recommended chunk size in characters
    pub chunk_size: u32,
    /// Recommended overlap in characters
    pub overlap: u32,
    /// Model identifier (e.g., "AllMiniLML6V2Q", "BGEBaseENV15")
    pub model_name: String,
    /// Embedding vector dimensions
    pub dimensions: u32,
    /// Human-readable description of the preset
    pub description: String,
}

/// List all available embedding preset names.
///
/// Returns an array of preset names that can be used with `getEmbeddingPreset`.
///
/// # Returns
///
/// Array of 4 preset names: ["fast", "balanced", "quality", "multilingual"]
///
/// # Example
///
/// ```typescript
/// import { listEmbeddingPresets } from 'kreuzberg';
///
/// const presets = listEmbeddingPresets();
/// console.log(presets); // ['fast', 'balanced', 'quality', 'multilingual']
/// ```
#[napi(js_name = "listEmbeddingPresets")]
pub fn list_embedding_presets() -> Vec<String> {
    kreuzberg::embeddings::list_presets()
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

/// Get a specific embedding preset by name.
///
/// Returns a preset configuration object, or null if the preset name is not found.
///
/// # Arguments
///
/// * `name` - The preset name (case-sensitive)
///
/// # Returns
///
/// An `EmbeddingPreset` object with the following properties:
/// - `name`: string - Preset name
/// - `chunkSize`: number - Recommended chunk size in characters
/// - `overlap`: number - Recommended overlap in characters
/// - `modelName`: string - Model identifier
/// - `dimensions`: number - Embedding vector dimensions
/// - `description`: string - Human-readable description
///
/// Returns `null` if preset name is not found.
///
/// # Example
///
/// ```typescript
/// import { getEmbeddingPreset } from 'kreuzberg';
///
/// const preset = getEmbeddingPreset('balanced');
/// if (preset) {
///   console.log(`Model: ${preset.modelName}, Dims: ${preset.dimensions}`);
///   // Model: BGEBaseENV15, Dims: 768
/// }
/// ```
#[napi(js_name = "getEmbeddingPreset")]
pub fn get_embedding_preset(name: String) -> Option<EmbeddingPreset> {
    let preset = kreuzberg::embeddings::get_preset(&name)?;

    let model_name = format!("{:?}", preset.model);

    Some(EmbeddingPreset {
        name: preset.name.to_string(),
        chunk_size: preset.chunk_size as u32,
        overlap: preset.overlap as u32,
        model_name,
        dimensions: preset.dimensions as u32,
        description: preset.description.to_string(),
    })
}

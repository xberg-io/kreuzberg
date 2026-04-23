//! Provider-hosted embeddings via liter-llm.
//!
//! Generates text embeddings using cloud-hosted models (e.g., OpenAI
//! `text-embedding-3-small`, Cohere `embed-english-v3.0`) through the
//! liter-llm client.  This is an alternative to local ONNX-based embeddings
//! and is useful when a provider-hosted model is preferred or when ONNX
//! Runtime is not available.

use liter_llm::{EmbeddingInput, EmbeddingRequest, LlmClient};

use crate::core::config::LlmConfig;

/// Generate embeddings using a provider-hosted model via liter-llm.
///
/// Sends the input texts to a remote embedding model and returns one
/// embedding vector per input text, in the same order as the input.
///
/// # Arguments
///
/// * `texts` - Slice of strings to embed (must all be non-empty)
/// * `config` - LLM provider/model configuration
/// * `normalize` - Whether to L2-normalize the resulting vectors
///
/// # Returns
///
/// `Vec<Vec<f32>>` with one embedding per input text.
///
/// # Errors
///
/// - `KreuzbergError::Embedding` if the API call fails or returns unexpected data
/// - `KreuzbergError::MissingDependency` if the liter-llm client cannot be created
pub(crate) async fn embed_via_llm<T: AsRef<str>>(
    texts: &[T],
    config: &LlmConfig,
    normalize: bool,
) -> crate::Result<(Vec<Vec<f32>>, Option<crate::types::LlmUsage>)> {
    if texts.is_empty() {
        return Ok((Vec::new(), None));
    }

    let client = super::client::create_client(config)?;

    // Build the embedding request with all texts.
    let input_strings: Vec<String> = texts.iter().map(|t| t.as_ref().to_string()).collect();
    let input = if input_strings.len() == 1 {
        EmbeddingInput::Single(input_strings.into_iter().next().expect("checked non-empty"))
    } else {
        EmbeddingInput::Multiple(input_strings)
    };

    let request = EmbeddingRequest {
        model: config.model.clone(),
        input,
        encoding_format: None,
        dimensions: None,
        user: None,
    };

    let response = client.embed(request).await.map_err(|e| {
        crate::KreuzbergError::embedding(format!("LLM embedding request failed (model={}): {e}", config.model))
    })?;

    let usage = super::usage::extract_usage_from_embedding(&response, "embeddings");

    // Sort by index to guarantee order matches input order.
    let mut data = response.data;
    data.sort_by_key(|obj| obj.index);

    // Convert f64 embeddings from liter-llm to f32 for kreuzberg's embedding type.
    let mut embeddings: Vec<Vec<f32>> = data
        .into_iter()
        .map(|obj| obj.embedding.into_iter().map(|v| v as f32).collect())
        .collect();

    if normalize {
        for embedding in &mut embeddings {
            normalize_l2(embedding);
        }
    }

    Ok((embeddings, usage))
}

/// L2-normalize an embedding vector in-place.
fn normalize_l2(embedding: &mut [f32]) {
    let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if magnitude > f32::EPSILON {
        let inv_mag = 1.0 / magnitude;
        embedding.iter_mut().for_each(|x| *x *= inv_mag);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_l2_unit_vector() {
        let mut v = vec![1.0f32, 0.0, 0.0];
        normalize_l2(&mut v);
        assert!((v[0] - 1.0).abs() < f32::EPSILON);
        assert!((v[1]).abs() < f32::EPSILON);
    }

    #[test]
    fn test_normalize_l2_arbitrary_vector() {
        let mut v = vec![3.0f32, 4.0];
        normalize_l2(&mut v);
        let magnitude: f32 = v.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_l2_zero_vector() {
        let mut v = vec![0.0f32, 0.0, 0.0];
        normalize_l2(&mut v);
        // Zero vector should remain zero (no division by zero).
        assert!(v.iter().all(|&x| x == 0.0));
    }
}

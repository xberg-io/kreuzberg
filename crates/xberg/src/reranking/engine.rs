//! Cross-encoder ONNX inference engine.
//!
//! Core inference pipeline for ONNX-based cross-encoder reranking.
//! Key design: `rerank()` takes `&self` instead of `&mut self`, enabling
//! concurrent inference from multiple threads without mutex contention.
//!
//! This is safe because `ort::Session::run()` takes `&mut self` purely as
//! an API constraint — its internal `run_inner()` takes `&self`, and the
//! ONNX Runtime C API (`OrtApi::Run`) is documented as thread-safe for
//! concurrent calls on the same session.
//!
//! Mirrors `crates/xberg/src/embeddings/engine.rs` with three changes:
//! - Tokenizer encodes `(query, document)` pairs via `EncodeInput::Dual`.
//! - Output is `[batch, 1]` or `[batch]` logits — squeezed to `Vec<f32>`.
//! - No pooling step — cross-encoders pool internally.
//!
//! Since v5.0.0.

use ndarray::{ArrayView, Dim, Dimension, IxDynImpl, s};
use ort::session::Session;
use ort::value::Value;
use thiserror::Error;
use tokenizers::{EncodeInput, InputSequence, Tokenizer};

/// Errors that can occur during cross-encoder reranking inference.
///
/// Since v5.0.0.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Error)]
pub enum RerankError {
    /// Tokenization failed with the given message.
    #[error("Tokenizer error: {0}")]
    Tokenizer(String),
    /// ONNX Runtime returned an error during inference.
    #[error("ONNX Runtime error: {0}")]
    Ort(#[from] ort::Error),
    /// The model output tensor had an unexpected shape.
    #[error("Tensor shape error: {0}")]
    Shape(String),
    /// The model produced no output tensors.
    #[error("Model produced no output tensors")]
    NoOutput,
}

/// Cross-encoder reranking model with thread-safe inference.
///
/// The `rerank()` method takes `&self` instead of `&mut self`, allowing it to
/// be shared across threads via `Arc<RerankerEngine>` without mutex contention.
///
/// Since v5.0.0.
#[cfg_attr(alef, alef(skip))]
pub struct RerankerEngine {
    tokenizer: Tokenizer,
    session: Session,
    need_token_type_ids: bool,
}

impl RerankerEngine {
    /// Create a new reranker engine from a pre-built session and tokenizer.
    pub(crate) fn new(tokenizer: Tokenizer, session: Session) -> Self {
        let need_token_type_ids = session.inputs().iter().any(|input| input.name() == "token_type_ids");
        Self {
            tokenizer,
            session,
            need_token_type_ids,
        }
    }

    /// Score a batch of `(query, document)` pairs.
    ///
    /// Returns one logit per pair in the same order as the input.
    /// Apply sigmoid to convert logits to `[0, 1]` scores.
    ///
    /// This method is **thread-safe** — multiple threads can call `rerank()`
    /// concurrently on the same `RerankerEngine` instance.
    ///
    /// # Safety note
    ///
    /// Uses an internal unsafe cast because `ort::Session::run()` takes
    /// `&mut self` despite performing no mutation (its `run_inner()` takes
    /// `&self`). The ONNX Runtime C API is documented as thread-safe for
    /// concurrent `Run()` calls on the same session.
    pub(crate) fn rerank(&self, query: &str, documents: &[&str], batch_size: usize) -> Result<Vec<f32>, RerankError> {
        if documents.is_empty() {
            return Ok(Vec::new());
        }

        // Defensive: callers from polyglot bindings may pass batch_size=0 when the
        // host-side `RerankerConfig` mirror omits the serde default.
        let batch_size = if batch_size == 0 { 32 } else { batch_size };

        let mut all_scores = Vec::with_capacity(documents.len());

        for batch in documents.chunks(batch_size) {
            let batch_scores = self.rerank_batch(query, batch)?;
            all_scores.extend(batch_scores);
        }

        Ok(all_scores)
    }

    /// Score a single batch of `(query, document)` pairs.
    fn rerank_batch(&self, query: &str, documents: &[&str]) -> Result<Vec<f32>, RerankError> {
        // Encode (query, document) pairs using the tokenizer's pair-encoding API.
        let pairs: Vec<EncodeInput<'_>> = documents
            .iter()
            .map(|doc| {
                EncodeInput::Dual(
                    InputSequence::Raw(std::borrow::Cow::Borrowed(query)),
                    InputSequence::Raw(std::borrow::Cow::Borrowed(doc)),
                )
            })
            .collect();

        let encodings = self
            .tokenizer
            .encode_batch(pairs, true)
            .map_err(|e| RerankError::Tokenizer(e.to_string()))?;

        let encoding_length = encodings
            .first()
            .ok_or_else(|| RerankError::Tokenizer("Empty encodings".to_string()))?
            .len();
        let batch_size = documents.len();
        let max_size = encoding_length * batch_size;

        // Build input tensors.
        let mut ids_array = Vec::with_capacity(max_size);
        let mut mask_array = Vec::with_capacity(max_size);
        let mut type_ids_array = Vec::with_capacity(max_size);

        for encoding in &encodings {
            ids_array.extend(encoding.get_ids().iter().map(|&x| x as i64));
            mask_array.extend(encoding.get_attention_mask().iter().map(|&x| x as i64));
            type_ids_array.extend(encoding.get_type_ids().iter().map(|&x| x as i64));
        }

        let ids_tensor = ndarray::Array::from_shape_vec((batch_size, encoding_length), ids_array)
            .map_err(|e| RerankError::Shape(e.to_string()))?;
        let type_ids_tensor = ndarray::Array::from_shape_vec((batch_size, encoding_length), type_ids_array)
            .map_err(|e| RerankError::Shape(e.to_string()))?;
        let mask_tensor = ndarray::Array::from_shape_vec((batch_size, encoding_length), mask_array)
            .map_err(|e| RerankError::Shape(e.to_string()))?;

        let mut session_inputs = ort::inputs![
            "input_ids" => Value::from_array(ids_tensor)?,
            "attention_mask" => Value::from_array(mask_tensor)?,
        ];

        if self.need_token_type_ids {
            session_inputs.push(("token_type_ids".into(), Value::from_array(type_ids_tensor)?.into()));
        }

        // Run inference — thread-safe despite &mut self signature on Session::run()
        //
        // SAFETY: ort::Session::run() takes &mut self but delegates to run_inner(&self)
        // with zero actual mutation. The ONNX Runtime C API (OrtApi::Run) is documented
        // as thread-safe for concurrent Run() calls on the same session.
        #[allow(unsafe_code)]
        let outputs = unsafe {
            let session_ptr = &self.session as *const Session as *mut Session;
            (*session_ptr).run(session_inputs)
        }
        .map_err(RerankError::Ort)?;

        // Extract the logit output tensor.
        let (_, output_value) = outputs.iter().next().ok_or(RerankError::NoOutput)?;
        let tensor: ArrayView<f32, Dim<IxDynImpl>> = output_value.try_extract_array().map_err(RerankError::Ort)?;

        // Squeeze [batch, 1] or [batch] to Vec<f32>.
        // Cross-encoders typically output [batch, 1]; squeeze to [batch].
        let scores = match tensor.dim().ndim() {
            1 => tensor.slice(s![..]).iter().copied().collect(),
            2 => tensor.slice(s![.., 0]).iter().copied().collect(),
            n => return Err(RerankError::Shape(format!("Expected 1D or 2D output tensor, got {n}D"))),
        };

        Ok(scores)
    }
}

// SAFETY: RerankerEngine is Send + Sync because:
// 1. Tokenizer is Send + Sync (confirmed in tokenizers crate)
// 2. Session: we only call run() which is internally thread-safe per ONNX Runtime docs
// 3. All other fields are immutable after construction
#[allow(unsafe_code)]
unsafe impl Send for RerankerEngine {}
#[allow(unsafe_code)]
unsafe impl Sync for RerankerEngine {}

#[cfg(test)]
mod tests {
    use super::*;
    // Exercise the production sigmoid, not a local copy — keeps engine tests
    // honest if the mod-level sigmoid ever changes.
    use super::super::sigmoid_f32 as sigmoid;

    #[test]
    fn sigmoid_zero_gives_half() {
        let s = sigmoid(0.0);
        assert!((s - 0.5).abs() < 1e-6, "sigmoid(0) should be 0.5, got {s}");
    }

    #[test]
    fn sigmoid_large_positive_approaches_one() {
        let s = sigmoid(100.0);
        assert!(s > 0.99, "sigmoid(100) should be close to 1.0, got {s}");
    }

    #[test]
    fn sigmoid_large_negative_approaches_zero() {
        let s = sigmoid(-100.0);
        assert!(s < 0.01, "sigmoid(-100) should be close to 0.0, got {s}");
    }

    #[test]
    fn rerank_error_display_does_not_panic() {
        let err = RerankError::Tokenizer("test".to_string());
        assert!(format!("{err}").contains("Tokenizer"));

        let err = RerankError::Shape("bad shape".to_string());
        assert!(format!("{err}").contains("shape"));

        let err = RerankError::NoOutput;
        assert!(format!("{err}").contains("no output"));
    }

    #[test]
    fn rerank_error_implements_error_trait() {
        let err = RerankError::Shape("test".to_string());
        let _: &dyn std::error::Error = &err;
    }
}

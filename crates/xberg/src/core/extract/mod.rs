//! Unified public extraction API.
//!
//! These functions are the stable, binding-generated public surface. Their
//! signatures must remain byte-identical (they are scanned by the alef binding
//! generator). The implementation delegates to a process-global default
//! [`crate::engine::Engine`]; the extraction internals live in
//! [`crate::engine`] and are a pure refactor of what previously lived here.

use std::sync::LazyLock;

use crate::Result;
use crate::core::config::{ExtractInput, ExtractionConfig, ExtractionResult};

/// Process-global default engine backing the free `extract` / `extract_batch`
/// functions. Construction is cheap and side-effect free.
static DEFAULT_ENGINE: LazyLock<crate::engine::Engine> = LazyLock::new(crate::engine::Engine::new_default);

/// Extract content from a single bytes or URI input.
pub async fn extract(input: ExtractInput, config: &ExtractionConfig) -> Result<ExtractionResult> {
    DEFAULT_ENGINE.extract(input, config).await
}

/// Extract content from multiple bytes or URI inputs.
pub async fn extract_batch(inputs: Vec<ExtractInput>, config: &ExtractionConfig) -> Result<ExtractionResult> {
    DEFAULT_ENGINE.extract_batch(inputs, config).await
}

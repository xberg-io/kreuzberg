//! Rust-only extraction engine.
//!
//! [`Engine`] owns the extraction internals that previously lived as free
//! functions in [`crate::core::extract`]. The crate-level [`crate::extract`]
//! and [`crate::extract_batch`] functions delegate to a process-global default
//! [`Engine`]. This is a pure refactor: behavior is identical to the previous
//! free-function implementation.
//!
//! This module is intentionally **not** part of the language-binding surface.
//! It is declared with a bare `pub mod engine;` in `lib.rs` and its files are
//! not listed in `alef.toml` `sources`, so the binding generator emits nothing
//! for it. The public types here are also listed in `alef.toml`
//! `[crates.exclude] types` as belt-and-suspenders.

use std::sync::Arc;

use crate::Result;
use crate::core::config::{ExtractInput, ExtractionConfig, ExtractionResult};

mod crawl_handle;
mod extract_impl;

/// Internal engine state.
///
/// Holds the process-shared, fingerprinted crawl-engine memo so that multi-URL
/// batch extraction can reuse a single [`crawlberg::CrawlEngine`] (and its
/// shared middleware/cache/rate-limiter) across all URLs in a batch. The
/// single-URL `extract` path does not touch this state.
#[derive(Default)]
struct EngineInner {
    /// Single-slot, fingerprinted memo of the last-built crawl engine. The slot
    /// is reused when the incoming [`crawlberg::CrawlConfig`] fingerprint
    /// matches, otherwise a fresh engine is built and stored.
    #[cfg(feature = "url-ingestion")]
    crawl: parking_lot::Mutex<Option<crawl_handle::CrawlHandleMemo>>,
}

/// A reusable, cheaply-cloneable extraction engine.
///
/// Cloning an [`Engine`] shares the same underlying state via [`Arc`].
#[derive(Clone)]
pub struct Engine {
    inner: Arc<EngineInner>,
}

impl Engine {
    /// Start building an [`Engine`].
    pub fn builder() -> EngineBuilder {
        EngineBuilder::default()
    }

    /// Construct an [`Engine`] with default configuration.
    pub fn new_default() -> Self {
        EngineBuilder::default().build()
    }

    /// Extract content from a single bytes or URI input.
    pub async fn extract(&self, input: ExtractInput, config: &ExtractionConfig) -> Result<ExtractionResult> {
        extract_impl::extract(input, config).await
    }

    /// Extract content from multiple bytes or URI inputs.
    pub async fn extract_batch(
        &self,
        inputs: Vec<ExtractInput>,
        config: &ExtractionConfig,
    ) -> Result<ExtractionResult> {
        extract_impl::extract_batch(&self.inner, inputs, config).await
    }
}

/// Builder for [`Engine`].
#[derive(Default)]
pub struct EngineBuilder {}

impl EngineBuilder {
    /// Finalize the builder into an [`Engine`].
    pub fn build(self) -> Engine {
        Engine {
            inner: Arc::new(EngineInner::default()),
        }
    }
}

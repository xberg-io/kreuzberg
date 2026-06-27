//! Instrumented extractor wrapper for automatic telemetry.
//!
//! Wraps any native extractor capability to add tracing spans and metrics
//! without requiring per-extractor instrumentation annotations.

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::plugins::{InternalDocumentExtractor, Plugin};
use crate::telemetry::conventions;
use crate::types::internal::InternalDocument;
use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use tracing::Instrument;

/// A wrapper around an internal extractor that adds tracing spans and
/// metrics recording automatically.
///
/// When the `otel` feature is enabled, [`get_extractor`](crate::core::extractor::helpers::get_extractor)
/// wraps the registry result in this type so that every extraction is
/// instrumented uniformly — individual extractors do not need their own
/// `#[instrument]` annotations.
pub(crate) struct InstrumentedExtractor {
    inner: Arc<dyn InternalDocumentExtractor>,
}

impl InstrumentedExtractor {
    /// Create a new instrumented wrapper around an existing extractor.
    pub(crate) fn new(inner: Arc<dyn InternalDocumentExtractor>) -> Self {
        Self { inner }
    }
}

// ---------------------------------------------------------------------------
// Plugin delegation
// ---------------------------------------------------------------------------

impl Plugin for InstrumentedExtractor {
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn version(&self) -> String {
        self.inner.version()
    }

    fn initialize(&self) -> Result<()> {
        self.inner.initialize()
    }

    fn shutdown(&self) -> Result<()> {
        self.inner.shutdown()
    }

    fn description(&self) -> &str {
        self.inner.description()
    }

    fn author(&self) -> &str {
        self.inner.author()
    }
}

// ---------------------------------------------------------------------------
// DocumentExtractor with instrumentation
// ---------------------------------------------------------------------------

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl InternalDocumentExtractor for InstrumentedExtractor {
    async fn extract_content(
        &self,
        content: &[u8],
        mime_type: &str,
        config: &ExtractionConfig,
    ) -> Result<InternalDocument> {
        let extractor_name = self.inner.name().to_owned();
        let size_bytes = content.len();

        let span = crate::telemetry::spans::extractor_span(&extractor_name, mime_type, size_bytes);
        let start = Instant::now();

        let result = self
            .inner
            .extract_content(content, mime_type, config)
            .instrument(span.clone())
            .await;

        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        record_metrics(&extractor_name, mime_type, size_bytes, elapsed_ms, &result);
        record_span_status(&span, &result);

        result
    }

    async fn extract_path(&self, path: &Path, mime_type: &str, config: &ExtractionConfig) -> Result<InternalDocument> {
        let extractor_name = self.inner.name().to_owned();
        let size_bytes = path.metadata().map(|m| m.len() as usize).unwrap_or(0);

        let span = crate::telemetry::spans::extractor_span(&extractor_name, mime_type, size_bytes);

        // Also record the sanitized filename on the span.
        let filename = crate::telemetry::spans::sanitize_path(path);
        span.record(conventions::DOCUMENT_FILENAME, &*filename);

        let start = Instant::now();

        let result = self
            .inner
            .extract_path(path, mime_type, config)
            .instrument(span.clone())
            .await;

        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        record_metrics(&extractor_name, mime_type, size_bytes, elapsed_ms, &result);
        record_span_status(&span, &result);

        result
    }

    fn supported_mime_types(&self) -> &[&str] {
        self.inner.supported_mime_types()
    }

    fn priority(&self) -> i32 {
        self.inner.priority()
    }

    fn can_handle(&self, path: &Path, mime_type: &str) -> bool {
        self.inner.can_handle(path, mime_type)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn record_span_status(span: &tracing::Span, result: &Result<InternalDocument>) {
    let _entered = span.enter();
    match result {
        Ok(_) => crate::telemetry::spans::record_success_on_current_span(),
        Err(e) => crate::telemetry::spans::record_error_on_current_span(e),
    }
}

fn record_metrics(
    extractor_name: &str,
    mime_type: &str,
    input_size: usize,
    elapsed_ms: f64,
    result: &Result<InternalDocument>,
) {
    let metrics = crate::telemetry::metrics::get_metrics();

    let status = if result.is_ok() { "ok" } else { "error" };
    let attrs = [
        opentelemetry::KeyValue::new(conventions::DOCUMENT_MIME_TYPE, mime_type.to_owned()),
        opentelemetry::KeyValue::new(conventions::EXTRACTOR_NAME, extractor_name.to_owned()),
        opentelemetry::KeyValue::new("status", status),
    ];

    metrics.extraction_total.add(1, &attrs);
    metrics.extraction_duration_ms.record(elapsed_ms, &attrs[..2]);
    metrics.extraction_input_bytes.record(input_size as u64, &attrs[..1]);

    if let Ok(res) = result {
        let output_bytes: usize = res.elements.iter().map(|e| e.text.len()).sum();
        metrics.extraction_output_bytes.record(output_bytes as u64, &attrs[..1]);
    }
}

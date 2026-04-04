//! Span helpers for kreuzberg telemetry.
//!
//! Provides functions to create properly-attributed tracing spans using
//! the semantic conventions defined in [`super::conventions`].

use super::conventions;

/// Record an error on the current span using semantic conventions.
///
/// Sets `otel.status_code = "ERROR"`, `kreuzberg.error.type`, and `error.message`.
pub fn record_error_on_current_span(error: &crate::KreuzbergError) {
    let span = tracing::Span::current();
    span.record(conventions::OTEL_STATUS_CODE, "ERROR");
    span.record(conventions::ERROR_TYPE, format!("{:?}", error));
    span.record(conventions::ERROR_MESSAGE, error.to_string());
}

/// Record extraction success on the current span.
pub fn record_success_on_current_span() {
    let span = tracing::Span::current();
    span.record(conventions::OTEL_STATUS_CODE, "OK");
}

/// Sanitize a file path to return only the filename.
///
/// Prevents PII (personally identifiable information) from appearing in
/// traces by only recording filenames instead of full paths.
pub fn sanitize_path(path: &std::path::Path) -> String {
    conventions::sanitize_filename(path).to_owned()
}

/// Create an extractor-level span with semantic convention fields.
///
/// Returns a `tracing::Span` with all `kreuzberg.extractor.*` and
/// `kreuzberg.document.*` fields pre-allocated (set to `Empty` for
/// lazy recording).
pub fn extractor_span(extractor_name: &str, mime_type: &str, size_bytes: usize) -> tracing::Span {
    tracing::info_span!(
        "kreuzberg.extract",
        { conventions::EXTRACTOR_NAME } = extractor_name,
        { conventions::DOCUMENT_MIME_TYPE } = mime_type,
        { conventions::DOCUMENT_SIZE_BYTES } = size_bytes,
        { conventions::OTEL_STATUS_CODE } = tracing::field::Empty,
        { conventions::ERROR_TYPE } = tracing::field::Empty,
        { conventions::ERROR_MESSAGE } = tracing::field::Empty,
    )
}

/// Create a pipeline stage span.
pub fn pipeline_stage_span(stage: &str) -> tracing::Span {
    tracing::info_span!("kreuzberg.pipeline", { conventions::PIPELINE_STAGE } = stage,)
}

/// Create a pipeline processor span.
pub fn pipeline_processor_span(stage: &str, processor_name: &str) -> tracing::Span {
    tracing::debug_span!(
        "kreuzberg.pipeline.processor",
        { conventions::PIPELINE_STAGE } = stage,
        { conventions::PIPELINE_PROCESSOR_NAME } = processor_name,
    )
}

/// Create an OCR operation span.
pub fn ocr_span(backend: &str, language: &str) -> tracing::Span {
    tracing::info_span!(
        "kreuzberg.ocr",
        { conventions::OCR_BACKEND } = backend,
        { conventions::OCR_LANGUAGE } = language,
    )
}

/// Create a model inference span.
pub fn model_inference_span(model_name: &str) -> tracing::Span {
    tracing::info_span!(
        "kreuzberg.model.inference",
        { conventions::MODEL_NAME } = model_name,
        { conventions::MODEL_INFERENCE_MS } = tracing::field::Empty,
    )
}

//! Metrics instruments for kreuzberg telemetry.
//!
//! Provides counters, histograms, and gauges for monitoring extraction
//! operations. Instruments are initialised lazily on first use via a
//! global [`OnceLock`].

use super::conventions::metrics as names;
use opentelemetry::metrics::{Counter, Histogram, Meter, UpDownCounter};
use std::sync::OnceLock;

/// Collection of all kreuzberg metric instruments.
pub struct ExtractionMetrics {
    // -- Counters --
    /// Total extractions (attributes: mime_type, extractor, status).
    pub extraction_total: Counter<u64>,
    /// Cache hits.
    pub cache_hits: Counter<u64>,
    /// Cache misses.
    pub cache_misses: Counter<u64>,
    /// Total batch requests (attributes: status).
    pub batch_total: Counter<u64>,

    // -- Histograms --
    /// Extraction wall-clock duration in milliseconds (attributes: mime_type, extractor).
    pub extraction_duration_ms: Histogram<f64>,
    /// Input document size in bytes (attributes: mime_type).
    pub extraction_input_bytes: Histogram<u64>,
    /// Output content size in bytes (attributes: mime_type).
    pub extraction_output_bytes: Histogram<u64>,
    /// Pipeline stage duration in milliseconds (attributes: stage).
    pub pipeline_duration_ms: Histogram<f64>,
    /// OCR duration in milliseconds (attributes: backend, language).
    pub ocr_duration_ms: Histogram<f64>,
    /// Batch total duration in milliseconds.
    pub batch_duration_ms: Histogram<f64>,

    // -- Gauges --
    /// Currently in-flight extractions.
    pub concurrent_extractions: UpDownCounter<i64>,
}

/// Global metrics instance.
static METRICS: OnceLock<ExtractionMetrics> = OnceLock::new();

/// Get the global extraction metrics, initialising on first call.
///
/// Uses the global [`opentelemetry::global::meter`] to create instruments.
pub fn get_metrics() -> &'static ExtractionMetrics {
    METRICS.get_or_init(|| {
        let meter = opentelemetry::global::meter("kreuzberg");
        ExtractionMetrics::new(&meter)
    })
}

impl ExtractionMetrics {
    /// Create a new set of metric instruments from the given meter.
    fn new(meter: &Meter) -> Self {
        Self {
            extraction_total: meter
                .u64_counter(names::EXTRACTION_TOTAL)
                .with_description("Total document extractions")
                .build(),
            cache_hits: meter
                .u64_counter(names::CACHE_HITS)
                .with_description("Extraction cache hits")
                .build(),
            cache_misses: meter
                .u64_counter(names::CACHE_MISSES)
                .with_description("Extraction cache misses")
                .build(),
            batch_total: meter
                .u64_counter(names::BATCH_TOTAL)
                .with_description("Total batch extraction requests")
                .build(),

            extraction_duration_ms: meter
                .f64_histogram(names::EXTRACTION_DURATION_MS)
                .with_description("Extraction wall-clock duration in milliseconds")
                .with_unit("ms")
                .build(),
            extraction_input_bytes: meter
                .u64_histogram(names::EXTRACTION_INPUT_BYTES)
                .with_description("Input document size in bytes")
                .with_unit("By")
                .build(),
            extraction_output_bytes: meter
                .u64_histogram(names::EXTRACTION_OUTPUT_BYTES)
                .with_description("Output content size in bytes")
                .with_unit("By")
                .build(),
            pipeline_duration_ms: meter
                .f64_histogram(names::PIPELINE_DURATION_MS)
                .with_description("Pipeline stage duration in milliseconds")
                .with_unit("ms")
                .build(),
            ocr_duration_ms: meter
                .f64_histogram(names::OCR_DURATION_MS)
                .with_description("OCR duration in milliseconds")
                .with_unit("ms")
                .build(),
            batch_duration_ms: meter
                .f64_histogram(names::BATCH_DURATION_MS)
                .with_description("Batch extraction duration in milliseconds")
                .with_unit("ms")
                .build(),

            concurrent_extractions: meter
                .i64_up_down_counter(names::CONCURRENT_EXTRACTIONS)
                .with_description("Currently in-flight extractions")
                .build(),
        }
    }
}

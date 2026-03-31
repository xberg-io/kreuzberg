//! Benchmark harness for comparing document extraction frameworks
//!
//! This crate provides infrastructure for benchmarking Kreuzberg against other
//! document extraction frameworks, measuring performance (throughput, memory, latency)
//! and quality (F1 scores, text accuracy).

pub mod adapter;
pub mod adapters;
pub mod aggregate;
pub mod comparison;
pub mod config;
pub mod consolidate;
pub mod corpus;
pub mod embed_benchmark;
pub mod error;
pub mod fixture;
pub mod groups;
pub mod markdown_quality;
pub mod model_benchmark;
pub mod monitoring;
pub mod output;
pub mod pipeline_benchmark;
pub mod pool_metrics;
pub mod profile_report;
pub mod profiling;
pub mod quality;
pub mod registry;
pub mod runner;
pub mod sizes;
pub mod stats;
pub mod survey;
pub mod types;
pub mod validate_gt;

pub use adapter::FrameworkAdapter;
pub use adapters::{NativeAdapter, NodeAdapter, PythonAdapter, RubyAdapter};
pub use aggregate::{
    ComparisonData, ConsolidationMetadata, DeltaMetrics, DurationPercentiles, FileTypeAggregation,
    FrameworkModeAggregation, NewConsolidatedResults, Percentiles, PerformancePercentiles, QualityPercentiles,
    RankedFramework, aggregate_new_format,
};
pub use config::{BenchmarkConfig, BenchmarkMode, ProfilingConfig, load_framework_sizes};
pub use consolidate::load_run_results;
pub use error::{Error, Result};
pub use fixture::{Fixture, FixtureManager};
pub use monitoring::{ResourceMonitor, ResourceSample, ResourceStats};
pub use output::{write_by_extension_analysis, write_json};
pub use pool_metrics::{FilePoolMetrics, PoolMetricsReport};
pub use profile_report::{Hotspot, MemorySnapshot, ProfileReport};
pub use quality::{compute_quality, compute_quality_with_structure};
pub use registry::AdapterRegistry;
pub use runner::BenchmarkRunner;
pub use types::{BenchmarkResult, DiskSizeInfo, FrameworkCapabilities, PdfMetadata};

pub use sizes::{
    FrameworkSize, FrameworkSizes, load_framework_sizes as load_sizes_json, measure_framework_sizes,
    save_framework_sizes,
};

//! MCP response DTO types with JSON Schema support.
//!
//! These types are used as structured output for MCP tool calls,
//! providing both human-readable text content and machine-parseable
//! structured data in a single response.

use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// Structured output for unified extraction.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ExtractionResult {
    /// Extraction results in discovery order.
    #[schemars(description = "Extraction results in discovery order")]
    pub results: Vec<serde_json::Value>,
    /// Non-fatal per-input errors.
    #[schemars(description = "Non-fatal per-input errors")]
    pub errors: Vec<serde_json::Value>,
    /// Aggregate extraction counts.
    #[schemars(description = "Aggregate extraction counts")]
    pub summary: ExtractionSummaryOutput,
    /// Final URLs reached after redirects during URL ingestion.
    #[schemars(description = "Final URLs reached after redirects during URL ingestion")]
    pub crawl_final_urls: Vec<String>,
    /// Total redirects followed while fetching or crawling URLs.
    #[schemars(description = "Total redirects followed while fetching or crawling URLs")]
    pub crawl_redirect_count: usize,
    /// Unique normalized URLs discovered by crawls.
    #[schemars(description = "Unique normalized URLs discovered by crawls")]
    pub crawl_unique_normalized_urls: Vec<String>,
}

/// Structured summary for unified extraction.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ExtractionSummaryOutput {
    /// Number of inputs submitted by the caller.
    pub inputs: usize,
    /// Number of extraction results produced.
    pub results: usize,
    /// Number of per-input errors.
    pub errors: usize,
    /// Number of remote HTTP(S) URLs resolved.
    pub remote_urls: usize,
    /// Number of HTML pages crawled or scraped.
    pub pages_crawled: usize,
    /// Number of downloaded non-HTML documents extracted from URLs.
    pub documents_downloaded: usize,
}

/// Structured output for MIME type detection.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct DetectMimeTypeOutput {
    /// Detected MIME type string.
    #[schemars(description = "Detected MIME type string")]
    pub mime_type: String,
}

/// Structured output listing all supported formats.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct ListFormatsOutput {
    /// List of supported document formats with extension and MIME type.
    #[schemars(description = "List of supported document formats")]
    pub formats: Vec<serde_json::Value>,
}

/// Structured output for library version information.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct VersionOutput {
    /// Xberg library version string.
    #[schemars(description = "Xberg library version string")]
    pub version: String,
}

/// Structured output for cache statistics.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CacheStatsOutput {
    /// Absolute path to the cache directory.
    #[schemars(description = "Absolute path to the cache directory")]
    pub directory: String,
    /// Total number of cached files.
    #[schemars(description = "Total number of cached files")]
    pub total_files: u64,
    /// Total cache size in megabytes.
    #[schemars(description = "Total cache size in megabytes")]
    pub total_size_mb: f64,
    /// Available disk space in megabytes.
    #[schemars(description = "Available disk space in megabytes")]
    pub available_space_mb: f64,
}

/// Structured output for the model manifest.
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct CacheManifestOutput {
    /// Xberg library version.
    #[schemars(description = "Xberg library version")]
    pub xberg_version: String,
    /// Number of model files in the manifest.
    #[schemars(description = "Number of model files in the manifest")]
    pub model_count: usize,
    /// Total size of all model files in bytes.
    #[schemars(description = "Total size of all model files in bytes")]
    pub total_size_bytes: u64,
    /// Model file entries with name, size, and checksum.
    #[schemars(description = "Model file entries")]
    pub models: Vec<serde_json::Value>,
}

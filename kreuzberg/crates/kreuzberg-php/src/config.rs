//! Configuration parsing for PHP bindings
//!
//! This module provides a bridge between PHP configuration arrays/JSON and Rust
//! ExtractionConfig structs. Instead of exposing Rust structs directly to PHP
//! (which creates API mismatches), we let PHP classes serialize to JSON and
//! deserialize on the Rust side using serde.
//!
//! This pattern matches the approach used in Go, Java, Ruby, and Elixir bindings.

use ext_php_rs::prelude::*;
use kreuzberg::FileExtractionConfig;
use kreuzberg::core::config::ExtractionConfig as RustExtractionConfig;
use serde_json;

/// Parse ExtractionConfig from JSON string.
///
/// This function receives JSON from PHP (typically from ExtractionConfig::toJson())
/// and deserializes it into a Rust ExtractionConfig struct using serde.
///
/// # Arguments
///
/// * `json` - JSON string representation of extraction config
///
/// # Returns
///
/// Result containing the parsed ExtractionConfig or error message
///
/// # Example (PHP side)
///
/// ```php
/// $config = new \Kreuzberg\Config\ExtractionConfig(
///     useCache: true,
///     chunking: new \Kreuzberg\Config\ChunkingConfig(maxChars: 512)
/// );
/// $json = $config->toJson();
/// // Rust receives this JSON and parses it
/// ```
pub fn parse_config_from_json(json: &str) -> Result<RustExtractionConfig, String> {
    serde_json::from_str(json).map_err(|e| format!("Failed to parse config JSON: {}", e))
}

/// Validate config JSON without fully parsing.
///
/// Quick validation to check if JSON is well-formed and contains valid config.
///
/// # Arguments
///
/// * `json` - JSON string to validate
///
/// # Returns
///
/// true if valid, false otherwise
pub fn validate_config_json(json: &str) -> bool {
    parse_config_from_json(json).is_ok()
}

/// Serialize ExtractionConfig to JSON string.
///
/// Converts a Rust ExtractionConfig back to JSON for PHP consumption.
///
/// # Arguments
///
/// * `config` - Reference to ExtractionConfig to serialize
///
/// # Returns
///
/// JSON string representation
pub fn config_to_json(config: &RustExtractionConfig) -> Result<String, String> {
    serde_json::to_string_pretty(config).map_err(|e| format!("Failed to serialize config: {}", e))
}

/// Parse an optional JSON string into `Option<FileExtractionConfig>`.
///
/// If the input is `None` or an empty string, returns `Ok(None)`.
/// Otherwise, deserializes the JSON into a `FileExtractionConfig`.
pub fn parse_file_config_from_json(json: &Option<String>) -> Result<Option<FileExtractionConfig>, String> {
    match json {
        None => Ok(None),
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => serde_json::from_str(s)
            .map(Some)
            .map_err(|e| format!("Failed to parse file config JSON: {}", e)),
    }
}

// Note: PHP config classes in packages/php/src/Config/*.php are the source of truth.
// They already have toArray(), fromArray(), toJson(), fromJson(), and fromFile() methods.
// This module simply provides the Rust-side parsing when needed for extraction operations.

/// Get function builders for registration with ext-php-rs module.
///
/// This function is called by lib.rs during module initialization to register
/// any PHP functions exported by the config module.
///
/// # Returns
///
/// Empty vector (no functions currently exported)
pub fn get_function_builders() -> Vec<ext_php_rs::builders::FunctionBuilder<'static>> {
    // No functions exported currently - config parsing happens internally
    // during extraction operations
    vec![]
}

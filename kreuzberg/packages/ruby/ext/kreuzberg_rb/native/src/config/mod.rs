//! Configuration parsing and conversion for Ruby bindings
//!
//! Handles conversion between Ruby Hash configurations and Rust config types.
//! Includes parsing for all nested configuration structures.

// Main types module containing all configuration parsing
mod types;

// Re-export all configuration functions for backward compatibility
pub use types::*;

use crate::error_handling::runtime_error;
use crate::helpers::ruby_value_to_json;
use magnus::value::ReprValue;
use magnus::{Error, Ruby, Value};

/// Parse a Ruby value (nil or Hash) into an `Option<kreuzberg::FileExtractionConfig>`.
///
/// - `nil` → `None` (use batch-level defaults)
/// - Hash  → serialize to JSON, then deserialize to `FileExtractionConfig`
pub fn parse_file_extraction_config(value: Value) -> Result<Option<kreuzberg::FileExtractionConfig>, Error> {
    let ruby = Ruby::get().expect("Ruby not initialized");
    if value.equal(ruby.qnil())? {
        return Ok(None);
    }

    let json_value = ruby_value_to_json(value)?;
    let file_config: kreuzberg::FileExtractionConfig = serde_json::from_value(json_value)
        .map_err(|e| runtime_error(format!("Invalid file extraction config: {}", e)))?;
    Ok(Some(file_config))
}

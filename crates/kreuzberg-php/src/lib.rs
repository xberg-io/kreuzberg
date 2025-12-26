//! Kreuzberg PHP Bindings
//!
//! This module exposes the Rust core extraction API to PHP using ext-php-rs.
//!
//! # Architecture
//!
//! - All extraction logic is in the Rust core (crates/kreuzberg)
//! - PHP is a thin wrapper that adds language-specific features
//! - Zero duplication of core functionality
//! - Modern ext-php-rs patterns throughout

#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;

mod config;
mod embeddings;
mod error;
mod extraction;
mod plugins;
mod types;
mod validation;

use config::*;
use embeddings::*;
use error::*;
use extraction::*;
use plugins::*;
use types::*;
use validation::*;

/// Get the Kreuzberg library version.
///
/// # Returns
///
/// Version string in semver format (e.g., "4.0.0-rc.20")
///
/// # Example
///
/// ```php
/// $version = kreuzberg_version();
/// echo "Kreuzberg version: $version\n";
/// ```
#[php_function]
pub fn kreuzberg_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Kreuzberg PHP extension module.
///
/// Exports all extraction functions, configuration types, error handling, and plugin management.
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        // Version function
        .function(kreuzberg_version)
        // Error classification functions
        .function(kreuzberg_classify_error)
        .function(kreuzberg_error_code_name)
        .function(kreuzberg_error_code_description)
        // Configuration functions
        .function(kreuzberg_config_to_json)
        .function(kreuzberg_config_get_field)
        .function(kreuzberg_config_merge)
        // Validation functions
        .function(kreuzberg_validate_binarization_method)
        .function(kreuzberg_validate_ocr_backend)
        .function(kreuzberg_validate_language_code)
        .function(kreuzberg_validate_token_reduction_level)
        .function(kreuzberg_validate_tesseract_psm)
        .function(kreuzberg_validate_tesseract_oem)
        .function(kreuzberg_validate_output_format)
        .function(kreuzberg_validate_confidence)
        .function(kreuzberg_validate_dpi)
        .function(kreuzberg_validate_chunking_params)
        .function(kreuzberg_validate_mime_type)
        // Getter functions for valid values
        .function(kreuzberg_get_valid_binarization_methods)
        .function(kreuzberg_get_valid_language_codes)
        .function(kreuzberg_get_valid_ocr_backends)
        .function(kreuzberg_get_valid_token_reduction_levels)
        .function(kreuzberg_get_extensions_for_mime)
        // Post-processor plugin functions
        .function(kreuzberg_register_post_processor)
        .function(kreuzberg_unregister_post_processor)
        .function(kreuzberg_list_post_processors)
        .function(kreuzberg_clear_post_processors)
        .function(kreuzberg_run_post_processors)
        // Validator plugin functions
        .function(kreuzberg_register_validator)
        .function(kreuzberg_unregister_validator)
        .function(kreuzberg_list_validators)
        .function(kreuzberg_clear_validators)
        .function(kreuzberg_run_validators)
        // Custom extractor plugin functions
        .function(kreuzberg_register_extractor)
        .function(kreuzberg_unregister_extractor)
        .function(kreuzberg_list_extractors)
        .function(kreuzberg_clear_extractors)
        .function(kreuzberg_test_plugin)
        // Extraction functions
        .function(kreuzberg_extract_file)
        .function(kreuzberg_extract_bytes)
        .function(kreuzberg_batch_extract_files)
        .function(kreuzberg_batch_extract_bytes)
        // MIME type detection functions
        .function(kreuzberg_detect_mime_type_from_bytes)
        .function(kreuzberg_detect_mime_type_from_path)
        // Embedding functions
        .function(kreuzberg_list_embedding_presets)
        .function(kreuzberg_get_embedding_preset)
}

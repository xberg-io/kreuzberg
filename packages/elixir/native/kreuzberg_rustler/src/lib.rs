#![allow(clippy::let_unit_value)]

//! Kreuzberg Rustler - Elixir NIF bindings for Kreuzberg document intelligence
//!
//! This module provides Elixir Native Implemented Functions (NIFs) for document extraction,
//! MIME type detection, configuration, and cache management.
//!
//! # Architecture
//!
//! The bindings are organized into focused modules:
//! - `atoms` - Elixir atom definitions
//! - `conversion` - Type conversion between Rust and Elixir
//! - `config` - Configuration parsing and validation
//! - `extraction` - Single document extraction NIFs
//! - `batch` - Batch extraction NIFs
//! - `utilities` - Validation, MIME detection, cache, and config NIFs

mod atoms;
pub(crate) mod batch;
pub(crate) mod config;
pub(crate) mod conversion;
pub(crate) mod extraction;
mod types;
pub(crate) mod utilities;
mod utils;

// Re-export NIF functions - rustler init macro will find them
pub use batch::{
    batch_extract_bytes, batch_extract_bytes_with_options, batch_extract_files, batch_extract_files_with_options,
};
pub use extraction::{extract, extract_file, extract_file_with_options, extract_with_options};
pub use utilities::{
    cache_stats, clear_cache, config_discover, config_from_file, detect_mime_type, detect_mime_type_from_path,
    get_embedding_preset, get_extensions_for_mime, list_embedding_presets, validate_binarization_method,
    validate_chunking_params, validate_confidence, validate_dpi, validate_language_code, validate_mime_type,
    validate_ocr_backend, validate_tesseract_oem, validate_tesseract_psm,
};

rustler::init!("Elixir.Kreuzberg.Native", load = on_load);

#[allow(non_local_definitions)]
fn on_load(_env: rustler::Env, _info: rustler::Term) -> bool {
    true
}

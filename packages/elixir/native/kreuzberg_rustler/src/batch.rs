//! Batch extraction NIFs
//!
//! This module provides Native Implemented Functions (NIFs) for batch document extraction,
//! processing multiple files or binary inputs efficiently.

use crate::atoms;
use crate::config::parse_extraction_config;
use crate::conversion::convert_extraction_result_to_term;
use rustler::{Binary, Encoder, Env, NifResult, Term};

// Constants for validation
const MAX_BINARY_SIZE: usize = 500 * 1024 * 1024; // 500MB

/// Batch extract text and data from multiple files with default configuration
///
/// # Arguments
/// * `paths` - Vec of file paths as strings
/// * `mime_type` - Optional string representing the MIME type for all files; if None, MIME type is detected per file
///
/// # Returns
/// * `{:ok, [result_map]}` - List of extraction result maps
/// * `{:error, reason}` - Error tuple with reason string
#[rustler::nif(schedule = "DirtyCpu")]
pub fn batch_extract_files<'a>(env: Env<'a>, paths: Vec<String>, mime_type: Option<String>) -> NifResult<Term<'a>> {
    if paths.is_empty() {
        return Ok((atoms::error(), "File paths list cannot be empty").encode(env));
    }

    let config = kreuzberg::core::config::ExtractionConfig::default();
    let mime_ref = mime_type.as_deref();

    let mut results = Vec::new();

    // Process each file
    for path in paths {
        match kreuzberg::extract_file_sync(&path, mime_ref, &config) {
            Ok(result) => match convert_extraction_result_to_term(env, &result) {
                Ok(term) => results.push(term),
                Err(e) => {
                    return Ok((atoms::error(), format!("Failed to encode result for '{}': {}", path, e)).encode(env))
                }
            },
            Err(e) => return Ok((atoms::error(), format!("Extraction failed for '{}': {}", path, e)).encode(env)),
        }
    }

    Ok((atoms::ok(), results).encode(env))
}

/// Batch extract text and data from multiple files with custom configuration
///
/// # Arguments
/// * `paths` - Vec of file paths as strings
/// * `mime_type` - Optional string representing the MIME type for all files; if None, MIME type is detected per file
/// * `options_term` - Term containing extraction options (as map or keyword list)
///
/// # Returns
/// * `{:ok, [result_map]}` - List of extraction result maps
/// * `{:error, reason}` - Error tuple with reason string
#[rustler::nif(schedule = "DirtyCpu")]
pub fn batch_extract_files_with_options<'a>(
    env: Env<'a>,
    paths: Vec<String>,
    mime_type: Option<String>,
    options_term: Term<'a>,
) -> NifResult<Term<'a>> {
    if paths.is_empty() {
        return Ok((atoms::error(), "File paths list cannot be empty").encode(env));
    }

    // Parse options from Elixir term to ExtractionConfig
    let config = match parse_extraction_config(env, options_term) {
        Ok(cfg) => cfg,
        Err(e) => return Ok((atoms::error(), format!("Invalid options: {}", e)).encode(env)),
    };

    let mime_ref = mime_type.as_deref();
    let mut results = Vec::new();

    // Process each file
    for path in paths {
        match kreuzberg::extract_file_sync(&path, mime_ref, &config) {
            Ok(result) => match convert_extraction_result_to_term(env, &result) {
                Ok(term) => results.push(term),
                Err(e) => {
                    return Ok((atoms::error(), format!("Failed to encode result for '{}': {}", path, e)).encode(env))
                }
            },
            Err(e) => return Ok((atoms::error(), format!("Extraction failed for '{}': {}", path, e)).encode(env)),
        }
    }

    Ok((atoms::ok(), results).encode(env))
}

/// Batch extract text and data from multiple binary inputs with default configuration
///
/// # Arguments
/// * `data_list` - Vec of binary data inputs
/// * `mime_types` - Vec of MIME type strings (one per input)
///
/// # Returns
/// * `{:ok, [result_map]}` - List of extraction result maps
/// * `{:error, reason}` - Error tuple with reason string
#[rustler::nif(schedule = "DirtyCpu")]
pub fn batch_extract_bytes<'a>(
    env: Env<'a>,
    data_list: Vec<Binary<'a>>,
    mime_types: Vec<String>,
) -> NifResult<Term<'a>> {
    if data_list.is_empty() {
        return Ok((atoms::error(), "Data list cannot be empty").encode(env));
    }

    if data_list.len() != mime_types.len() {
        return Ok((
            atoms::error(),
            format!(
                "Mismatch: {} data inputs but {} MIME types",
                data_list.len(),
                mime_types.len()
            ),
        )
            .encode(env));
    }

    let config = kreuzberg::core::config::ExtractionConfig::default();
    let mut results = Vec::new();

    // Process each binary input with its corresponding MIME type
    for (idx, (data, mime_type)) in data_list.iter().zip(mime_types.iter()).enumerate() {
        if data.is_empty() {
            return Ok((atoms::error(), format!("Binary input at index {} cannot be empty", idx)).encode(env));
        }

        if data.len() > MAX_BINARY_SIZE {
            return Ok((
                atoms::error(),
                format!("Binary input at index {} exceeds maximum size of 500MB", idx),
            )
                .encode(env));
        }

        match kreuzberg::extract_bytes_sync(data.as_slice(), mime_type, &config) {
            Ok(result) => match convert_extraction_result_to_term(env, &result) {
                Ok(term) => results.push(term),
                Err(e) => {
                    return Ok((
                        atoms::error(),
                        format!("Failed to encode result at index {}: {}", idx, e),
                    )
                        .encode(env))
                }
            },
            Err(e) => return Ok((atoms::error(), format!("Extraction failed at index {}: {}", idx, e)).encode(env)),
        }
    }

    Ok((atoms::ok(), results).encode(env))
}

/// Batch extract text and data from multiple binary inputs with custom configuration
///
/// # Arguments
/// * `data_list` - Vec of binary data inputs
/// * `mime_types` - Vec of MIME type strings (one per input)
/// * `options_term` - Term containing extraction options (as map or keyword list)
///
/// # Returns
/// * `{:ok, [result_map]}` - List of extraction result maps
/// * `{:error, reason}` - Error tuple with reason string
#[rustler::nif(schedule = "DirtyCpu")]
pub fn batch_extract_bytes_with_options<'a>(
    env: Env<'a>,
    data_list: Vec<Binary<'a>>,
    mime_types: Vec<String>,
    options_term: Term<'a>,
) -> NifResult<Term<'a>> {
    if data_list.is_empty() {
        return Ok((atoms::error(), "Data list cannot be empty").encode(env));
    }

    if data_list.len() != mime_types.len() {
        return Ok((
            atoms::error(),
            format!(
                "Mismatch: {} data inputs but {} MIME types",
                data_list.len(),
                mime_types.len()
            ),
        )
            .encode(env));
    }

    // Parse options from Elixir term to ExtractionConfig
    let config = match parse_extraction_config(env, options_term) {
        Ok(cfg) => cfg,
        Err(e) => return Ok((atoms::error(), format!("Invalid options: {}", e)).encode(env)),
    };

    let mut results = Vec::new();

    // Process each binary input with its corresponding MIME type
    for (idx, (data, mime_type)) in data_list.iter().zip(mime_types.iter()).enumerate() {
        if data.is_empty() {
            return Ok((atoms::error(), format!("Binary input at index {} cannot be empty", idx)).encode(env));
        }

        if data.len() > MAX_BINARY_SIZE {
            return Ok((
                atoms::error(),
                format!("Binary input at index {} exceeds maximum size of 500MB", idx),
            )
                .encode(env));
        }

        match kreuzberg::extract_bytes_sync(data.as_slice(), mime_type, &config) {
            Ok(result) => match convert_extraction_result_to_term(env, &result) {
                Ok(term) => results.push(term),
                Err(e) => {
                    return Ok((
                        atoms::error(),
                        format!("Failed to encode result at index {}: {}", idx, e),
                    )
                        .encode(env))
                }
            },
            Err(e) => return Ok((atoms::error(), format!("Extraction failed at index {}: {}", idx, e)).encode(env)),
        }
    }

    Ok((atoms::ok(), results).encode(env))
}

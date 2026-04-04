//! Batch extraction NIFs
//!
//! This module provides Native Implemented Functions (NIFs) for batch document extraction,
//! processing multiple files or binary inputs efficiently.
//!
//! All extraction calls are wrapped with `catch_unwind` to prevent panics in native
//! C libraries (pdfium, tesseract) from crashing the BEAM VM.

use crate::atoms;
use crate::config::{parse_extraction_config, parse_file_extraction_config};
use crate::conversion::convert_extraction_result_to_term;
use crate::safe::catch_native_panic;
use rustler::{Binary, Encoder, Env, NifResult, Term};
use std::path::PathBuf;

// Constants for validation
const MAX_BINARY_SIZE: usize = 500 * 1024 * 1024; // 500MB

/// Batch extract text and data from multiple files with default configuration
#[rustler::nif(schedule = "DirtyCpu")]
pub fn batch_extract_files<'a>(env: Env<'a>, paths: Vec<String>, mime_type: Option<String>) -> NifResult<Term<'a>> {
    if paths.is_empty() {
        return Ok((atoms::error(), "File paths list cannot be empty").encode(env));
    }

    let config = kreuzberg::core::config::ExtractionConfig::default();
    let mime_ref = mime_type.as_deref();

    let mut results = Vec::new();

    for path in paths {
        let p = path.clone();
        let extraction_result = catch_native_panic("batch_extract_file", || {
            kreuzberg::extract_file_sync(&p, mime_ref, &config)
        });

        match extraction_result {
            Err(panic_msg) => {
                return Ok((atoms::error(), format!("Panic extracting '{}': {}", path, panic_msg)).encode(env));
            }
            Ok(Err(e)) => {
                return Ok((atoms::error(), format!("Extraction failed for '{}': {}", path, e)).encode(env));
            }
            Ok(Ok(result)) => match convert_extraction_result_to_term(env, &result) {
                Ok(term) => results.push(term),
                Err(e) => {
                    return Ok((atoms::error(), format!("Failed to encode result for '{}': {}", path, e)).encode(env));
                }
            },
        }
    }

    Ok((atoms::ok(), results).encode(env))
}

/// Batch extract text and data from multiple files with custom configuration
///
/// Supports optional per-file config overrides via `file_configs` parameter.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn batch_extract_files_with_options<'a>(
    env: Env<'a>,
    paths: Vec<String>,
    mime_type: Option<String>,
    options_term: Term<'a>,
    file_configs: Option<Vec<Term<'a>>>,
) -> NifResult<Term<'a>> {
    if paths.is_empty() {
        return Ok((atoms::error(), "File paths list cannot be empty").encode(env));
    }

    let config = match parse_extraction_config(env, options_term) {
        Ok(cfg) => cfg,
        Err(e) => return Ok((atoms::error(), format!("Invalid options: {}", e)).encode(env)),
    };

    match file_configs {
        Some(fc_list) => {
            if paths.len() != fc_list.len() {
                return Ok((
                    atoms::error(),
                    format!("Mismatch: {} paths but {} file configs", paths.len(), fc_list.len()),
                )
                    .encode(env));
            }

            let mut items: Vec<(PathBuf, Option<kreuzberg::FileExtractionConfig>)> = Vec::with_capacity(paths.len());
            for (idx, (path, fc_term)) in paths.into_iter().zip(fc_list.into_iter()).enumerate() {
                let fc = match parse_file_extraction_config(env, fc_term) {
                    Ok(fc) => fc,
                    Err(e) => {
                        return Ok((atoms::error(), format!("Invalid file config at index {}: {}", idx, e)).encode(env));
                    }
                };
                items.push((PathBuf::from(path), fc));
            }

            let batch_result = catch_native_panic("batch_extract_files_with_options", || {
                kreuzberg::batch_extract_file_sync(items, &config)
            });

            match batch_result {
                Err(panic_msg) => Ok((atoms::error(), panic_msg).encode(env)),
                Ok(Err(e)) => Ok((atoms::error(), format!("Batch extraction failed: {}", e)).encode(env)),
                Ok(Ok(results)) => {
                    let mut result_terms = Vec::with_capacity(results.len());
                    for (idx, result) in results.into_iter().enumerate() {
                        match convert_extraction_result_to_term(env, &result) {
                            Ok(term) => result_terms.push(term),
                            Err(e) => {
                                return Ok((
                                    atoms::error(),
                                    format!("Failed to encode result at index {}: {}", idx, e),
                                )
                                    .encode(env));
                            }
                        }
                    }
                    Ok((atoms::ok(), result_terms).encode(env))
                }
            }
        }
        None => {
            let mime_ref = mime_type.as_deref();
            let mut results = Vec::new();

            for path in paths {
                let p = path.clone();
                let extraction_result = catch_native_panic("batch_extract_file_with_options", || {
                    kreuzberg::extract_file_sync(&p, mime_ref, &config)
                });

                match extraction_result {
                    Err(panic_msg) => {
                        return Ok((atoms::error(), format!("Panic extracting '{}': {}", path, panic_msg)).encode(env));
                    }
                    Ok(Err(e)) => {
                        return Ok((atoms::error(), format!("Extraction failed for '{}': {}", path, e)).encode(env));
                    }
                    Ok(Ok(result)) => match convert_extraction_result_to_term(env, &result) {
                        Ok(term) => results.push(term),
                        Err(e) => {
                            return Ok(
                                (atoms::error(), format!("Failed to encode result for '{}': {}", path, e)).encode(env),
                            );
                        }
                    },
                }
            }

            Ok((atoms::ok(), results).encode(env))
        }
    }
}

/// Batch extract text and data from multiple binary inputs with default configuration
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

        let bytes = data.as_slice().to_vec();
        let extraction_result = catch_native_panic("batch_extract_bytes", || {
            kreuzberg::extract_bytes_sync(&bytes, mime_type, &config)
        });

        match extraction_result {
            Err(panic_msg) => {
                return Ok((atoms::error(), format!("Panic at index {}: {}", idx, panic_msg)).encode(env));
            }
            Ok(Err(e)) => {
                return Ok((atoms::error(), format!("Extraction failed at index {}: {}", idx, e)).encode(env));
            }
            Ok(Ok(result)) => match convert_extraction_result_to_term(env, &result) {
                Ok(term) => results.push(term),
                Err(e) => {
                    return Ok((
                        atoms::error(),
                        format!("Failed to encode result at index {}: {}", idx, e),
                    )
                        .encode(env));
                }
            },
        }
    }

    Ok((atoms::ok(), results).encode(env))
}

/// Batch extract text and data from multiple binary inputs with custom configuration
///
/// Supports optional per-file config overrides via `file_configs` parameter.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn batch_extract_bytes_with_options<'a>(
    env: Env<'a>,
    data_list: Vec<Binary<'a>>,
    mime_types: Vec<String>,
    options_term: Term<'a>,
    file_configs: Option<Vec<Term<'a>>>,
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

    let config = match parse_extraction_config(env, options_term) {
        Ok(cfg) => cfg,
        Err(e) => return Ok((atoms::error(), format!("Invalid options: {}", e)).encode(env)),
    };

    match file_configs {
        Some(fc_list) => {
            if data_list.len() != fc_list.len() {
                return Ok((
                    atoms::error(),
                    format!(
                        "Mismatch: {} data inputs but {} file configs",
                        data_list.len(),
                        fc_list.len()
                    ),
                )
                    .encode(env));
            }

            let mut items: Vec<(Vec<u8>, String, Option<kreuzberg::FileExtractionConfig>)> =
                Vec::with_capacity(data_list.len());
            for (idx, ((data, mime_type), fc_term)) in data_list
                .iter()
                .zip(mime_types.into_iter())
                .zip(fc_list.into_iter())
                .enumerate()
            {
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

                let fc = match parse_file_extraction_config(env, fc_term) {
                    Ok(fc) => fc,
                    Err(e) => {
                        return Ok((atoms::error(), format!("Invalid file config at index {}: {}", idx, e)).encode(env));
                    }
                };
                items.push((data.as_slice().to_vec(), mime_type, fc));
            }

            let batch_result = catch_native_panic("batch_extract_bytes_with_options", || {
                kreuzberg::batch_extract_bytes_sync(items, &config)
            });

            match batch_result {
                Err(panic_msg) => Ok((atoms::error(), panic_msg).encode(env)),
                Ok(Err(e)) => Ok((atoms::error(), format!("Batch extraction failed: {}", e)).encode(env)),
                Ok(Ok(results)) => {
                    let mut result_terms = Vec::with_capacity(results.len());
                    for (idx, result) in results.into_iter().enumerate() {
                        match convert_extraction_result_to_term(env, &result) {
                            Ok(term) => result_terms.push(term),
                            Err(e) => {
                                return Ok((
                                    atoms::error(),
                                    format!("Failed to encode result at index {}: {}", idx, e),
                                )
                                    .encode(env));
                            }
                        }
                    }
                    Ok((atoms::ok(), result_terms).encode(env))
                }
            }
        }
        None => {
            let mut results = Vec::new();

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

                let bytes = data.as_slice().to_vec();
                let extraction_result = catch_native_panic("batch_extract_bytes_with_options", || {
                    kreuzberg::extract_bytes_sync(&bytes, mime_type, &config)
                });

                match extraction_result {
                    Err(panic_msg) => {
                        return Ok((atoms::error(), format!("Panic at index {}: {}", idx, panic_msg)).encode(env));
                    }
                    Ok(Err(e)) => {
                        return Ok((atoms::error(), format!("Extraction failed at index {}: {}", idx, e)).encode(env));
                    }
                    Ok(Ok(result)) => match convert_extraction_result_to_term(env, &result) {
                        Ok(term) => results.push(term),
                        Err(e) => {
                            return Ok((
                                atoms::error(),
                                format!("Failed to encode result at index {}: {}", idx, e),
                            )
                                .encode(env));
                        }
                    },
                }
            }

            Ok((atoms::ok(), results).encode(env))
        }
    }
}

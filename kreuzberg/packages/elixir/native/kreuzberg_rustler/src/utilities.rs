//! Utility NIFs
//!
//! This module provides utility Native Implemented Functions (NIFs) for validation,
//! MIME type detection, cache management, and configuration operations.

use crate::atoms;
use rustler::types::map::map_new;
use rustler::{Binary, Encoder, Env, NifResult, Term};

// =============================================================================
// VALIDATION FUNCTIONS - Configuration validators for extraction parameters
// =============================================================================

/// Validate chunking parameters (max_chars and max_overlap).
///
/// # Arguments
/// * `max_chars` - Maximum characters per chunk (must be > 0)
/// * `max_overlap` - Maximum overlap between chunks (must be < max_chars)
///
/// # Returns
/// * `:ok` - If parameters are valid
/// * `{:error, reason}` - If parameters are invalid
#[rustler::nif]
pub fn validate_chunking_params<'a>(env: Env<'a>, max_chars: usize, max_overlap: usize) -> NifResult<Term<'a>> {
    match kreuzberg::core::config_validation::validate_chunking_params(max_chars, max_overlap) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(e) => {
            let error_msg = format!("{}", e);
            Ok((atoms::error(), error_msg).encode(env))
        }
    }
}

/// Validate a language code (ISO 639-1 or 639-3 format).
#[rustler::nif]
pub fn validate_language_code<'a>(env: Env<'a>, code: String) -> NifResult<Term<'a>> {
    match kreuzberg::core::config_validation::validate_language_code(&code) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(e) => {
            let error_msg = format!("{}", e);
            Ok((atoms::error(), error_msg).encode(env))
        }
    }
}

/// Validate a DPI (dots per inch) value.
#[rustler::nif]
pub fn validate_dpi<'a>(env: Env<'a>, dpi: i32) -> NifResult<Term<'a>> {
    match kreuzberg::core::config_validation::validate_dpi(dpi) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(e) => {
            let error_msg = format!("{}", e);
            Ok((atoms::error(), error_msg).encode(env))
        }
    }
}

/// Validate a confidence threshold value.
#[rustler::nif]
pub fn validate_confidence<'a>(env: Env<'a>, confidence: f64) -> NifResult<Term<'a>> {
    match kreuzberg::core::config_validation::validate_confidence(confidence) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(e) => {
            let error_msg = format!("{}", e);
            Ok((atoms::error(), error_msg).encode(env))
        }
    }
}

/// Validate an OCR backend name.
#[rustler::nif]
pub fn validate_ocr_backend<'a>(env: Env<'a>, backend: String) -> NifResult<Term<'a>> {
    match kreuzberg::core::config_validation::validate_ocr_backend(&backend) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(e) => {
            let error_msg = format!("{}", e);
            Ok((atoms::error(), error_msg).encode(env))
        }
    }
}

/// Validate a binarization method.
#[rustler::nif]
pub fn validate_binarization_method<'a>(env: Env<'a>, method: String) -> NifResult<Term<'a>> {
    match kreuzberg::core::config_validation::validate_binarization_method(&method) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(e) => {
            let error_msg = format!("{}", e);
            Ok((atoms::error(), error_msg).encode(env))
        }
    }
}

/// Validate a Tesseract Page Segmentation Mode (PSM) value.
#[rustler::nif]
pub fn validate_tesseract_psm<'a>(env: Env<'a>, psm: i32) -> NifResult<Term<'a>> {
    match kreuzberg::core::config_validation::validate_tesseract_psm(psm) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(e) => {
            let error_msg = format!("{}", e);
            Ok((atoms::error(), error_msg).encode(env))
        }
    }
}

/// Validate a Tesseract OCR Engine Mode (OEM) value.
#[rustler::nif]
pub fn validate_tesseract_oem<'a>(env: Env<'a>, oem: i32) -> NifResult<Term<'a>> {
    match kreuzberg::core::config_validation::validate_tesseract_oem(oem) {
        Ok(_) => Ok(atoms::ok().encode(env)),
        Err(e) => {
            let error_msg = format!("{}", e);
            Ok((atoms::error(), error_msg).encode(env))
        }
    }
}

// =============================================================================
// MIME TYPE FUNCTIONS
// =============================================================================

/// Detect MIME type from binary data using content inspection.
#[rustler::nif]
pub fn detect_mime_type<'a>(env: Env<'a>, data: Binary<'a>) -> NifResult<Term<'a>> {
    if data.is_empty() {
        return Ok((atoms::error(), "Binary input cannot be empty").encode(env));
    }

    match kreuzberg::detect_mime_type_from_bytes(data.as_slice()) {
        Ok(mime_type) => Ok((atoms::ok(), mime_type).encode(env)),
        Err(e) => Ok((atoms::error(), format!("MIME detection failed: {}", e)).encode(env)),
    }
}

/// Detect MIME type from file path using extension and optional content inspection.
#[rustler::nif]
pub fn detect_mime_type_from_path<'a>(env: Env<'a>, path: String) -> NifResult<Term<'a>> {
    if path.is_empty() {
        return Ok((atoms::error(), "File path cannot be empty").encode(env));
    }

    match kreuzberg::detect_mime_type(&path, true) {
        Ok(mime_type) => Ok((atoms::ok(), mime_type).encode(env)),
        Err(e) => Ok((atoms::error(), format!("MIME detection failed: {}", e)).encode(env)),
    }
}

/// Validate that a MIME type is supported by Kreuzberg.
#[rustler::nif]
pub fn validate_mime_type<'a>(env: Env<'a>, mime_type: String) -> NifResult<Term<'a>> {
    if mime_type.is_empty() {
        return Ok((atoms::error(), "MIME type cannot be empty").encode(env));
    }

    match kreuzberg::validate_mime_type(&mime_type) {
        Ok(validated) => Ok((atoms::ok(), validated).encode(env)),
        Err(e) => Ok((atoms::error(), format!("MIME validation failed: {}", e)).encode(env)),
    }
}

/// Get file extensions for a given MIME type.
#[rustler::nif]
pub fn get_extensions_for_mime<'a>(env: Env<'a>, mime_type: String) -> NifResult<Term<'a>> {
    if mime_type.is_empty() {
        return Ok((atoms::error(), "MIME type cannot be empty").encode(env));
    }

    match kreuzberg::get_extensions_for_mime(&mime_type) {
        Ok(extensions) => Ok((atoms::ok(), extensions).encode(env)),
        Err(e) => Ok((atoms::error(), format!("Failed to get extensions: {}", e)).encode(env)),
    }
}

// =============================================================================
// EMBEDDING PRESET FUNCTIONS
// =============================================================================

/// List all available embedding presets.
#[rustler::nif]
pub fn list_embedding_presets<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    let presets = kreuzberg::list_presets();
    let preset_names: Vec<&str> = presets;
    Ok((atoms::ok(), preset_names).encode(env))
}

/// Get detailed information about a specific embedding preset.
#[rustler::nif]
pub fn get_embedding_preset<'a>(env: Env<'a>, preset_name: String) -> NifResult<Term<'a>> {
    if preset_name.is_empty() {
        return Ok((atoms::error(), "Preset name cannot be empty").encode(env));
    }

    match kreuzberg::get_preset(&preset_name) {
        Some(preset) => {
            // Manually construct a map from preset fields
            let mut map = map_new(env);

            map = match map.map_put("name".encode(env), preset.name.encode(env)) {
                Ok(m) => m,
                Err(_) => return Ok((atoms::error(), "Failed to build preset map").encode(env)),
            };

            map = match map.map_put("chunk_size".encode(env), (preset.chunk_size as i64).encode(env)) {
                Ok(m) => m,
                Err(_) => return Ok((atoms::error(), "Failed to build preset map").encode(env)),
            };

            map = match map.map_put("overlap".encode(env), (preset.overlap as i64).encode(env)) {
                Ok(m) => m,
                Err(_) => return Ok((atoms::error(), "Failed to build preset map").encode(env)),
            };

            map = match map.map_put("dimensions".encode(env), (preset.dimensions as i64).encode(env)) {
                Ok(m) => m,
                Err(_) => return Ok((atoms::error(), "Failed to build preset map").encode(env)),
            };

            map = match map.map_put("description".encode(env), preset.description.encode(env)) {
                Ok(m) => m,
                Err(_) => return Ok((atoms::error(), "Failed to build preset map").encode(env)),
            };

            Ok((atoms::ok(), map).encode(env))
        }
        None => Ok((atoms::error(), format!("Preset '{}' not found", preset_name)).encode(env)),
    }
}

// =============================================================================
// CACHE MANAGEMENT FUNCTIONS
// =============================================================================

/// Get cache statistics including file count, size, and disk space information.
#[rustler::nif]
pub fn cache_stats<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    // Get the cache directory - use kreuzberg's internal cache path
    let cache_dir = match std::env::current_dir() {
        Ok(dir) => {
            let mut path = dir;
            path.push(".kreuzberg");
            path.push("extraction");
            path
        }
        Err(_) => {
            return Ok((atoms::error(), "Failed to determine cache directory").encode(env));
        }
    };

    let cache_dir_str = match cache_dir.to_str() {
        Some(s) => s,
        None => {
            return Ok((atoms::error(), "Cache directory path contains invalid UTF-8").encode(env));
        }
    };

    // Get cache statistics using kreuzberg's cache module
    match kreuzberg::cache::get_cache_metadata(cache_dir_str) {
        Ok(stats) => {
            let mut map = map_new(env);

            // Add all statistics to the map
            map = match map.map_put("total_files".encode(env), (stats.total_files as i64).encode(env)) {
                Ok(m) => m,
                Err(_) => {
                    return Ok((atoms::error(), "Failed to encode cache statistics").encode(env));
                }
            };

            map = match map.map_put("total_size_mb".encode(env), stats.total_size_mb.encode(env)) {
                Ok(m) => m,
                Err(_) => {
                    return Ok((atoms::error(), "Failed to encode cache statistics").encode(env));
                }
            };

            map = match map.map_put("available_space_mb".encode(env), stats.available_space_mb.encode(env)) {
                Ok(m) => m,
                Err(_) => {
                    return Ok((atoms::error(), "Failed to encode cache statistics").encode(env));
                }
            };

            map = match map.map_put(
                "oldest_file_age_days".encode(env),
                stats.oldest_file_age_days.encode(env),
            ) {
                Ok(m) => m,
                Err(_) => {
                    return Ok((atoms::error(), "Failed to encode cache statistics").encode(env));
                }
            };

            map = match map.map_put(
                "newest_file_age_days".encode(env),
                stats.newest_file_age_days.encode(env),
            ) {
                Ok(m) => m,
                Err(_) => {
                    return Ok((atoms::error(), "Failed to encode cache statistics").encode(env));
                }
            };

            Ok((atoms::ok(), map).encode(env))
        }
        Err(e) => Ok((atoms::error(), format!("Failed to get cache statistics: {}", e)).encode(env)),
    }
}

/// Clear all cached extraction results.
#[rustler::nif]
pub fn clear_cache<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    // Get the cache directory - use kreuzberg's internal cache path
    let cache_dir = match std::env::current_dir() {
        Ok(dir) => {
            let mut path = dir;
            path.push(".kreuzberg");
            path.push("extraction");
            path
        }
        Err(_) => {
            return Ok((atoms::error(), "Failed to determine cache directory").encode(env));
        }
    };

    let cache_dir_str = match cache_dir.to_str() {
        Some(s) => s,
        None => {
            return Ok((atoms::error(), "Cache directory path contains invalid UTF-8").encode(env));
        }
    };

    // Clear the cache using kreuzberg's cache module
    match kreuzberg::cache::clear_cache_directory(cache_dir_str) {
        Ok((removed_count, removed_size_mb)) => {
            // Cache cleared successfully
            // Note: removed_count and removed_size_mb are available for future logging
            let _ = (removed_count, removed_size_mb);
            Ok(atoms::ok().encode(env))
        }
        Err(e) => Ok((atoms::error(), format!("Failed to clear cache: {}", e)).encode(env)),
    }
}

// =============================================================================
// CONFIG FUNCTIONS
// =============================================================================

/// Discover an ExtractionConfig by searching the current directory and parent directories.
#[rustler::nif]
pub fn config_discover<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    match kreuzberg::core::config::ExtractionConfig::discover() {
        Ok(Some(config)) => {
            // Convert config to JSON string
            match serde_json::to_string(&config) {
                Ok(json) => Ok((atoms::ok(), json).encode(env)),
                Err(e) => Ok((atoms::error(), format!("Failed to serialize config: {}", e)).encode(env)),
            }
        }
        Ok(None) => {
            // No config found - return error with :not_found atom
            Ok((atoms::error(), atoms::not_found()).encode(env))
        }
        Err(e) => Ok((atoms::error(), format!("Failed to discover config: {}", e)).encode(env)),
    }
}

/// Load an ExtractionConfig from a specific file path.
#[rustler::nif]
pub fn config_from_file<'a>(env: Env<'a>, file_path: String) -> NifResult<Term<'a>> {
    use std::path::Path;

    let path = Path::new(&file_path);

    match kreuzberg::core::config::ExtractionConfig::from_file(path) {
        Ok(config) => {
            // Convert config to JSON string
            match serde_json::to_string(&config) {
                Ok(json) => Ok((atoms::ok(), json).encode(env)),
                Err(e) => Ok((atoms::error(), format!("Failed to serialize config: {}", e)).encode(env)),
            }
        }
        Err(e) => Ok((atoms::error(), format!("Failed to load config from file: {}", e)).encode(env)),
    }
}

//! Extraction NIFs
//!
//! This module provides Native Implemented Functions (NIFs) for document extraction,
//! including single file/bytes extraction and batch operations.
//!
//! All extraction calls are wrapped with `catch_unwind` to prevent panics in native
//! C libraries (pdfium, tesseract) from crashing the BEAM VM.

use crate::atoms;
use crate::config::parse_extraction_config;
use crate::conversion::convert_extraction_result_to_term;
use crate::safe::catch_native_panic;
use rustler::{Binary, Encoder, Env, NifResult, ResourceArc, Term};
use std::sync::Mutex;

// Constants for validation
const MAX_BINARY_SIZE: usize = 500 * 1024 * 1024; // 500MB

/// Extract text and data from a document binary with default configuration
///
/// # Arguments
/// * `input` - Binary containing the document data
/// * `mime_type` - String representing the MIME type (e.g., "application/pdf")
///
/// # Returns
/// * `{:ok, result_map}` - Map containing extraction results
/// * `{:error, reason}` - Error tuple with reason string
#[rustler::nif(schedule = "DirtyCpu")]
pub fn extract<'a>(env: Env<'a>, input: Binary<'a>, mime_type: String) -> NifResult<Term<'a>> {
    if input.is_empty() {
        return Ok((atoms::error(), "Binary input cannot be empty").encode(env));
    }

    if input.len() > MAX_BINARY_SIZE {
        return Ok((atoms::error(), "Binary input exceeds maximum size of 500MB").encode(env));
    }

    let config = kreuzberg::core::config::ExtractionConfig::default();
    let bytes = input.as_slice().to_vec();

    let extraction_result = catch_native_panic("extract_bytes", || {
        kreuzberg::extract_bytes_sync(&bytes, &mime_type, &config)
    });

    match extraction_result {
        Err(panic_msg) => Ok((atoms::error(), panic_msg).encode(env)),
        Ok(Err(e)) => Ok((atoms::error(), format!("Extraction failed: {}", e)).encode(env)),
        Ok(Ok(result)) => match convert_extraction_result_to_term(env, &result) {
            Ok(term) => Ok((atoms::ok(), term).encode(env)),
            Err(e) => Ok((atoms::error(), format!("Failed to encode result: {}", e)).encode(env)),
        },
    }
}

/// Extract text and data from a document binary with custom configuration
#[rustler::nif(schedule = "DirtyCpu")]
pub fn extract_with_options<'a>(
    env: Env<'a>,
    input: Binary<'a>,
    mime_type: String,
    options: Term<'a>,
) -> NifResult<Term<'a>> {
    if input.is_empty() {
        return Ok((atoms::error(), "Binary input cannot be empty").encode(env));
    }

    if input.len() > MAX_BINARY_SIZE {
        return Ok((atoms::error(), "Binary input exceeds maximum size of 500MB").encode(env));
    }

    let config = match parse_extraction_config(env, options) {
        Ok(cfg) => cfg,
        Err(e) => return Ok((atoms::error(), format!("Invalid options: {}", e)).encode(env)),
    };

    let bytes = input.as_slice().to_vec();

    let extraction_result = catch_native_panic("extract_bytes_with_options", || {
        kreuzberg::extract_bytes_sync(&bytes, &mime_type, &config)
    });

    match extraction_result {
        Err(panic_msg) => Ok((atoms::error(), panic_msg).encode(env)),
        Ok(Err(e)) => Ok((atoms::error(), format!("Extraction failed: {}", e)).encode(env)),
        Ok(Ok(result)) => match convert_extraction_result_to_term(env, &result) {
            Ok(term) => Ok((atoms::ok(), term).encode(env)),
            Err(e) => Ok((atoms::error(), format!("Failed to encode result: {}", e)).encode(env)),
        },
    }
}

/// Extract text and data from a file at the given path with default configuration
#[rustler::nif(schedule = "DirtyCpu")]
pub fn extract_file<'a>(env: Env<'a>, path: String, mime_type: Option<String>) -> NifResult<Term<'a>> {
    let config = kreuzberg::core::config::ExtractionConfig::default();

    let extraction_result = catch_native_panic("extract_file", || {
        kreuzberg::extract_file_sync(&path, mime_type.as_deref(), &config)
    });

    match extraction_result {
        Err(panic_msg) => Ok((atoms::error(), panic_msg).encode(env)),
        Ok(Err(e)) => Ok((atoms::error(), format!("Extraction failed: {}", e)).encode(env)),
        Ok(Ok(result)) => match convert_extraction_result_to_term(env, &result) {
            Ok(term) => Ok((atoms::ok(), term).encode(env)),
            Err(e) => Ok((atoms::error(), format!("Failed to encode result: {}", e)).encode(env)),
        },
    }
}

/// Extract text and data from a file at the given path with custom configuration
#[rustler::nif(schedule = "DirtyCpu")]
pub fn extract_file_with_options<'a>(
    env: Env<'a>,
    path: String,
    mime_type: Option<String>,
    options_term: Term<'a>,
) -> NifResult<Term<'a>> {
    let config = match parse_extraction_config(env, options_term) {
        Ok(cfg) => cfg,
        Err(e) => return Ok((atoms::error(), format!("Invalid options: {}", e)).encode(env)),
    };

    let extraction_result = catch_native_panic("extract_file_with_options", || {
        kreuzberg::extract_file_sync(&path, mime_type.as_deref(), &config)
    });

    match extraction_result {
        Err(panic_msg) => Ok((atoms::error(), panic_msg).encode(env)),
        Ok(Err(e)) => Ok((atoms::error(), format!("Extraction failed: {}", e)).encode(env)),
        Ok(Ok(result)) => match convert_extraction_result_to_term(env, &result) {
            Ok(term) => Ok((atoms::ok(), term).encode(env)),
            Err(e) => Ok((atoms::error(), format!("Failed to encode result: {}", e)).encode(env)),
        },
    }
}

/// Render a single page of a PDF file to a PNG byte buffer
#[rustler::nif(schedule = "DirtyCpu")]
pub fn render_pdf_page<'a>(env: Env<'a>, input: String, page_index: usize, dpi: Option<i32>) -> NifResult<Term<'a>> {
    if input.is_empty() {
        return Ok((atoms::error(), "File path cannot be empty").encode(env));
    }

    let pdf_bytes = match std::fs::read(&input) {
        Ok(b) => b,
        Err(e) => return Ok((atoms::error(), format!("Failed to read file: {}", e)).encode(env)),
    };

    let render_result = catch_native_panic("render_pdf_page", || {
        kreuzberg::pdf::render_pdf_page_to_png(&pdf_bytes, page_index, dpi, None)
    });

    match render_result {
        Err(panic_msg) => Ok((atoms::error(), panic_msg).encode(env)),
        Ok(Err(e)) => Ok((atoms::error(), format!("Rendering failed: {}", e)).encode(env)),
        Ok(Ok(png)) => {
            let mut obin = match rustler::OwnedBinary::new(png.len()) {
                Some(b) => b,
                None => {
                    return Ok((
                        atoms::error(),
                        format!("failed to allocate binary of {} bytes", png.len()),
                    )
                        .encode(env));
                }
            };
            obin.as_mut_slice().copy_from_slice(&png);
            Ok((atoms::ok(), obin.release(env)).encode(env))
        }
    }
}

/// Resource wrapper for PdfPageIterator to allow passing between NIF calls.
pub struct PdfPageIteratorResource {
    inner: Mutex<Option<kreuzberg::pdf::PdfPageIterator>>,
}

#[rustler::resource_impl]
impl rustler::Resource for PdfPageIteratorResource {}

/// Open a new PDF page iterator, returning a resource handle.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn render_pdf_pages_iter_open<'a>(env: Env<'a>, path: String, dpi: Option<i32>) -> NifResult<Term<'a>> {
    if path.is_empty() {
        return Ok((atoms::error(), "File path cannot be empty").encode(env));
    }

    let open_result = catch_native_panic("render_pdf_pages_iter_open", || {
        kreuzberg::pdf::PdfPageIterator::from_file(&path, dpi, None)
    });

    match open_result {
        Err(panic_msg) => Ok((atoms::error(), panic_msg).encode(env)),
        Ok(Err(e)) => Ok((atoms::error(), format!("Failed to open iterator: {}", e)).encode(env)),
        Ok(Ok(iter)) => {
            let resource = ResourceArc::new(PdfPageIteratorResource {
                inner: Mutex::new(Some(iter)),
            });
            Ok(resource.encode(env))
        }
    }
}

/// Advance the iterator and return the next page.
///
/// Returns `{:ok, {page_index, png_binary}}` or `:done` when exhausted.
#[rustler::nif(schedule = "DirtyCpu")]
pub fn render_pdf_pages_iter_next<'a>(
    env: Env<'a>,
    resource: ResourceArc<PdfPageIteratorResource>,
) -> NifResult<Term<'a>> {
    let mut guard = resource
        .inner
        .lock()
        .map_err(|_| rustler::Error::Term(Box::new("iterator lock poisoned")))?;

    let iter = match guard.as_mut() {
        Some(it) => it,
        None => return Ok(atoms::done().encode(env)),
    };

    // Note: catch_unwind can't easily wrap Iterator::next with a mutable borrow,
    // so we accept the risk here. The iterator is already behind a Mutex and
    // runs on a dirty scheduler.
    match iter.next() {
        Some(Ok((page_index, png))) => {
            let mut obin = match rustler::OwnedBinary::new(png.len()) {
                Some(b) => b,
                None => {
                    return Ok((
                        atoms::error(),
                        format!("failed to allocate binary of {} bytes", png.len()),
                    )
                        .encode(env));
                }
            };
            obin.as_mut_slice().copy_from_slice(&png);
            Ok((atoms::ok(), (page_index, obin.release(env))).encode(env))
        }
        Some(Err(e)) => Ok((atoms::error(), format!("Iterator error: {}", e)).encode(env)),
        None => Ok(atoms::done().encode(env)),
    }
}

/// Free the iterator resource.
#[rustler::nif]
pub fn render_pdf_pages_iter_free(resource: ResourceArc<PdfPageIteratorResource>) -> rustler::NifResult<()> {
    if let Ok(mut guard) = resource.inner.lock() {
        *guard = None;
    }
    Ok(())
}

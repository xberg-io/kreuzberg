use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::WORKER_POOL;
use crate::config::JsExtractionConfig;
use crate::error_handling::convert_error;
use crate::result::{JsExtractionResult, resolve_config};

#[napi]
pub fn extract_file_sync(
    file_path: String,
    mime_type: Option<String>,
    config: Option<JsExtractionConfig>,
) -> Result<JsExtractionResult> {
    let rust_config = resolve_config(config)?;

    kreuzberg::extract_file_sync(&file_path, mime_type.as_deref(), &rust_config)
        .map_err(convert_error)
        .and_then(JsExtractionResult::try_from)
}

/// Extract content from a file (asynchronous).
///
/// Asynchronously extracts text, tables, images, and metadata from a document file.
/// Non-blocking alternative to `extractFileSync` for use in async/await contexts.
///
/// # Parameters
///
/// * `file_path` - Path to the file to extract (absolute or relative)
/// * `mime_type` - Optional MIME type hint (auto-detected if omitted)
/// * `config` - Optional extraction configuration (OCR, chunking, etc.)
///
/// # Returns
///
/// Promise resolving to `ExtractionResult` with extracted content and metadata.
///
/// # Errors
///
/// Rejects if file processing fails (see `extractFileSync` for error conditions).
///
/// # Example
///
/// ```typescript
/// import { extractFile } from '@kreuzberg/node';
///
/// // Async/await usage
/// const result = await extractFile('document.pdf', null, null);
/// console.log(result.content);
///
/// // Promise usage
/// extractFile('report.docx', null, null)
///   .then(result => console.log(result.content))
///   .catch(err => console.error(err));
/// ```
#[napi]
pub async fn extract_file(
    file_path: String,
    mime_type: Option<String>,
    config: Option<JsExtractionConfig>,
) -> Result<JsExtractionResult> {
    let rust_config = resolve_config(config)?;

    let result = WORKER_POOL
        .spawn_blocking(move || kreuzberg::extract_file_sync(&file_path, mime_type.as_deref(), &rust_config))
        .await
        .map_err(|e| Error::from_reason(format!("Worker thread error: {}", e)))?
        .map_err(convert_error)?;

    JsExtractionResult::try_from(result)
}

/// Extract content from bytes (synchronous).
///
/// Synchronously extracts content from a byte buffer without requiring a file path.
/// Useful for processing in-memory data, network streams, or database BLOBs.
///
/// # Parameters
///
/// * `data` - Buffer containing the document bytes
/// * `mime_type` - MIME type of the data (e.g., "application/pdf", "image/png")
/// * `config` - Optional extraction configuration
///
/// # Returns
///
/// `ExtractionResult` with extracted content and metadata.
///
/// # Errors
///
/// Throws an error if data is malformed or MIME type is unsupported.
///
/// # Example
///
/// ```typescript
/// import { extractBytesSync } from '@kreuzberg/node';
/// import fs from 'fs';
///
/// const buffer = fs.readFileSync('document.pdf');
/// const result = extractBytesSync(buffer, 'application/pdf', null);
/// console.log(result.content);
/// ```
#[napi]
pub fn extract_bytes_sync(
    data: Buffer,
    mime_type: String,
    config: Option<JsExtractionConfig>,
) -> Result<JsExtractionResult> {
    let rust_config = resolve_config(config)?;

    let bytes = data.as_ref();

    kreuzberg::extract_bytes_sync(bytes, &mime_type, &rust_config)
        .map_err(convert_error)
        .and_then(JsExtractionResult::try_from)
}

/// Extract content from bytes (asynchronous).
///
/// Asynchronously extracts content from a byte buffer. Non-blocking alternative
/// to `extractBytesSync` for processing in-memory data.
///
/// # Parameters
///
/// * `data` - Buffer containing the document bytes
/// * `mime_type` - MIME type of the data
/// * `config` - Optional extraction configuration
///
/// # Returns
///
/// Promise resolving to `ExtractionResult`.
///
/// # Example
///
/// ```typescript
/// import { extractBytes } from '@kreuzberg/node';
///
/// const response = await fetch('https://example.com/document.pdf');
/// const buffer = Buffer.from(await response.arrayBuffer());
/// const result = await extractBytes(buffer, 'application/pdf', null);
/// ```
#[napi]
pub async fn extract_bytes(
    data: Buffer,
    mime_type: String,
    config: Option<JsExtractionConfig>,
) -> Result<JsExtractionResult> {
    let rust_config = resolve_config(config)?;
    let data_vec = data.to_vec();

    let result = WORKER_POOL
        .spawn_blocking(move || kreuzberg::extract_bytes_sync(&data_vec, &mime_type, &rust_config))
        .await
        .map_err(|e| Error::from_reason(format!("Worker thread error: {}", e)))?
        .map_err(convert_error)?;

    JsExtractionResult::try_from(result)
}

/// Render a single page of a PDF file to a PNG buffer (synchronous).
///
/// # Parameters
///
/// * `file_path` - Path to the PDF file
/// * `page_index` - Zero-based page index
/// * `dpi` - Optional DPI (default 150)
///
/// # Returns
///
/// Buffer containing PNG image data.
#[napi]
pub fn render_pdf_page_sync(file_path: String, page_index: u32, dpi: Option<i32>) -> Result<Buffer> {
    let pdf_bytes = std::fs::read(&file_path).map_err(|e| convert_error(kreuzberg::KreuzbergError::Io(e)))?;
    let page = kreuzberg::pdf::render_pdf_page_to_png(&pdf_bytes, page_index as usize, dpi, None)
        .map_err(|e| convert_error(e.into()))?;
    Ok(Buffer::from(page.as_slice()))
}

/// Render a single page of a PDF file to a PNG buffer (asynchronous).
///
/// Non-blocking alternative to `renderPdfPageSync`.
///
/// # Parameters
///
/// * `file_path` - Path to the PDF file
/// * `page_index` - Zero-based page index
/// * `dpi` - Optional DPI (default 150)
///
/// # Returns
///
/// Promise resolving to a Buffer containing PNG image data.
#[napi]
pub async fn render_pdf_page(file_path: String, page_index: u32, dpi: Option<i32>) -> Result<Buffer> {
    let result = WORKER_POOL
        .spawn_blocking(move || {
            let pdf_bytes = std::fs::read(&file_path).map_err(kreuzberg::KreuzbergError::Io)?;
            kreuzberg::pdf::render_pdf_page_to_png(&pdf_bytes, page_index as usize, dpi, None)
                .map_err(|e| kreuzberg::KreuzbergError::Other(e.to_string()))
        })
        .await
        .map_err(|e| Error::from_reason(format!("Worker thread error: {}", e)))?
        .map_err(convert_error)?;

    Ok(Buffer::from(result.as_slice()))
}

/// A rendered PDF page with its zero-based index and PNG data.
#[napi(object)]
pub struct PdfPageResult {
    /// Zero-based page index.
    pub page_index: u32,
    /// PNG image data.
    pub data: Buffer,
}

/// Create a PDF page iterator and collect all pages (synchronous).
///
/// Opens the PDF once and renders pages lazily, returning an array of
/// `{ pageIndex, data }` objects. Each page is rendered one at a time so
/// only one raw image is in memory at a time.
///
/// Note: Pages are collected eagerly into an array. For true lazy iteration,
/// use `new PdfPageIterator(filePath, dpi)` which exposes a `.next()` method
/// that renders one page at a time.
///
/// # Parameters
///
/// * `file_path` - Path to the PDF file
/// * `dpi` - Optional DPI (default 150)
///
/// # Returns
///
/// Array of `PdfPageResult` objects.
#[napi]
pub fn iterate_pdf_pages_sync(file_path: String, dpi: Option<i32>) -> Result<Vec<PdfPageResult>> {
    let iter =
        kreuzberg::pdf::PdfPageIterator::from_file(&file_path, dpi, None).map_err(|e| convert_error(e.into()))?;

    let mut results = Vec::with_capacity(iter.page_count());
    for item in iter {
        let (page_index, png) = item.map_err(|e| convert_error(e.into()))?;
        results.push(PdfPageResult {
            page_index: page_index as u32,
            data: Buffer::from(png.as_slice()),
        });
    }
    Ok(results)
}

/// Create a PDF page iterator and collect all pages (asynchronous).
///
/// Non-blocking variant of `iteratePdfPagesSync`. Rendering is offloaded
/// to the worker pool.
///
/// Note: Pages are collected eagerly into an array. For true lazy iteration,
/// use `new PdfPageIterator(filePath, dpi)` which exposes a `.next()` method
/// that renders one page at a time.
///
/// # Parameters
///
/// * `file_path` - Path to the PDF file
/// * `dpi` - Optional DPI (default 150)
///
/// # Returns
///
/// Promise resolving to an array of `PdfPageResult` objects.
#[napi]
pub async fn iterate_pdf_pages(file_path: String, dpi: Option<i32>) -> Result<Vec<PdfPageResult>> {
    let pages = WORKER_POOL
        .spawn_blocking(move || {
            let iter = kreuzberg::pdf::PdfPageIterator::from_file(&file_path, dpi, None)
                .map_err(|e| kreuzberg::KreuzbergError::Other(e.to_string()))?;

            let mut results = Vec::with_capacity(iter.page_count());
            for item in iter {
                let (page_index, png) = item.map_err(|e| kreuzberg::KreuzbergError::Other(e.to_string()))?;
                results.push((page_index, png));
            }
            Ok::<_, kreuzberg::KreuzbergError>(results)
        })
        .await
        .map_err(|e| Error::from_reason(format!("Worker thread error: {}", e)))?
        .map_err(convert_error)?;

    Ok(pages
        .into_iter()
        .map(|(page_index, png)| PdfPageResult {
            page_index: page_index as u32,
            data: Buffer::from(png.as_slice()),
        })
        .collect())
}

/// Lazy PDF page iterator. A more memory-efficient alternative to
/// `iteratePdfPagesSync`/`iteratePdfPages` when memory is a concern or when
/// pages should be processed as they are rendered (e.g., sending each page to
/// a vision model for OCR).
///
/// Renders one page at a time via the `.next()` method. Callers must call
/// `.close()` when done to free native resources.
///
/// # Example
///
/// ```javascript
/// const iter = new PdfPageIterator("doc.pdf", 150);
/// let result;
/// while ((result = iter.next()) !== null) {
///     const { pageIndex, data } = result;
///     // process page...
/// }
/// iter.close();
/// ```
#[napi]
pub struct JsPdfPageIterator {
    inner: Option<kreuzberg::pdf::PdfPageIterator>,
}

#[napi]
impl JsPdfPageIterator {
    /// Create a new PDF page iterator.
    ///
    /// # Parameters
    ///
    /// * `file_path` - Path to the PDF file
    /// * `dpi` - Optional DPI (default 150)
    #[napi(constructor)]
    pub fn new(file_path: String, dpi: Option<i32>) -> Result<Self> {
        let iter =
            kreuzberg::pdf::PdfPageIterator::from_file(&file_path, dpi, None).map_err(|e| convert_error(e.into()))?;
        Ok(Self { inner: Some(iter) })
    }

    /// Advance the iterator and return the next page.
    ///
    /// Returns `{ pageIndex, data }` or `null` when exhausted.
    #[napi]
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<Option<PdfPageResult>> {
        let iter = match &mut self.inner {
            Some(it) => it,
            None => return Ok(None),
        };

        match iter.next() {
            Some(Ok((page_index, png))) => Ok(Some(PdfPageResult {
                page_index: page_index as u32,
                data: Buffer::from(png.as_slice()),
            })),
            Some(Err(e)) => Err(convert_error(e.into())),
            None => Ok(None),
        }
    }

    /// Total number of pages in the PDF.
    #[napi]
    pub fn page_count(&self) -> u32 {
        match &self.inner {
            Some(it) => it.page_count() as u32,
            None => 0,
        }
    }

    /// Free native resources. Safe to call multiple times.
    #[napi]
    pub fn close(&mut self) {
        self.inner = None;
    }
}

/// Get the number of pages in a PDF file without rendering.
///
/// # Parameters
///
/// * `file_path` - Path to the PDF file
/// * `dpi` - Optional DPI (not used for counting, but validates the PDF)
///
/// # Returns
///
/// Number of pages in the PDF.
#[napi]
pub fn pdf_page_count(file_path: String, dpi: Option<i32>) -> Result<u32> {
    let iter =
        kreuzberg::pdf::PdfPageIterator::from_file(&file_path, dpi, None).map_err(|e| convert_error(e.into()))?;
    Ok(iter.page_count() as u32)
}

//! PDF page rendering FFI functions.
//!
//! Provides C-compatible functions for rendering PDF pages to PNG byte buffers.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use crate::ffi_panic_guard;
use crate::helpers::{clear_last_error, set_last_error};

/// A single rendered page image (PNG bytes).
#[repr(C)]
pub struct CPageImage {
    /// Pointer to PNG data. Owned by this struct; freed via `kreuzberg_free_render_page_result`.
    pub data: *mut u8,
    /// Length of PNG data in bytes.
    pub len: usize,
}

/// A single page from the PDF page iterator, including its zero-based index.
#[repr(C)]
pub struct CPageIterResult {
    /// Zero-based page index within the PDF.
    pub page_index: usize,
    /// Pointer to PNG data. Owned by this struct; freed via `kreuzberg_pdf_page_iterator_free_result`.
    pub data: *mut u8,
    /// Length of PNG data in bytes.
    pub len: usize,
}

/// Render a single page of a PDF file to a PNG byte buffer.
///
/// # Safety
///
/// - `file_path` must be a valid null-terminated C string
/// - The returned pointer must be freed with `kreuzberg_free_render_page_result`
/// - Returns NULL on panic (check `kreuzberg_last_error`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_render_pdf_page(
    file_path: *const c_char,
    page_index: usize,
    dpi: i32,
) -> *mut CPageImage {
    ffi_panic_guard!("kreuzberg_render_pdf_page", {
        clear_last_error();

        if file_path.is_null() {
            set_last_error("file_path cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let path_str = match unsafe { CStr::from_ptr(file_path) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8 in file path: {}", e));
                return ptr::null_mut();
            }
        };

        let dpi_opt = if dpi <= 0 { None } else { Some(dpi) };
        let pdf_bytes = match std::fs::read(path_str) {
            Ok(b) => b,
            Err(e) => {
                set_last_error(format!("Failed to read file: {}", e));
                return ptr::null_mut();
            }
        };

        match kreuzberg::pdf::render_pdf_page_to_png(&pdf_bytes, page_index, dpi_opt, None) {
            Ok(png) => {
                let mut boxed = png.into_boxed_slice();
                let data = boxed.as_mut_ptr();
                let len = boxed.len();
                std::mem::forget(boxed);

                Box::into_raw(Box::new(CPageImage { data, len }))
            }
            Err(e) => {
                set_last_error(e.to_string());
                ptr::null_mut()
            }
        }
    })
}

/// Free a single page result returned by `kreuzberg_render_pdf_page`.
///
/// # Safety
///
/// - `page` must be a pointer returned by `kreuzberg_render_pdf_page`, or NULL (no-op)
/// - `page` must not be used after this call
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_free_render_page_result(page: *mut CPageImage) {
    if page.is_null() {
        return;
    }

    let page_box = unsafe { Box::from_raw(page) };

    if !page_box.data.is_null() {
        unsafe {
            drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                page_box.data,
                page_box.len,
            )));
        }
    }
}

/// Opaque handle to a PDF page iterator.
///
/// Created by `kreuzberg_pdf_page_iterator_new`, freed by
/// `kreuzberg_pdf_page_iterator_free`.
#[repr(C)]
pub struct CPdfPageIterator {
    _private: [u8; 0],
}

/// Create a new PDF page iterator from a file path.
///
/// # Safety
///
/// - `file_path` must be a valid null-terminated C string
/// - The returned pointer must be freed with `kreuzberg_pdf_page_iterator_free`
/// - Returns NULL on error (check `kreuzberg_last_error`)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_pdf_page_iterator_new(file_path: *const c_char, dpi: i32) -> *mut CPdfPageIterator {
    ffi_panic_guard!("kreuzberg_pdf_page_iterator_new", {
        clear_last_error();

        if file_path.is_null() {
            set_last_error("file_path cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let path_str = match unsafe { CStr::from_ptr(file_path) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                set_last_error(format!("Invalid UTF-8 in file path: {}", e));
                return ptr::null_mut();
            }
        };

        let dpi_opt = if dpi <= 0 { None } else { Some(dpi) };

        match kreuzberg::pdf::PdfPageIterator::from_file(path_str, dpi_opt, None) {
            Ok(iter) => Box::into_raw(Box::new(iter)) as *mut CPdfPageIterator,
            Err(e) => {
                set_last_error(e.to_string());
                ptr::null_mut()
            }
        }
    })
}

/// Advance the iterator and return the next rendered page with its page index.
///
/// Returns NULL when iteration is complete. Check `kreuzberg_last_error()` to
/// distinguish exhaustion (no error) from failure (error set).
///
/// # Safety
///
/// - `iter` must be a valid pointer returned by `kreuzberg_pdf_page_iterator_new`
/// - The returned `CPageIterResult` must be freed with `kreuzberg_pdf_page_iterator_free_result`
/// - Returns NULL when the iterator is exhausted (no error) or on error (error set)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_pdf_page_iterator_next(iter: *mut CPdfPageIterator) -> *mut CPageIterResult {
    ffi_panic_guard!("kreuzberg_pdf_page_iterator_next", {
        clear_last_error();

        if iter.is_null() {
            set_last_error("iter cannot be NULL".to_string());
            return ptr::null_mut();
        }

        let iterator = unsafe { &mut *(iter as *mut kreuzberg::pdf::PdfPageIterator) };

        match iterator.next() {
            Some(Ok((page_index, png))) => {
                let mut boxed = png.into_boxed_slice();
                let data = boxed.as_mut_ptr();
                let len = boxed.len();
                std::mem::forget(boxed);
                Box::into_raw(Box::new(CPageIterResult { page_index, data, len }))
            }
            Some(Err(e)) => {
                set_last_error(e.to_string());
                ptr::null_mut()
            }
            None => ptr::null_mut(),
        }
    })
}

/// Free a single iterator result returned by `kreuzberg_pdf_page_iterator_next`.
///
/// # Safety
///
/// - `result` must be a pointer returned by `kreuzberg_pdf_page_iterator_next`, or NULL (no-op)
/// - `result` must not be used after this call
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_pdf_page_iterator_free_result(result: *mut CPageIterResult) {
    if result.is_null() {
        return;
    }

    let result_box = unsafe { Box::from_raw(result) };

    if !result_box.data.is_null() {
        unsafe {
            drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                result_box.data,
                result_box.len,
            )));
        }
    }
}

/// Return the total number of pages in the PDF.
///
/// # Safety
///
/// - `iter` must be a valid pointer returned by `kreuzberg_pdf_page_iterator_new`
/// - Returns 0 if `iter` is NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_pdf_page_iterator_page_count(iter: *const CPdfPageIterator) -> usize {
    ffi_panic_guard!(
        "kreuzberg_pdf_page_iterator_page_count",
        {
            if iter.is_null() {
                return 0;
            }

            let iterator = unsafe { &*(iter as *const kreuzberg::pdf::PdfPageIterator) };
            iterator.page_count()
        },
        0
    )
}

/// Free a PDF page iterator.
///
/// # Safety
///
/// - `iter` must be a pointer returned by `kreuzberg_pdf_page_iterator_new`, or NULL (no-op)
/// - `iter` must not be used after this call
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kreuzberg_pdf_page_iterator_free(iter: *mut CPdfPageIterator) {
    if iter.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(iter as *mut kreuzberg::pdf::PdfPageIterator));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_page_image_size() {
        assert_eq!(
            std::mem::size_of::<CPageImage>(),
            16,
            "CPageImage must be 16 bytes (ptr + usize)"
        );
    }

    #[test]
    fn test_c_page_iter_result_size() {
        assert_eq!(
            std::mem::size_of::<CPageIterResult>(),
            24,
            "CPageIterResult must be 24 bytes (usize + ptr + usize)"
        );
    }

    #[test]
    fn test_free_render_page_result_null() {
        unsafe { kreuzberg_free_render_page_result(ptr::null_mut()) };
    }

    #[test]
    fn test_render_pdf_page_null_path() {
        let result = unsafe { kreuzberg_render_pdf_page(ptr::null(), 0, 150) };
        assert!(result.is_null());
    }

    #[test]
    fn test_iterator_new_null_path() {
        let result = unsafe { kreuzberg_pdf_page_iterator_new(ptr::null(), 150) };
        assert!(result.is_null());
    }

    #[test]
    fn test_iterator_next_null() {
        let result = unsafe { kreuzberg_pdf_page_iterator_next(ptr::null_mut()) };
        assert!(result.is_null());
    }

    #[test]
    fn test_iterator_free_result_null() {
        unsafe { kreuzberg_pdf_page_iterator_free_result(ptr::null_mut()) };
    }

    #[test]
    fn test_iterator_page_count_null() {
        let count = unsafe { kreuzberg_pdf_page_iterator_page_count(ptr::null()) };
        assert_eq!(count, 0);
    }

    #[test]
    fn test_iterator_free_null() {
        unsafe { kreuzberg_pdf_page_iterator_free(ptr::null_mut()) };
    }

    #[test]
    fn test_iterator_new_invalid_path() {
        let path = std::ffi::CString::new("/nonexistent/file.pdf").unwrap();
        let result = unsafe { kreuzberg_pdf_page_iterator_new(path.as_ptr(), 150) };
        assert!(result.is_null());
    }
}

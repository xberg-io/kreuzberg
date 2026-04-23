pub mod blank_detection;
pub mod derive;
pub mod structured;
pub mod text;
pub mod transform;

#[cfg(feature = "hwp")]
pub mod hwp;

#[cfg(any(feature = "ocr", feature = "ocr-wasm"))]
pub mod image;

/// Capacity estimation utilities for string pre-allocation.
///
/// This module provides functions to estimate the capacity needed for string buffers
/// based on input file sizes and content types. This enables pre-allocation, reducing
/// reallocation cycles during string building operations.
pub mod capacity;

#[cfg(feature = "archives")]
pub mod archive;

#[cfg(feature = "email")]
pub mod email;

#[cfg(feature = "email")]
pub mod pst;

#[cfg(any(feature = "excel", feature = "excel-wasm"))]
pub mod excel;

#[cfg(feature = "html")]
pub mod html;

#[cfg(feature = "office")]
pub mod doc;

#[cfg(feature = "office")]
pub mod docx;

#[cfg(feature = "office")]
pub mod office_metadata;

#[cfg(feature = "office")]
pub mod ooxml_constants;

#[cfg(feature = "office")]
pub mod ooxml_embedded;

#[cfg(feature = "office")]
pub mod image_format;

#[cfg(all(feature = "ocr", feature = "tokio-runtime"))]
pub mod image_ocr;

#[cfg(feature = "office")]
pub mod ppt;

#[cfg(feature = "office")]
pub mod pptx;

#[cfg(feature = "xml")]
pub mod xml;

#[cfg(any(feature = "office", feature = "html", feature = "xml"))]
pub mod markdown;

#[cfg(feature = "html")]
pub use html::convert_html_to_markdown;

#[cfg(feature = "office")]
pub use doc::extract_doc_text;

#[cfg(any(feature = "office", feature = "html", feature = "xml"))]
pub use markdown::cells_to_markdown;
#[cfg(any(feature = "office", feature = "html", feature = "xml"))]
pub use markdown::cells_to_text;

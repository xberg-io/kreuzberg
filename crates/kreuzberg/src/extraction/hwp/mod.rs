//! Vendored HWP text extraction from hwpers v0.5.0 (MIT OR Apache-2.0)
//!
//! Supports HWP 5.0 Compound File Binary (CFB) documents.  Only text
//! extraction is implemented; write, render, crypto, HWPX, and preview paths
//! from the original crate are omitted.
//!
//! # Entry point
//!
//! ```ignore
//! let text = extract_hwp_text(bytes)?;
//! ```

pub mod error;
pub mod model;
pub mod parser;
pub mod reader;

use error::{HwpError, Result};
use model::HwpDocument;
use parser::{FileHeader, parse_body_text};
use reader::CfbReader;

/// Extract all plain text from an HWP 5.0 document given its raw bytes.
///
/// # Errors
///
/// Returns `HwpError` if the bytes do not form a valid HWP 5.0 compound file,
/// if the document is password-encrypted, or if a critical parsing step fails.
pub(crate) fn extract_hwp_text(bytes: &[u8]) -> Result<String> {
    let mut cfb = CfbReader::from_bytes(bytes)?;

    // Parse the 256-byte file header
    let header_data = cfb.read_stream("FileHeader")?;
    let header = FileHeader::parse(header_data)?;

    if header.is_encrypted() {
        return Err(HwpError::UnsupportedVersion(
            "Password-encrypted HWP documents are not supported".to_string(),
        ));
    }

    // Distribution documents store body text under ViewText/SectionN
    let stream_prefix = if header.is_distribute() {
        "ViewText/Section"
    } else {
        "BodyText/Section"
    };

    let mut doc = HwpDocument::default();
    let mut section_idx = 0u32;

    loop {
        let section_name = format!("{stream_prefix}{section_idx}");
        if !cfb.stream_exists(&section_name) {
            break;
        }

        let section_data = cfb.read_stream(&section_name)?;
        let sections = parse_body_text(section_data, header.is_compressed())?;
        doc.sections.extend(sections);

        section_idx += 1;
    }

    if doc.sections.is_empty() {
        return Err(HwpError::InvalidFormat(
            "No BodyText sections found in HWP document".to_string(),
        ));
    }

    Ok(doc.extract_text())
}

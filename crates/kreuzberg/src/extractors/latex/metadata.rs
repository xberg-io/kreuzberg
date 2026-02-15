//! Metadata extraction for LaTeX documents.
//!
//! This module handles extraction of document metadata like title, author, and date
//! from LaTeX preamble commands.

use super::utilities::extract_braced;
use crate::types::Metadata;
use std::borrow::Cow;

/// Extracts metadata from a LaTeX line.
///
/// Looks for \title{}, \author{}, and \date{} commands and populates
/// the provided Metadata structure.
pub fn extract_metadata_from_line(line: &str, metadata: &mut Metadata) {
    if line.starts_with("\\title{") {
        if let Some(title) = extract_braced(line, "title") {
            if metadata.title.is_none() {
                metadata.title = Some(title.clone());
            }
            // DEPRECATED: kept for backward compatibility; will be removed in next major version.
            metadata.additional.insert(Cow::Borrowed("title"), title.into());
        }
    } else if line.starts_with("\\author{") {
        if let Some(author) = extract_braced(line, "author") {
            if metadata.created_by.is_none() {
                metadata.created_by = Some(author.clone());
            }
            // DEPRECATED: kept for backward compatibility; will be removed in next major version.
            metadata.additional.insert(Cow::Borrowed("author"), author.into());
        }
    } else if line.starts_with("\\date{")
        && let Some(date) = extract_braced(line, "date")
    {
        if metadata.created_at.is_none() {
            metadata.created_at = Some(date.clone());
        }
        // DEPRECATED: kept for backward compatibility; will be removed in next major version.
        metadata.additional.insert(Cow::Borrowed("date"), date.into());
    }
}

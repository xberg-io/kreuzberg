//! Metadata extraction for LaTeX documents.
//!
//! This module handles extraction of document metadata like title, author, and date
//! from LaTeX preamble commands.

use super::utilities::extract_braced;
use crate::types::Metadata;

/// Extracts metadata from a LaTeX line.
///
/// Looks for \title{}, \author{}, and \date{} commands and populates
/// the provided Metadata structure.
pub(crate) fn extract_metadata_from_line(line: &str, metadata: &mut Metadata) {
    if line.starts_with("\\title{") {
        if let Some(title) = extract_braced(line, "title")
            && metadata.title.is_none()
        {
            metadata.title = Some(title);
        }
    } else if line.starts_with("\\author{") {
        if let Some(author) = extract_braced(line, "author")
            && metadata.created_by.is_none()
        {
            metadata.created_by = Some(author);
        }
    } else if line.starts_with("\\date{")
        && let Some(date) = extract_braced(line, "date")
        && metadata.created_at.is_none()
    {
        metadata.created_at = Some(date);
    }
}

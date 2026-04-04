//! Corpus discovery and filtering for benchmark documents.
//!
//! Builds on the existing [`FixtureManager`] to provide structured corpus access
//! with filtering by file type, ground truth availability, and name patterns.

use crate::Result;
use crate::fixture::FixtureManager;
use std::path::{Path, PathBuf};

/// A document in the benchmark corpus with resolved paths.
#[derive(Debug, Clone)]
pub struct CorpusDocument {
    /// Human-readable name (fixture stem, e.g. "nougat_001")
    pub name: String,
    /// Absolute path to the source document
    pub document_path: PathBuf,
    /// File type (e.g. "pdf", "docx")
    pub file_type: String,
    /// File size in bytes
    pub file_size: u64,
    /// Absolute path to text ground truth (if available)
    pub ground_truth_text: Option<PathBuf>,
    /// Absolute path to markdown ground truth (if available)
    pub ground_truth_markdown: Option<PathBuf>,
}

/// Filter criteria for corpus discovery.
#[derive(Debug, Clone, Default)]
pub struct CorpusFilter {
    /// Only include these file types (None = all)
    pub file_types: Option<Vec<String>>,
    /// Require text ground truth
    pub require_ground_truth: bool,
    /// Require markdown ground truth
    pub require_markdown_ground_truth: bool,
    /// Maximum file size in bytes (None = no limit)
    pub max_file_size: Option<u64>,
    /// Only include fixtures whose name contains one of these strings
    pub name_patterns: Vec<String>,
}

/// Build a filtered corpus from the fixture directory.
pub fn build_corpus(fixtures_dir: &Path, filter: &CorpusFilter) -> Result<Vec<CorpusDocument>> {
    let mut manager = FixtureManager::new();
    if fixtures_dir.is_dir() {
        manager.load_fixtures_from_dir(fixtures_dir)?;
    } else {
        manager.load_fixture(fixtures_dir)?;
    }

    let mut docs = Vec::new();

    for (fixture_path, fixture) in manager.fixtures() {
        let fixture_dir = match fixture_path.parent() {
            Some(d) => d,
            None => continue,
        };

        let name = fixture_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        // Apply name filter (match ANY pattern)
        if !filter.name_patterns.is_empty() && !filter.name_patterns.iter().any(|p| name.contains(p.as_str())) {
            continue;
        }

        // Apply file type filter
        if let Some(ref types) = filter.file_types
            && !types.contains(&fixture.file_type)
        {
            continue;
        }

        // Apply file size filter
        if let Some(max_size) = filter.max_file_size
            && fixture.file_size > max_size
        {
            continue;
        }

        let document_path = fixture.resolve_document_path(fixture_dir);
        let gt_text = fixture.resolve_ground_truth_path(fixture_dir);
        let gt_markdown = fixture.resolve_ground_truth_markdown_path(fixture_dir);

        // Apply ground truth filters
        if filter.require_ground_truth && gt_text.is_none() {
            continue;
        }
        if filter.require_markdown_ground_truth && gt_markdown.is_none() {
            continue;
        }

        docs.push(CorpusDocument {
            name,
            document_path,
            file_type: fixture.file_type.clone(),
            file_size: fixture.file_size,
            ground_truth_text: gt_text,
            ground_truth_markdown: gt_markdown,
        });
    }

    docs.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(docs)
}

/// Convenience: all PDFs with text ground truth.
pub fn pdf_corpus(fixtures_dir: &Path) -> Result<Vec<CorpusDocument>> {
    build_corpus(
        fixtures_dir,
        &CorpusFilter {
            file_types: Some(vec!["pdf".to_string()]),
            require_ground_truth: true,
            ..Default::default()
        },
    )
}

/// Convenience: all PDFs with markdown ground truth.
pub fn pdf_markdown_corpus(fixtures_dir: &Path) -> Result<Vec<CorpusDocument>> {
    build_corpus(
        fixtures_dir,
        &CorpusFilter {
            file_types: Some(vec!["pdf".to_string()]),
            require_ground_truth: true,
            require_markdown_ground_truth: true,
            ..Default::default()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_filter_is_permissive() {
        let filter = CorpusFilter::default();
        assert!(filter.file_types.is_none());
        assert!(!filter.require_ground_truth);
        assert!(!filter.require_markdown_ground_truth);
        assert!(filter.max_file_size.is_none());
        assert!(filter.name_patterns.is_empty());
    }
}

//! PDF text hierarchy extraction using pdfium character positions.
//!
//! This module provides functions for extracting character information from PDFs,
//! preserving font size and position data for text hierarchy analysis.
//!
//! Note: Requires the "pdf" feature to be enabled.

mod bounding_box;
mod clustering;
mod extraction;

// Re-export all public types and functions for backward compatibility
pub use bounding_box::BoundingBox;
pub(crate) use clustering::{assign_heading_levels_smart, cluster_font_sizes};
pub(crate) use extraction::{
    HierarchyLevel, KMeansResult, SegmentData, TextBlock, assign_hierarchy_levels, extract_chars_with_fonts,
    merge_chars_into_blocks,
};

//! Shared data types for PDF hierarchy extraction (backend-agnostic).

use serde::{Deserialize, Serialize};

use super::bounding_box::BoundingBox;

/// A block of text with spatial and semantic information.
#[derive(Debug, Clone, PartialEq)]
pub struct TextBlock {
    /// The text content
    pub text: String,
    /// The bounding box of the block
    pub bbox: BoundingBox,
    /// The font size of the text in this block
    pub font_size: f32,
}

/// Result of KMeans clustering on font sizes.
#[derive(Debug, Clone)]
pub struct KMeansResult {
    /// Cluster label for each block (0-indexed)
    pub labels: Vec<u32>,
}

/// Hierarchy level assignment result.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyLevel {
    H1 = 1,
    H2 = 2,
    H3 = 3,
    H4 = 4,
    H5 = 5,
    H6 = 6,
    #[default]
    Body = 0,
}

/// Text segment data extracted from PDF.
///
/// Backend-agnostic: populated by either the pdf_oxide or another extractor.
#[derive(Debug, Clone)]
pub struct SegmentData {
    /// The segment text content (may contain spaces / multiple words)
    pub text: String,
    /// Left x position in PDF units
    pub x: f32,
    /// Bottom y position in PDF units (PDF coordinate system, y=0 at bottom)
    pub y: f32,
    /// Width of the segment bounding box
    pub width: f32,
    /// Height of the segment bounding box
    pub height: f32,
    /// Font size in points
    pub font_size: f32,
    /// Whether the font is bold
    pub is_bold: bool,
    /// Whether the font is italic
    pub is_italic: bool,
    /// Whether the font is monospace
    pub is_monospace: bool,
    /// Baseline Y position
    pub baseline_y: f32,
    /// Pre-assigned heading level from the PDF structure tree (1-6), or `None`
    /// when the heading level is unknown and must be inferred via font-size clustering.
    pub assigned_role: Option<u8>,
}

/// Assign hierarchy levels to text blocks based on KMeans clustering results.
pub(crate) fn assign_hierarchy_levels(blocks: &[TextBlock], kmeans_result: &KMeansResult) -> Vec<HierarchyBlock> {
    if blocks.is_empty() || kmeans_result.labels.is_empty() {
        return Vec::new();
    }

    blocks
        .iter()
        .zip(kmeans_result.labels.iter())
        .map(|(block, &cluster_id)| {
            let hierarchy_level = match cluster_id {
                0 => HierarchyLevel::H1,
                1 => HierarchyLevel::H2,
                2 => HierarchyLevel::H3,
                3 => HierarchyLevel::H4,
                4 => HierarchyLevel::H5,
                5 => HierarchyLevel::H6,
                _ => HierarchyLevel::Body,
            };
            HierarchyBlock {
                text: block.text.clone(),
                bbox: block.bbox,
                font_size: block.font_size,
                hierarchy_level,
            }
        })
        .collect()
}

/// A TextBlock with hierarchy level assignment.
#[derive(Debug, Clone)]
pub struct HierarchyBlock {
    pub text: String,
    pub bbox: BoundingBox,
    pub font_size: f32,
    pub hierarchy_level: HierarchyLevel,
}

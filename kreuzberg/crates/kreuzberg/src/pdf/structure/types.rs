//! Core types for the PDF-to-Markdown pipeline.

use crate::pdf::hierarchy::SegmentData;

/// A line of text composed of segments sharing a common baseline.
#[derive(Debug, Clone)]
pub(crate) struct PdfLine {
    pub segments: Vec<SegmentData>,
    pub baseline_y: f32,
    #[allow(dead_code)]
    pub dominant_font_size: f32,
    #[allow(dead_code)]
    pub is_bold: bool,
    pub is_monospace: bool,
}

/// A paragraph composed of lines, with optional heading classification.
#[derive(Debug, Clone)]
pub(crate) struct PdfParagraph {
    /// Full text content from `page.text().all()` (heuristic path).
    /// When populated, assembly uses this directly instead of joining segment texts.
    /// Empty for the structure tree path (which uses lines/segments).
    pub text: String,
    pub lines: Vec<PdfLine>,
    pub dominant_font_size: f32,
    pub heading_level: Option<u8>,
    pub is_bold: bool,
    pub is_list_item: bool,
    pub is_code_block: bool,
    pub is_formula: bool,
    pub is_page_furniture: bool,
    pub layout_class: Option<LayoutHintClass>,
    /// Index of the parent element this caption is associated with (tables/pictures).
    pub caption_for: Option<usize>,
    /// Block-level bounding box from structure tree extraction.
    /// Used for spatial matching when per-segment positions aren't available.
    /// Format: (left, bottom, right, top) in PDF coordinate space.
    pub block_bbox: Option<(f32, f32, f32, f32)>,
}

impl PdfParagraph {
    /// Check if this paragraph is monospace (full-text path uses is_code_block flag,
    /// structure tree path checks line-level flags).
    pub(crate) fn is_monospace_hint(&self) -> bool {
        self.is_code_block
    }
}

/// Simplified layout class for the markdown pipeline.
///
/// Decoupled from `crate::layout::LayoutClass` so the markdown module
/// compiles without the `layout-detection` feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Variants constructed via layout-detection feature
pub(crate) enum LayoutHintClass {
    Title,
    SectionHeader,
    Code,
    Formula,
    ListItem,
    Caption,
    Footnote,
    PageHeader,
    PageFooter,
    Table,
    Picture,
    Text,
    Other,
}

/// A layout hint for paragraph classification.
///
/// Contains a simplified layout class with confidence and bounding box
/// in PDF coordinate space (points, y=0 at bottom of page).
#[derive(Debug, Clone)]
pub(crate) struct LayoutHint {
    pub class: LayoutHintClass,
    pub confidence: f32,
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

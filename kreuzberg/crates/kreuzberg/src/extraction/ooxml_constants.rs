//! Shared Office Open XML (OOXML) namespace constants.
//!
//! These constants define the XML namespace URIs used across DOCX, PPTX, and XLSX
//! document formats. Centralizing them avoids duplication and ensures consistency.

/// DrawingML namespace - shared by DOCX/PPTX for images, shapes, charts, and themes.
///
/// Used in: `<a:...>` elements (text runs, fonts, colors, shapes)
pub const DRAWINGML_NAMESPACE: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";

/// WordprocessingML namespace - DOCX-specific content elements.
///
/// Used in: `<w:...>` elements (paragraphs, runs, tables, sections)
pub const WORDPROCESSINGML_NAMESPACE: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

/// PresentationML namespace - PPTX-specific content elements.
///
/// Used in: `<p:...>` elements (slides, shapes, transitions)
pub const PRESENTATIONML_NAMESPACE: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";

/// Office Document Relationships namespace.
///
/// Used in: `.rels` files for linking document parts (images, styles, etc.)
pub const RELATIONSHIPS_NAMESPACE: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

/// Package Relationships namespace.
///
/// Used in: `_rels/.rels` for top-level package relationships
pub const PACKAGE_RELATIONSHIPS_NAMESPACE: &str = "http://schemas.openxmlformats.org/package/2006/relationships";

/// Content Types namespace.
///
/// Used in: `[Content_Types].xml`
pub const CONTENT_TYPES_NAMESPACE: &str = "http://schemas.openxmlformats.org/package/2006/content-types";

/// WordprocessingDrawing namespace - DOCX drawing positioning and wrapping.
///
/// Used in: `<wp:...>` elements (inline, anchor, extent, positionH, positionV)
pub const WORDPROCESSINGDRAWING_NAMESPACE: &str =
    "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing";

/// Picture namespace - DrawingML picture elements.
///
/// Used in: `<pic:...>` elements (pic, blipFill)
pub const PICTURE_NAMESPACE: &str = "http://schemas.openxmlformats.org/drawingml/2006/picture";

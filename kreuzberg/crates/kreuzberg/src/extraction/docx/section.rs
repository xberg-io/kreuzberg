//! DOCX section properties parsing from `w:sectPr` elements.
//!
//! Parses page size, margins, orientation, columns, and other
//! section-level properties from OOXML documents.

use crate::extraction::ooxml_constants::WORDPROCESSINGML_NAMESPACE;
use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};

// --- Types ---

/// Page orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

/// Page margins in twips (twentieths of a point).
#[derive(Debug, Clone, Default)]
pub struct PageMargins {
    /// Top margin in twips.
    pub top: Option<i32>,
    /// Right margin in twips.
    pub right: Option<i32>,
    /// Bottom margin in twips.
    pub bottom: Option<i32>,
    /// Left margin in twips.
    pub left: Option<i32>,
    /// Header offset in twips.
    pub header: Option<i32>,
    /// Footer offset in twips.
    pub footer: Option<i32>,
    /// Gutter margin in twips.
    pub gutter: Option<i32>,
}

/// Page margins converted to points (1/72 inch).
#[derive(Debug, Clone, Default)]
pub struct PageMarginsPoints {
    pub top: Option<f64>,
    pub right: Option<f64>,
    pub bottom: Option<f64>,
    pub left: Option<f64>,
    pub header: Option<f64>,
    pub footer: Option<f64>,
    pub gutter: Option<f64>,
}

/// Column layout configuration.
#[derive(Debug, Clone, Default)]
pub struct ColumnLayout {
    /// Number of columns.
    pub count: Option<i32>,
    /// Space between columns in twips.
    pub space_twips: Option<i32>,
    /// Whether columns have equal width.
    pub equal_width: Option<bool>,
}

/// DOCX section properties parsed from `w:sectPr` element.
#[derive(Debug, Clone, Default)]
pub struct SectionProperties {
    /// Page width in twips (from `w:pgSz w:w`).
    pub page_width_twips: Option<i32>,
    /// Page height in twips (from `w:pgSz w:h`).
    pub page_height_twips: Option<i32>,
    /// Page orientation (from `w:pgSz w:orient`).
    pub orientation: Option<Orientation>,
    /// Page margins (from `w:pgMar`).
    pub margins: PageMargins,
    /// Column layout (from `w:cols`).
    pub columns: ColumnLayout,
    /// Document grid line pitch in twips (from `w:docGrid w:linePitch`).
    pub doc_grid_line_pitch: Option<i32>,
}

// --- Conversion helpers ---

impl PageMargins {
    /// Convert all margins from twips to points.
    ///
    /// Conversion factor: 1 twip = 1/20 point, or equivalently divide by 20.
    pub fn to_points(&self) -> PageMarginsPoints {
        PageMarginsPoints {
            top: self.top.map(|v| v as f64 / 20.0),
            right: self.right.map(|v| v as f64 / 20.0),
            bottom: self.bottom.map(|v| v as f64 / 20.0),
            left: self.left.map(|v| v as f64 / 20.0),
            header: self.header.map(|v| v as f64 / 20.0),
            footer: self.footer.map(|v| v as f64 / 20.0),
            gutter: self.gutter.map(|v| v as f64 / 20.0),
        }
    }
}

impl SectionProperties {
    /// Convert page width from twips to points.
    pub fn page_width_points(&self) -> Option<f64> {
        self.page_width_twips.map(|v| v as f64 / 20.0)
    }

    /// Convert page height from twips to points.
    pub fn page_height_points(&self) -> Option<f64> {
        self.page_height_twips.map(|v| v as f64 / 20.0)
    }
}

// --- XML Helpers ---

/// Get a namespaced integer attribute.
fn get_w_attr_i32(element: &BytesStart, local_name: &str) -> Option<i32> {
    // First try "w:localname" format
    let w_prefixed = format!("w:{}", local_name);
    for attr in element.attributes().flatten() {
        let key = attr.key.as_ref();
        if (key == w_prefixed.as_bytes() || key == local_name.as_bytes())
            && let Ok(val) = std::str::from_utf8(&attr.value)
            && let Ok(num) = val.parse::<i32>()
        {
            return Some(num);
        }
    }
    None
}

/// Get a namespaced string attribute.
fn get_w_attr_string(element: &BytesStart, local_name: &str) -> Option<String> {
    for attr in element.attributes().flatten() {
        let key_str = std::str::from_utf8(attr.key.as_ref()).ok()?;
        if (key_str == local_name || key_str.ends_with(&format!(":{}", local_name)))
            && let Ok(val) = std::str::from_utf8(&attr.value)
        {
            return Some(val.to_string());
        }
    }
    None
}

/// Check if an attribute has a specific value (for roxmltree).
fn roxmltree_get_attr(node: &roxmltree::Node, local_name: &str) -> Option<String> {
    node.attribute((WORDPROCESSINGML_NAMESPACE, local_name))
        .or_else(|| node.attribute(local_name))
        .map(String::from)
}

/// Get a namespaced integer attribute (for roxmltree).
fn roxmltree_get_i32_attr(node: &roxmltree::Node, local_name: &str) -> Option<i32> {
    node.attribute((WORDPROCESSINGML_NAMESPACE, local_name))
        .or_else(|| node.attribute(local_name))
        .and_then(|v| v.parse::<i32>().ok())
}

// --- Parsing with roxmltree ---

/// Parse a `w:sectPr` XML element (roxmltree node) into `SectionProperties`.
pub fn parse_section_properties(node: &roxmltree::Node) -> SectionProperties {
    let mut props = SectionProperties::default();

    for child in node.children() {
        if !child.is_element() {
            continue;
        }

        match child.tag_name().name() {
            "pgSz" => {
                props.page_width_twips = roxmltree_get_i32_attr(&child, "w");
                props.page_height_twips = roxmltree_get_i32_attr(&child, "h");

                // ECMA-376: default orientation is portrait when w:orient is absent
                props.orientation = match roxmltree_get_attr(&child, "orient").as_deref() {
                    Some("landscape") => Some(Orientation::Landscape),
                    _ => Some(Orientation::Portrait),
                };
            }
            "pgMar" => {
                props.margins.top = roxmltree_get_i32_attr(&child, "top");
                props.margins.right = roxmltree_get_i32_attr(&child, "right");
                props.margins.bottom = roxmltree_get_i32_attr(&child, "bottom");
                props.margins.left = roxmltree_get_i32_attr(&child, "left");
                props.margins.header = roxmltree_get_i32_attr(&child, "header");
                props.margins.footer = roxmltree_get_i32_attr(&child, "footer");
                props.margins.gutter = roxmltree_get_i32_attr(&child, "gutter");
            }
            "cols" => {
                props.columns.count = roxmltree_get_i32_attr(&child, "num");
                props.columns.space_twips = roxmltree_get_i32_attr(&child, "space");

                if let Some(eq_width_str) = roxmltree_get_attr(&child, "equalWidth") {
                    props.columns.equal_width = Some(eq_width_str == "1" || eq_width_str == "true");
                }
            }
            "docGrid" => {
                props.doc_grid_line_pitch = roxmltree_get_i32_attr(&child, "linePitch");
            }
            _ => {}
        }
    }

    props
}

// --- Streaming parser for quick_xml ---

/// Parse section properties from a quick_xml event stream.
///
/// Reads events from the reader until `</w:sectPr>` is encountered,
/// extracting the same properties as the roxmltree parser.
///
/// **Important:** This function advances the reader past the closing `</w:sectPr>` tag.
/// The caller must not attempt to process the `w:sectPr` end event again.
pub fn parse_section_properties_streaming(reader: &mut Reader<&[u8]>) -> SectionProperties {
    let mut props = SectionProperties::default();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                apply_section_element(e, &mut props);
            }
            Ok(Event::End(ref e)) if e.name().as_ref() as &[u8] == b"w:sectPr" => {
                break;
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }

    // Apply OOXML defaults: absent orientation means portrait
    if props.page_width_twips.is_some() && props.orientation.is_none() {
        props.orientation = Some(Orientation::Portrait);
    }

    props
}

/// Extract section properties from a quick_xml element.
///
/// Shared handler for both `Event::Start` and `Event::Empty` events,
/// since OOXML section child elements (`w:pgSz`, `w:pgMar`, etc.)
/// are typically self-closing.
fn apply_section_element(e: &BytesStart, props: &mut SectionProperties) {
    match e.name().as_ref() as &[u8] {
        b"w:pgSz" => {
            props.page_width_twips = get_w_attr_i32(e, "w");
            props.page_height_twips = get_w_attr_i32(e, "h");
            props.orientation = get_w_attr_string(e, "orient").and_then(|s| match s.as_str() {
                "landscape" => Some(Orientation::Landscape),
                "portrait" => Some(Orientation::Portrait),
                _ => None,
            });
        }
        b"w:pgMar" => {
            props.margins.top = get_w_attr_i32(e, "top");
            props.margins.right = get_w_attr_i32(e, "right");
            props.margins.bottom = get_w_attr_i32(e, "bottom");
            props.margins.left = get_w_attr_i32(e, "left");
            props.margins.header = get_w_attr_i32(e, "header");
            props.margins.footer = get_w_attr_i32(e, "footer");
            props.margins.gutter = get_w_attr_i32(e, "gutter");
        }
        b"w:cols" => {
            props.columns.count = get_w_attr_i32(e, "num");
            props.columns.space_twips = get_w_attr_i32(e, "space");
            if let Some(eq_width_str) = get_w_attr_string(e, "equalWidth") {
                props.columns.equal_width = Some(eq_width_str == "1" || eq_width_str == "true");
            }
        }
        b"w:docGrid" => {
            props.doc_grid_line_pitch = get_w_attr_i32(e, "linePitch");
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_a4_page_size() {
        let xml = r#"<?xml version="1.0"?>
<w:sectPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:pgSz w:w="11906" w:h="16838"/>
</w:sectPr>"#;

        let doc = roxmltree::Document::parse(xml).unwrap();
        let root = doc.root_element();
        let props = parse_section_properties(&root);

        assert_eq!(props.page_width_twips, Some(11906));
        assert_eq!(props.page_height_twips, Some(16838));

        // Convert to points
        assert_eq!(props.page_width_points(), Some(595.3));
        assert_eq!(props.page_height_points(), Some(841.9));
    }

    #[test]
    fn test_parse_us_letter_page_size() {
        let xml = r#"<?xml version="1.0"?>
<w:sectPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:pgSz w:w="12240" w:h="15840"/>
</w:sectPr>"#;

        let doc = roxmltree::Document::parse(xml).unwrap();
        let root = doc.root_element();
        let props = parse_section_properties(&root);

        assert_eq!(props.page_width_twips, Some(12240));
        assert_eq!(props.page_height_twips, Some(15840));

        // 12240 / 20 = 612.0 points, 15840 / 20 = 792.0 points (US Letter)
        assert_eq!(props.page_width_points(), Some(612.0));
        assert_eq!(props.page_height_points(), Some(792.0));
    }

    #[test]
    fn test_parse_landscape_orientation() {
        let xml = r#"<?xml version="1.0"?>
<w:sectPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:pgSz w:w="16838" w:h="11906" w:orient="landscape"/>
</w:sectPr>"#;

        let doc = roxmltree::Document::parse(xml).unwrap();
        let root = doc.root_element();
        let props = parse_section_properties(&root);

        assert_eq!(props.orientation, Some(Orientation::Landscape));
        assert_eq!(props.page_width_twips, Some(16838));
        assert_eq!(props.page_height_twips, Some(11906));
    }

    #[test]
    fn test_parse_margins_all_fields() {
        let xml = r#"<?xml version="1.0"?>
<w:sectPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440" w:header="720" w:footer="720" w:gutter="0"/>
</w:sectPr>"#;

        let doc = roxmltree::Document::parse(xml).unwrap();
        let root = doc.root_element();
        let props = parse_section_properties(&root);

        assert_eq!(props.margins.top, Some(1440));
        assert_eq!(props.margins.right, Some(1440));
        assert_eq!(props.margins.bottom, Some(1440));
        assert_eq!(props.margins.left, Some(1440));
        assert_eq!(props.margins.header, Some(720));
        assert_eq!(props.margins.footer, Some(720));
        assert_eq!(props.margins.gutter, Some(0));

        // Convert to points
        let margins_points = props.margins.to_points();
        assert_eq!(margins_points.top, Some(72.0)); // 1440 / 20
        assert_eq!(margins_points.header, Some(36.0)); // 720 / 20
    }

    #[test]
    fn test_parse_columns() {
        let xml = r#"<?xml version="1.0"?>
<w:sectPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:cols w:num="2" w:space="720" w:equalWidth="1"/>
</w:sectPr>"#;

        let doc = roxmltree::Document::parse(xml).unwrap();
        let root = doc.root_element();
        let props = parse_section_properties(&root);

        assert_eq!(props.columns.count, Some(2));
        assert_eq!(props.columns.space_twips, Some(720));
        assert_eq!(props.columns.equal_width, Some(true));
    }

    #[test]
    fn test_parse_empty_section_properties() {
        let xml = r#"<?xml version="1.0"?>
<w:sectPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
</w:sectPr>"#;

        let doc = roxmltree::Document::parse(xml).unwrap();
        let root = doc.root_element();
        let props = parse_section_properties(&root);

        assert_eq!(props.page_width_twips, None);
        assert_eq!(props.page_height_twips, None);
        assert_eq!(props.orientation, None);
        assert_eq!(props.margins.top, None);
        assert_eq!(props.columns.count, None);
        assert_eq!(props.doc_grid_line_pitch, None);
    }

    #[test]
    fn test_streaming_parse_a4_page_size() {
        // Streaming parser must handle self-closing elements (Event::Empty)
        let xml = r#"<w:sectPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:pgSz w:w="11906" w:h="16838"/>
            <w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440" w:header="708" w:footer="708" w:gutter="0"/>
            <w:cols w:space="708"/>
            <w:docGrid w:linePitch="360"/>
        </w:sectPr>"#;

        // Simulate what happens in the parser: reader has already consumed <w:sectPr>,
        // so parse_section_properties_streaming reads from after that opening tag.
        // We need to read past the opening tag first.
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);
        let mut buf = Vec::new();

        // Consume the opening <w:sectPr> tag
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() as &[u8] == b"w:sectPr" => break,
                Ok(Event::Eof) => panic!("unexpected EOF"),
                Err(e) => panic!("unexpected error: {}", e),
                _ => {}
            }
            buf.clear();
        }

        let props = parse_section_properties_streaming(&mut reader);

        assert_eq!(props.page_width_twips, Some(11906));
        assert_eq!(props.page_height_twips, Some(16838));
        assert_eq!(props.orientation, Some(Orientation::Portrait)); // Default when absent
        assert_eq!(props.margins.top, Some(1440));
        assert_eq!(props.margins.right, Some(1440));
        assert_eq!(props.margins.header, Some(708));
        assert_eq!(props.margins.gutter, Some(0));
        assert_eq!(props.columns.space_twips, Some(708));
        assert_eq!(props.doc_grid_line_pitch, Some(360));
    }

    #[test]
    fn test_streaming_parse_landscape() {
        let xml = r#"<w:sectPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:pgSz w:w="16838" w:h="11906" w:orient="landscape"/>
        </w:sectPr>"#;

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(false);
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) if e.name().as_ref() as &[u8] == b"w:sectPr" => break,
                Ok(Event::Eof) => panic!("unexpected EOF"),
                _ => {}
            }
            buf.clear();
        }

        let props = parse_section_properties_streaming(&mut reader);
        assert_eq!(props.orientation, Some(Orientation::Landscape));
        assert_eq!(props.page_width_twips, Some(16838));
    }

    #[test]
    fn test_page_margins_conversion() {
        let margins = PageMargins {
            top: Some(1440),
            right: Some(1080),
            bottom: Some(1440),
            left: Some(1080),
            header: Some(720),
            footer: Some(720),
            gutter: None,
        };

        let points = margins.to_points();

        assert_eq!(points.top, Some(72.0));
        assert_eq!(points.right, Some(54.0));
        assert_eq!(points.bottom, Some(72.0));
        assert_eq!(points.left, Some(54.0));
        assert_eq!(points.header, Some(36.0));
        assert_eq!(points.footer, Some(36.0));
        assert_eq!(points.gutter, None);
    }

    #[test]
    fn test_section_properties_with_doc_grid() {
        let xml = r#"<?xml version="1.0"?>
<w:sectPr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
    <w:docGrid w:linePitch="360"/>
</w:sectPr>"#;

        let doc = roxmltree::Document::parse(xml).unwrap();
        let root = doc.root_element();
        let props = parse_section_properties(&root);

        assert_eq!(props.doc_grid_line_pitch, Some(360));
    }
}

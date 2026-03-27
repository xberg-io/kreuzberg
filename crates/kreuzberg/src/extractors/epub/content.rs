//! EPUB content extraction and text processing.
//!
//! Handles extraction of text content from XHTML files in spine order.
//!
//! For `OutputFormat::Plain`, this uses direct XML tree traversal to avoid lossy conversions.
//! For `OutputFormat::Markdown` / `OutputFormat::Djot`, this converts XHTML through the HTML
//! conversion pipeline so structural elements (like headings) are preserved.

use crate::types::ProcessingWarning;
use ahash::AHashSet;
use std::cmp::Ordering;
use std::io::Cursor;
use zip::ZipArchive;

use super::metadata::{EpubPackageDocument, ManifestItem};
use super::parsing::read_file_from_zip;

#[derive(Debug, Clone)]
/// A resolved XHTML spine document prepared for EPUB extraction.
///
/// The stored XHTML is sanitized so all downstream extraction paths see the
/// same body content: packaging prelude is removed, and specialized navigation
/// sections are stripped when they should not surface in extracted text.
pub(super) struct EpubSpineDocument {
    pub(super) file_path: String,
    pub(super) xhtml: String,
}

/// Read all body documents from the EPUB archive and downgrade per-item I/O
/// failures into processing warnings.
pub(super) fn read_body_documents(
    archive: &mut ZipArchive<Cursor<Vec<u8>>>,
    package: &EpubPackageDocument,
) -> crate::Result<(Vec<EpubSpineDocument>, Vec<ProcessingWarning>)> {
    let mut documents = Vec::new();
    let mut warnings = Vec::new();

    for spine_item in &package.spine_items {
        let Some(source_item) = package.manifest.get(&spine_item.idref) else {
            warnings.push(ProcessingWarning {
                source: std::borrow::Cow::Borrowed("epub"),
                message: std::borrow::Cow::Owned(format!(
                    "Spine item '{}' references a missing manifest entry",
                    spine_item.idref
                )),
            });
            continue;
        };

        let render_item = match resolve_renderable_manifest_item(package, &spine_item.idref) {
            Ok(render_item) => render_item,
            Err(err) => {
                warnings.push(ProcessingWarning {
                    source: std::borrow::Cow::Borrowed("epub"),
                    message: std::borrow::Cow::Owned(format!(
                        "Skipping spine item '{}' (href '{}'): {}",
                        spine_item.idref, source_item.raw_href, err
                    )),
                });
                continue;
            }
        };

        let file_path =
            render_item
                .resolved_path()
                .map(str::to_owned)
                .map_err(|message| crate::KreuzbergError::Parsing {
                    message: format!(
                        "Unsafe manifest href for spine item '{}' (href '{}'): {}",
                        spine_item.idref, render_item.raw_href, message
                    ),
                    source: None,
                })?;
        let guide_toc_candidate = source_item
            .path
            .as_deref()
            .is_some_and(|path| package.is_guide_toc_candidate_path(path))
            || render_item
                .path
                .as_deref()
                .is_some_and(|path| package.is_guide_toc_candidate_path(path));

        match read_file_from_zip(archive, &file_path) {
            Ok(raw_xhtml) => {
                let normalized_xhtml = normalize_xhtml(&raw_xhtml);
                let render_xhtml = strip_specialized_navigation_sections(&strip_document_head(&normalized_xhtml));

                if guide_toc_candidate && looks_like_navigation_document(&render_xhtml) {
                    continue;
                }

                if extract_text_from_xhtml(&render_xhtml).is_empty() {
                    continue;
                }

                documents.push(EpubSpineDocument {
                    file_path,
                    xhtml: render_xhtml,
                });
            }
            Err(err) => {
                warnings.push(ProcessingWarning {
                    source: std::borrow::Cow::Borrowed("epub"),
                    message: std::borrow::Cow::Owned(format!(
                        "Failed to read body spine item '{}' (idref '{}') from EPUB archive: {}",
                        file_path, spine_item.idref, err
                    )),
                });
            }
        }
    }

    Ok((documents, warnings))
}

fn resolve_renderable_manifest_item<'a>(
    package: &'a EpubPackageDocument,
    start_idref: &str,
) -> Result<&'a ManifestItem, String> {
    let mut current_id = start_idref;
    let mut visited = AHashSet::new();

    loop {
        if !visited.insert(current_id.to_string()) {
            return Err(format!("manifest fallback cycle detected at '{}'", current_id));
        }

        let Some(item) = package.manifest.get(current_id) else {
            return Err(format!("missing manifest entry '{}'", current_id));
        };

        if item.is_renderable_body_document() {
            return Ok(item);
        }

        let Some(next_id) = item.fallback.as_deref() else {
            let media_type = item.media_type.as_deref().unwrap_or("unknown");
            return Err(format!(
                "no renderable XHTML/DTBook fallback found for media type '{}'",
                media_type
            ));
        };

        current_id = next_id;
    }
}

fn strip_xml_elements<F>(xhtml: &str, mut predicate: F) -> String
where
    F: FnMut(roxmltree::Node<'_, '_>) -> bool,
{
    let Ok(doc) = roxmltree::Document::parse(xhtml) else {
        return xhtml.to_string();
    };

    let mut ranges = doc
        .descendants()
        .filter(|node| node.is_element())
        .filter(|node| predicate(*node))
        .map(|node| node.range())
        .collect::<Vec<_>>();

    if ranges.is_empty() {
        return xhtml.to_string();
    }

    ranges.sort_by(|left, right| match left.start.cmp(&right.start) {
        Ordering::Equal => right.end.cmp(&left.end),
        order => order,
    });

    let mut stripped = String::with_capacity(xhtml.len());
    let mut cursor = 0usize;
    for range in ranges {
        if range.start < cursor {
            continue;
        }
        stripped.push_str(&xhtml[cursor..range.start]);
        cursor = range.end;
    }
    stripped.push_str(&xhtml[cursor..]);
    stripped
}

fn strip_document_head(xhtml: &str) -> String {
    strip_xml_elements(xhtml, |node| node.tag_name().name().eq_ignore_ascii_case("head"))
}

fn strip_specialized_navigation_sections(xhtml: &str) -> String {
    strip_xml_elements(xhtml, |node| {
        node.tag_name().name().eq_ignore_ascii_case("nav") && is_specialized_navigation_node(node)
    })
}

fn is_specialized_navigation_node(node: roxmltree::Node<'_, '_>) -> bool {
    node.attributes().any(|attr| {
        attr.name().eq_ignore_ascii_case("type")
            && attr
                .value()
                .split_ascii_whitespace()
                .any(|value| matches!(value.to_ascii_lowercase().as_str(), "toc" | "landmarks" | "page-list"))
    })
}

fn looks_like_navigation_document(xhtml: &str) -> bool {
    let Ok(doc) = roxmltree::Document::parse(xhtml) else {
        return false;
    };

    let mut link_count = 0usize;
    let mut list_item_count = 0usize;
    let mut paragraph_count = 0usize;
    let mut heading_or_title_mentions_contents = false;

    for node in doc.descendants().filter(|node| node.is_element()) {
        match node.tag_name().name().to_ascii_lowercase().as_str() {
            "nav"
                if node.attributes().any(|attr| {
                    attr.name().eq_ignore_ascii_case("type")
                        && attr
                            .value()
                            .split_ascii_whitespace()
                            .any(|value| value.eq_ignore_ascii_case("toc"))
                }) =>
            {
                return true;
            }
            "a" => link_count += 1,
            "li" => list_item_count += 1,
            "p" => paragraph_count += 1,
            "title" | "h1" | "h2"
                if node.text().is_some_and(|text| {
                    matches!(
                        text.trim().to_ascii_lowercase().as_str(),
                        "contents" | "table of contents"
                    )
                }) =>
            {
                heading_or_title_mentions_contents = true;
            }
            _ => {}
        }
    }

    (link_count >= 2 && list_item_count >= 2 && paragraph_count <= 1)
        || (heading_or_title_mentions_contents && link_count >= 2)
}

/// Block-level HTML/XHTML elements that should produce newlines before/after their content.
const BLOCK_ELEMENTS: &[&str] = &[
    "address",
    "article",
    "aside",
    "blockquote",
    "caption",
    "dd",
    "details",
    "dialog",
    "div",
    "dl",
    "dt",
    "fieldset",
    "figcaption",
    "figure",
    "footer",
    "form",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "header",
    "hgroup",
    "hr",
    "legend",
    "li",
    "main",
    "nav",
    "ol",
    "p",
    "pre",
    "section",
    "summary",
    "table",
    "tbody",
    "td",
    "tfoot",
    "th",
    "thead",
    "title",
    "tr",
    "ul",
];

/// Elements whose entire subtree should be skipped (no text extracted).
const SKIP_ELEMENTS: &[&str] = &[
    "head", "script", "style", "svg", "math", "video", "audio", "source", "track", "object", "embed", "iframe",
];

/// Extract text from XHTML content by traversing the XML tree directly.
///
/// This avoids the double lossy conversion XHTML → markdown → plain-text that
/// previously stripped underscores, asterisks, and numeric content. Instead,
/// text nodes are collected verbatim from the parse tree, with newlines inserted
/// at block-level element boundaries.
pub(super) fn extract_text_from_xhtml(xhtml: &str) -> String {
    // Try direct XML tree traversal first (lossless path).
    if let Some(text) = try_extract_via_roxmltree(xhtml) {
        return text;
    }

    // Fallback: strip HTML tags character-by-character.
    let normalized = normalize_xhtml(xhtml);
    strip_html_tags(&normalized)
}

/// Attempt to extract plain text via `roxmltree` XML parsing.
///
/// Returns `None` if the document cannot be parsed as XML/XHTML.
fn try_extract_via_roxmltree(xhtml: &str) -> Option<String> {
    let sanitized = normalize_xhtml(xhtml);

    match roxmltree::Document::parse(&sanitized) {
        Ok(doc) => {
            let root = doc.root();

            let mut output = String::with_capacity(xhtml.len() / 2);
            visit_node(root, &mut output);

            // Normalise multiple consecutive blank lines to a single blank line.
            let result = collapse_blank_lines(&output);
            let result = result.trim().to_string();

            if result.is_empty() { None } else { Some(result) }
        }
        Err(_) => None,
    }
}

/// Normalize XHTML so downstream HTML/XHTML processing sees chapter markup
/// without EPUB packaging prelude.
///
/// This strips XML declarations and doctypes, which are valid in EPUB chapter
/// files but should not surface in extracted Markdown or interfere with safe parsing.
pub(super) fn normalize_xhtml(xml: &str) -> String {
    strip_xml_prelude(xml)
}

/// Remove XML declarations and DOCTYPE declarations from XML/XHTML.
///
/// EPUB chapter files often begin with an XML declaration followed by a DOCTYPE.
/// Those are packaging details, not body content, and `roxmltree` rejects DTDs.
fn strip_xml_prelude(xml: &str) -> String {
    let mut rest = xml.trim_start();

    loop {
        if let Some(tail) = rest.strip_prefix("<?xml")
            && let Some(end) = tail.find("?>")
        {
            rest = tail[end + 2..].trim_start();
            continue;
        }

        if let Some(tail) = rest.strip_prefix("<!DOCTYPE")
            && let Some(end) = find_doctype_end(tail)
        {
            rest = tail[end + 1..].trim_start();
            continue;
        }

        break;
    }

    rest.to_string()
}

fn find_doctype_end(tail: &str) -> Option<usize> {
    let mut bracket_depth: usize = 0;

    for (idx, ch) in tail.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '>' if bracket_depth == 0 => return Some(idx),
            _ => {}
        }
    }

    None
}

/// Recursively visit an XML node and append its text to `output`.
fn visit_node(node: roxmltree::Node<'_, '_>, output: &mut String) {
    match node.node_type() {
        roxmltree::NodeType::Text => {
            let text = node.text().unwrap_or("");
            // Normalise whitespace within a text run (collapse runs of
            // whitespace to single spaces) but keep the text itself intact.
            let normalised = normalise_inline_whitespace(text);
            if !normalised.is_empty() {
                // If the output already ends with a newline (or is empty),
                // trim leading spaces from this fragment to avoid spurious
                // indentation; otherwise append as-is.
                let fragment = if output.is_empty() || output.ends_with('\n') {
                    normalised.trim_start().to_string()
                } else {
                    normalised
                };
                if !fragment.is_empty() {
                    output.push_str(&fragment);
                }
            }
        }
        roxmltree::NodeType::Element => {
            let tag = node.tag_name().name().to_ascii_lowercase();

            // Skip elements whose content should never appear in plain text.
            if SKIP_ELEMENTS.iter().any(|&s| s == tag) {
                return;
            }

            // Self-closing elements that produce whitespace.
            if tag == "br" {
                output.push('\n');
                return;
            }
            if tag == "hr" {
                if !output.is_empty() && !output.ends_with('\n') {
                    output.push('\n');
                }
                return;
            }

            let is_block = BLOCK_ELEMENTS.iter().any(|&s| s == tag);

            if is_block {
                // Ensure block starts on a new line.
                if !output.is_empty() && !output.ends_with('\n') {
                    output.push('\n');
                }
            }

            // Recurse into children.
            for child in node.children() {
                visit_node(child, output);
            }

            if is_block {
                // Ensure block ends on a new line.
                if !output.is_empty() && !output.ends_with('\n') {
                    output.push('\n');
                }
            }
        }
        roxmltree::NodeType::Root => {
            // Visit all children of the document root.
            for child in node.children() {
                visit_node(child, output);
            }
        }
        // Ignore PI, Comment, CDATA, etc.
        _ => {}
    }
}

/// Collapse runs of whitespace (spaces/tabs/newlines) inside a text node to a
/// single space, matching what a browser would render for inline content.
fn normalise_inline_whitespace(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut prev_was_ws = false;

    for ch in text.chars() {
        if ch == '\n' || ch == '\r' || ch == '\t' || ch == ' ' {
            if !prev_was_ws {
                result.push(' ');
            }
            prev_was_ws = true;
        } else {
            result.push(ch);
            prev_was_ws = false;
        }
    }

    result
}

/// Collapse three or more consecutive newlines into exactly two (one blank line).
fn collapse_blank_lines(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut consecutive_newlines: usize = 0;

    for ch in text.chars() {
        if ch == '\n' {
            consecutive_newlines += 1;
            if consecutive_newlines <= 2 {
                result.push('\n');
            }
        } else {
            consecutive_newlines = 0;
            result.push(ch);
        }
    }

    result
}

/// Fallback: strip HTML tags without using specialized libraries
pub(super) fn strip_html_tags(html: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;
    let mut in_script_style = false;
    let mut tag_name = String::new();

    for ch in html.chars() {
        if ch == '<' {
            in_tag = true;
            tag_name.clear();
            continue;
        }

        if ch == '>' {
            in_tag = false;

            let tag_lower = tag_name.to_lowercase();
            if tag_lower.contains("script") || tag_lower.contains("style") {
                in_script_style = !tag_name.starts_with('/');
            }
            continue;
        }

        if in_tag {
            tag_name.push(ch);
            continue;
        }

        if in_script_style {
            continue;
        }

        if ch == '\n' || ch == '\r' || ch == '\t' || ch == ' ' {
            if !text.is_empty() && !text.ends_with(' ') {
                text.push(' ');
            }
        } else {
            text.push(ch);
        }
    }

    let mut result = String::new();
    let mut prev_space = false;
    for ch in text.chars() {
        if ch == ' ' {
            if !prev_space {
                result.push(ch);
            }
            prev_space = true;
        } else {
            result.push(ch);
            prev_space = false;
        }
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_html_tags_simple() {
        let html = "<html><body><p>Hello World</p></body></html>";
        let text = strip_html_tags(html);
        assert!(text.contains("Hello World"));
    }

    #[test]
    fn test_strip_html_tags_with_scripts() {
        let html = "<body><p>Text</p><script>alert('bad');</script><p>More</p></body>";
        let text = strip_html_tags(html);
        assert!(!text.contains("bad"));
        assert!(text.contains("Text"));
        assert!(text.contains("More"));
    }

    #[test]
    fn test_strip_html_tags_with_styles() {
        let html = "<body><p>Text</p><style>.class { color: red; }</style><p>More</p></body>";
        let text = strip_html_tags(html);
        assert!(!text.to_lowercase().contains("color"));
        assert!(text.contains("Text"));
        assert!(text.contains("More"));
    }

    #[test]
    fn test_strip_html_tags_normalizes_whitespace() {
        let html = "<p>Hello   \n\t   World</p>";
        let text = strip_html_tags(html);
        assert!(text.contains("Hello") && text.contains("World"));
    }

    // --- Direct XHTML extraction tests ---

    #[test]
    fn test_extract_text_from_xhtml_basic() {
        let xhtml = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
  <head><title>Test</title></head>
  <body>
    <h1>Chapter One</h1>
    <p>This is paragraph text.</p>
  </body>
</html>"#;
        let result = extract_text_from_xhtml(xhtml);
        assert!(result.contains("Chapter One"), "got: {result}");
        assert!(result.contains("This is paragraph text."), "got: {result}");
        // head/title content should not appear in body text
        assert!(!result.contains("Test"), "head title should be excluded, got: {result}");
    }

    #[test]
    fn test_extract_text_from_xhtml_skips_script_style() {
        let xhtml = r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
  <body>
    <p>Visible text</p>
    <script>var x = 1;</script>
    <style>.c { color: red; }</style>
    <p>More visible</p>
  </body>
</html>"#;
        let result = extract_text_from_xhtml(xhtml);
        assert!(result.contains("Visible text"), "got: {result}");
        assert!(result.contains("More visible"), "got: {result}");
        assert!(!result.contains("var x"), "got: {result}");
        assert!(!result.contains("color"), "got: {result}");
    }

    #[test]
    fn test_extract_text_from_xhtml_preserves_underscores_and_numbers() {
        let xhtml = r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
  <body>
    <p>The value_count is 1,000 items worth 3.14 each.</p>
    <p>See http://example.com/path_to/resource for details.</p>
  </body>
</html>"#;
        let result = extract_text_from_xhtml(xhtml);
        assert!(result.contains("value_count"), "underscore preserved, got: {result}");
        assert!(result.contains("1,000"), "number preserved, got: {result}");
        assert!(result.contains("3.14"), "decimal preserved, got: {result}");
        assert!(
            result.contains("http://example.com/path_to/resource"),
            "URL preserved, got: {result}"
        );
    }

    #[test]
    fn test_extract_text_from_xhtml_block_elements_add_newlines() {
        let xhtml = r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
  <body>
    <h1>Heading</h1>
    <p>Paragraph one.</p>
    <p>Paragraph two.</p>
    <ul>
      <li>Item A</li>
      <li>Item B</li>
    </ul>
  </body>
</html>"#;
        let result = extract_text_from_xhtml(xhtml);
        assert!(result.contains("Heading"), "got: {result}");
        assert!(result.contains("Paragraph one."), "got: {result}");
        assert!(result.contains("Paragraph two."), "got: {result}");
        assert!(result.contains("Item A"), "got: {result}");
        assert!(result.contains("Item B"), "got: {result}");
        // The two paragraphs should be on different lines
        assert!(result.contains('\n'), "should have newlines, got: {result}");
    }

    #[test]
    fn test_extract_text_from_xhtml_inline_formatting_preserved() {
        let xhtml = r#"<?xml version="1.0" encoding="UTF-8"?>
<html xmlns="http://www.w3.org/1999/xhtml">
  <body>
    <p>This has <strong>bold</strong> and <em>italic</em> text.</p>
  </body>
</html>"#;
        let result = extract_text_from_xhtml(xhtml);
        // Text content should be preserved; no markdown syntax introduced
        assert!(result.contains("bold"), "got: {result}");
        assert!(result.contains("italic"), "got: {result}");
        assert!(!result.contains("**"), "no markdown bold, got: {result}");
        assert!(!result.contains('_'), "no markdown italic, got: {result}");
    }

    #[test]
    fn test_extract_text_from_xhtml_fallback_for_invalid_xml() {
        // Malformed XHTML that roxmltree cannot parse should fall back to tag stripping.
        let bad_xhtml = "<p>Hello <b>World</b> unclosed <p>second";
        let result = extract_text_from_xhtml(bad_xhtml);
        assert!(result.contains("Hello"), "got: {result}");
        assert!(result.contains("World"), "got: {result}");
    }

    #[test]
    fn test_normalise_inline_whitespace() {
        assert_eq!(normalise_inline_whitespace("hello   world"), "hello world");
        assert_eq!(normalise_inline_whitespace("  leading"), " leading");
        assert_eq!(normalise_inline_whitespace("trailing  "), "trailing ");
        assert_eq!(normalise_inline_whitespace("a\n\t b"), "a b");
    }

    #[test]
    fn test_collapse_blank_lines() {
        let input = "a\n\n\n\nb";
        let result = collapse_blank_lines(input);
        assert_eq!(result, "a\n\nb");

        let input2 = "a\n\nb";
        assert_eq!(collapse_blank_lines(input2), "a\n\nb");
    }
}

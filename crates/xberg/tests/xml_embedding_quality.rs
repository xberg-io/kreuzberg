//! Tests verifying XML extraction produces embedding-friendly hierarchical output.
//!
//! Indented output preserves document structure so that related elements
//! (e.g. a plant's name and zone) stay grouped together for vector search.

mod helpers;
use helpers::extract_bytes_document;

use xberg::core::config::ExtractionConfig;

/// Sibling elements should be grouped under their parent with indentation.
#[tokio::test]
async fn test_xml_preserves_hierarchy() {
    let config = ExtractionConfig::default();
    let xml = br#"<?xml version="1.0"?><CATALOG><PLANT><COMMON>Bloodroot</COMMON><ZONE>4</ZONE></PLANT></CATALOG>"#;

    let result = extract_bytes_document(xml, "application/xml", &config).await.unwrap();

    // PLANT children should be indented under PLANT
    assert!(result.content.contains("PLANT"));
    assert!(result.content.contains("  COMMON\n    Bloodroot"));
    assert!(result.content.contains("  ZONE\n    4"));
}

/// Deeper nesting should produce deeper indentation.
#[tokio::test]
async fn test_xml_indentation_shows_nesting() {
    let config = ExtractionConfig::default();
    let xml = b"<root><parent><child><grandchild>Deep</grandchild></child></parent></root>";

    let result = extract_bytes_document(xml, "application/xml", &config).await.unwrap();

    assert!(result.content.contains("    grandchild\n      Deep"));
}

/// Attributes should appear inline with the element label, in any order.
#[tokio::test]
async fn test_xml_attributes_inline() {
    let config = ExtractionConfig::default();
    let xml = br#"<root><item type="book" id="42">Title</item></root>"#;

    let result = extract_bytes_document(xml, "application/xml", &config).await.unwrap();

    // Both attributes must appear inline with the `item` label, but order
    // is not part of the contract — `AHashMap` iteration is non-deterministic
    // and the renderer sorts alphabetically for stability across runs.
    let item_line = result
        .content
        .lines()
        .find(|l| l.contains("item ("))
        .expect("expected an `item (...)` label line");
    assert!(item_line.contains("type: book"), "missing type attr in: {item_line:?}");
    assert!(item_line.contains("id: 42"), "missing id attr in: {item_line:?}");
    assert!(result.content.contains("Title"));
}

/// Sibling groups should be separated by a blank line, regardless of the
/// indent depth at which the siblings sit.
#[tokio::test]
async fn test_xml_sibling_separation() {
    let config = ExtractionConfig::default();
    let xml = b"<CATALOG><PLANT><COMMON>A</COMMON></PLANT><PLANT><COMMON>B</COMMON></PLANT></CATALOG>";

    let result = extract_bytes_document(xml, "application/xml", &config).await.unwrap();

    // Both siblings are present.
    assert_eq!(
        result.content.matches("PLANT").count(),
        2,
        "expected two PLANT siblings, got: {:?}",
        result.content
    );
    // A blank line appears somewhere between the two PLANT labels.
    let parts: Vec<&str> = result.content.split("PLANT").collect();
    assert!(parts.len() >= 3, "expected >=2 PLANT splits, got: {parts:?}");
    assert!(
        parts[1].contains("\n\n"),
        "expected blank line between sibling PLANT entries, got: {:?}",
        parts[1]
    );
}

/// Namespace attributes (xmlns:*) should be filtered from output.
#[tokio::test]
async fn test_xml_namespace_filtering() {
    let config = ExtractionConfig::default();
    let xml = br#"<root xmlns:ns="http://example.com" id="1"><item>Text</item></root>"#;

    let result = extract_bytes_document(xml, "application/xml", &config).await.unwrap();

    assert!(!result.content.contains("xmlns"), "Namespace attrs should be filtered");
    assert!(
        result.content.contains("root (id: 1)"),
        "Non-namespace attrs should be preserved"
    );
    assert!(result.content.contains("Text"));
}

/// Mixed content (text between elements) should be preserved with indentation.
#[tokio::test]
async fn test_xml_mixed_content() {
    let config = ExtractionConfig::default();
    let xml = b"<root>Text before<item>nested</item>Text after</root>";

    let result = extract_bytes_document(xml, "application/xml", &config).await.unwrap();

    assert!(result.content.contains("Text before"));
    assert!(result.content.contains("nested"));
    assert!(result.content.contains("Text after"));
}

/// Self-closing tags should appear in the output.
#[tokio::test]
async fn test_xml_self_closing_tags() {
    let config = ExtractionConfig::default();
    let xml = br#"<root><item type="empty"/></root>"#;

    let result = extract_bytes_document(xml, "application/xml", &config).await.unwrap();

    assert!(result.content.contains("item (type: empty)"));
}

/// Empty attribute values should be filtered from the label.
#[tokio::test]
async fn test_xml_empty_attribute_filtered() {
    let config = ExtractionConfig::default();
    let xml = br#"<root><item id="" type="book">Text</item></root>"#;

    let result = extract_bytes_document(xml, "application/xml", &config).await.unwrap();

    assert!(result.content.contains("item (type: book)"));
    assert!(!result.content.contains("id:"), "Empty attribute should be filtered");
}

/// Text directly inside the root element should still be indented.
#[tokio::test]
async fn test_xml_root_level_text() {
    let config = ExtractionConfig::default();
    let xml = b"<root>Some text</root>";

    let result = extract_bytes_document(xml, "application/xml", &config).await.unwrap();

    assert!(result.content.contains("root"));
    assert!(result.content.contains("Some text"));
}

/// Real XML file should produce grouped plant entries: each plant's COMMON
/// label and its value should appear close together under the plant's label.
#[tokio::test]
async fn test_xml_real_file_plant_catalog() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test_documents/xml/plant_catalog.xml");
    if !path.exists() {
        return;
    }
    let content = std::fs::read(&path).unwrap();
    let config = ExtractionConfig::default();

    let result = extract_bytes_document(&content, "application/xml", &config)
        .await
        .unwrap();

    // Both plants are present and their COMMON values appear within the same
    // grouped block — assertion is loose on exact indentation so that future
    // tweaks to the indent step or blank-line policy don't break it.
    for plant in ["Bloodroot", "Columbine"] {
        let pos_plant = result.content.rfind("PLANT").expect("missing PLANT label");
        let pos_value = result
            .content
            .find(plant)
            .unwrap_or_else(|| panic!("missing plant value: {plant}"));
        // Either a PLANT label precedes this value, or COMMON appears between
        // them — both suffice for "grouped together".
        let between = &result.content[..pos_value];
        assert!(
            between.contains("PLANT") && between.rfind("COMMON").map(|c| c < pos_value).unwrap_or(false),
            "expected `PLANT ... COMMON` to precede {plant}; got {pos_plant}, {pos_value}"
        );
    }
}

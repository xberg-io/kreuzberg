//! Core types for document extraction.

// Module declarations
pub mod djot;
pub mod document_structure;
pub mod extraction;
pub mod formats;
pub mod metadata;
pub mod ocr_elements;
pub mod page;
pub mod serde_helpers;
pub mod tables;

// Re-export all types for backward compatibility
pub use djot::*;
pub use document_structure::{
    AnnotationKind, ContentLayer, DocumentNode, DocumentStructure, GridCell, NodeContent, NodeId, NodeIndex, TableGrid,
    TextAnnotation,
};
pub use extraction::*;
pub use formats::*;
pub use metadata::*;
pub use ocr_elements::*;
pub use page::*;
pub use tables::*;

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use std::borrow::Cow;
    use std::sync::Arc;

    #[test]
    fn test_metadata_serialization_with_format() {
        let mut metadata = Metadata {
            format: Some(FormatMetadata::Text(TextMetadata {
                line_count: 1,
                word_count: 2,
                character_count: 13,
                headers: None,
                links: None,
                code_blocks: None,
            })),
            ..Default::default()
        };

        metadata
            .additional
            .insert(Cow::Borrowed("quality_score"), serde_json::json!(1.0));

        let json = serde_json::to_value(&metadata).unwrap();
        println!("Serialized metadata: {}", serde_json::to_string_pretty(&json).unwrap());

        assert!(
            json.get("format_type").is_some(),
            "format_type should be present in serialized JSON"
        );
        assert_eq!(json.get("format_type").unwrap(), "text");

        assert_eq!(json.get("line_count").unwrap(), 1);
        assert_eq!(json.get("word_count").unwrap(), 2);
        assert_eq!(json.get("character_count").unwrap(), 13);

        assert_eq!(json.get("quality_score").unwrap(), 1.0);
    }

    #[test]
    fn test_arc_table_serialization_format() {
        let table = Table {
            cells: vec![vec!["A".to_string(), "B".to_string()]],
            markdown: "| A | B |\n|---|---|\n".to_string(),
            page_number: 1,
        };

        let json = serde_json::to_value(&table).unwrap();

        assert_eq!(json.get("cells").unwrap()[0][0], "A");
        assert_eq!(json.get("markdown").unwrap(), "| A | B |\n|---|---|\n");
        assert_eq!(json.get("page_number").unwrap(), 1);
    }

    #[test]
    fn test_arc_table_roundtrip() {
        let original = Table {
            cells: vec![
                vec!["X".to_string(), "Y".to_string()],
                vec!["1".to_string(), "2".to_string()],
            ],
            markdown: "| X | Y |\n|---|---|\n| 1 | 2 |\n".to_string(),
            page_number: 5,
        };

        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Table = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.cells, original.cells);
        assert_eq!(deserialized.markdown, original.markdown);
        assert_eq!(deserialized.page_number, original.page_number);
    }

    #[test]
    fn test_arc_sharing_preserved_before_serialization() {
        let shared_table = Arc::new(Table {
            cells: vec![vec!["shared".to_string()]],
            markdown: "| shared |".to_string(),
            page_number: 1,
        });

        let tables_before = [Arc::clone(&shared_table), Arc::clone(&shared_table)].to_vec();
        assert_eq!(Arc::strong_count(&tables_before[0]), 3);
        assert_eq!(Arc::strong_count(&tables_before[1]), 3);
        assert!(Arc::ptr_eq(&tables_before[0], &tables_before[1]));
    }

    #[test]
    fn test_vec_arc_table_serialization_format() {
        let tables = vec![
            Table {
                cells: vec![vec!["A".to_string()]],
                markdown: "| A |".to_string(),
                page_number: 1,
            },
            Table {
                cells: vec![vec!["B".to_string()]],
                markdown: "| B |".to_string(),
                page_number: 2,
            },
        ];

        let json = serde_json::to_string(&tables).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.is_array());
        assert_eq!(parsed.as_array().unwrap().len(), 2);
        assert_eq!(parsed[0]["cells"][0][0], "A");
        assert_eq!(parsed[1]["cells"][0][0], "B");
    }

    #[test]
    fn test_page_content_arc_tables_roundtrip() {
        let page = PageContent {
            page_number: 3,
            content: "Page 3 content".to_string(),
            tables: vec![
                Arc::new(Table {
                    cells: vec![vec!["Table1".to_string()]],
                    markdown: "| Table1 |".to_string(),
                    page_number: 3,
                }),
                Arc::new(Table {
                    cells: vec![vec!["Table2".to_string()]],
                    markdown: "| Table2 |".to_string(),
                    page_number: 3,
                }),
            ],
            images: Vec::new(),
            hierarchy: None,
            is_blank: None,
        };

        let json = serde_json::to_string(&page).unwrap();
        let deserialized: PageContent = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.page_number, 3);
        assert_eq!(deserialized.content, "Page 3 content");
        assert_eq!(deserialized.tables.len(), 2);
        assert_eq!(deserialized.tables[0].cells[0][0], "Table1");
        assert_eq!(deserialized.tables[1].cells[0][0], "Table2");
    }

    #[test]
    fn test_page_content_arc_images_roundtrip() {
        let image1 = Arc::new(ExtractedImage {
            data: Bytes::from_static(&[0xFF, 0xD8, 0xFF]),
            format: Cow::Borrowed("jpeg"),
            image_index: 0,
            page_number: Some(1),
            width: Some(100),
            height: Some(200),
            colorspace: Some("RGB".to_string()),
            bits_per_component: Some(8),
            is_mask: false,
            description: Some("Image 1".to_string()),
            ocr_result: None,
        });

        let image2 = Arc::new(ExtractedImage {
            data: Bytes::from_static(&[0x89, 0x50, 0x4E]),
            format: Cow::Borrowed("png"),
            image_index: 1,
            page_number: Some(1),
            width: Some(300),
            height: Some(400),
            colorspace: Some("RGBA".to_string()),
            bits_per_component: Some(8),
            is_mask: false,
            description: Some("Image 2".to_string()),
            ocr_result: None,
        });

        let page = PageContent {
            page_number: 1,
            content: "Page with images".to_string(),
            tables: Vec::new(),
            images: vec![image1, image2],
            hierarchy: None,
            is_blank: None,
        };

        let json = serde_json::to_string(&page).unwrap();
        let deserialized: PageContent = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.images.len(), 2);
        assert_eq!(deserialized.images[0].format, "jpeg");
        assert_eq!(deserialized.images[0].width, Some(100));
        assert_eq!(deserialized.images[1].format, "png");
        assert_eq!(deserialized.images[1].height, Some(400));
    }

    #[test]
    fn test_arc_sharing_loss_with_page_content() {
        let shared_table = Arc::new(Table {
            cells: vec![vec!["shared across pages".to_string()]],
            markdown: "| shared across pages |".to_string(),
            page_number: 0,
        });

        let page1 = PageContent {
            page_number: 1,
            content: "Page 1".to_string(),
            tables: vec![Arc::clone(&shared_table)],
            images: Vec::new(),
            hierarchy: None,
            is_blank: None,
        };

        let page2 = PageContent {
            page_number: 2,
            content: "Page 2".to_string(),
            tables: vec![Arc::clone(&shared_table)],
            images: Vec::new(),
            hierarchy: None,
            is_blank: None,
        };

        assert!(Arc::ptr_eq(&page1.tables[0], &page2.tables[0]));

        let pages = vec![page1, page2];
        let json = serde_json::to_string(&pages).unwrap();
        let deserialized: Vec<PageContent> = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.len(), 2);
        assert_eq!(deserialized[0].tables[0].cells, deserialized[1].tables[0].cells);
        assert!(!Arc::ptr_eq(&deserialized[0].tables[0], &deserialized[1].tables[0]));
    }

    #[test]
    fn test_empty_page_content_arcs() {
        let page = PageContent {
            page_number: 5,
            content: "No tables or images".to_string(),
            tables: Vec::new(),
            images: Vec::new(),
            hierarchy: None,
            is_blank: None,
        };

        let json = serde_json::to_string(&page).unwrap();
        let deserialized: PageContent = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.page_number, 5);
        assert_eq!(deserialized.tables.len(), 0);
        assert_eq!(deserialized.images.len(), 0);
    }

    #[test]
    fn test_serde_vec_arc_module_behavior() {
        let table1 = Table {
            cells: vec![vec!["A".to_string()]],
            markdown: "| A |".to_string(),
            page_number: 1,
        };

        let table2 = Table {
            cells: vec![vec!["B".to_string()]],
            markdown: "| B |".to_string(),
            page_number: 2,
        };

        let json = serde_json::to_string(&vec![table1, table2]).unwrap();
        assert!(json.contains("\"A\""));
        assert!(json.contains("\"B\""));
    }
}

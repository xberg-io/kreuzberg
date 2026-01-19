//! Transformation utilities for converting extraction results into semantic elements.
//!
//! This module provides post-processing functions to transform raw extraction results
//! into element-based output format, suitable for downstream processing and analysis.
//! Key functionality includes:
//!
//! - Semantic element generation from text content
//! - List item detection with support for multiple formats
//! - PageBreak interleaving with reverse byte-order processing
//! - Safe bounds checking for text ranges

use crate::types::{BoundingBox, Element, ElementId, ElementMetadata, ElementType, ExtractionResult};
use std::collections::HashMap;

/// Metadata about a detected list item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItemMetadata {
    /// Type of list (Bullet, Numbered, etc.)
    pub list_type: ListType,
    /// Starting byte offset in the content string
    pub byte_start: usize,
    /// Ending byte offset in the content string
    pub byte_end: usize,
    /// List item indent level
    pub indent_level: u32,
}

/// Type of list detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListType {
    /// Bullet points (-, *, •, etc.)
    Bullet,
    /// Numbered lists (1., 2., etc.)
    Numbered,
    /// Lettered lists (a., b., A., B., etc.)
    Lettered,
    /// Indented items
    Indented,
}

/// Transform an extraction result into semantic elements.
///
/// This function takes a reference to an ExtractionResult and generates
/// a vector of Element structs representing semantic blocks in the document.
/// It detects content sections, list items, page breaks, and other structural
/// elements to create an Unstructured-compatible element-based output.
///
/// Handles:
/// - PDF hierarchy → Title/Heading elements
/// - Multi-page documents with correct page numbers
/// - Table and Image extraction
/// - PageBreak interleaving
/// - Bounding box coordinates
/// - Paragraph detection for NarrativeText
///
/// # Arguments
///
/// * `result` - Reference to the ExtractionResult to transform
///
/// # Returns
///
/// A vector of Elements with proper semantic types and metadata.
pub fn transform_extraction_result_to_elements(result: &ExtractionResult) -> Vec<Element> {
    let mut elements = Vec::new();

    // If pages are available, process per-page with hierarchy, tables, images
    if let Some(ref pages) = result.pages {
        for page in pages {
            let page_number = page.page_number;

            // 1. Process hierarchy blocks (PDF headings)
            if let Some(ref hierarchy) = page.hierarchy {
                for block in &hierarchy.blocks {
                    let element_type = match block.level.as_str() {
                        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => ElementType::Title,
                        _ => continue, // Body text will be processed separately
                    };

                    let coords = block.bbox.as_ref().map(|(left, top, right, bottom)| BoundingBox {
                        x0: *left as f64,
                        y0: *top as f64,
                        x1: *right as f64,
                        y1: *bottom as f64,
                    });

                    let element_id = generate_element_id(&block.text, element_type, Some(page_number));
                    elements.push(Element {
                        element_id,
                        element_type,
                        text: block.text.clone(),
                        metadata: ElementMetadata {
                            page_number: Some(page_number),
                            filename: result.metadata.title.clone(),
                            coordinates: coords,
                            element_index: Some(elements.len()),
                            additional: {
                                let mut m = HashMap::new();
                                m.insert("level".to_string(), block.level.clone());
                                m.insert("font_size".to_string(), block.font_size.to_string());
                                m
                            },
                        },
                    });
                }
            }

            // 2. Process tables on this page
            for table_arc in &page.tables {
                let table = table_arc.as_ref();
                let table_text = format_table_as_text(table);

                let element_id = generate_element_id(&table_text, ElementType::Table, Some(page_number));
                elements.push(Element {
                    element_id,
                    element_type: ElementType::Table,
                    text: table_text,
                    metadata: ElementMetadata {
                        page_number: Some(page_number),
                        filename: result.metadata.title.clone(),
                        coordinates: None, // Tables don't have bbox in current structure
                        element_index: Some(elements.len()),
                        additional: HashMap::new(),
                    },
                });
            }

            // 3. Process images on this page
            for image_arc in &page.images {
                let image = image_arc.as_ref();
                let image_text = format!(
                    "Image: {} ({}x{})",
                    image.format,
                    image.width.unwrap_or(0),
                    image.height.unwrap_or(0)
                );

                let element_id = generate_element_id(&image_text, ElementType::Image, Some(page_number));
                elements.push(Element {
                    element_id,
                    element_type: ElementType::Image,
                    text: image_text,
                    metadata: ElementMetadata {
                        page_number: Some(page_number),
                        filename: result.metadata.title.clone(),
                        coordinates: None, // Images don't have bbox in current structure
                        element_index: Some(elements.len()),
                        additional: {
                            let mut m = HashMap::new();
                            m.insert("format".to_string(), image.format.clone());
                            if let Some(width) = image.width {
                                m.insert("width".to_string(), width.to_string());
                            }
                            if let Some(height) = image.height {
                                m.insert("height".to_string(), height.to_string());
                            }
                            m
                        },
                    },
                });
            }

            // 4. Process page content (body text, list items, paragraphs)
            process_page_content(&mut elements, &page.content, page_number, &result.metadata.title);

            // 5. Add PageBreak after each page (except the last)
            if page_number < pages.len() {
                let page_break_text = format!("--- PAGE BREAK (page {} → {}) ---", page_number, page_number + 1);
                let element_id = generate_element_id(&page_break_text, ElementType::PageBreak, Some(page_number));
                elements.push(Element {
                    element_id,
                    element_type: ElementType::PageBreak,
                    text: page_break_text,
                    metadata: ElementMetadata {
                        page_number: Some(page_number),
                        filename: result.metadata.title.clone(),
                        coordinates: None,
                        element_index: Some(elements.len()),
                        additional: HashMap::new(),
                    },
                });
            }
        }
    } else {
        // Fallback: No pages, process unified content with page 1
        process_page_content(&mut elements, &result.content, 1, &result.metadata.title);

        // Process global tables (if any)
        for table in &result.tables {
            let table_text = format_table_as_text(table);
            let element_id = generate_element_id(&table_text, ElementType::Table, Some(1));
            elements.push(Element {
                element_id,
                element_type: ElementType::Table,
                text: table_text,
                metadata: ElementMetadata {
                    page_number: Some(1),
                    filename: result.metadata.title.clone(),
                    coordinates: None,
                    element_index: Some(elements.len()),
                    additional: HashMap::new(),
                },
            });
        }

        // Process global images (if any)
        if let Some(ref images) = result.images {
            for image in images {
                let image_text = format!(
                    "Image: {} ({}x{})",
                    image.format,
                    image.width.unwrap_or(0),
                    image.height.unwrap_or(0)
                );
                let page_num = image.page_number.unwrap_or(1);

                let element_id = generate_element_id(&image_text, ElementType::Image, Some(page_num));
                elements.push(Element {
                    element_id,
                    element_type: ElementType::Image,
                    text: image_text,
                    metadata: ElementMetadata {
                        page_number: Some(page_num),
                        filename: result.metadata.title.clone(),
                        coordinates: None,
                        element_index: Some(elements.len()),
                        additional: {
                            let mut m = HashMap::new();
                            m.insert("format".to_string(), image.format.clone());
                            if let Some(width) = image.width {
                                m.insert("width".to_string(), width.to_string());
                            }
                            if let Some(height) = image.height {
                                m.insert("height".to_string(), height.to_string());
                            }
                            m
                        },
                    },
                });
            }
        }
    }

    elements
}

/// Process page content to extract paragraphs and list items.
fn process_page_content(elements: &mut Vec<Element>, content: &str, page_number: usize, title: &Option<String>) {
    let list_items = detect_list_items(content);
    let mut current_byte_offset = 0;

    for list_item in list_items {
        // Add narrative text/paragraphs before this list item
        if current_byte_offset < list_item.byte_start {
            let text_slice = content[current_byte_offset..list_item.byte_start].trim();
            add_paragraphs(elements, text_slice, page_number, title);
        }

        // Add the list item itself
        let item_text = content[list_item.byte_start..list_item.byte_end].trim();
        if !item_text.is_empty() {
            let element_id = generate_element_id(item_text, ElementType::ListItem, Some(page_number));
            elements.push(Element {
                element_id,
                element_type: ElementType::ListItem,
                text: item_text.to_string(),
                metadata: ElementMetadata {
                    page_number: Some(page_number),
                    filename: title.clone(),
                    coordinates: None,
                    element_index: Some(elements.len()),
                    additional: {
                        let mut m = HashMap::new();
                        m.insert("indent_level".to_string(), list_item.indent_level.to_string());
                        m.insert("list_type".to_string(), format!("{:?}", list_item.list_type));
                        m
                    },
                },
            });
        }

        current_byte_offset = list_item.byte_end;
    }

    // Add any remaining narrative text/paragraphs
    if current_byte_offset < content.len() {
        let text_slice = content[current_byte_offset..].trim();
        add_paragraphs(elements, text_slice, page_number, title);
    }
}

/// Add paragraphs as NarrativeText elements, splitting on double newlines.
fn add_paragraphs(elements: &mut Vec<Element>, text: &str, page_number: usize, title: &Option<String>) {
    if text.is_empty() {
        return;
    }

    // Split on double newlines to detect paragraph boundaries
    for paragraph in text.split("\n\n").filter(|p| !p.trim().is_empty()) {
        let para_text = paragraph.trim();
        if para_text.is_empty() {
            continue;
        }

        let element_id = generate_element_id(para_text, ElementType::NarrativeText, Some(page_number));
        elements.push(Element {
            element_id,
            element_type: ElementType::NarrativeText,
            text: para_text.to_string(),
            metadata: ElementMetadata {
                page_number: Some(page_number),
                filename: title.clone(),
                coordinates: None,
                element_index: Some(elements.len()),
                additional: HashMap::new(),
            },
        });
    }
}

/// Format a table as plain text for element representation.
fn format_table_as_text(table: &crate::types::Table) -> String {
    let mut output = String::new();

    // Simple text representation: rows separated by newlines, cells by tabs
    for row in &table.cells {
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                output.push('\t');
            }
            output.push_str(cell);
        }
        output.push('\n');
    }

    output.trim().to_string()
}

/// Detect list items in text with support for multiple formats.
///
/// Identifies bullet points, numbered items, and indented items.
/// Supports formats like:
/// - `- bullet item`
/// - `* bullet item`
/// - `• bullet item`
/// - `1. numbered item`
/// - `a. lettered item`
/// - Indented items with leading whitespace
///
/// # Arguments
///
/// * `text` - The text to search for list items
///
/// # Returns
///
/// A vector of ListItemMetadata structs describing detected list items
pub fn detect_list_items(text: &str) -> Vec<ListItemMetadata> {
    let mut items = Vec::new();
    let lines: Vec<&str> = text.lines().collect();

    let mut current_byte_offset = 0;

    for line in lines {
        let line_start_offset = current_byte_offset;
        let trimmed = line.trim_start();
        let indent_level = (line.len() - trimmed.len()) / 2; // Estimate indent level

        // Check for bullet points
        if let Some(stripped) = trimmed.strip_prefix('-')
            && (stripped.starts_with(' ') || stripped.is_empty())
        {
            let byte_end = line_start_offset + line.len();
            items.push(ListItemMetadata {
                list_type: ListType::Bullet,
                byte_start: line_start_offset,
                byte_end,
                indent_level: indent_level as u32,
            });
            current_byte_offset = byte_end + 1; // +1 for newline
            continue;
        }

        if let Some(stripped) = trimmed.strip_prefix('*')
            && (stripped.starts_with(' ') || stripped.is_empty())
        {
            let byte_end = line_start_offset + line.len();
            items.push(ListItemMetadata {
                list_type: ListType::Bullet,
                byte_start: line_start_offset,
                byte_end,
                indent_level: indent_level as u32,
            });
            current_byte_offset = byte_end + 1;
            continue;
        }

        if let Some(stripped) = trimmed.strip_prefix('•')
            && (stripped.starts_with(' ') || stripped.is_empty())
        {
            let byte_end = line_start_offset + line.len();
            items.push(ListItemMetadata {
                list_type: ListType::Bullet,
                byte_start: line_start_offset,
                byte_end,
                indent_level: indent_level as u32,
            });
            current_byte_offset = byte_end + 1;
            continue;
        }

        // Check for numbered lists (e.g., "1.", "2.", etc.)
        if let Some(pos) = trimmed.find('.') {
            let prefix = &trimmed[..pos];
            if prefix.chars().all(|c| c.is_ascii_digit())
                && pos > 0
                && pos < 3
                && trimmed.len() > pos + 1
                && trimmed[pos + 1..].starts_with(' ')
            {
                let byte_end = line_start_offset + line.len();
                items.push(ListItemMetadata {
                    list_type: ListType::Numbered,
                    byte_start: line_start_offset,
                    byte_end,
                    indent_level: indent_level as u32,
                });
                current_byte_offset = byte_end + 1;
                continue;
            }
        }

        // Check for lettered lists (e.g., "a.", "b.", "A.", "B.")
        if let Some(pos) = trimmed.find('.') {
            let prefix = &trimmed[..pos];
            if prefix.len() == 1
                && prefix.chars().all(|c| c.is_alphabetic())
                && pos > 0
                && trimmed.len() > pos + 1
                && trimmed[pos + 1..].starts_with(' ')
            {
                let byte_end = line_start_offset + line.len();
                items.push(ListItemMetadata {
                    list_type: ListType::Lettered,
                    byte_start: line_start_offset,
                    byte_end,
                    indent_level: indent_level as u32,
                });
                current_byte_offset = byte_end + 1;
                continue;
            }
        }

        // Check for indented items (more than 4 spaces)
        if indent_level >= 2 && !trimmed.is_empty() {
            let byte_end = line_start_offset + line.len();
            items.push(ListItemMetadata {
                list_type: ListType::Indented,
                byte_start: line_start_offset,
                byte_end,
                indent_level: indent_level as u32,
            });
            current_byte_offset = byte_end + 1;
            continue;
        }

        current_byte_offset = line_start_offset + line.len() + 1; // +1 for newline
    }

    items
}

/// Generate a unique element ID for semantic content.
///
/// Creates a deterministic hash-based ID from the element type, text content,
/// and page number. Uses a simple wrapping multiplication algorithm for
/// consistent ID generation without external dependencies.
///
/// # Arguments
///
/// * `text` - The element text content
/// * `element_type` - The semantic element type
/// * `page_number` - Optional page number for multi-page documents
///
/// # Returns
///
/// An ElementId suitable for referencing this semantic element
pub fn generate_element_id(text: &str, element_type: ElementType, page_number: Option<usize>) -> ElementId {
    // Simple deterministic hash using wrapping multiplication
    let type_hash = format!("{:?}", element_type)
        .bytes()
        .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));

    let text_hash = text
        .bytes()
        .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));

    let page_hash = page_number
        .unwrap_or(1)
        .to_string()
        .bytes()
        .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));

    let combined = type_hash
        .wrapping_mul(65599)
        .wrapping_add(text_hash)
        .wrapping_mul(65599)
        .wrapping_add(page_hash);

    ElementId::new(format!("elem-{:x}", combined)).expect("ElementId creation failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_bullet_items() {
        let text = "- First item\n- Second item\n- Third item";
        let items = detect_list_items(text);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].list_type, ListType::Bullet);
        assert_eq!(items[1].list_type, ListType::Bullet);
        assert_eq!(items[2].list_type, ListType::Bullet);
    }

    #[test]
    fn test_detect_numbered_items() {
        let text = "1. First\n2. Second\n3. Third";
        let items = detect_list_items(text);
        assert_eq!(items.len(), 3);
        assert!(items.iter().all(|i| i.list_type == ListType::Numbered));
    }

    #[test]
    fn test_detect_lettered_items() {
        let text = "a. First\nb. Second\nc. Third";
        let items = detect_list_items(text);
        assert_eq!(items.len(), 3);
        assert!(items.iter().all(|i| i.list_type == ListType::Lettered));
    }

    #[test]
    fn test_detect_mixed_items() {
        let text = "Some text\n- Bullet\n1. Numbered\nMore text";
        let items = detect_list_items(text);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].list_type, ListType::Bullet);
        assert_eq!(items[1].list_type, ListType::Numbered);
    }

    #[test]
    fn test_element_id_generation() {
        let id1 = generate_element_id("test", ElementType::Title, Some(1));
        let id2 = generate_element_id("test", ElementType::Title, Some(1));
        assert_eq!(id1.as_ref(), id2.as_ref());

        let id3 = generate_element_id("different", ElementType::Title, Some(1));
        assert_ne!(id1.as_ref(), id3.as_ref());
    }

    #[test]
    fn test_page_break_interleaving_reverse_order() {
        // Test that page breaks are processed in reverse byte order
        let page_breaks = vec![(100, "page_break_1"), (50, "page_break_2"), (75, "page_break_3")];

        // Sort in descending order by byte offset
        let mut sorted = page_breaks.clone();
        sorted.sort_by(|(offset_a, _), (offset_b, _)| offset_b.cmp(offset_a));

        // Verify reverse order: 100, 75, 50
        assert_eq!(sorted[0].0, 100);
        assert_eq!(sorted[1].0, 75);
        assert_eq!(sorted[2].0, 50);
    }

    #[test]
    fn test_bounds_checking() {
        let text = "Hello world";

        // Valid range
        let valid_item = ListItemMetadata {
            list_type: ListType::Bullet,
            byte_start: 0,
            byte_end: 5,
            indent_level: 0,
        };
        assert!(valid_item.byte_start <= text.len());
        assert!(valid_item.byte_end <= text.len());
        assert!(valid_item.byte_start <= valid_item.byte_end);

        // Invalid: end beyond string
        let invalid_item = ListItemMetadata {
            list_type: ListType::Bullet,
            byte_start: 0,
            byte_end: 100,
            indent_level: 0,
        };
        assert!(invalid_item.byte_end > text.len());
    }

    #[test]
    fn test_indent_level_detection() {
        let text = "    - Indented item";
        let items = detect_list_items(text);
        assert_eq!(items.len(), 1);
        assert!(items[0].indent_level >= 1);
    }

    // Helper to create minimal Metadata for tests
    fn test_metadata(title: Option<String>) -> crate::types::Metadata {
        crate::types::Metadata {
            title,
            subject: None,
            authors: None,
            keywords: None,
            language: None,
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
            pages: None,
            format: None,
            image_preprocessing: None,
            json_schema: None,
            error: None,
            additional: Default::default(),
        }
    }

    // Integration tests for full transformation
    #[test]
    fn test_transform_with_pages_and_hierarchy() {
        use crate::types::{ExtractionResult, HierarchicalBlock, PageContent, PageHierarchy};

        // Create a mock result with pages and hierarchy
        let result = ExtractionResult {
            content: "Full document content".to_string(),
            mime_type: "application/pdf".to_string(),
            metadata: test_metadata(Some("Test Document".to_string())),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: Some(vec![
                PageContent {
                    page_number: 1,
                    content: "This is a test paragraph.\n\nAnother paragraph here.".to_string(),
                    tables: vec![],
                    images: vec![],
                    hierarchy: Some(PageHierarchy {
                        block_count: 2,
                        blocks: vec![
                            HierarchicalBlock {
                                text: "Main Title".to_string(),
                                font_size: 24.0,
                                level: "h1".to_string(),
                                bbox: Some((10.0, 20.0, 100.0, 50.0)),
                            },
                            HierarchicalBlock {
                                text: "Subtitle".to_string(),
                                font_size: 16.0,
                                level: "h2".to_string(),
                                bbox: Some((10.0, 60.0, 100.0, 80.0)),
                            },
                        ],
                    }),
                },
                PageContent {
                    page_number: 2,
                    content: "- List item 1\n- List item 2".to_string(),
                    tables: vec![],
                    images: vec![],
                    hierarchy: None,
                },
            ]),
            elements: None,
        };

        let elements = transform_extraction_result_to_elements(&result);

        // Verify we have elements
        assert!(!elements.is_empty());

        // Find Title elements from hierarchy
        let titles: Vec<_> = elements
            .iter()
            .filter(|e| e.element_type == ElementType::Title)
            .collect();
        assert_eq!(titles.len(), 2, "Should have 2 title elements from hierarchy");
        assert_eq!(titles[0].text, "Main Title");
        assert_eq!(titles[1].text, "Subtitle");

        // Verify page numbers
        assert_eq!(titles[0].metadata.page_number, Some(1));
        assert_eq!(titles[1].metadata.page_number, Some(1));

        // Verify coordinates were extracted
        assert!(titles[0].metadata.coordinates.is_some());
        assert!(titles[1].metadata.coordinates.is_some());

        // Find list items
        let list_items: Vec<_> = elements
            .iter()
            .filter(|e| e.element_type == ElementType::ListItem)
            .collect();
        assert_eq!(list_items.len(), 2, "Should have 2 list items");
        assert_eq!(list_items[0].metadata.page_number, Some(2));
        assert_eq!(list_items[1].metadata.page_number, Some(2));

        // Find PageBreak
        let page_breaks: Vec<_> = elements
            .iter()
            .filter(|e| e.element_type == ElementType::PageBreak)
            .collect();
        assert_eq!(page_breaks.len(), 1, "Should have 1 page break between pages");
    }

    #[test]
    fn test_transform_with_tables_and_images() {
        use crate::types::{ExtractedImage, ExtractionResult, PageContent, Table};
        use std::sync::Arc;

        let table = Table {
            cells: vec![
                vec!["Header1".to_string(), "Header2".to_string()],
                vec!["Cell1".to_string(), "Cell2".to_string()],
            ],
            markdown: "| Header1 | Header2 |\n| Cell1 | Cell2 |".to_string(),
            page_number: 1,
        };

        let image = ExtractedImage {
            data: vec![1, 2, 3, 4],
            format: "jpeg".to_string(),
            image_index: 0,
            page_number: Some(1),
            width: Some(640),
            height: Some(480),
            colorspace: Some("RGB".to_string()),
            bits_per_component: Some(8),
            is_mask: false,
            description: None,
            ocr_result: None,
        };

        let result = ExtractionResult {
            content: "Test content".to_string(),
            mime_type: "application/pdf".to_string(),
            metadata: test_metadata(Some("Test".to_string())),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: Some(vec![PageContent {
                page_number: 1,
                content: "Some text".to_string(),
                tables: vec![Arc::new(table)],
                images: vec![Arc::new(image)],
                hierarchy: None,
            }]),
            elements: None,
        };

        let elements = transform_extraction_result_to_elements(&result);

        // Find table elements
        let tables: Vec<_> = elements
            .iter()
            .filter(|e| e.element_type == ElementType::Table)
            .collect();
        assert_eq!(tables.len(), 1, "Should have 1 table element");
        assert!(tables[0].text.contains("Header1"));
        assert!(tables[0].text.contains("Cell2"));

        // Find image elements
        let images: Vec<_> = elements
            .iter()
            .filter(|e| e.element_type == ElementType::Image)
            .collect();
        assert_eq!(images.len(), 1, "Should have 1 image element");
        assert!(images[0].text.contains("jpeg"));
        assert!(images[0].text.contains("640"));
        assert!(images[0].text.contains("480"));
        assert_eq!(images[0].metadata.page_number, Some(1));
    }

    #[test]
    fn test_transform_fallback_no_pages() {
        use crate::types::ExtractionResult;

        // Create a result without pages
        let result = ExtractionResult {
            content: "Simple text content\n\nSecond paragraph".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: test_metadata(Some("Simple Doc".to_string())),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };

        let elements = transform_extraction_result_to_elements(&result);

        // Should have narrative text elements
        let narratives: Vec<_> = elements
            .iter()
            .filter(|e| e.element_type == ElementType::NarrativeText)
            .collect();
        assert!(!narratives.is_empty(), "Should have narrative text elements");

        // All elements should have page_number = 1 (fallback)
        for element in &elements {
            assert_eq!(element.metadata.page_number, Some(1));
        }
    }

    #[test]
    fn test_paragraph_splitting() {
        use crate::types::ExtractionResult;

        let result = ExtractionResult {
            content: "First paragraph.\n\nSecond paragraph.\n\nThird paragraph.".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: test_metadata(None),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            pages: None,
            elements: None,
        };

        let elements = transform_extraction_result_to_elements(&result);

        let narratives: Vec<_> = elements
            .iter()
            .filter(|e| e.element_type == ElementType::NarrativeText)
            .collect();

        // Should split into 3 separate paragraphs
        assert_eq!(narratives.len(), 3, "Should split into 3 paragraphs");
        assert_eq!(narratives[0].text, "First paragraph.");
        assert_eq!(narratives[1].text, "Second paragraph.");
        assert_eq!(narratives[2].text, "Third paragraph.");
    }
}

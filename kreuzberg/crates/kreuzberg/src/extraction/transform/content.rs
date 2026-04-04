//! Content processing utilities for transformation.
//!
//! This module handles processing of page content, tables, and images
//! during the transformation to semantic elements.

use crate::types::{BoundingBox, Element, ElementMetadata, ElementType};
use std::collections::HashMap;

use super::elements::{add_paragraphs, detect_list_items, generate_element_id};

/// Adjust a byte offset to the nearest valid UTF-8 char boundary, searching forward.
fn snap_to_char_boundary(s: &str, offset: usize) -> usize {
    let clamped = offset.min(s.len());
    // Search forward for the next valid char boundary
    let mut pos = clamped;
    while pos < s.len() && !s.is_char_boundary(pos) {
        pos += 1;
    }
    pos
}

/// Process page content to extract paragraphs and list items.
pub(super) fn process_content(elements: &mut Vec<Element>, content: &str, page_number: usize, title: &Option<String>) {
    let list_items = detect_list_items(content);
    let mut current_byte_offset = 0;

    for list_item in list_items {
        // Snap offsets to valid char boundaries to prevent panics on multi-byte UTF-8
        let safe_start = snap_to_char_boundary(content, list_item.byte_start);
        let safe_end = snap_to_char_boundary(content, list_item.byte_end);
        let safe_current = snap_to_char_boundary(content, current_byte_offset);

        // Add narrative text/paragraphs before this list item
        if safe_current < safe_start {
            let text_slice = content[safe_current..safe_start].trim();
            add_paragraphs(elements, text_slice, page_number, title);
        }

        // Add the list item itself
        let item_text = content[safe_start..safe_end].trim();
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

        current_byte_offset = safe_end;
    }

    // Add any remaining narrative text/paragraphs
    if current_byte_offset < content.len() {
        let safe_current = snap_to_char_boundary(content, current_byte_offset);
        let text_slice = content[safe_current..].trim();
        add_paragraphs(elements, text_slice, page_number, title);
    }
}

/// Format a table as plain text for element representation.
pub(super) fn format_table_as_text(table: &crate::types::Table) -> String {
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

/// Process hierarchy blocks into Title and NarrativeText elements.
///
/// Returns `true` when any body-level block was emitted, indicating
/// the caller should skip the plain-text `process_content` pass to avoid
/// producing duplicate elements. Body blocks without bounding boxes are still
/// emitted (without coordinates); the flag is set regardless of bbox presence.
pub(super) fn process_hierarchy(
    elements: &mut Vec<Element>,
    hierarchy: &crate::types::PageHierarchy,
    page_number: usize,
    title: &Option<String>,
) -> bool {
    let mut has_any_body_blocks = false;

    for block in &hierarchy.blocks {
        let coords = block.bbox.as_ref().map(|(left, top, right, bottom)| BoundingBox {
            x0: *left as f64,
            y0: *top as f64,
            x1: *right as f64,
            y1: *bottom as f64,
        });

        let element_type = match block.level.as_str() {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => ElementType::Title,
            _ => {
                // Body text: emit as NarrativeText with coordinates when available.
                if block.text.trim().is_empty() {
                    continue;
                }
                has_any_body_blocks = true;
                let element_id = generate_element_id(&block.text, ElementType::NarrativeText, Some(page_number));
                elements.push(Element {
                    element_id,
                    element_type: ElementType::NarrativeText,
                    text: block.text.clone(),
                    metadata: ElementMetadata {
                        page_number: Some(page_number),
                        filename: title.clone(),
                        coordinates: coords,
                        element_index: Some(elements.len()),
                        additional: {
                            let mut m = HashMap::new();
                            m.insert("font_size".to_string(), block.font_size.to_string());
                            m
                        },
                    },
                });
                continue;
            }
        };

        let element_id = generate_element_id(&block.text, element_type, Some(page_number));
        elements.push(Element {
            element_id,
            element_type,
            text: block.text.clone(),
            metadata: ElementMetadata {
                page_number: Some(page_number),
                filename: title.clone(),
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

    has_any_body_blocks
}

/// Process tables on a page into Table elements.
pub(super) fn process_tables(
    elements: &mut Vec<Element>,
    tables: &[std::sync::Arc<crate::types::Table>],
    page_number: usize,
    title: &Option<String>,
) {
    for table_arc in tables {
        let table = table_arc.as_ref();
        let table_text = format_table_as_text(table);

        let element_id = generate_element_id(&table_text, ElementType::Table, Some(page_number));
        elements.push(Element {
            element_id,
            element_type: ElementType::Table,
            text: table_text,
            metadata: ElementMetadata {
                page_number: Some(page_number),
                filename: title.clone(),
                coordinates: None, // Tables don't have bbox in current structure
                element_index: Some(elements.len()),
                additional: HashMap::new(),
            },
        });
    }
}

/// Process images on a page into Image elements.
pub(super) fn process_images(
    elements: &mut Vec<Element>,
    images: &[std::sync::Arc<crate::types::ExtractedImage>],
    page_number: usize,
    title: &Option<String>,
) {
    for image_arc in images {
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
                filename: title.clone(),
                coordinates: None, // Images don't have bbox in current structure
                element_index: Some(elements.len()),
                additional: {
                    let mut m = HashMap::new();
                    m.insert("format".to_string(), image.format.to_string());
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

/// Add a PageBreak element between pages.
pub(super) fn add_page_break(
    elements: &mut Vec<Element>,
    current_page: usize,
    next_page: usize,
    title: &Option<String>,
) {
    let page_break_text = format!("--- PAGE BREAK (page {} → {}) ---", current_page, next_page);
    let element_id = generate_element_id(&page_break_text, ElementType::PageBreak, Some(current_page));
    elements.push(Element {
        element_id,
        element_type: ElementType::PageBreak,
        text: page_break_text,
        metadata: ElementMetadata {
            page_number: Some(current_page),
            filename: title.clone(),
            coordinates: None,
            element_index: Some(elements.len()),
            additional: HashMap::new(),
        },
    });
}

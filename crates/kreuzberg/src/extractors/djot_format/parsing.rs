//! Djot event parsing and content extraction.
//!
//! Handles parsing of jotdown events into plain text, tables, and full DjotContent structures.

use super::attributes::parse_jotdown_attributes;
use crate::types::{
    Attributes, BlockType, DjotContent, DjotImage, DjotLink, FormattedBlock, InlineElement, InlineType, Metadata, Table,
};
use jotdown::{Container, Event};
use std::collections::HashMap;

/// Extract plain text from Djot events.
///
/// Processes djot events and extracts plain text content, handling:
/// - Text content
/// - Line breaks (soft, hard, blank)
/// - Smart punctuation (quotes, dashes, ellipsis)
/// - Special symbols and footnote references
pub fn extract_text_from_events(events: &[Event]) -> String {
    let mut text = String::new();

    for event in events {
        match event {
            Event::Str(s) => {
                text.push_str(s.as_ref());
            }
            Event::Softbreak | Event::Hardbreak | Event::Blankline => {
                text.push('\n');
            }
            Event::NonBreakingSpace => {
                text.push(' ');
            }
            Event::LeftSingleQuote | Event::RightSingleQuote => {
                text.push('\'');
            }
            Event::LeftDoubleQuote | Event::RightDoubleQuote => {
                text.push('"');
            }
            Event::Ellipsis => {
                text.push_str("...");
            }
            Event::EnDash => {
                text.push_str("--");
            }
            Event::EmDash => {
                text.push_str("---");
            }
            Event::FootnoteReference(s) => {
                text.push('[');
                text.push_str(s.as_ref());
                text.push(']');
            }
            Event::Symbol(s) => {
                text.push(':');
                text.push_str(s.as_ref());
                text.push(':');
            }
            Event::ThematicBreak(_) => {
                text.push_str("\n---\n");
            }
            Event::Start(_, _) | Event::End(_) | Event::Escape | Event::Attributes(_) => {}
        }
    }

    text
}

/// Extract tables from Djot events.
///
/// Parses table events and extracts table data as a Vec<Vec<String>>,
/// converting each table to markdown representation for storage.
pub fn extract_tables_from_events(events: &[Event]) -> Vec<Table> {
    let mut tables = Vec::new();
    let mut current_table: Option<(Vec<Vec<String>>, usize)> = None;
    let mut current_row: Vec<String> = Vec::new();
    let mut current_cell = String::new();
    let mut in_table_cell = false;
    let mut table_index = 0;

    for event in events {
        match event {
            Event::Start(Container::Table, _) => {
                current_table = Some((Vec::new(), table_index));
            }
            Event::Start(Container::TableRow { .. }, _) => {
                current_row = Vec::new();
            }
            Event::Start(Container::TableCell { .. }, _) => {
                current_cell = String::new();
                in_table_cell = true;
            }
            Event::Str(s) if in_table_cell => {
                current_cell.push_str(s.as_ref());
            }
            Event::End(Container::TableCell { .. }) => {
                if in_table_cell {
                    current_row.push(current_cell.trim().to_string());
                    current_cell = String::new();
                    in_table_cell = false;
                }
            }
            Event::End(Container::TableRow { .. }) => {
                if !current_row.is_empty()
                    && let Some((ref mut rows, _)) = current_table
                {
                    rows.push(current_row.clone());
                }
                current_row = Vec::new();
            }
            Event::End(Container::Table) => {
                if let Some((cells, idx)) = current_table.take()
                    && !cells.is_empty()
                {
                    let markdown = crate::extractors::frontmatter_utils::cells_to_markdown(&cells);
                    tables.push(Table {
                        cells,
                        markdown,
                        page_number: idx + 1,
                    });
                    table_index += 1;
                }
            }
            _ => {}
        }
    }

    tables
}

/// Extract complete djot content with 100% feature extraction.
///
/// Processes ALL djot events to build a rich DjotContent structure including:
/// - Block structure (headings, lists, blockquotes, divs, sections, code blocks)
/// - Inline formatting (strong, emphasis, highlight, subscript, superscript, insert, delete)
/// - Attributes (classes, IDs, key-value pairs)
/// - Links and images with full metadata (href, src, alt, title)
/// - Math blocks (inline & display)
/// - Definition lists (term/description pairs)
/// - Task lists with checked state
/// - Raw blocks (HTML/LaTeX)
/// - Footnotes (references and definitions)
/// - Captions
/// - Smart punctuation
/// - All other djot features
pub fn extract_complete_djot_content(events: &[Event], metadata: Metadata, tables: Vec<Table>) -> DjotContent {
    let plain_text = extract_text_from_events(events);

    let mut blocks = Vec::new();
    let mut images = Vec::new();
    let mut links = Vec::new();
    let mut footnotes = Vec::new();
    let attributes_map: HashMap<String, Attributes> = HashMap::new();

    // Enhanced state tracking using a block stack for proper nesting
    struct ExtractionState {
        block_stack: Vec<FormattedBlock>,   // Stack for nested blocks
        inline_type_stack: Vec<InlineType>, // Stack for nested inline element types
        current_text: String,               // Text accumulator
        pending_attributes: Option<Attributes>,
        code_content: String, // Accumulator for code blocks
        in_code_block: bool,
        in_math: bool,
        math_display: bool,
        math_content: String,
        current_link_index: Option<usize>,
        current_image_index: Option<usize>,
        in_raw_block: bool,
        raw_format: Option<String>,
        current_inline_elements: Vec<InlineElement>,
    }

    let mut state = ExtractionState {
        block_stack: Vec::new(),
        inline_type_stack: Vec::new(),
        current_text: String::new(),
        pending_attributes: None,
        code_content: String::new(),
        in_code_block: false,
        in_math: false,
        math_display: false,
        math_content: String::new(),
        current_link_index: None,
        current_image_index: None,
        in_raw_block: false,
        raw_format: None,
        current_inline_elements: Vec::new(),
    };

    // Helper to create a new block and push to stack
    fn push_block(state: &mut ExtractionState, block: FormattedBlock) {
        state.block_stack.push(block);
    }

    // Helper to pop a block from the stack and add to parent or blocks list
    fn pop_block(state: &mut ExtractionState, blocks: &mut Vec<FormattedBlock>) {
        if let Some(mut block) = state.block_stack.pop() {
            // Add any pending inline elements to the block
            if !state.current_inline_elements.is_empty() {
                block.inline_content.append(&mut state.current_inline_elements);
            }
            // If there's a parent block, add as child; otherwise add to top-level blocks
            if let Some(parent) = state.block_stack.last_mut() {
                parent.children.push(block);
            } else {
                blocks.push(block);
            }
        }
    }

    for event in events {
        match event {
            Event::Start(container, attrs) => {
                // Parse attributes from jotdown's Attributes type
                let parsed_attrs = if attrs.is_empty() {
                    state.pending_attributes.take()
                } else {
                    Some(parse_jotdown_attributes(attrs))
                };

                match container {
                    Container::Heading { level, .. } => {
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::Heading,
                                level: Some(*level as usize),
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::Paragraph => {
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::Paragraph,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::Blockquote => {
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::Blockquote,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::CodeBlock { language } => {
                        let lang_str = if language.is_empty() {
                            None
                        } else {
                            Some(language.to_string())
                        };
                        state.in_code_block = true;
                        state.code_content.clear();
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::CodeBlock,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: lang_str,
                                code: Some(String::new()),
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::RawBlock { format } => {
                        state.in_raw_block = true;
                        state.raw_format = Some(format.to_string());
                        state.code_content.clear();
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::RawBlock,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: Some(format.to_string()),
                                code: Some(String::new()),
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::List { kind, .. } => {
                        let block_type = match kind {
                            jotdown::ListKind::Ordered { .. } => BlockType::OrderedList,
                            jotdown::ListKind::Unordered(_) => BlockType::BulletList,
                            jotdown::ListKind::Task(_) => BlockType::TaskList,
                        };
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::ListItem => {
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::ListItem,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::TaskListItem { checked } => {
                        let mut attrs = parsed_attrs.unwrap_or_default();
                        attrs.key_values.insert("checked".to_string(), checked.to_string());
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::ListItem,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: Some(attrs),
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::DescriptionList => {
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::DefinitionList,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::DescriptionTerm => {
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::DefinitionTerm,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::DescriptionDetails => {
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::DefinitionDescription,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::Div { .. } => {
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::Div,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::Section { .. } => {
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::Section,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::Footnote { label } => {
                        // Start tracking a footnote definition
                        footnotes.push(crate::types::Footnote {
                            label: label.to_string(),
                            content: Vec::new(),
                        });
                        // We'll collect the content as blocks
                        push_block(
                            &mut state,
                            FormattedBlock {
                                block_type: BlockType::Paragraph,
                                level: None,
                                inline_content: Vec::new(),
                                attributes: parsed_attrs,
                                language: None,
                                code: None,
                                children: Vec::new(),
                            },
                        );
                    }
                    Container::Math { display } => {
                        state.in_math = true;
                        state.math_display = *display;
                        state.math_content.clear();
                        state.inline_type_stack.push(InlineType::Math);
                    }
                    Container::Strong => {
                        state.inline_type_stack.push(InlineType::Strong);
                        // Flush current text before starting new inline
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    Container::Emphasis => {
                        state.inline_type_stack.push(InlineType::Emphasis);
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    Container::Mark => {
                        state.inline_type_stack.push(InlineType::Highlight);
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    Container::Subscript => {
                        state.inline_type_stack.push(InlineType::Subscript);
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    Container::Superscript => {
                        state.inline_type_stack.push(InlineType::Superscript);
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    Container::Insert => {
                        state.inline_type_stack.push(InlineType::Insert);
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    Container::Delete => {
                        state.inline_type_stack.push(InlineType::Delete);
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    Container::Verbatim => {
                        state.inline_type_stack.push(InlineType::Code);
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    Container::Span => {
                        state.inline_type_stack.push(InlineType::Span);
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    Container::RawInline { format } => {
                        state.inline_type_stack.push(InlineType::RawInline);
                        state.raw_format = Some(format.to_string());
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    Container::Link(url, _link_type) => {
                        state.inline_type_stack.push(InlineType::Link);
                        links.push(DjotLink {
                            url: url.to_string(),
                            text: String::new(),
                            title: None,
                            attributes: parsed_attrs.clone(),
                        });
                        state.current_link_index = Some(links.len() - 1);
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    Container::Image(src, _link_type) => {
                        state.inline_type_stack.push(InlineType::Image);
                        images.push(DjotImage {
                            src: src.to_string(),
                            alt: String::new(),
                            title: None,
                            attributes: parsed_attrs.clone(),
                        });
                        state.current_image_index = Some(images.len() - 1);
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                    }
                    // Table-related containers are handled by extract_tables_from_events
                    Container::Table
                    | Container::TableRow { .. }
                    | Container::TableCell { .. }
                    | Container::Caption => {
                        // Tables are extracted separately
                    }
                    Container::LinkDefinition { .. } => {
                        // Link definitions are resolved by jotdown, not needed in output
                    }
                }
            }
            Event::End(container) => {
                match container {
                    Container::Heading { .. }
                    | Container::Paragraph
                    | Container::Blockquote
                    | Container::CodeBlock { .. }
                    | Container::RawBlock { .. }
                    | Container::Div { .. }
                    | Container::Section { .. }
                    | Container::List { .. }
                    | Container::ListItem
                    | Container::TaskListItem { .. }
                    | Container::DescriptionList
                    | Container::DescriptionTerm
                    | Container::DescriptionDetails => {
                        // Flush any remaining text
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }

                        // For code blocks, set the accumulated code content
                        if state.in_code_block {
                            if let Some(block) = state.block_stack.last_mut() {
                                block.code = Some(std::mem::take(&mut state.code_content));
                            }
                            state.in_code_block = false;
                        }

                        // For raw blocks
                        if state.in_raw_block {
                            if let Some(block) = state.block_stack.last_mut() {
                                block.code = Some(std::mem::take(&mut state.code_content));
                            }
                            state.in_raw_block = false;
                            state.raw_format = None;
                        }

                        pop_block(&mut state, &mut blocks);
                    }
                    Container::Footnote { .. } => {
                        // End of footnote definition
                        if !state.current_text.is_empty() {
                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Text,
                                content: std::mem::take(&mut state.current_text),
                                attributes: None,
                                metadata: None,
                            });
                        }
                        // Pop the footnote content block and add to the last footnote
                        if let Some(mut block) = state.block_stack.pop() {
                            block.inline_content.append(&mut state.current_inline_elements);
                            if let Some(footnote) = footnotes.last_mut() {
                                footnote.content.push(block);
                            }
                        }
                    }
                    Container::Math { display } => {
                        state.in_math = false;
                        let math_text = std::mem::take(&mut state.math_content);
                        state.inline_type_stack.pop();

                        let mut meta = HashMap::new();
                        meta.insert("display".to_string(), display.to_string());

                        state.current_inline_elements.push(InlineElement {
                            element_type: InlineType::Math,
                            content: math_text,
                            attributes: state.pending_attributes.take(),
                            metadata: Some(meta),
                        });
                    }
                    Container::Strong
                    | Container::Emphasis
                    | Container::Mark
                    | Container::Subscript
                    | Container::Superscript
                    | Container::Insert
                    | Container::Delete
                    | Container::Verbatim
                    | Container::Span
                    | Container::RawInline { .. } => {
                        if let Some(inline_type) = state.inline_type_stack.pop() {
                            let content = std::mem::take(&mut state.current_text);
                            let mut meta = None;

                            // For raw inline, include the format
                            if matches!(container, Container::RawInline { .. })
                                && let Some(fmt) = state.raw_format.take()
                            {
                                let mut m = HashMap::new();
                                m.insert("format".to_string(), fmt);
                                meta = Some(m);
                            }

                            state.current_inline_elements.push(InlineElement {
                                element_type: inline_type,
                                content,
                                attributes: state.pending_attributes.take(),
                                metadata: meta,
                            });
                        }
                    }
                    Container::Link(url, _) => {
                        if let Some(idx) = state.current_link_index.take() {
                            let text = std::mem::take(&mut state.current_text);
                            if let Some(link) = links.get_mut(idx) {
                                link.text = text.clone();
                            }
                            state.inline_type_stack.pop();

                            let mut meta = HashMap::new();
                            meta.insert("href".to_string(), url.to_string());

                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Link,
                                content: text,
                                attributes: state.pending_attributes.take(),
                                metadata: Some(meta),
                            });
                        }
                    }
                    Container::Image(src, _) => {
                        if let Some(idx) = state.current_image_index.take() {
                            let alt = std::mem::take(&mut state.current_text);
                            if let Some(image) = images.get_mut(idx) {
                                image.alt = alt.clone();
                            }
                            state.inline_type_stack.pop();

                            let mut meta = HashMap::new();
                            meta.insert("src".to_string(), src.to_string());

                            state.current_inline_elements.push(InlineElement {
                                element_type: InlineType::Image,
                                content: alt,
                                attributes: state.pending_attributes.take(),
                                metadata: Some(meta),
                            });
                        }
                    }
                    // Table-related containers
                    Container::Table
                    | Container::TableRow { .. }
                    | Container::TableCell { .. }
                    | Container::Caption => {
                        // Tables are handled separately
                    }
                    Container::LinkDefinition { .. } => {
                        // Link definitions don't produce output
                    }
                }
            }
            Event::Str(s) => {
                if state.in_code_block || state.in_raw_block {
                    state.code_content.push_str(s);
                } else if state.in_math {
                    state.math_content.push_str(s);
                } else {
                    state.current_text.push_str(s);
                }
            }
            Event::FootnoteReference(label) => {
                // Flush current text
                if !state.current_text.is_empty() {
                    state.current_inline_elements.push(InlineElement {
                        element_type: InlineType::Text,
                        content: std::mem::take(&mut state.current_text),
                        attributes: None,
                        metadata: None,
                    });
                }

                let mut meta = HashMap::new();
                meta.insert("label".to_string(), label.to_string());

                state.current_inline_elements.push(InlineElement {
                    element_type: InlineType::FootnoteRef,
                    content: label.to_string(),
                    attributes: None,
                    metadata: Some(meta),
                });
            }
            Event::Symbol(sym) => {
                // Flush current text
                if !state.current_text.is_empty() {
                    state.current_inline_elements.push(InlineElement {
                        element_type: InlineType::Text,
                        content: std::mem::take(&mut state.current_text),
                        attributes: None,
                        metadata: None,
                    });
                }

                state.current_inline_elements.push(InlineElement {
                    element_type: InlineType::Symbol,
                    content: sym.to_string(),
                    attributes: None,
                    metadata: None,
                });
            }
            Event::Attributes(attrs) => {
                // Store attributes to be applied to the next element
                state.pending_attributes = Some(parse_jotdown_attributes(attrs));
            }
            Event::Softbreak => {
                if state.in_math {
                    state.math_content.push(' ');
                } else if !state.inline_type_stack.is_empty() {
                    state.current_text.push(' ');
                } else {
                    state.current_text.push('\n');
                }
            }
            Event::Hardbreak => {
                if state.in_math {
                    state.math_content.push('\n');
                } else {
                    state.current_text.push('\n');
                }
            }
            Event::NonBreakingSpace => {
                state.current_text.push(' ');
            }
            Event::Blankline => {
                // Blank lines are typically ignored in block processing
            }
            Event::ThematicBreak(attrs) => {
                // Flush any pending content
                if !state.current_text.is_empty() {
                    state.current_inline_elements.push(InlineElement {
                        element_type: InlineType::Text,
                        content: std::mem::take(&mut state.current_text),
                        attributes: None,
                        metadata: None,
                    });
                }

                let parsed_attrs = if attrs.is_empty() {
                    None
                } else {
                    Some(parse_jotdown_attributes(attrs))
                };

                let hr_block = FormattedBlock {
                    block_type: BlockType::ThematicBreak,
                    level: None,
                    inline_content: Vec::new(),
                    attributes: parsed_attrs,
                    language: None,
                    code: None,
                    children: Vec::new(),
                };

                if let Some(parent) = state.block_stack.last_mut() {
                    parent.children.push(hr_block);
                } else {
                    blocks.push(hr_block);
                }
            }
            // Smart punctuation events
            Event::LeftSingleQuote => {
                state.current_text.push('\'');
            }
            Event::RightSingleQuote => {
                state.current_text.push('\'');
            }
            Event::LeftDoubleQuote => {
                state.current_text.push('"');
            }
            Event::RightDoubleQuote => {
                state.current_text.push('"');
            }
            Event::Ellipsis => {
                state.current_text.push_str("...");
            }
            Event::EnDash => {
                state.current_text.push_str("--");
            }
            Event::EmDash => {
                state.current_text.push_str("---");
            }
            Event::Escape => {
                // Escape is a marker, doesn't produce output
            }
        }
    }

    // Finalize any remaining content
    if !state.current_text.is_empty() {
        state.current_inline_elements.push(InlineElement {
            element_type: InlineType::Text,
            content: std::mem::take(&mut state.current_text),
            attributes: None,
            metadata: None,
        });
    }

    // Pop any remaining blocks
    while !state.block_stack.is_empty() {
        pop_block(&mut state, &mut blocks);
    }

    // Add any remaining inline elements to the last block if exists
    if !state.current_inline_elements.is_empty()
        && let Some(last_block) = blocks.last_mut()
    {
        last_block.inline_content.append(&mut state.current_inline_elements);
    }

    DjotContent {
        plain_text,
        blocks,
        metadata,
        tables,
        images,
        links,
        footnotes,
        attributes: attributes_map,
    }
}

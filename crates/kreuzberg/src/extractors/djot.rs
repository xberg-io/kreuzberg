//! Djot markup extractor with YAML frontmatter support.
//!
//! This extractor provides:
//! - Djot parsing using the jotdown crate
//! - YAML frontmatter metadata extraction (same as Markdown)
//! - Table extraction as structured data
//! - Heading structure preservation
//! - Code block and link extraction
//!
//! Djot is a modern markup language with simpler parsing rules than CommonMark.
//! See https://djot.net for the specification.
//!
//! Requires the `djot` feature.

use super::frontmatter_utils::{
    cells_to_markdown, extract_frontmatter, extract_metadata_from_yaml, extract_title_from_content,
};
use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::plugins::{DocumentExtractor, Plugin};
use crate::types::{ExtractionResult, Metadata, Table};
use async_trait::async_trait;
use jotdown::{Container, Event, Parser};

/// Djot markup extractor with metadata and table support.
///
/// Parses Djot documents with YAML frontmatter, extracting:
/// - Metadata from YAML frontmatter
/// - Plain text content
/// - Tables as structured data
/// - Document structure (headings, links, code blocks)
pub struct DjotExtractor;

impl DjotExtractor {
    /// Create a new Djot extractor.
    pub fn new() -> Self {
        Self
    }

    // Frontmatter utilities moved to shared frontmatter_utils module

    /// Extract plain text from Djot events.
    fn extract_text_from_events<'a>(events: &[Event<'a>]) -> String {
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
    fn extract_tables_from_events<'a>(events: &[Event<'a>]) -> Vec<Table> {
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
                        let markdown = cells_to_markdown(&cells);
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
    fn extract_complete_djot_content<'a>(
        events: &[Event<'a>],
        metadata: Metadata,
        tables: Vec<Table>,
    ) -> crate::types::DjotContent {
        use crate::types::*;
        use std::collections::HashMap;

        let plain_text = Self::extract_text_from_events(events);

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
                            footnotes.push(Footnote {
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

        crate::types::DjotContent {
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

    // cells_to_markdown and extract_title_from_content moved to shared frontmatter_utils module
}

/// Parse jotdown's Attributes type into our Attributes type
///
/// Iterates over jotdown's attribute elements and extracts:
/// - ID from `#id` syntax (AttributeKind::Id)
/// - Classes from `.class` syntax (AttributeKind::Class)
/// - Key-value pairs from `key=value` syntax (AttributeKind::Pair)
/// - Comments are ignored (AttributeKind::Comment)
fn parse_jotdown_attributes(attrs: &jotdown::Attributes) -> crate::types::Attributes {
    use crate::types::Attributes;
    use jotdown::AttributeKind;
    use std::collections::HashMap;

    let mut id = None;
    let mut classes = Vec::new();
    let mut key_values = HashMap::new();

    for (kind, value) in attrs.iter() {
        match kind {
            AttributeKind::Id => {
                // Last ID wins if multiple are specified
                id = Some(value.to_string());
            }
            AttributeKind::Class => {
                classes.push(value.to_string());
            }
            AttributeKind::Pair { key } => {
                key_values.insert(key.to_string(), value.to_string());
            }
            AttributeKind::Comment => {
                // Comments are ignored in our representation
            }
        }
    }

    Attributes {
        id,
        classes,
        key_values,
    }
}

/// Parse djot attribute syntax from string: {.class #id key="value"}
#[allow(dead_code)]
fn parse_djot_attributes(attr_str: &str) -> crate::types::Attributes {
    use crate::types::Attributes;
    use std::collections::HashMap;

    let mut attrs = Attributes {
        id: None,
        classes: Vec::new(),
        key_values: HashMap::new(),
    };

    // Simple parser for attribute syntax
    let tokens = attr_str.split_whitespace();

    for token in tokens {
        if let Some(class) = token.strip_prefix('.') {
            // Class
            attrs.classes.push(class.to_string());
        } else if let Some(id) = token.strip_prefix('#') {
            // ID
            attrs.id = Some(id.to_string());
        } else if token.contains('=') {
            // Key-value pair
            if let Some((key, value)) = token.split_once('=') {
                let clean_value = value.trim_matches('"').trim_matches('\'');
                attrs.key_values.insert(key.to_string(), clean_value.to_string());
            }
        }
    }

    attrs
}

impl Default for DjotExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for DjotExtractor {
    fn name(&self) -> &str {
        "djot-extractor"
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn initialize(&self) -> Result<()> {
        Ok(())
    }

    fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    fn description(&self) -> &str {
        "Extracts content from Djot markup files with YAML frontmatter and table support"
    }

    fn author(&self) -> &str {
        "Kreuzberg Team"
    }
}

#[async_trait]
impl DocumentExtractor for DjotExtractor {
    #[cfg_attr(feature = "otel", tracing::instrument(
        skip(self, content, _config),
        fields(
            extractor.name = self.name(),
            content.size_bytes = content.len(),
        )
    ))]
    async fn extract_bytes(
        &self,
        content: &[u8],
        mime_type: &str,
        _config: &ExtractionConfig,
    ) -> Result<ExtractionResult> {
        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml, remaining_content) = extract_frontmatter(&text);

        let mut metadata = if let Some(ref yaml_value) = yaml {
            extract_metadata_from_yaml(yaml_value)
        } else {
            Metadata::default()
        };

        if !metadata.additional.contains_key("title")
            && let Some(title) = extract_title_from_content(&remaining_content)
        {
            metadata.additional.insert("title".to_string(), title.into());
        }

        // Parse with jotdown and collect events once for extraction
        let parser = Parser::new(&remaining_content);
        let events: Vec<Event> = parser.collect();

        let extracted_text = Self::extract_text_from_events(&events);
        let tables = Self::extract_tables_from_events(&events);

        // Extract complete djot content with all features
        let djot_content = Self::extract_complete_djot_content(&events, metadata.clone(), tables.clone());

        Ok(ExtractionResult {
            content: extracted_text,
            mime_type: mime_type.to_string(),
            metadata,
            tables,
            detected_languages: None,
            chunks: None,
            images: None,
            pages: None,
            djot_content: Some(djot_content),
            elements: None,
        })
    }

    fn supported_mime_types(&self) -> &[&str] {
        &["text/djot", "text/x-djot"]
    }

    fn priority(&self) -> i32 {
        50
    }
}

// ============================================================================
// Djot Output Generation Functions
// ============================================================================

/// Convert DjotContent back to djot markup.
///
/// This function takes a `DjotContent` structure and generates valid djot markup
/// from it, preserving:
/// - Block structure (headings, code blocks, lists, blockquotes, etc.)
/// - Inline formatting (strong, emphasis, highlight, subscript, superscript, etc.)
/// - Attributes where present ({.class #id key="value"})
///
/// # Arguments
///
/// * `content` - The DjotContent to convert
///
/// # Returns
///
/// A String containing valid djot markup
///
/// # Example
///
/// ```ignore
/// let djot_content = // ... extract from some source
/// let markup = djot_content_to_djot(&djot_content);
/// println!("{}", markup);
/// ```
pub fn djot_content_to_djot(content: &crate::types::DjotContent) -> String {
    let mut output = String::new();

    for block in &content.blocks {
        render_block_to_djot(&mut output, block, 0);
    }

    output
}

/// Render a single block to djot markup.
fn render_block_to_djot(output: &mut String, block: &crate::types::FormattedBlock, indent_level: usize) {
    use crate::types::BlockType;

    let indent = "  ".repeat(indent_level);

    // Render attributes if present
    let attrs_str = block.attributes.as_ref().map(render_attributes).unwrap_or_default();

    match block.block_type {
        BlockType::Heading => {
            let level = block.level.unwrap_or(1);
            let hashes = "#".repeat(level);
            output.push_str(&indent);
            output.push_str(&hashes);
            output.push(' ');
            render_inline_content(output, &block.inline_content);
            if !attrs_str.is_empty() {
                output.push(' ');
                output.push_str(&attrs_str);
            }
            output.push('\n');
            output.push('\n');
        }
        BlockType::Paragraph => {
            output.push_str(&indent);
            render_inline_content(output, &block.inline_content);
            if !attrs_str.is_empty() {
                output.push(' ');
                output.push_str(&attrs_str);
            }
            output.push('\n');
            output.push('\n');
        }
        BlockType::CodeBlock => {
            if !attrs_str.is_empty() {
                output.push_str(&indent);
                output.push_str(&attrs_str);
                output.push('\n');
            }
            output.push_str(&indent);
            output.push_str("```");
            if let Some(ref lang) = block.language {
                output.push(' ');
                output.push_str(lang);
            }
            output.push('\n');
            if let Some(ref code) = block.code {
                for line in code.lines() {
                    output.push_str(&indent);
                    output.push_str(line);
                    output.push('\n');
                }
            } else {
                // Fall back to inline content if code field is empty
                for elem in &block.inline_content {
                    output.push_str(&indent);
                    output.push_str(&elem.content);
                    output.push('\n');
                }
            }
            output.push_str(&indent);
            output.push_str("```\n\n");
        }
        BlockType::Blockquote => {
            if !attrs_str.is_empty() {
                output.push_str(&indent);
                output.push_str(&attrs_str);
                output.push('\n');
            }
            // Render inline content as quoted
            output.push_str(&indent);
            output.push_str("> ");
            render_inline_content(output, &block.inline_content);
            output.push('\n');
            // Render children (nested content)
            for child in &block.children {
                let child_output = {
                    let mut s = String::new();
                    render_block_to_djot(&mut s, child, 0);
                    s
                };
                for line in child_output.lines() {
                    output.push_str(&indent);
                    output.push_str("> ");
                    output.push_str(line);
                    output.push('\n');
                }
            }
            output.push('\n');
        }
        BlockType::BulletList => {
            if !attrs_str.is_empty() {
                output.push_str(&indent);
                output.push_str(&attrs_str);
                output.push('\n');
            }
            for child in &block.children {
                render_list_item(output, child, &indent, "- ");
            }
            output.push('\n');
        }
        BlockType::OrderedList => {
            if !attrs_str.is_empty() {
                output.push_str(&indent);
                output.push_str(&attrs_str);
                output.push('\n');
            }
            for (i, child) in block.children.iter().enumerate() {
                let marker = format!("{}. ", i + 1);
                render_list_item(output, child, &indent, &marker);
            }
            output.push('\n');
        }
        BlockType::TaskList => {
            if !attrs_str.is_empty() {
                output.push_str(&indent);
                output.push_str(&attrs_str);
                output.push('\n');
            }
            for child in &block.children {
                // Task list items use [ ] or [x] syntax
                render_list_item(output, child, &indent, "- [ ] ");
            }
            output.push('\n');
        }
        BlockType::ListItem => {
            // List items are typically rendered by their parent list
            output.push_str(&indent);
            render_inline_content(output, &block.inline_content);
            output.push('\n');
            for child in &block.children {
                render_block_to_djot(output, child, indent_level + 1);
            }
        }
        BlockType::DefinitionList => {
            if !attrs_str.is_empty() {
                output.push_str(&indent);
                output.push_str(&attrs_str);
                output.push('\n');
            }
            for child in &block.children {
                render_block_to_djot(output, child, indent_level);
            }
            output.push('\n');
        }
        BlockType::DefinitionTerm => {
            output.push_str(&indent);
            render_inline_content(output, &block.inline_content);
            output.push('\n');
        }
        BlockType::DefinitionDescription => {
            output.push_str(&indent);
            output.push_str(": ");
            render_inline_content(output, &block.inline_content);
            output.push('\n');
        }
        BlockType::Div => {
            output.push_str(&indent);
            output.push_str(":::");
            if !attrs_str.is_empty() {
                output.push(' ');
                output.push_str(&attrs_str);
            }
            output.push('\n');
            for child in &block.children {
                render_block_to_djot(output, child, indent_level);
            }
            output.push_str(&indent);
            output.push_str(":::\n\n");
        }
        BlockType::Section => {
            // Sections don't have special syntax, just render children
            if !attrs_str.is_empty() {
                output.push_str(&indent);
                output.push_str(&attrs_str);
                output.push('\n');
            }
            for child in &block.children {
                render_block_to_djot(output, child, indent_level);
            }
        }
        BlockType::ThematicBreak => {
            output.push_str(&indent);
            output.push_str("---\n\n");
        }
        BlockType::RawBlock => {
            // Raw blocks use ``` with format specifier
            output.push_str(&indent);
            output.push_str("```");
            if let Some(ref lang) = block.language {
                output.push('=');
                output.push_str(lang);
            }
            output.push('\n');
            for elem in &block.inline_content {
                output.push_str(&indent);
                output.push_str(&elem.content);
                output.push('\n');
            }
            output.push_str(&indent);
            output.push_str("```\n\n");
        }
        BlockType::MathDisplay => {
            output.push_str(&indent);
            output.push_str("$$\n");
            for elem in &block.inline_content {
                output.push_str(&indent);
                output.push_str(&elem.content);
                output.push('\n');
            }
            output.push_str(&indent);
            output.push_str("$$\n\n");
        }
    }
}

/// Render a list item with the given marker.
fn render_list_item(output: &mut String, item: &crate::types::FormattedBlock, indent: &str, marker: &str) {
    output.push_str(indent);
    output.push_str(marker);
    render_inline_content(output, &item.inline_content);
    output.push('\n');
    for child in &item.children {
        render_block_to_djot(output, child, 1);
    }
}

/// Render inline content to djot markup.
fn render_inline_content(output: &mut String, elements: &[crate::types::InlineElement]) {
    use crate::types::InlineType;

    for elem in elements {
        let attrs_str = elem.attributes.as_ref().map(render_attributes).unwrap_or_default();

        match elem.element_type {
            InlineType::Text => {
                output.push_str(&elem.content);
            }
            InlineType::Strong => {
                output.push('*');
                output.push_str(&elem.content);
                output.push('*');
                if !attrs_str.is_empty() {
                    output.push_str(&attrs_str);
                }
            }
            InlineType::Emphasis => {
                output.push('_');
                output.push_str(&elem.content);
                output.push('_');
                if !attrs_str.is_empty() {
                    output.push_str(&attrs_str);
                }
            }
            InlineType::Highlight => {
                output.push_str("{=");
                output.push_str(&elem.content);
                output.push_str("=}");
                if !attrs_str.is_empty() {
                    output.push_str(&attrs_str);
                }
            }
            InlineType::Subscript => {
                output.push('~');
                output.push_str(&elem.content);
                output.push('~');
                if !attrs_str.is_empty() {
                    output.push_str(&attrs_str);
                }
            }
            InlineType::Superscript => {
                output.push('^');
                output.push_str(&elem.content);
                output.push('^');
                if !attrs_str.is_empty() {
                    output.push_str(&attrs_str);
                }
            }
            InlineType::Insert => {
                output.push_str("{+");
                output.push_str(&elem.content);
                output.push_str("+}");
                if !attrs_str.is_empty() {
                    output.push_str(&attrs_str);
                }
            }
            InlineType::Delete => {
                output.push_str("{-");
                output.push_str(&elem.content);
                output.push_str("-}");
                if !attrs_str.is_empty() {
                    output.push_str(&attrs_str);
                }
            }
            InlineType::Code => {
                output.push('`');
                output.push_str(&elem.content);
                output.push('`');
                if !attrs_str.is_empty() {
                    output.push_str(&attrs_str);
                }
            }
            InlineType::Link => {
                let href = elem
                    .metadata
                    .as_ref()
                    .and_then(|m| m.get("href"))
                    .map(|s| s.as_str())
                    .unwrap_or("");
                output.push('[');
                output.push_str(&elem.content);
                output.push_str("](");
                output.push_str(href);
                output.push(')');
                if !attrs_str.is_empty() {
                    output.push_str(&attrs_str);
                }
            }
            InlineType::Image => {
                let src = elem
                    .metadata
                    .as_ref()
                    .and_then(|m| m.get("src"))
                    .map(|s| s.as_str())
                    .unwrap_or("");
                output.push_str("![");
                output.push_str(&elem.content); // alt text
                output.push_str("](");
                output.push_str(src);
                output.push(')');
                if !attrs_str.is_empty() {
                    output.push_str(&attrs_str);
                }
            }
            InlineType::Span => {
                output.push('[');
                output.push_str(&elem.content);
                output.push(']');
                if !attrs_str.is_empty() {
                    output.push_str(&attrs_str);
                }
            }
            InlineType::Math => {
                output.push('$');
                output.push_str(&elem.content);
                output.push('$');
            }
            InlineType::RawInline => {
                // Raw inline uses `content`{=format}
                let format = elem
                    .metadata
                    .as_ref()
                    .and_then(|m| m.get("format"))
                    .map(|s| s.as_str())
                    .unwrap_or("html");
                output.push('`');
                output.push_str(&elem.content);
                output.push_str("`{=");
                output.push_str(format);
                output.push('}');
            }
            InlineType::FootnoteRef => {
                output.push_str("[^");
                output.push_str(&elem.content);
                output.push(']');
            }
            InlineType::Symbol => {
                output.push(':');
                output.push_str(&elem.content);
                output.push(':');
            }
        }
    }
}

/// Render attributes to djot attribute syntax.
fn render_attributes(attrs: &crate::types::Attributes) -> String {
    let mut parts = Vec::new();

    if let Some(ref id) = attrs.id {
        parts.push(format!("#{}", id));
    }

    for class in &attrs.classes {
        parts.push(format!(".{}", class));
    }

    for (key, value) in &attrs.key_values {
        parts.push(format!("{}=\"{}\"", key, value));
    }

    if parts.is_empty() {
        String::new()
    } else {
        format!("{{{}}}", parts.join(" "))
    }
}

/// Convert any ExtractionResult to djot format.
///
/// This function converts an `ExtractionResult` to djot markup:
/// - If `djot_content` is `Some`, uses `djot_content_to_djot` for full fidelity conversion
/// - Otherwise, wraps the plain text content in paragraphs
///
/// # Arguments
///
/// * `result` - The ExtractionResult to convert
///
/// # Returns
///
/// A `Result` containing the djot markup string
///
/// # Example
///
/// ```ignore
/// let result = extractor.extract_bytes(bytes, "text/plain", &config).await?;
/// let djot_markup = extraction_result_to_djot(&result)?;
/// ```
pub fn extraction_result_to_djot(result: &crate::types::ExtractionResult) -> crate::Result<String> {
    if let Some(ref djot_content) = result.djot_content {
        Ok(djot_content_to_djot(djot_content))
    } else {
        // Convert plain text to basic djot paragraphs
        let mut output = String::new();

        // Split content by double newlines to create paragraphs
        let paragraphs: Vec<&str> = result.content.split("\n\n").collect();

        for para in paragraphs {
            let trimmed = para.trim();
            if !trimmed.is_empty() {
                output.push_str(trimmed);
                output.push_str("\n\n");
            }
        }

        Ok(output)
    }
}

/// Render djot content to HTML.
///
/// This function takes djot source text and renders it to HTML using jotdown's
/// built-in HTML renderer.
///
/// # Arguments
///
/// * `djot_source` - The djot markup text to render
///
/// # Returns
///
/// A `Result` containing the rendered HTML string
///
/// # Example
///
/// ```ignore
/// let djot = "# Hello\n\nThis is *bold* and _italic_.";
/// let html = djot_to_html(djot)?;
/// assert!(html.contains("<h1>"));
/// assert!(html.contains("<strong>"));
/// assert!(html.contains("<em>"));
/// ```
pub fn djot_to_html(djot_source: &str) -> crate::Result<String> {
    let parser = Parser::new(djot_source);
    let html = jotdown::html::render_to_string(parser);
    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_extract_djot_mime_types() {
        let extractor = DjotExtractor::new();
        let mime_types = extractor.supported_mime_types();

        assert!(mime_types.contains(&"text/djot"));
        assert!(mime_types.contains(&"text/x-djot"));
    }

    #[test]
    fn test_extract_simple_djot() {
        let content =
            b"# Header\n\nThis is a paragraph with *bold* and _italic_ text.\n\n## Subheading\n\nMore content here.";
        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml, remaining) = extract_frontmatter(&text);
        assert!(yaml.is_none());
        assert!(!remaining.is_empty());

        let parser = Parser::new(&remaining);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(&events);

        assert!(extracted.contains("Header"));
        assert!(extracted.contains("This is a paragraph"));
        assert!(extracted.contains("bold"));
        assert!(extracted.contains("italic"));
    }

    #[test]
    fn test_extract_frontmatter_metadata() {
        let content = b"---\ntitle: My Document\nauthor: John Doe\ndate: 2024-01-15\nkeywords: rust, djot, extraction\ndescription: A test document\n---\n\n# Content\n\nBody text.";

        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml_opt, remaining) = extract_frontmatter(&text);
        assert!(yaml_opt.is_some());
        assert!(remaining.contains("# Content"));

        let yaml = yaml_opt.expect("Should extract YAML frontmatter");
        let metadata = extract_metadata_from_yaml(&yaml);

        assert_eq!(
            metadata.additional.get("title").and_then(|v| v.as_str()),
            Some("My Document")
        );
        assert_eq!(
            metadata.additional.get("author").and_then(|v| v.as_str()),
            Some("John Doe")
        );
        assert_eq!(metadata.created_at, Some("2024-01-15".to_string()));
        assert!(metadata.subject.is_some());
        assert!(
            metadata
                .subject
                .as_ref()
                .expect("Should have subject description")
                .contains("test document")
        );
    }

    #[test]
    fn test_extract_frontmatter_metadata_array_keywords() {
        let content = b"---\ntitle: Document\nkeywords:\n  - rust\n  - djot\n  - parsing\n---\n\nContent";

        let text = String::from_utf8_lossy(content).into_owned();
        let (yaml_opt, _remaining) = extract_frontmatter(&text);

        assert!(yaml_opt.is_some());
        let yaml = yaml_opt.expect("Should extract YAML frontmatter");
        let metadata = extract_metadata_from_yaml(&yaml);

        let keywords = metadata
            .additional
            .get("keywords")
            .and_then(|v: &serde_json::Value| v.as_str());
        assert!(keywords.is_some());
        let keywords_str = keywords.expect("Should extract keywords from metadata");
        assert!(keywords_str.contains("rust"));
        assert!(keywords_str.contains("djot"));
    }

    #[tokio::test]
    async fn test_extract_tables() {
        let content = b"# Tables Example\n\n| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |\n| Cell 3   | Cell 4   |";

        let extractor = DjotExtractor::new();
        let result = extractor
            .extract_bytes(content, "text/djot", &ExtractionConfig::default())
            .await
            .expect("Should extract djot with tables");

        assert!(!result.tables.is_empty());
        let table = &result.tables[0];
        assert!(!table.cells.is_empty());
        assert_eq!(table.cells[0].len(), 2);
        assert!(!table.markdown.is_empty());
    }

    #[test]
    fn test_extract_without_frontmatter() {
        let content = b"# Main Title\n\nSome content\n\nMore text";
        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml, remaining) = extract_frontmatter(&text);
        assert!(yaml.is_none());
        assert_eq!(remaining, text);

        let title = extract_title_from_content(&remaining);
        assert_eq!(title, Some("Main Title".to_string()));
    }

    #[test]
    fn test_empty_document() {
        let content = b"";
        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml, remaining) = extract_frontmatter(&text);
        assert!(yaml.is_none());
        assert!(remaining.is_empty());

        let parser = Parser::new(&remaining);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(&events);
        assert!(extracted.is_empty());
    }

    #[test]
    fn test_unicode_content() {
        let content = "# 日本語のタイトル\n\nこれは日本語の内容です。\n\n## Español\n\nEste es un documento en español.\n\n## Русский\n\nЭто русский текст.".as_bytes();

        let text = String::from_utf8_lossy(content).into_owned();

        let (yaml, remaining) = extract_frontmatter(&text);
        assert!(yaml.is_none());

        let parser = Parser::new(&remaining);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(&events);

        assert!(extracted.contains("日本語"));
        assert!(extracted.contains("Español"));
        assert!(extracted.contains("Русский"));
    }

    #[tokio::test]
    async fn test_full_extraction_with_frontmatter_and_tables() {
        let content = b"---\ntitle: Complete Document\nauthor: Test Author\ndate: 2024-01-20\n---\n\n# Document\n\nIntroduction text.\n\n| Name | Value |\n|------|-------|\n| A    | 1     |\n| B    | 2     |";

        let extractor = DjotExtractor::new();
        let result = extractor
            .extract_bytes(content, "text/x-djot", &ExtractionConfig::default())
            .await
            .expect("Should extract djot with frontmatter and tables");

        assert_eq!(result.mime_type, "text/x-djot");
        assert!(result.content.contains("Introduction text"));
        assert_eq!(
            result.metadata.additional.get("title").and_then(|v| v.as_str()),
            Some("Complete Document")
        );
        assert_eq!(
            result.metadata.additional.get("author").and_then(|v| v.as_str()),
            Some("Test Author")
        );
        assert!(!result.tables.is_empty());
    }

    #[test]
    fn test_plugin_interface() {
        let extractor = DjotExtractor::new();
        assert_eq!(extractor.name(), "djot-extractor");
        assert_eq!(extractor.version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(extractor.priority(), 50);
        assert!(extractor.supported_mime_types().contains(&"text/djot"));
    }

    #[test]
    fn test_cells_to_markdown() {
        let cells = vec![
            vec!["Header 1".to_string(), "Header 2".to_string()],
            vec!["Data 1".to_string(), "Data 2".to_string()],
            vec!["Data 3".to_string(), "Data 4".to_string()],
        ];

        let markdown = cells_to_markdown(&cells);
        assert!(markdown.contains("Header 1"));
        assert!(markdown.contains("Data 1"));
        assert!(markdown.contains("---"));
        let lines: Vec<&str> = markdown.lines().collect();
        assert!(lines.len() >= 4);
    }

    #[test]
    fn test_extract_djot_with_links() {
        let content = b"# Page\n\nCheck [Google](https://google.com) and [Rust](https://rust-lang.org).";
        let text = String::from_utf8_lossy(content).into_owned();

        let parser = Parser::new(&text);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(&events);

        assert!(extracted.contains("Google"));
        assert!(extracted.contains("Rust"));
    }

    #[test]
    fn test_extract_djot_with_code_blocks() {
        let content = b"# Code Example\n\n``` rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let text = String::from_utf8_lossy(content).into_owned();

        let parser = Parser::new(&text);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(&events);

        assert!(extracted.contains("main"));
        assert!(extracted.contains("println"));
    }

    #[test]
    fn test_djot_emphasis_syntax() {
        // Djot uses _ for emphasis and * for strong (opposite of Markdown)
        let content = b"This has _emphasis_ and *strong* text.";
        let text = String::from_utf8_lossy(content).into_owned();

        let parser = Parser::new(&text);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(&events);

        assert!(extracted.contains("emphasis"));
        assert!(extracted.contains("strong"));
    }

    #[test]
    fn test_djot_smart_punctuation() {
        let content = b"He said \"Hello\" and she said 'Hi'... That's nice---really.";
        let text = String::from_utf8_lossy(content).into_owned();

        let parser = Parser::new(&text);
        let events: Vec<Event> = parser.collect();
        let extracted = DjotExtractor::extract_text_from_events(&events);

        // Smart quotes and dashes should be converted to ASCII equivalents
        assert!(extracted.contains("Hello"));
        assert!(extracted.contains("Hi"));
    }

    // ============================================================================
    // Djot Output Generation Tests
    // ============================================================================

    #[test]
    fn test_djot_content_to_djot_heading() {
        use crate::types::*;

        let content = DjotContent {
            plain_text: "Hello World".to_string(),
            blocks: vec![FormattedBlock {
                block_type: BlockType::Heading,
                level: Some(1),
                inline_content: vec![InlineElement {
                    element_type: InlineType::Text,
                    content: "Hello World".to_string(),
                    attributes: None,
                    metadata: None,
                }],
                attributes: None,
                language: None,
                code: None,
                children: vec![],
            }],
            metadata: Metadata::default(),
            tables: vec![],
            images: vec![],
            links: vec![],
            footnotes: vec![],
            attributes: std::collections::HashMap::new(),
        };

        let djot = djot_content_to_djot(&content);
        assert!(djot.contains("# Hello World"));
    }

    #[test]
    fn test_djot_content_to_djot_paragraph() {
        use crate::types::*;

        let content = DjotContent {
            plain_text: "This is a paragraph.".to_string(),
            blocks: vec![FormattedBlock {
                block_type: BlockType::Paragraph,
                level: None,
                inline_content: vec![InlineElement {
                    element_type: InlineType::Text,
                    content: "This is a paragraph.".to_string(),
                    attributes: None,
                    metadata: None,
                }],
                attributes: None,
                language: None,
                code: None,
                children: vec![],
            }],
            metadata: Metadata::default(),
            tables: vec![],
            images: vec![],
            links: vec![],
            footnotes: vec![],
            attributes: std::collections::HashMap::new(),
        };

        let djot = djot_content_to_djot(&content);
        assert!(djot.contains("This is a paragraph."));
    }

    #[test]
    fn test_djot_content_to_djot_code_block() {
        use crate::types::*;

        let content = DjotContent {
            plain_text: "fn main() {}".to_string(),
            blocks: vec![FormattedBlock {
                block_type: BlockType::CodeBlock,
                level: None,
                inline_content: vec![],
                attributes: None,
                language: Some("rust".to_string()),
                code: Some("fn main() {}".to_string()),
                children: vec![],
            }],
            metadata: Metadata::default(),
            tables: vec![],
            images: vec![],
            links: vec![],
            footnotes: vec![],
            attributes: std::collections::HashMap::new(),
        };

        let djot = djot_content_to_djot(&content);
        assert!(djot.contains("```"));
        assert!(djot.contains("rust"));
        assert!(djot.contains("fn main()"));
    }

    #[test]
    fn test_djot_content_to_djot_inline_formatting() {
        use crate::types::*;

        let content = DjotContent {
            plain_text: "bold italic".to_string(),
            blocks: vec![FormattedBlock {
                block_type: BlockType::Paragraph,
                level: None,
                inline_content: vec![
                    InlineElement {
                        element_type: InlineType::Strong,
                        content: "bold".to_string(),
                        attributes: None,
                        metadata: None,
                    },
                    InlineElement {
                        element_type: InlineType::Text,
                        content: " ".to_string(),
                        attributes: None,
                        metadata: None,
                    },
                    InlineElement {
                        element_type: InlineType::Emphasis,
                        content: "italic".to_string(),
                        attributes: None,
                        metadata: None,
                    },
                ],
                attributes: None,
                language: None,
                code: None,
                children: vec![],
            }],
            metadata: Metadata::default(),
            tables: vec![],
            images: vec![],
            links: vec![],
            footnotes: vec![],
            attributes: std::collections::HashMap::new(),
        };

        let djot = djot_content_to_djot(&content);
        assert!(djot.contains("*bold*"));
        assert!(djot.contains("_italic_"));
    }

    #[test]
    fn test_djot_content_to_djot_link() {
        use crate::types::*;
        use std::collections::HashMap;

        let mut meta = HashMap::new();
        meta.insert("href".to_string(), "https://example.com".to_string());

        let content = DjotContent {
            plain_text: "click here".to_string(),
            blocks: vec![FormattedBlock {
                block_type: BlockType::Paragraph,
                level: None,
                inline_content: vec![InlineElement {
                    element_type: InlineType::Link,
                    content: "click here".to_string(),
                    attributes: None,
                    metadata: Some(meta),
                }],
                attributes: None,
                language: None,
                code: None,
                children: vec![],
            }],
            metadata: Metadata::default(),
            tables: vec![],
            images: vec![],
            links: vec![],
            footnotes: vec![],
            attributes: std::collections::HashMap::new(),
        };

        let djot = djot_content_to_djot(&content);
        assert!(djot.contains("[click here](https://example.com)"));
    }

    #[test]
    fn test_extraction_result_to_djot_with_djot_content() {
        use crate::types::*;

        let result = ExtractionResult {
            content: "Hello World".to_string(),
            mime_type: "text/djot".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            pages: None,
            elements: None,
            djot_content: Some(DjotContent {
                plain_text: "Hello World".to_string(),
                blocks: vec![FormattedBlock {
                    block_type: BlockType::Heading,
                    level: Some(1),
                    inline_content: vec![InlineElement {
                        element_type: InlineType::Text,
                        content: "Hello World".to_string(),
                        attributes: None,
                        metadata: None,
                    }],
                    attributes: None,
                    language: None,
                    code: None,
                    children: vec![],
                }],
                metadata: Metadata::default(),
                tables: vec![],
                images: vec![],
                links: vec![],
                footnotes: vec![],
                attributes: std::collections::HashMap::new(),
            }),
        };

        let djot = extraction_result_to_djot(&result).unwrap();
        assert!(djot.contains("# Hello World"));
    }

    #[test]
    fn test_extraction_result_to_djot_without_djot_content() {
        use crate::types::*;

        let result = ExtractionResult {
            content: "First paragraph.\n\nSecond paragraph.".to_string(),
            mime_type: "text/plain".to_string(),
            metadata: Metadata::default(),
            tables: vec![],
            detected_languages: None,
            chunks: None,
            images: None,
            pages: None,
            elements: None,
            djot_content: None,
        };

        let djot = extraction_result_to_djot(&result).unwrap();
        // Should wrap plain text in paragraphs
        assert!(djot.contains("First paragraph."));
        assert!(djot.contains("Second paragraph."));
    }

    #[test]
    fn test_djot_to_html_heading() {
        let djot = "# Hello World";
        let html = djot_to_html(djot).unwrap();
        assert!(html.contains("<h1>"));
        assert!(html.contains("Hello World"));
        assert!(html.contains("</h1>"));
    }

    #[test]
    fn test_djot_to_html_paragraph() {
        let djot = "This is a paragraph.";
        let html = djot_to_html(djot).unwrap();
        assert!(html.contains("<p>"));
        assert!(html.contains("This is a paragraph."));
        assert!(html.contains("</p>"));
    }

    #[test]
    fn test_djot_to_html_formatting() {
        let djot = "This is *strong* and _emphasis_.";
        let html = djot_to_html(djot).unwrap();
        assert!(html.contains("<strong>"));
        assert!(html.contains("strong"));
        assert!(html.contains("<em>"));
        assert!(html.contains("emphasis"));
    }

    #[test]
    fn test_djot_to_html_code_block() {
        let djot = "``` rust\nfn main() {}\n```";
        let html = djot_to_html(djot).unwrap();
        assert!(html.contains("<pre>") || html.contains("<code"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_djot_to_html_link() {
        let djot = "[Example](https://example.com)";
        let html = djot_to_html(djot).unwrap();
        assert!(html.contains("<a"));
        assert!(html.contains("href=\"https://example.com\""));
        assert!(html.contains("Example"));
    }

    #[tokio::test]
    async fn test_djot_roundtrip_heading() {
        let original = "# Hello World";
        let extractor = DjotExtractor::new();
        let result = extractor
            .extract_bytes(original.as_bytes(), "text/djot", &ExtractionConfig::default())
            .await
            .unwrap();

        // Verify the djot_content was extracted
        assert!(result.djot_content.is_some());
        let djot_content = result.djot_content.as_ref().unwrap();

        // Generate djot markup from the content
        let regenerated = djot_content_to_djot(djot_content);

        // Verify key content is preserved
        assert!(regenerated.contains("Hello World"), "Heading text should be preserved");
        // The heading level should be preserved (# for h1)
        assert!(regenerated.contains("#"), "Heading marker should be present");
    }

    #[tokio::test]
    async fn test_djot_roundtrip_formatting() {
        let original = "This is *bold* and _italic_.";
        let extractor = DjotExtractor::new();
        let result = extractor
            .extract_bytes(original.as_bytes(), "text/djot", &ExtractionConfig::default())
            .await
            .unwrap();

        assert!(result.djot_content.is_some());
        let djot_content = result.djot_content.as_ref().unwrap();

        let regenerated = djot_content_to_djot(djot_content);

        // Verify content is preserved
        assert!(regenerated.contains("bold"), "Bold text should be preserved");
        assert!(regenerated.contains("italic"), "Italic text should be preserved");
    }

    #[tokio::test]
    async fn test_djot_roundtrip_code_block() {
        let original = "``` rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
        let extractor = DjotExtractor::new();
        let result = extractor
            .extract_bytes(original.as_bytes(), "text/djot", &ExtractionConfig::default())
            .await
            .unwrap();

        assert!(result.djot_content.is_some());
        let djot_content = result.djot_content.as_ref().unwrap();

        let regenerated = djot_content_to_djot(djot_content);

        assert!(regenerated.contains("```"), "Code fence should be present");
        assert!(regenerated.contains("rust"), "Language should be preserved");
        assert!(regenerated.contains("fn main()"), "Code content should be preserved");
    }

    #[tokio::test]
    async fn test_djot_roundtrip_link() {
        let original = "Check out [Example](https://example.com) for more.";
        let extractor = DjotExtractor::new();
        let result = extractor
            .extract_bytes(original.as_bytes(), "text/djot", &ExtractionConfig::default())
            .await
            .unwrap();

        assert!(result.djot_content.is_some());
        let djot_content = result.djot_content.as_ref().unwrap();

        let regenerated = djot_content_to_djot(djot_content);

        assert!(regenerated.contains("Example"), "Link text should be preserved");
        assert!(
            regenerated.contains("https://example.com"),
            "Link URL should be preserved"
        );
    }

    #[tokio::test]
    async fn test_djot_roundtrip_complex_document() {
        let original = r#"# Main Title

This is an introduction paragraph with *bold* and _italic_ text.

## Section One

- Item one
- Item two
- Item three

``` python
def hello():
    print("Hello, World!")
```

Check [this link](https://rust-lang.org) for more information.

## Section Two

> This is a blockquote
> with multiple lines.

The end.
"#;
        let extractor = DjotExtractor::new();
        let result = extractor
            .extract_bytes(original.as_bytes(), "text/djot", &ExtractionConfig::default())
            .await
            .unwrap();

        assert!(result.djot_content.is_some());
        let djot_content = result.djot_content.as_ref().unwrap();

        let regenerated = djot_content_to_djot(djot_content);

        // Verify main structural elements are preserved
        assert!(regenerated.contains("Main Title"), "Main title should be preserved");
        assert!(regenerated.contains("bold"), "Bold text should be preserved");
        assert!(regenerated.contains("italic"), "Italic text should be preserved");
        assert!(
            regenerated.contains("python") || regenerated.contains("def hello"),
            "Code block should be preserved"
        );
        assert!(regenerated.contains("rust-lang.org"), "Link should be preserved");
        assert!(
            regenerated.contains("blockquote"),
            "Blockquote content should be preserved"
        );
    }

    #[test]
    fn test_djot_content_to_djot_blockquote() {
        use crate::types::*;

        let content = DjotContent {
            plain_text: "A quoted text".to_string(),
            blocks: vec![FormattedBlock {
                block_type: BlockType::Blockquote,
                level: None,
                inline_content: vec![InlineElement {
                    element_type: InlineType::Text,
                    content: "A quoted text".to_string(),
                    attributes: None,
                    metadata: None,
                }],
                attributes: None,
                language: None,
                code: None,
                children: vec![],
            }],
            metadata: Metadata::default(),
            tables: vec![],
            images: vec![],
            links: vec![],
            footnotes: vec![],
            attributes: std::collections::HashMap::new(),
        };

        let djot = djot_content_to_djot(&content);
        assert!(djot.contains(">"), "Blockquote marker should be present");
        assert!(djot.contains("A quoted text"), "Blockquote content should be present");
    }

    #[test]
    fn test_djot_content_to_djot_bullet_list() {
        use crate::types::*;

        let content = DjotContent {
            plain_text: "Item 1\nItem 2".to_string(),
            blocks: vec![FormattedBlock {
                block_type: BlockType::BulletList,
                level: None,
                inline_content: vec![],
                attributes: None,
                language: None,
                code: None,
                children: vec![
                    FormattedBlock {
                        block_type: BlockType::ListItem,
                        level: None,
                        inline_content: vec![InlineElement {
                            element_type: InlineType::Text,
                            content: "Item 1".to_string(),
                            attributes: None,
                            metadata: None,
                        }],
                        attributes: None,
                        language: None,
                        code: None,
                        children: vec![],
                    },
                    FormattedBlock {
                        block_type: BlockType::ListItem,
                        level: None,
                        inline_content: vec![InlineElement {
                            element_type: InlineType::Text,
                            content: "Item 2".to_string(),
                            attributes: None,
                            metadata: None,
                        }],
                        attributes: None,
                        language: None,
                        code: None,
                        children: vec![],
                    },
                ],
            }],
            metadata: Metadata::default(),
            tables: vec![],
            images: vec![],
            links: vec![],
            footnotes: vec![],
            attributes: std::collections::HashMap::new(),
        };

        let djot = djot_content_to_djot(&content);
        assert!(djot.contains("- Item 1"), "First list item should be present");
        assert!(djot.contains("- Item 2"), "Second list item should be present");
    }

    #[test]
    fn test_djot_content_to_djot_ordered_list() {
        use crate::types::*;

        let content = DjotContent {
            plain_text: "First\nSecond".to_string(),
            blocks: vec![FormattedBlock {
                block_type: BlockType::OrderedList,
                level: None,
                inline_content: vec![],
                attributes: None,
                language: None,
                code: None,
                children: vec![
                    FormattedBlock {
                        block_type: BlockType::ListItem,
                        level: None,
                        inline_content: vec![InlineElement {
                            element_type: InlineType::Text,
                            content: "First".to_string(),
                            attributes: None,
                            metadata: None,
                        }],
                        attributes: None,
                        language: None,
                        code: None,
                        children: vec![],
                    },
                    FormattedBlock {
                        block_type: BlockType::ListItem,
                        level: None,
                        inline_content: vec![InlineElement {
                            element_type: InlineType::Text,
                            content: "Second".to_string(),
                            attributes: None,
                            metadata: None,
                        }],
                        attributes: None,
                        language: None,
                        code: None,
                        children: vec![],
                    },
                ],
            }],
            metadata: Metadata::default(),
            tables: vec![],
            images: vec![],
            links: vec![],
            footnotes: vec![],
            attributes: std::collections::HashMap::new(),
        };

        let djot = djot_content_to_djot(&content);
        assert!(djot.contains("1. First"), "First ordered item should be present");
        assert!(djot.contains("2. Second"), "Second ordered item should be present");
    }

    #[test]
    fn test_djot_content_to_djot_inline_code() {
        use crate::types::*;

        let content = DjotContent {
            plain_text: "Use the println! macro.".to_string(),
            blocks: vec![FormattedBlock {
                block_type: BlockType::Paragraph,
                level: None,
                inline_content: vec![
                    InlineElement {
                        element_type: InlineType::Text,
                        content: "Use the ".to_string(),
                        attributes: None,
                        metadata: None,
                    },
                    InlineElement {
                        element_type: InlineType::Code,
                        content: "println!".to_string(),
                        attributes: None,
                        metadata: None,
                    },
                    InlineElement {
                        element_type: InlineType::Text,
                        content: " macro.".to_string(),
                        attributes: None,
                        metadata: None,
                    },
                ],
                attributes: None,
                language: None,
                code: None,
                children: vec![],
            }],
            metadata: Metadata::default(),
            tables: vec![],
            images: vec![],
            links: vec![],
            footnotes: vec![],
            attributes: std::collections::HashMap::new(),
        };

        let djot = djot_content_to_djot(&content);
        assert!(djot.contains("`println!`"), "Inline code should be present");
    }

    #[test]
    fn test_djot_to_html_list() {
        let djot = "- Item one\n- Item two\n- Item three";
        let html = djot_to_html(djot).unwrap();
        assert!(html.contains("<ul>") || html.contains("<li>"));
        assert!(html.contains("Item one"));
        assert!(html.contains("Item two"));
    }

    #[test]
    fn test_djot_to_html_blockquote() {
        let djot = "> This is a quote";
        let html = djot_to_html(djot).unwrap();
        assert!(html.contains("<blockquote>"));
        assert!(html.contains("This is a quote"));
    }

    #[test]
    fn test_djot_to_html_multiple_headings() {
        let djot = "# H1\n\n## H2\n\n### H3";
        let html = djot_to_html(djot).unwrap();
        assert!(html.contains("<h1>"));
        assert!(html.contains("<h2>"));
        assert!(html.contains("<h3>"));
    }
}

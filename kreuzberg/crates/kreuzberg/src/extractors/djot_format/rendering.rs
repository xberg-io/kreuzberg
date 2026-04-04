//! Djot content rendering to djot markup.
//!
//! Converts DjotContent structures back to valid djot markup with full formatting preservation.

use super::attributes::render_attributes;
use crate::types::{BlockType, FormattedBlock, InlineElement, InlineType};

/// Render a single block to djot markup.
pub fn render_block_to_djot(output: &mut String, block: &FormattedBlock, indent_level: usize) {
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
pub fn render_list_item(output: &mut String, item: &FormattedBlock, indent: &str, marker: &str) {
    output.push_str(indent);
    output.push_str(marker);
    render_inline_content(output, &item.inline_content);
    output.push('\n');
    for child in &item.children {
        render_block_to_djot(output, child, 1);
    }
}

/// Render inline content to djot markup.
pub fn render_inline_content(output: &mut String, elements: &[InlineElement]) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{DjotContent, Metadata};

    #[test]
    fn test_render_heading() {
        let content = DjotContent {
            plain_text: "Test Heading".to_string(),
            blocks: vec![FormattedBlock {
                block_type: BlockType::Heading,
                level: Some(1),
                inline_content: vec![InlineElement {
                    element_type: InlineType::Text,
                    content: "Test Heading".to_string(),
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
            attributes: Default::default(),
        };

        let mut output = String::new();
        for block in &content.blocks {
            render_block_to_djot(&mut output, block, 0);
        }

        assert!(output.contains("# Test Heading"));
    }

    #[test]
    fn test_render_paragraph() {
        let content = DjotContent {
            plain_text: "Test paragraph".to_string(),
            blocks: vec![FormattedBlock {
                block_type: BlockType::Paragraph,
                level: None,
                inline_content: vec![InlineElement {
                    element_type: InlineType::Text,
                    content: "Test paragraph".to_string(),
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
            attributes: Default::default(),
        };

        let mut output = String::new();
        for block in &content.blocks {
            render_block_to_djot(&mut output, block, 0);
        }

        assert!(output.contains("Test paragraph"));
    }
}

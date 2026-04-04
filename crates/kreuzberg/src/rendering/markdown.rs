//! Render an `InternalDocument` to GFM-compliant Markdown via comrak.

use comrak::{Arena, Options, format_commonmark};

use crate::types::internal::InternalDocument;

use super::comrak_bridge::build_comrak_ast;

/// Render an `InternalDocument` to GFM Markdown.
pub fn render_markdown(doc: &InternalDocument) -> String {
    tracing::debug!(element_count = doc.elements.len(), "markdown rendering starting");
    let arena = Arena::new();
    let root = build_comrak_ast(doc, &arena);

    // Guard: empty AST causes index-out-of-bounds in comrak's formatter.
    if root.first_child().is_none() {
        tracing::debug!("markdown rendering: empty AST, returning empty string");
        return String::new();
    }

    let mut options = comrak_options();
    options.render.width = 0; // no line wrapping

    let mut output = String::new();
    format_commonmark(root, &options, &mut output).expect("comrak formatting should not fail");

    // Strip comrak-generated HTML comments (e.g. `<!-- end list -->`) that leak
    // into markdown output when adjacent lists are rendered. Only page marker
    // comments (`<!-- PAGE N -->`) should appear in output, and those are inserted
    // by our own code, not by comrak.
    if output.contains("<!--") {
        output = output
            .lines()
            .filter(|line| {
                let trimmed = line.trim();
                !trimmed.starts_with("<!--") || !trimmed.ends_with("-->")
            })
            .collect::<Vec<_>>()
            .join("\n");
    }

    // Strip escaped HTML tags from PDFs that embed raw HTML in their text layer.
    // comrak escapes `<tag>` as `\<tag\>` to prevent interpretation. We strip these
    // entirely since our output should be clean markdown, not HTML artifacts.
    // Handles: \<p\>, \</p\>, \<br /\>, \<a href="..."\>, etc.
    if output.contains("\\<") {
        // Remove escaped HTML tags: \<....\> patterns
        let mut cleaned = String::with_capacity(output.len());
        let mut chars = output.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\\' && chars.peek() == Some(&'<') {
                // Skip the \< ... \> sequence
                chars.next(); // consume '<'
                let mut found_close = false;
                while let Some(inner) = chars.next() {
                    if inner == '\\' && chars.peek() == Some(&'>') {
                        chars.next(); // consume '>'
                        found_close = true;
                        break;
                    }
                    if inner == '\n' {
                        // Tag spans newline — not a real HTML tag, put content back
                        cleaned.push('\\');
                        cleaned.push('<');
                        cleaned.push(inner);
                        break;
                    }
                }
                if !found_close {
                    // Reached end without closing — not a tag
                }
                // Add a space if we removed a block-level tag (prevents word merging)
                if found_close && !cleaned.ends_with(' ') && !cleaned.ends_with('\n') {
                    cleaned.push(' ');
                }
            } else {
                cleaned.push(ch);
            }
        }
        output = cleaned;
        // Clean up multiple spaces left by tag removal
        while output.contains("  ") {
            output = output.replace("  ", " ");
        }
    }

    // Safety net: decode any HTML entities that slipped through from other code paths.
    // `&#10;` (newline) → space, `&#2;` (STX control char) → removed.
    if output.contains("&#") {
        output = output.replace("&#10;", " ").replace("&#2;", "");
    }

    // Un-escape underscores: comrak's format_commonmark escapes underscores as `\_`
    // to prevent emphasis interpretation, but our rendered content uses underscores
    // literally (e.g. sheet names like `first_sheet`). Since we never emit intentional
    // `\_` sequences, globally replacing is safe.
    if output.contains("\\_") {
        output = output.replace("\\_", "_");
    }

    // Un-escape brackets and parentheses: comrak's format_commonmark escapes `[`, `]`,
    // `(`, `)` in text nodes to prevent accidental link syntax. Since the AST already
    // handles real links via NodeValue::Link (rendered as `[text](url)` without
    // escaping), all remaining `\[`, `\]`, `\(`, `\)` are literal characters that
    // should appear un-escaped.
    if output.contains("\\[") || output.contains("\\]") || output.contains("\\(") || output.contains("\\)") {
        output = output
            .replace("\\[", "[")
            .replace("\\]", "]")
            .replace("\\(", "(")
            .replace("\\)", ")");
    }

    // Un-escape stars and hashes at the START of lines only.
    // comrak escapes `*` → `\*` and `#` → `\#` to prevent false emphasis / ATX-heading
    // interpretation. We need to un-escape these for RST list markers (`\* item` → `* item`)
    // and auto-numbered lists (`\#. item` → `#. item`), but NOT inside table cells where
    // `\*\*text\*\*` should remain escaped as literal asterisks.
    if output.contains("\\*") || output.contains("\\#") {
        output = output
            .lines()
            .map(|line| {
                let trimmed = line.trim_start();
                if trimmed.starts_with("\\* ") || trimmed.starts_with("\\#.") || trimmed.starts_with("\\#\\.") {
                    line.replacen("\\*", "*", 1).replacen("\\#", "#", 1)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
    }

    // Trim trailing whitespace but keep single trailing newline
    let trimmed_len = output.trim_end().len();
    if trimmed_len == 0 {
        return String::new();
    }
    output.truncate(trimmed_len);
    output.push('\n');
    tracing::debug!(output_length = output.len(), "markdown rendering complete");
    output
}

/// Shared comrak options with all GFM extensions enabled.
pub(crate) fn comrak_options<'a>() -> Options<'a> {
    let mut options = Options::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.math_dollars = true;
    options.extension.underline = true;
    options.extension.subscript = true;
    options.extension.superscript = true;
    options.extension.highlight = true;
    options.extension.alerts = true;
    // Use fenced code blocks (```) instead of 4-space indentation.
    options.render.prefer_fenced = true;
    options
}

//! Unified rendering of document content to output formats.
//!
//! - [`render_markdown`] — GFM Markdown (via comrak)
//! - [`render_html`] — HTML5 (via comrak)
//! - [`render_djot`] — Djot markup
//! - [`render_plain`] — Plain text (no formatting)
//! - [`render_html_str`] — parse HTML string then render to HTML
//! - [`render_markdown_str`] — parse HTML string then render to Markdown
//! - [`render_djot_str`] — parse HTML string then render to Djot
//! - [`render_json_str`] — parse HTML string then render to JSON tree
//! - [`render_plain_str`] — parse HTML string then render to plain text

pub(crate) mod common;
mod comrak_bridge;
mod djot;
mod html;
#[cfg(feature = "html")]
pub mod html_styled;
mod json;
mod markdown;
mod plain;

pub use djot::render_djot;
pub use html::render_html;
#[cfg(feature = "html")]
pub use html_styled::StyledHtmlRenderer;
pub use json::render_json;
pub use markdown::render_markdown;
pub use plain::render_plain;

// ============================================================================
// HTML-string convenience wrappers
// ============================================================================

#[cfg(feature = "html")]
/// Parse an HTML string into an [`InternalDocument`] using the default config.
fn parse_html_to_internal(html: &str) -> crate::Result<crate::types::internal::InternalDocument> {
    use crate::extractors::SyncExtractor;
    use crate::extractors::html::HtmlExtractor;
    let config = crate::core::config::ExtractionConfig::default();
    HtmlExtractor.extract_sync(html.as_bytes(), "text/html", &config)
}

#[cfg(feature = "html")]
/// Parse an HTML string and render it to HTML5.
///
/// # Errors
///
/// Returns an error if the HTML cannot be parsed.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::render_html_str;
///
/// # fn example() -> kreuzberg::Result<()> {
/// let out = render_html_str("<h1>Hello</h1><p>World</p>")?;
/// assert!(out.contains("Hello"));
/// # Ok(())
/// # }
/// ```
pub fn render_html_str(html: &str) -> crate::Result<String> {
    let doc = parse_html_to_internal(html)?;
    Ok(render_html(&doc))
}

#[cfg(feature = "html")]
/// Parse an HTML string and render it to GFM Markdown.
///
/// # Errors
///
/// Returns an error if the HTML cannot be parsed.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::render_markdown_str;
///
/// # fn example() -> kreuzberg::Result<()> {
/// let out = render_markdown_str("<h1>Hello</h1><p>World</p>")?;
/// assert!(out.contains("# Hello"));
/// # Ok(())
/// # }
/// ```
pub fn render_markdown_str(html: &str) -> crate::Result<String> {
    let doc = parse_html_to_internal(html)?;
    Ok(render_markdown(&doc))
}

#[cfg(feature = "html")]
/// Parse an HTML string and render it to Djot markup.
///
/// # Errors
///
/// Returns an error if the HTML cannot be parsed.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::render_djot_str;
///
/// # fn example() -> kreuzberg::Result<()> {
/// let out = render_djot_str("<h1>Hello</h1><p>World</p>")?;
/// assert!(out.contains("# Hello"));
/// # Ok(())
/// # }
/// ```
pub fn render_djot_str(html: &str) -> crate::Result<String> {
    let doc = parse_html_to_internal(html)?;
    Ok(render_djot(&doc))
}

#[cfg(feature = "html")]
/// Parse an HTML string and render it to a JSON tree string.
///
/// # Errors
///
/// Returns an error if the HTML cannot be parsed.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::render_json_str;
///
/// # fn example() -> kreuzberg::Result<()> {
/// let out = render_json_str("<p>Hello</p>")?;
/// assert!(out.contains("paragraph"));
/// # Ok(())
/// # }
/// ```
pub fn render_json_str(html: &str) -> crate::Result<String> {
    let doc = parse_html_to_internal(html)?;
    Ok(render_json(&doc))
}

#[cfg(feature = "html")]
/// Parse an HTML string and render it to plain text.
///
/// # Errors
///
/// Returns an error if the HTML cannot be parsed.
///
/// # Examples
///
/// ```rust
/// use kreuzberg::render_plain_str;
///
/// # fn example() -> kreuzberg::Result<()> {
/// let out = render_plain_str("<p>Hello world</p>")?;
/// assert!(out.contains("Hello world"));
/// # Ok(())
/// # }
/// ```
pub fn render_plain_str(html: &str) -> crate::Result<String> {
    let doc = parse_html_to_internal(html)?;
    Ok(render_plain(&doc))
}

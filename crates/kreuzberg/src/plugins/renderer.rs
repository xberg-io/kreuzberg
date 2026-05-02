//! Renderer plugin trait.
//!
//! This module defines the trait for implementing custom document renderers
//! that convert [`InternalDocument`] to output format strings.

use crate::Result;
use crate::types::internal::InternalDocument;

/// Trait for document renderers that convert [`InternalDocument`] to output strings.
///
/// Renderers are stateless converters that transform the internal document
/// representation into a specific output format (Markdown, HTML, Djot, plain text, etc.).
///
/// # Thread Safety
///
/// Renderers must be `Send + Sync` to support concurrent rendering across threads.
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::Renderer;
/// use kreuzberg::types::internal::InternalDocument;
/// use kreuzberg::Result;
///
/// struct CustomRenderer;
///
/// impl Renderer for CustomRenderer {
///     fn name(&self) -> &str { "custom" }
///
///     fn render(&self, doc: &InternalDocument) -> Result<String> {
///         // Custom rendering logic
///         Ok(format!("Custom output with {} elements", doc.elements.len()))
///     }
/// }
/// ```
pub trait Renderer: Send + Sync {
    /// The format name (e.g., "markdown", "html", "djot", "plain").
    fn name(&self) -> &str;

    /// Render an [`InternalDocument`] to the output format.
    ///
    /// # Arguments
    ///
    /// * `doc` - The internal document to render
    ///
    /// # Returns
    ///
    /// The rendered output as a string.
    ///
    /// # Errors
    ///
    /// Returns an error if rendering fails.
    fn render(&self, doc: &InternalDocument) -> Result<String>;
}

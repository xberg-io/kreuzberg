//! Renderer plugin trait.
//!
//! This module defines the trait for implementing custom document renderers
//! that convert [`InternalDocument`] to output format strings.

use crate::Result;
use crate::types::internal::InternalDocument;
use std::sync::Arc;

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

/// Register a renderer with the global registry.
///
/// # Arguments
///
/// * `renderer` - The renderer implementation wrapped in Arc
///
/// # Returns
///
/// - `Ok(())` if registration succeeded
/// - `Err(...)` if validation failed
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::{Renderer, register_renderer};
/// use kreuzberg::types::internal::InternalDocument;
/// use kreuzberg::Result;
/// use std::sync::Arc;
///
/// struct MyRenderer;
/// impl Renderer for MyRenderer {
///     fn name(&self) -> &str { "my-format" }
///     fn render(&self, _doc: &InternalDocument) -> Result<String> {
///         Ok("rendered".to_string())
///     }
/// }
///
/// register_renderer(Arc::new(MyRenderer)).unwrap();
/// ```
pub fn register_renderer(renderer: Arc<dyn Renderer>) -> crate::Result<()> {
    use crate::plugins::registry::get_renderer_registry;

    let registry = get_renderer_registry();
    let mut registry = registry.write();

    registry.register(renderer)
}

/// Unregister a renderer by name.
///
/// Removes the renderer from the global registry.
///
/// # Arguments
///
/// * `name` - Name of the renderer to unregister
///
/// # Returns
///
/// - `Ok(())` if the renderer was unregistered or didn't exist
pub fn unregister_renderer(name: &str) -> crate::Result<()> {
    use crate::plugins::registry::get_renderer_registry;

    let registry = get_renderer_registry();
    let mut registry = registry.write();

    registry.remove(name);
    Ok(())
}

/// List all registered renderers.
///
/// Returns the names of all renderers currently registered in the global registry.
///
/// # Returns
///
/// A vector of renderer names.
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::list_renderers;
///
/// let renderers = list_renderers();
/// for name in renderers {
///     println!("Registered renderer: {}", name);
/// }
/// ```
pub fn list_renderers() -> Vec<String> {
    use crate::plugins::registry::get_renderer_registry;

    let registry = get_renderer_registry();
    let registry = registry.read();

    registry.list()
}

/// Clear all renderers from the global registry and re-register built-in defaults.
///
/// # Returns
///
/// - `Ok(())` if all renderers were cleared and defaults re-registered
pub fn clear_renderers() -> crate::Result<()> {
    use crate::plugins::registry::get_renderer_registry;

    let registry = get_renderer_registry();
    let mut registry = registry.write();

    registry.reset_to_defaults()
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestRenderer {
        format_name: &'static str,
    }

    impl Renderer for TestRenderer {
        fn name(&self) -> &str {
            self.format_name
        }

        fn render(&self, doc: &InternalDocument) -> Result<String> {
            Ok(format!("rendered {} elements", doc.elements.len()))
        }
    }

    #[test]
    fn test_renderer_trait() {
        let renderer = TestRenderer {
            format_name: "test-format",
        };
        assert_eq!(renderer.name(), "test-format");

        let doc = InternalDocument::new("text/plain");
        let result = renderer.render(&doc).unwrap();
        assert!(result.contains("rendered"));
    }

    #[test]
    fn test_register_and_list_renderer() {
        let renderer = Arc::new(TestRenderer {
            format_name: "test-register",
        });
        register_renderer(renderer).unwrap();

        let renderers = list_renderers();
        assert!(renderers.contains(&"test-register".to_string()));

        unregister_renderer("test-register").unwrap();
    }
}

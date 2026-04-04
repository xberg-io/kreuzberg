//! Post-processor registry management.
//!
//! This module provides functions for querying the global post-processor registry.

/// List all registered post-processor names.
///
/// Returns a vector of all post-processor names currently registered in the
/// global registry.
///
/// # Returns
///
/// - `Ok(Vec<String>)` - Vector of post-processor names
/// - `Err(...)` if the registry lock is poisoned
///
/// # Example
///
/// ```rust
/// use kreuzberg::plugins::list_post_processors;
///
/// # tokio_test::block_on(async {
/// let processors = list_post_processors()?;
/// for name in processors {
///     println!("Registered post-processor: {}", name);
/// }
/// # Ok::<(), kreuzberg::KreuzbergError>(())
/// # });
/// ```
pub fn list_post_processors() -> crate::Result<Vec<String>> {
    use crate::plugins::registry::get_post_processor_registry;

    let registry = get_post_processor_registry();
    let registry = registry.read();

    Ok(registry.list())
}

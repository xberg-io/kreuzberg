use napi::bindgen_prelude::*;

mod ocr_backend;
/// Plugin system implementations for Kreuzberg
///
/// This module provides support for extending Kreuzberg's functionality through plugins:
/// - **PostProcessor**: Custom document post-processing
/// - **Validator**: Custom validation logic
/// - **OcrBackend**: Custom OCR implementations
mod post_processor;
mod validator;

pub use ocr_backend::*;
pub use post_processor::*;
pub use validator::*;

/// Helper function to validate that a plugin object has all required methods.
///
/// # Arguments
///
/// * `obj` - The JavaScript object to validate
/// * `plugin_type` - Human-readable plugin type name for error messages
/// * `required_methods` - Array of method names that must be present
///
/// # Returns
///
/// Ok(()) if all required methods are present, Err otherwise
///
/// # Example
///
/// ```rust,ignore
/// validate_plugin_object(&processor, "PostProcessor", &["name", "process"])?;
/// ```
fn validate_plugin_object(obj: &Object, plugin_type: &str, required_methods: &[&str]) -> Result<()> {
    let mut missing_methods = Vec::new();

    for method_name in required_methods {
        if !obj.has_named_property(method_name)? {
            missing_methods.push(*method_name);
        }
    }

    if !missing_methods.is_empty() {
        return Err(napi::Error::new(
            napi::Status::InvalidArg,
            format!(
                "{} is missing required methods: {}. Please ensure your plugin implements all required methods.",
                plugin_type,
                missing_methods.join(", ")
            ),
        ));
    }

    Ok(())
}

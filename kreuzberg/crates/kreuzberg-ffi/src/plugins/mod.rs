//! Plugin system FFI bindings
//!
//! Provides FFI bindings for registering and managing plugins.

pub mod document_extractor;
pub mod ocr_backend;
pub mod post_processor;
pub mod validator;

// Re-export all public items
pub use document_extractor::*;
pub use ocr_backend::*;
pub use post_processor::*;
pub use validator::*;

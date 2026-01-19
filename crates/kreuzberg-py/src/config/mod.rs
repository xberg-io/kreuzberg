//! Configuration type bindings
//!
//! Provides Python-friendly wrappers around the Rust configuration structs.
//! All types support both construction and field access from Python.

// Main types module containing all configuration classes
mod types;

// Re-export all configuration types for backward compatibility
pub use types::*;

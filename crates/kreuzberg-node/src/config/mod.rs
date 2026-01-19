//! Configuration type bindings for Node.js
//!
//! Provides Node.js-friendly wrappers around the Rust configuration structs.

// Main types module containing all configuration classes
mod types;

// Re-export all configuration types for backward compatibility
pub use types::*;

//! Configuration parsing and conversion for Ruby bindings
//!
//! Handles conversion between Ruby Hash configurations and Rust config types.
//! Includes parsing for all nested configuration structures.

// Main types module containing all configuration parsing
mod types;

// Re-export all configuration functions for backward compatibility
pub use types::*;

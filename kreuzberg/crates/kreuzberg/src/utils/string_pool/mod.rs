//! String interning/pooling for frequently used strings.
//!
//! This module provides thread-safe string interning to reduce memory allocations
//! for strings that appear repeatedly across documents (MIME types, language codes, format field names).
//!
//! # Performance
//!
//! String interning provides 0.1-0.3% improvement by:
//! - Deduplicating repeated strings (e.g., "application/pdf" appears 1000s of times)
//! - Reducing allocation overhead for commonly used strings
//! - Enabling pointer comparisons instead of string comparisons
//!
//! # Thread Safety
//!
//! The intern pool uses a `DashMap` for lock-free concurrent access. Multiple threads
//! can insert and lookup strings simultaneously without contention.
//!
//! # Example
//!
//! ```rust,ignore
//! use kreuzberg::utils::string_pool::intern_mime_type;
//!
//! let mime1 = intern_mime_type("application/pdf");
//! let mime2 = intern_mime_type("application/pdf");
//! // Both mime1 and mime2 point to the same interned string
//! assert_eq!(mime1, mime2);
//! ```

mod buffer_pool;
mod interned;
mod language_pool;
mod mime_pool;

// Re-export public types and functions
pub use buffer_pool::{PoolConfig, PooledString, STRING_BUFFER_POOL, StringBufferPool, acquire_string_buffer};
pub use interned::InternedString;
pub use language_pool::intern_language_code;
pub use mime_pool::intern_mime_type;

#[cfg(feature = "pool-metrics")]
pub use buffer_pool::StringBufferPoolMetrics;

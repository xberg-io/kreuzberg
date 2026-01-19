//! Plugin registration functions for Python-Rust FFI bridge.
//!
//! Allows Python-based plugins (OCR backends, PostProcessors, Validators) to register
//! with the Rust core and be used by the Rust CLI, API server, and MCP server.
//!
//! # Architecture
//!
//! This module provides the FFI bridge that enables:
//! - **Python OCR backends** (EasyOCR, PaddleOCR, custom backends) to be used by Rust extraction
//! - **Python PostProcessors** (entity extraction, keyword extraction, metadata enrichment) to enrich results
//! - **Python Validators** (content validation, quality checks) to validate extraction results
//!
//! # GIL (Global Interpreter Lock) Management
//!
//! This module implements sophisticated GIL management patterns to bridge Python and Rust safely
//! and efficiently. Understanding these patterns is critical for maintaining thread safety and performance.
//!
//! ## Core GIL Patterns
//!
//! ### 1. `Python::attach()` - Temporary GIL Acquisition
//!
//! Used when calling Python code from Rust:
//! ```rust,ignore
//! Python::attach(|py| {
//!     let result = self.python_obj.bind(py).call_method0("name")?;
//!     result.extract::<String>()
//! })
//! ```
//! - **When**: Reading Python object attributes, calling Python methods
//! - **GIL held**: Only during the closure execution
//! - **Thread safety**: Safe to call from any thread, blocks if GIL unavailable
//! - **Performance**: Minimal overhead for quick operations
//!
//! ### 2. `py.detach()` - GIL Release During Expensive Operations
//!
//! Used when performing expensive Rust operations that don't need Python access:
//! ```rust,ignore
//! py.detach(|| {
//!     // GIL is released here - other Python threads can run
//!     let registry = get_ocr_backend_registry();
//!     let mut registry = registry.write()?; // Expensive lock acquisition
//!     registry.register(backend)
//! })
//! ```
//! - **When**: Writing to registries, I/O operations, expensive computations
//! - **GIL held**: Released during the closure, reacquired after
//! - **Why critical**: Prevents blocking Python threads during Rust operations
//! - **Performance**: Allows Python code to continue running in other threads
//!
//! ### 3. `tokio::task::spawn_blocking` - Async-to-Sync Bridge
//!
//! Used when calling Python code from async Rust (Python is inherently synchronous):
//! ```rust,ignore
//! let python_obj = Python::attach(|py| self.python_obj.clone_ref(py));
//! tokio::task::spawn_blocking(move || {
//!     Python::attach(|py| {
//!         let obj = python_obj.bind(py);
//!         obj.call_method1("process_image", (py_bytes, language))
//!     })
//! })
//! .await?
//! ```
//! - **When**: Async trait implementations (OcrBackend::process_image, PostProcessor::process)
//! - **Why necessary**: Python calls block, incompatible with async Rust
//! - **GIL management**: Acquires GIL inside blocking task, doesn't block tokio runtime
//! - **Data transfer**: Use `clone_ref(py)` to safely move Python objects across thread boundary
//!
//! ### 4. Caching to Minimize GIL Acquisitions
//!
//! Plugin wrappers cache frequently-accessed Python data in Rust fields:
//! ```rust,ignore
//! pub struct PythonOcrBackend {
//!     python_obj: Py<PyAny>,
//!     name: String,                    // Cached - no GIL needed
//!     supported_languages: Vec<String>, // Cached - no GIL needed
//! }
//! ```
//! - **When**: Data accessed frequently but rarely changes (name, supported languages)
//! - **Why important**: Avoids GIL acquisition overhead on every call
//! - **Trade-off**: Slightly more memory for significantly better performance
//! - **Pattern**: Cache in constructor, use cached values in trait methods
//!
//! ## Thread Safety Guarantees
//!
//! - **All plugin wrappers are `Send + Sync`**: Can be safely shared across threads
//! - **Py<PyAny> is thread-safe**: PyO3 ensures Python objects can cross thread boundaries
//! - **GIL prevents data races**: Only one thread accesses Python state at a time
//! - **Rust mutexes protect registries**: RwLock ensures safe concurrent registry access
//!
//! ## Performance Considerations
//!
//! - **GIL acquisition cost**: ~100ns per acquisition on modern hardware
//! - **Cache effectiveness**: Reduces GIL acquisitions by 10-100x for hot paths
//! - **Blocking tasks overhead**: ~1-5μs per spawn_blocking call
//! - **Design principle**: Minimize GIL acquisitions, maximize time GIL is released
//!
//! ## Error Handling with GIL
//!
//! - **hasattr() failures**: If GIL acquisition fails, log warning and use safe defaults
//! - **Method call failures**: Convert PyErr to KreuzbergError with context
//! - **Lock poisoning**: Handle registry lock poisoning gracefully
//! - **Pattern**: Always provide fallback behavior, never panic on GIL errors
//!
//! ## Common Pitfalls and Solutions
//!
//! 1. **Deadlock risk**: Never hold registry lock while acquiring GIL
//!    - Solution: Use `py.detach()` to release GIL before acquiring registry lock
//!
//! 2. **GIL contention**: Holding GIL too long blocks all Python threads
//!    - Solution: Cache data, release GIL with `detach()` during expensive operations
//!
//! 3. **Moving Python objects**: Can't send `&Bound<PyAny>` across threads
//!    - Solution: Use `Py<PyAny>` and `clone_ref(py)` for ownership transfer
//!
//! 4. **Async incompatibility**: Python code blocks, can't be awaited
//!    - Solution: Always use `spawn_blocking` for Python calls from async code

pub mod common;
pub mod ocr_bridge;
pub mod processor_bridge;
pub mod validator_bridge;

// Re-export public APIs for backward compatibility
pub use common::json_value_to_py;
pub use ocr_bridge::register_ocr_backend;
pub use processor_bridge::{
    clear_post_processors, list_post_processors, register_post_processor, unregister_post_processor,
};
pub use validator_bridge::{clear_validators, list_validators, register_validator, unregister_validator};

// OCR backend management functions
#[pyo3::pyfunction]
pub fn unregister_ocr_backend(name: &str) -> pyo3::PyResult<()> {
    kreuzberg::plugins::unregister_ocr_backend(name)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[pyo3::pyfunction]
pub fn list_ocr_backends() -> pyo3::PyResult<Vec<String>> {
    kreuzberg::plugins::list_ocr_backends().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[pyo3::pyfunction]
pub fn clear_ocr_backends() -> pyo3::PyResult<()> {
    kreuzberg::plugins::clear_ocr_backends().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

/// Document extractor management functions re-exported from kreuzberg
#[pyo3::pyfunction]
pub fn list_document_extractors() -> pyo3::PyResult<Vec<String>> {
    kreuzberg::plugins::list_extractors().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[pyo3::pyfunction]
pub fn unregister_document_extractor(name: &str) -> pyo3::PyResult<()> {
    kreuzberg::plugins::unregister_extractor(name).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

#[pyo3::pyfunction]
pub fn clear_document_extractors() -> pyo3::PyResult<()> {
    kreuzberg::plugins::clear_extractors().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
}

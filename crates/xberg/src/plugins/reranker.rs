//! Reranker backend plugin trait.
//!
//! Defines the trait for implementing custom reranker backends — the in-process
//! complement to the HTTP-based [`crate::core::config::RerankerModelType::Llm`]
//! variant. A [`RerankerBackend`] is a caller-supplied object that scores
//! `(query, document)` pairs and returns raw logits; xberg handles sorting
//! and top-k truncation.
//!
//! # Typical use
//!
//! Callers that already load their own cross-encoder (e.g. `sentence-transformers`,
//! a tuned ONNX model, or a provider client) register the wrapper once and
//! reference it by name in config:
//!
//! ```rust,no_run
//! use xberg::plugins::{RerankerBackend, Plugin, register_reranker_backend};
//! use xberg::Result;
//! use std::sync::Arc;
//!
//! struct MyReranker;
//!
//! impl Plugin for MyReranker {
//!     fn name(&self) -> &str { "my-reranker" }
//!     fn version(&self) -> String { "1.0.0".to_string() }
//!     fn initialize(&self) -> Result<()> { Ok(()) }
//!     fn shutdown(&self) -> Result<()> { Ok(()) }
//! }
//!
//! #[async_trait::async_trait]
//! impl RerankerBackend for MyReranker {
//!     async fn rerank(&self, _query: String, documents: Vec<String>) -> Result<Vec<f32>> {
//!         // Return a raw logit per document in input order.
//!         Ok(documents.iter().map(|_| 1.0_f32).collect())
//!     }
//! }
//!
//! register_reranker_backend(Arc::new(MyReranker))?;
//! # Ok::<(), xberg::XbergError>(())
//! ```
//!
//! Since v5.0.0.

use crate::Result;
use crate::plugins::Plugin;
use async_trait::async_trait;
use std::sync::Arc;

/// Trait for in-process reranker backend plugins.
///
/// Cross-encoders score `(query, document)` pairs jointly and return a
/// raw logit per document. The dispatcher in [`crate::rerank`] applies
/// sigmoid to convert logits to `[0, 1]` scores, sorts descending by score,
/// and truncates to `top_k`.
///
/// Async to match the convention used by [`crate::plugins::EmbeddingBackend`]
/// and other plugin traits. Host-language bridges wrap their synchronous
/// host callables in `spawn_blocking` or the equivalent.
///
/// # Thread safety
///
/// Backends must be `Send + Sync + 'static`. They are stored in
/// `Arc<dyn RerankerBackend>` and may be called concurrently from xberg's
/// dispatcher. If the backend's underlying model is not thread-safe, the
/// backend itself must serialize access internally (e.g. via `Mutex<Inner>`).
///
/// # Contract
///
/// - `rerank(query, documents)` MUST return exactly `documents.len()` scores.
///   The dispatcher validates this before sorting and returning to callers;
///   a non-conforming backend surfaces as a `XbergError::Validation`, not
///   a panic.
/// - Scores are raw logits in any range — callers must NOT assume `[0, 1]`.
///   The dispatcher applies sigmoid before sorting.
/// - `rerank` may be called from any thread. Its future must be `Send`
///   (enforced by `async_trait` when `#[async_trait]` is used on non-WASM
///   targets).
/// - `shutdown()` (inherited from [`crate::plugins::Plugin`]) may be invoked
///   concurrently with an in-flight `rerank()` call. Implementations must
///   tolerate this — letting in-flight calls finish via the `Arc` reference
///   and only releasing shared state that isn't needed by `rerank`.
///
/// # Runtime
///
/// The synchronous `rerank` entry uses
/// [`tokio::task::block_in_place`] to await the trait's async `rerank`, which
/// requires a multi-thread tokio runtime. Callers running inside a
/// `current_thread` runtime must use `rerank_async` instead.
///
/// Since v5.0.0.
#[doc(alias = "rerank")]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait RerankerBackend: Plugin {
    /// Score a list of documents against a query.
    ///
    /// Returns one raw logit per document in the same order as the input.
    /// The dispatcher applies sigmoid to convert to `[0, 1]` scores.
    ///
    /// # Errors
    ///
    /// Implementations should return [`crate::XbergError::Plugin`] for
    /// backend-specific failures. The dispatcher validates the returned length
    /// against `documents.len()` before sorting.
    async fn rerank(&self, query: String, documents: Vec<String>) -> Result<Vec<f32>>;
}

/// Register a reranker backend with the global registry.
///
/// The backend will be keyed by its `Plugin::name()` and can be referenced from
/// [`crate::core::config::RerankerModelType::Plugin`] by the same name.
///
/// # Errors
///
/// - [`crate::XbergError::Validation`] if the name is empty or contains whitespace.
/// - [`crate::XbergError::Plugin`] if a backend with that name is already registered.
/// - Any error from the backend's `initialize()` method.
///
/// Since v5.0.0.
#[cfg_attr(alef, alef(skip))]
pub fn register_reranker_backend(backend: Arc<dyn RerankerBackend>) -> Result<()> {
    use crate::plugins::registry::get_reranker_backend_registry;

    let registry = get_reranker_backend_registry();
    let mut registry = registry.write();
    registry.register(backend)
}

/// Unregister a reranker backend by name, calling its `shutdown()` method.
///
/// No-op if the backend is not registered.
///
/// # Errors
///
/// - Any error returned by the backend's `shutdown()` method.
///
/// Since v5.0.0.
#[cfg_attr(alef, alef(skip))]
pub fn unregister_reranker_backend(name: &str) -> Result<()> {
    use crate::plugins::registry::get_reranker_backend_registry;

    let registry = get_reranker_backend_registry();
    let mut registry = registry.write();
    registry.remove(name)
}

/// Clear all reranker backends from the global registry.
///
/// Calls `shutdown()` on every registered backend, then empties the registry.
///
/// # Errors
///
/// - Any error returned by a backend's `shutdown()` method. The first error
///   encountered stops processing of remaining backends.
///
/// Since v5.0.0.
pub fn clear_reranker_backends() -> Result<()> {
    use crate::plugins::registry::get_reranker_backend_registry;

    let registry = get_reranker_backend_registry();
    let mut registry = registry.write();
    registry.shutdown_all()
}

/// List the names of all registered reranker backends.
///
/// Used by `xberg-cli`, the api/mcp endpoints, and generated language
/// bindings.
///
/// Since v5.0.0.
pub fn list_reranker_backends() -> Result<Vec<String>> {
    use crate::plugins::registry::get_reranker_backend_registry;

    let registry = get_reranker_backend_registry();
    let registry = registry.read();
    Ok(registry.list())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::XbergError;
    use crate::plugins::Plugin;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct MockRerankerBackend {
        name: String,
    }

    impl Plugin for MockRerankerBackend {
        fn name(&self) -> &str {
            &self.name
        }
        fn version(&self) -> String {
            "1.0.0".to_string()
        }
        fn initialize(&self) -> Result<()> {
            Ok(())
        }
        fn shutdown(&self) -> Result<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl RerankerBackend for MockRerankerBackend {
        async fn rerank(&self, _query: String, documents: Vec<String>) -> Result<Vec<f32>> {
            Ok(documents.iter().map(|_| 1.0_f32).collect())
        }
    }

    /// Unique per-test name so parallel test runs don't collide in the shared
    /// global `RERANKER_BACKEND_REGISTRY`.
    fn unique_name(suffix: &str) -> String {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("mock-reranker-{suffix}-{id}")
    }

    #[test]
    fn register_list_unregister_roundtrip() {
        let name = unique_name("roundtrip");
        register_reranker_backend(Arc::new(MockRerankerBackend { name: name.clone() })).unwrap();

        assert!(list_reranker_backends().unwrap().contains(&name));

        unregister_reranker_backend(&name).unwrap();
        assert!(!list_reranker_backends().unwrap().contains(&name));
    }

    #[test]
    fn empty_name_rejected_via_global_api() {
        let result = register_reranker_backend(Arc::new(MockRerankerBackend { name: String::new() }));
        assert!(matches!(result, Err(XbergError::Validation { .. })));
    }

    #[test]
    fn duplicate_name_rejected_via_global_api() {
        let name = unique_name("dup");
        register_reranker_backend(Arc::new(MockRerankerBackend { name: name.clone() })).unwrap();

        let result = register_reranker_backend(Arc::new(MockRerankerBackend { name: name.clone() }));
        assert!(matches!(result, Err(XbergError::Plugin { .. })));

        // Clean up.
        unregister_reranker_backend(&name).unwrap();
    }

    #[tokio::test]
    async fn mock_backend_returns_expected_shape() {
        let backend = MockRerankerBackend {
            name: "local".to_string(),
        };
        let scores = backend
            .rerank("query".to_string(), vec!["doc1".into(), "doc2".into(), "doc3".into()])
            .await
            .unwrap();
        assert_eq!(scores.len(), 3);
    }

    #[test]
    fn register_list_clear_list_roundtrip() {
        let name = unique_name("clear");
        register_reranker_backend(Arc::new(MockRerankerBackend { name: name.clone() })).unwrap();

        assert!(list_reranker_backends().unwrap().contains(&name));

        clear_reranker_backends().unwrap();
        assert!(!list_reranker_backends().unwrap().contains(&name));
    }
}

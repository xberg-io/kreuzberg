//! Error model for the RAG base layer.
//!
//! De-tenanted from the enterprise `vectorstore` crate: the multi-tenant /
//! commercial variants (`QuotaExceeded`, `RateLimited`, `ProjectContextMissing`)
//! are intentionally absent — they belong to the closed-source product layer,
//! not the open engine.

use thiserror::Error;

/// Complexity constraint kind for filter validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityKind {
    /// Filter nesting depth exceeded.
    Depth,
    /// Total filter node count exceeded.
    NodeCount,
    /// `text_match` predicate count exceeded.
    TextMatchCount,
    /// `text_match` query string byte length exceeded.
    TextMatchQueryBytes,
}

/// Errors raised by vector-store operations and the RAG pipeline.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum RagError {
    /// The requested collection does not exist.
    #[error("collection not found: {0}")]
    CollectionNotFound(String),

    /// A collection with the same name already exists with a different spec.
    #[error("collection already exists: {0}")]
    CollectionAlreadyExists(String),

    /// Query/chunk embedding dimension does not match the collection.
    #[error("embedding dimension mismatch: expected {expected}, got {got}")]
    EmbeddingDimMismatch {
        /// Dimension declared by the collection.
        expected: u32,
        /// Dimension of the supplied vector.
        got: u32,
    },

    /// An embedder returned a different number of vectors than the inputs it
    /// was given — a broken embedder contract.
    #[error("embedding count mismatch: expected {expected} vectors, got {got}")]
    EmbeddingCountMismatch {
        /// Number of texts submitted for embedding.
        expected: usize,
        /// Number of vectors the embedder returned.
        got: usize,
    },

    /// A filter referenced a field outside the allowed namespaces.
    #[error("filter references unknown field: {field}")]
    FilterUnknownField {
        /// The offending field identifier.
        field: String,
    },

    /// A filter operation does not apply to the field's type.
    #[error("filter type mismatch on field {field}: operation {op} not applicable")]
    FilterTypeMismatch {
        /// The offending field identifier.
        field: String,
        /// The operation that did not apply.
        op: String,
    },

    /// A filter exceeded a complexity limit.
    #[error("filter {kind:?} complexity limit exceeded: cap {cap}, observed {observed}")]
    FilterComplexityExceeded {
        /// Which limit was exceeded.
        kind: ComplexityKind,
        /// The configured cap.
        cap: u32,
        /// The observed value.
        observed: u32,
    },

    /// The query was malformed (bad `top_k`, missing inputs for the mode, …).
    #[error("invalid query: {0}")]
    InvalidQuery(String),

    /// The backend does not support the requested retrieval mode.
    #[error("retrieval mode unsupported by backend '{backend}': {mode}")]
    UnsupportedMode {
        /// The backend's `name()`.
        backend: String,
        /// The requested mode (`full_text`, `hybrid`, …).
        mode: String,
    },

    /// A store with the given name is already registered.
    #[error("vector store '{0}' is already registered")]
    AlreadyRegistered(String),

    /// No store with the given name is registered.
    #[error("vector store '{0}' is not registered")]
    NotRegistered(String),

    /// A store name was empty or contained whitespace.
    #[error("invalid vector store name: {0}")]
    InvalidName(String),

    /// An error originating in xberg core (chunking, embeddings, reranking, …).
    #[cfg(any(feature = "pipeline", feature = "streaming"))]
    #[error(transparent)]
    Core(#[from] xberg::XbergError),

    /// A backend-specific error from an adapter implementation.
    #[error("backend error: {0}")]
    Backend(#[source] Box<dyn std::error::Error + Send + Sync>),
}

/// Result alias for RAG operations.
pub type RagResult<T> = Result<T, RagError>;

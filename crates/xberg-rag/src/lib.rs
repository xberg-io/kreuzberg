//! # xberg-rag
//!
//! The RAG base layer for the Xberg engine: a backend-agnostic
//! [`VectorStore`] trait, a neutral type + filter + query IR, a generic
//! ingest/retrieve pipeline that composes xberg core primitives, and streaming
//! answer primitives over liter-llm.
//!
//! This crate is the **engine contract** that the commercial products build on:
//! Xberg Pro and Xberg Enterprise each implement [`VectorStore`] externally
//! (single-node embedded store, tenant-scoped pgvector, …) while this crate stays
//! single-tenant and free of any product/tenant policy.
//!
//! It is **Rust-only** — deliberately not exposed through the language bindings.
//!
//! ## Feature layers
//!
//! - `vector-store` (default): the contract surface — trait, types, filter/query
//!   IR, registry. Depends only on `serde`/`async-trait`/`thiserror`; WASM-safe.
//! - `in-memory` (default): a brute-force [`InMemoryVectorStore`].
//! - `sqlite`: an embedded `rusqlite` + `sqlite-vec` store (native-only).
//! - `pipeline` (+ `pipeline-embeddings`/`pipeline-reranker`/`pipeline-keywords`/
//!   `pipeline-ner-*`): ingest/retrieve orchestration over xberg core.
//! - `streaming`: answer/progress streams over liter-llm (native-only).
//!
//! [`InMemoryVectorStore`]: crate::backends::memory::InMemoryVectorStore

#[cfg(feature = "vector-store")]
pub mod backends;
#[cfg(feature = "vector-store")]
pub mod capability;
#[cfg(feature = "vector-store")]
pub mod error;
#[cfg(feature = "vector-store")]
pub mod filter;
#[cfg(feature = "vector-store")]
pub mod query;
#[cfg(feature = "vector-store")]
pub mod registry;
#[cfg(feature = "vector-store")]
pub mod store;
#[cfg(feature = "vector-store")]
pub mod types;

#[cfg(feature = "pipeline")]
pub mod pipeline;
#[cfg(feature = "streaming")]
pub mod stream;

#[cfg(feature = "vector-store")]
pub use capability::Capabilities;
#[cfg(feature = "vector-store")]
pub use error::{ComplexityKind, RagError, RagResult};
#[cfg(feature = "vector-store")]
pub use filter::{Filter, FilterField, FilterNamespace};
#[cfg(feature = "vector-store")]
pub use query::{RetrieveMode, RetrieveOutput, RetrieveQuery};
#[cfg(feature = "vector-store")]
pub use registry::{
    VectorStoreRegistry, clear_vector_stores, get_vector_store, list_vector_stores, register_vector_store,
    unregister_vector_store, vector_store_registry,
};
#[cfg(feature = "vector-store")]
pub use store::VectorStore;
#[cfg(feature = "vector-store")]
pub use types::{
    ChunkId, ChunkRecord, CollectionSpec, CollectionStats, DistanceMetric, DocumentId, DocumentRecord, DocumentSummary,
    IndexMethod, PrimaryScore, RetrievedChunk,
};

#[cfg(feature = "in-memory")]
pub use backends::memory::InMemoryVectorStore;

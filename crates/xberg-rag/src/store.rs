//! The backend-agnostic [`VectorStore`] contract.
//!
//! This is the extension point the whole RAG layer is built around: xberg ships
//! the trait + a brute-force in-memory backend (and an optional embedded sqlite
//! one), and the commercial products implement it externally — Pro over a local
//! store, Enterprise over tenant-scoped pgvector. The trait is deliberately
//! **single-tenant**: one instance is one trust domain. Multi-tenancy is layered
//! on top by the caller (e.g. one scoped instance per tenant, or a decorator that
//! sets row-level-security context before delegating) and is never expressed in
//! these signatures.

use crate::capability::Capabilities;
use crate::error::RagResult;
use crate::filter::Filter;
use crate::query::{RetrieveOutput, RetrieveQuery};
use crate::types::{ChunkRecord, CollectionSpec, CollectionStats, DocumentId, DocumentRecord};
use async_trait::async_trait;

/// A vector store: collections of documents and their embedded chunks, queried
/// by vector / full-text / hybrid retrieval.
///
/// # Thread safety
///
/// Implementations are `Send + Sync + 'static` and held behind `Arc<dyn
/// VectorStore>`; they may be called concurrently. Backends wrapping
/// non-thread-safe handles must serialize access internally.
///
/// # Obtaining an instance
///
/// Either register a named instance in the global
/// [`registry`](crate::registry) (single-node convenience) or construct an
/// `Arc<dyn VectorStore>` directly and inject it (the dependency-injection path
/// the commercial multi-tenant layer relies on). Backends that need async,
/// fallible setup must do it in their own constructor and be fully connected
/// before registration.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait VectorStore: Send + Sync + 'static {
    /// A stable identifier for this store instance (its registry key).
    fn name(&self) -> &str;

    /// What this backend supports (full-text, hybrid, filtering, index methods).
    fn capabilities(&self) -> Capabilities;

    /// Create the collection if it does not exist.
    ///
    /// # Errors
    ///
    /// [`RagError::CollectionAlreadyExists`](crate::RagError::CollectionAlreadyExists)
    /// if a collection with the same name exists with an incompatible spec.
    async fn ensure_collection(&self, spec: &CollectionSpec) -> RagResult<()>;

    /// Drop a collection and all its contents.
    ///
    /// # Errors
    ///
    /// [`RagError::CollectionNotFound`](crate::RagError::CollectionNotFound) if
    /// it does not exist.
    async fn drop_collection(&self, collection: &str) -> RagResult<()>;

    /// Fetch a collection's spec, or `None` if absent.
    ///
    /// # Errors
    ///
    /// Backend errors only.
    async fn get_collection(&self, collection: &str) -> RagResult<Option<CollectionSpec>>;

    /// Upsert a document together with its chunks as one atomic unit.
    ///
    /// Identity is the document's `external_id` when present, otherwise a new
    /// backend-assigned id. Returns the resulting [`DocumentId`].
    ///
    /// # Errors
    ///
    /// [`RagError::EmbeddingDimMismatch`](crate::RagError::EmbeddingDimMismatch)
    /// if any chunk vector does not match the collection dimension.
    async fn upsert_document(
        &self,
        collection: &str,
        document: &DocumentRecord,
        chunks: &[ChunkRecord],
    ) -> RagResult<DocumentId>;

    /// Delete documents (and their chunks) by id. Returns the number removed.
    ///
    /// # Errors
    ///
    /// Backend errors only; unknown ids are ignored.
    async fn delete_documents(&self, collection: &str, ids: &[DocumentId]) -> RagResult<u64>;

    /// Delete documents matching a filter. Returns the number removed.
    ///
    /// # Errors
    ///
    /// Filter or backend errors.
    async fn delete_by_filter(&self, collection: &str, filter: &Filter) -> RagResult<u64>;

    /// Retrieve chunks matching a query.
    ///
    /// # Errors
    ///
    /// [`RagError::UnsupportedMode`](crate::RagError::UnsupportedMode) if the
    /// backend cannot serve the requested [`RetrieveMode`](crate::RetrieveMode);
    /// otherwise validation or backend errors.
    async fn retrieve(&self, collection: &str, query: &RetrieveQuery) -> RagResult<RetrieveOutput>;

    /// Aggregate statistics for a collection.
    ///
    /// # Errors
    ///
    /// [`RagError::CollectionNotFound`](crate::RagError::CollectionNotFound) if
    /// it does not exist.
    async fn collection_stats(&self, collection: &str) -> RagResult<CollectionStats>;
}

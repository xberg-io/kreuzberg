//! Global named registry for [`VectorStore`] instances.
//!
//! Mirrors xberg's reranker-backend registry: a process-global
//! `LazyLock<RwLock<…>>` keyed by store name, with `register` / `unregister` /
//! `list` / `clear` free functions. This is the single-node convenience path.
//!
//! The registry is **not** the only way to obtain a store: backends are plain
//! `Arc<dyn VectorStore>` and can be injected directly. The multi-tenant
//! commercial layer relies on that DI seam (a fresh, tenant-scoped store per
//! request) and must never depend on this global.

use crate::error::{RagError, RagResult};
use crate::store::VectorStore;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

/// Registry of named vector-store backends.
#[derive(Default)]
pub struct VectorStoreRegistry {
    stores: HashMap<String, Arc<dyn VectorStore>>,
}

impl VectorStoreRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self { stores: HashMap::new() }
    }

    /// Register a store under its [`VectorStore::name`].
    ///
    /// # Errors
    ///
    /// [`RagError::InvalidName`] for an empty/whitespace name, or
    /// [`RagError::AlreadyRegistered`] for a duplicate.
    pub fn register(&mut self, store: Arc<dyn VectorStore>) -> RagResult<()> {
        let name = store.name().to_string();
        validate_store_name(&name)?;
        if self.stores.contains_key(&name) {
            return Err(RagError::AlreadyRegistered(name));
        }
        tracing::info!(store = %name, "vector store registered");
        self.stores.insert(name, store);
        Ok(())
    }

    /// Get a registered store by name.
    ///
    /// # Errors
    ///
    /// [`RagError::NotRegistered`] if absent.
    pub fn get(&self, name: &str) -> RagResult<Arc<dyn VectorStore>> {
        self.stores
            .get(name)
            .cloned()
            .ok_or_else(|| RagError::NotRegistered(name.to_string()))
    }

    /// List registered store names.
    pub fn list(&self) -> Vec<String> {
        self.stores.keys().cloned().collect()
    }

    /// Remove a store by name. No-op if absent.
    pub fn remove(&mut self, name: &str) {
        self.stores.remove(name);
    }

    /// Remove all stores.
    pub fn clear(&mut self) {
        self.stores.clear();
    }
}

static VECTOR_STORE_REGISTRY: LazyLock<RwLock<VectorStoreRegistry>> =
    LazyLock::new(|| RwLock::new(VectorStoreRegistry::new()));

/// Access the process-global registry.
pub fn vector_store_registry() -> &'static RwLock<VectorStoreRegistry> {
    &VECTOR_STORE_REGISTRY
}

/// Register a store in the global registry.
///
/// # Errors
///
/// See [`VectorStoreRegistry::register`].
pub fn register_vector_store(store: Arc<dyn VectorStore>) -> RagResult<()> {
    vector_store_registry()
        .write()
        .expect("vector store registry poisoned")
        .register(store)
}

/// Fetch a store from the global registry by name.
///
/// # Errors
///
/// [`RagError::NotRegistered`] if absent.
pub fn get_vector_store(name: &str) -> RagResult<Arc<dyn VectorStore>> {
    vector_store_registry()
        .read()
        .expect("vector store registry poisoned")
        .get(name)
}

/// Remove a store from the global registry by name. No-op if absent.
pub fn unregister_vector_store(name: &str) {
    vector_store_registry()
        .write()
        .expect("vector store registry poisoned")
        .remove(name);
}

/// List names of all registered stores.
pub fn list_vector_stores() -> Vec<String> {
    vector_store_registry()
        .read()
        .expect("vector store registry poisoned")
        .list()
}

/// Remove all stores from the global registry.
pub fn clear_vector_stores() {
    vector_store_registry()
        .write()
        .expect("vector store registry poisoned")
        .clear();
}

/// Validate a store name: non-empty, no whitespace.
fn validate_store_name(name: &str) -> RagResult<()> {
    if name.is_empty() {
        return Err(RagError::InvalidName("name is empty".to_string()));
    }
    if name.chars().any(char::is_whitespace) {
        return Err(RagError::InvalidName(format!("name '{name}' contains whitespace")));
    }
    Ok(())
}

//! Single-slot, fingerprinted crawl-engine memo on [`super::EngineInner`].
//!
//! Multi-URL batch extraction routes every URL that shares the batch's base
//! crawl configuration through ONE [`crawlberg::CrawlEngine`], so the engine's
//! shared middleware chain, cache, and rate-limiter are reused across the
//! whole batch. The engine is keyed by a stable fingerprint of its
//! [`crawlberg::CrawlConfig`]: when the incoming config fingerprint matches the
//! memoized one the cached engine is cloned (it is `Arc`-backed and cheap to
//! clone), otherwise a fresh engine is validated, built, stored, and returned.
//!
//! This module is engine-internal and not part of the binding surface.

#[cfg(feature = "url-ingestion")]
use crawlberg::{CrawlConfig, CrawlEngine};

#[cfg(feature = "url-ingestion")]
use crate::Result;
#[cfg(feature = "url-ingestion")]
use crate::engine::EngineInner;
#[cfg(feature = "url-ingestion")]
use crate::engine::extract_impl::map_crawl_error;

/// Memoized crawl engine keyed by a stable fingerprint of its [`CrawlConfig`].
#[cfg(feature = "url-ingestion")]
pub(crate) struct CrawlHandleMemo {
    fingerprint: u64,
    engine: CrawlEngine,
}

/// Compute a stable fingerprint of a [`CrawlConfig`].
///
/// The config is serialized to its canonical JSON byte form and hashed. A
/// fingerprint *collision* (two different configs hashing equal) would let the
/// wrong engine be reused, which is effectively impossible for distinct JSON
/// payloads; a fingerprint *mismatch* for equal configs (e.g. nondeterministic
/// map ordering) only causes a harmless rebuild. Serialization failure falls
/// back to hashing the `Debug` representation.
#[cfg(feature = "url-ingestion")]
pub(crate) fn crawl_fingerprint(config: &CrawlConfig) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    match serde_json::to_vec(config) {
        Ok(bytes) => bytes.hash(&mut hasher),
        Err(_) => format!("{config:?}").hash(&mut hasher),
    }
    hasher.finish()
}

#[cfg(feature = "url-ingestion")]
impl EngineInner {
    /// Return a crawl engine for `config`, reusing the memoized engine when its
    /// fingerprint matches, otherwise validating and building a fresh one
    /// (identical construction to the single-URL `extract_remote_uri` path).
    pub(crate) fn crawl_engine_for(&self, config: &CrawlConfig) -> Result<CrawlEngine> {
        let fingerprint = crawl_fingerprint(config);

        let mut guard = self.crawl.lock();
        if let Some(memo) = guard.as_ref()
            && memo.fingerprint == fingerprint
        {
            return Ok(memo.engine.clone());
        }

        config.validate().map_err(map_crawl_error)?;
        let engine = CrawlEngine::builder()
            .config(config.clone())
            .build()
            .map_err(map_crawl_error)?;

        *guard = Some(CrawlHandleMemo {
            fingerprint,
            engine: engine.clone(),
        });
        Ok(engine)
    }
}

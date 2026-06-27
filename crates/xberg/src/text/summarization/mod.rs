//! Document summarisation.
//!
//! Two backends are exposed:
//!
//! - [`textrank::summarize`] — pure-Rust extractive summary that builds a TF-IDF
//!   cosine-similarity graph over sentences and runs PageRank. Deterministic and
//!   compiled into every target (including WASM and Android).
//! - [`llm::summarize_with_llm`] — abstractive summary produced by an LLM via
//!   the shared [`crate::llm::text_completion`] helper. Gated on the
//!   `summarization-llm` feature.
//!
//! The post-processor that ties both backends to
//! [`crate::types::ExtractedDocument::summary`] lives in
//! `crate::plugins::processor::builtin::summarization`.

pub mod textrank;

#[cfg(feature = "summarization-llm")]
pub mod llm;

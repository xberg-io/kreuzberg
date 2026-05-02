//! Vendored from <https://github.com/benbrandt/text-splitter> @ v0.30.1
//! MIT, © 2023 Benjamin Brandt. See `ATTRIBUTIONS.md` for full provenance.
//!
//! Trimmed: dropped `code` (tree-sitter) and `tiktoken-rs` features and their
//! files; rebuilt against `tokenizers 0.23`. The `markdown` splitter is always
//! compiled because `pulldown-cmark` is already a non-optional kreuzberg
//! dependency.

#![allow(
    clippy::pedantic,
    clippy::cargo,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    missing_docs,
    rust_2018_idioms
)]

mod chunk_size;
mod splitter;
mod trim;

pub(crate) use chunk_size::{Characters, ChunkCapacity, ChunkConfig, ChunkSizer};
pub(crate) use splitter::{MarkdownSplitter, TextSplitter};

//! Configuration types for text chunking.

use serde::{Deserialize, Serialize};

/// Configuration options for text chunking operations.
///
/// # Fields
///
/// * `max_characters` - Maximum number of characters per chunk (default: 2000)
/// * `overlap` - Number of characters to overlap between consecutive chunks (default: 100)
/// * `trim` - Whether to trim whitespace from chunk boundaries (default: true)
/// * `chunker_type` - Type of chunker to use (Text or Markdown) (default: Text)
pub struct ChunkingConfig {
    pub max_characters: usize,
    pub overlap: usize,
    pub trim: bool,
    pub chunker_type: ChunkerType,
}

impl Default for ChunkingConfig {
    fn default() -> Self {
        Self {
            max_characters: 2000,
            overlap: 100,
            trim: true,
            chunker_type: ChunkerType::Text,
        }
    }
}

/// Type of text chunker to use.
///
/// # Variants
///
/// * `Text` - Generic text splitter, splits on whitespace and punctuation
/// * `Markdown` - Markdown-aware splitter, preserves formatting and structure
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChunkerType {
    Text,
    Markdown,
}

/// Result of a text chunking operation.
///
/// Contains the generated chunks and metadata about the chunking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkingResult {
    /// List of text chunks
    pub chunks: Vec<crate::types::Chunk>,
    /// Total number of chunks generated
    pub chunk_count: usize,
}

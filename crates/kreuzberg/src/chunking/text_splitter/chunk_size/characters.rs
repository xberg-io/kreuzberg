//! Vendored from text-splitter v0.30.1 (MIT, © 2023 Benjamin Brandt). See ATTRIBUTIONS.md.

use super::super::ChunkSizer;

/// Used for splitting a piece of text into chunks based on the number of
/// characters in each chunk.
///
/// ```text
/// let splitter = TextSplitter::new(10);
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Characters;

impl ChunkSizer for Characters {
    /// Determine the size of a given chunk to use for validation.
    fn size(&self, chunk: &str) -> usize {
        chunk.chars().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_size() {
        let offsets = Characters.size("eé");
        assert_eq!(offsets, 2);
    }
}

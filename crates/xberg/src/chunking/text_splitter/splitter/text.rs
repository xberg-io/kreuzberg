//! Vendored from text-splitter v0.30.1 (MIT, © 2023 Benjamin Brandt). See ATTRIBUTIONS.md.
/*!
# [`TextSplitter`]
Semantic splitting of text documents.
*/

use std::ops::Range;

use itertools::Itertools;
use memchr::memchr2_iter;

use super::super::{
    ChunkConfig, ChunkSizer,
    splitter::{SemanticLevel, Splitter},
};

use super::fallback::GRAPHEME_SEGMENTER;

/// Default plain-text splitter. Recursively splits chunks into the largest
/// semantic units that fit within the chunk size. Also will attempt to merge
/// neighboring chunks if they can fit within the given chunk size.
#[derive(Debug)]
pub struct TextSplitter<Sizer>
where
    Sizer: ChunkSizer,
{
    /// Method of determining chunk sizes.
    chunk_config: ChunkConfig<Sizer>,
}

impl<Sizer> TextSplitter<Sizer>
where
    Sizer: ChunkSizer,
{
    /// Creates a new [`TextSplitter`].
    ///
    /// ```text
    /// let splitter = TextSplitter::new(512);
    /// ```
    #[must_use]
    pub fn new(chunk_config: impl Into<ChunkConfig<Sizer>>) -> Self {
        Self {
            chunk_config: chunk_config.into(),
        }
    }

    /// Generate a list of chunks from a given text. Each chunk will be up to the `chunk_capacity`.
    ///
    /// ## Method
    ///
    /// To preserve as much semantic meaning within a chunk as possible, each chunk is composed of the largest semantic units that can fit in the next given chunk. For each splitter type, there is a defined set of semantic levels. Here is an example of the steps used:
    ///
    /// 1. Split the text by a increasing semantic levels.
    /// 2. Check the first item for each level and select the highest level whose first item still fits within the chunk size.
    /// 3. Merge as many of these neighboring sections of this level or above into a chunk to maximize chunk length.
    ///    Boundaries of higher semantic levels are always included when merging, so that the chunk doesn't inadvertently cross semantic boundaries.
    ///
    /// The boundaries used to split the text if using the `chunks` method, in ascending order:
    ///
    /// 1. Characters
    /// 2. [Unicode Grapheme Cluster Boundaries](https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)
    /// 3. [Unicode Word Boundaries](https://www.unicode.org/reports/tr29/#Word_Boundaries)
    /// 4. [Unicode Sentence Boundaries](https://www.unicode.org/reports/tr29/#Sentence_Boundaries)
    /// 5. Ascending sequence length of newlines. (Newline is `\r\n`, `\n`, or `\r`)
    ///    Each unique length of consecutive newline sequences is treated as its own semantic level. So a sequence of 2 newlines is a higher level than a sequence of 1 newline, and so on.
    ///
    /// Splitting doesn't occur below the character level, otherwise you could get partial bytes of a char, which may not be a valid unicode str.
    ///
    /// ```text
    /// let splitter = TextSplitter::new(10);
    /// let chunks = splitter.chunks("Some text\n\nfrom a\ndocument").collect::<Vec<_>>();
    /// ```
    pub fn chunks<'splitter, 'text: 'splitter>(
        &'splitter self,
        text: &'text str,
    ) -> impl Iterator<Item = &'text str> + 'splitter {
        Splitter::<_>::chunks(self, text)
    }
}

impl<Sizer> Splitter<Sizer> for TextSplitter<Sizer>
where
    Sizer: ChunkSizer,
{
    type Level = LineBreaks;

    fn chunk_config(&self) -> &ChunkConfig<Sizer> {
        &self.chunk_config
    }

    fn parse(&self, text: &str) -> Vec<(Self::Level, Range<usize>)> {
        memchr2_iter(b'\n', b'\r', text.as_bytes())
            .map(|i| i..i + 1)
            .coalesce(|a, b| {
                if a.end == b.start {
                    Ok(a.start..b.end)
                } else {
                    Err((a, b))
                }
            })
            .map(|range| {
                let level = GRAPHEME_SEGMENTER
                    .segment_str(text.get(range.start..range.end).unwrap())
                    .tuple_windows::<(usize, usize)>()
                    .count();
                (
                    match level {
                        0 => unreachable!("regex should always match at least one newline"),
                        n => LineBreaks(n),
                    },
                    range,
                )
            })
            .collect()
    }
}

/// Different semantic levels that text can be split by.
/// Each level provides a method of splitting text into chunks of a given level
/// as well as a fallback in case a given fallback is too large.
///
/// Split by given number of linebreaks, either `\n`, `\r`, or `\r\n`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct LineBreaks(usize);

impl SemanticLevel for LineBreaks {}

//! Vendored from text-splitter v0.30.1 (MIT, © 2023 Benjamin Brandt). See ATTRIBUTIONS.md.

use std::sync::LazyLock;

use auto_enums::auto_enum;
use icu_segmenter::{
    GraphemeClusterSegmenter, GraphemeClusterSegmenterBorrowed, SentenceSegmenter, SentenceSegmenterBorrowed,
    WordSegmenter, WordSegmenterBorrowed,
    options::{SentenceBreakInvariantOptions, WordBreakInvariantOptions},
};
use itertools::Itertools;
use strum::EnumIter;

pub const GRAPHEME_SEGMENTER: GraphemeClusterSegmenterBorrowed<'static> = GraphemeClusterSegmenter::new();
static WORD_SEGMENTER: LazyLock<WordSegmenterBorrowed<'static>> =
    LazyLock::new(|| WordSegmenter::new_dictionary(WordBreakInvariantOptions::default()));
static SENTENCE_SEGMENTER: LazyLock<SentenceSegmenterBorrowed<'static>> =
    LazyLock::new(|| SentenceSegmenter::new(SentenceBreakInvariantOptions::default()));

/// When using a custom semantic level, it is possible that none of them will
/// be small enough to fit into the chunk size. In order to make sure we can
/// still move the cursor forward, we fallback to unicode segmentation.
#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq, Ord, PartialOrd)]
pub enum FallbackLevel {
    /// Split by individual chars. May be larger than a single byte,
    /// but we don't go lower so we always have valid UTF str's.
    Char,
    /// Split by [unicode grapheme clusters](https://www.unicode.org/reports/tr29/#Grapheme_Cluster_Boundaries)    Grapheme,
    GraphemeCluster,
    /// Split by [unicode words](https://www.unicode.org/reports/tr29/#Word_Boundaries)
    Word,
    /// Split by [unicode sentences](https://www.unicode.org/reports/tr29/#Sentence_Boundaries)
    Sentence,
}

impl FallbackLevel {
    #[auto_enum(Iterator)]
    pub fn sections(self, text: &str) -> impl Iterator<Item = (usize, &str)> {
        match self {
            Self::Char => text
                .char_indices()
                .map(move |(i, c)| (i, text.get(i..i + c.len_utf8()).expect("char should be valid"))),
            Self::GraphemeCluster => GRAPHEME_SEGMENTER
                .segment_str(text)
                .tuple_windows()
                .map(|(i, j)| (i, &text[i..j])),
            Self::Word => WORD_SEGMENTER
                .segment_str(text)
                .tuple_windows()
                .map(|(i, j)| (i, &text[i..j])),
            Self::Sentence => SENTENCE_SEGMENTER
                .segment_str(text)
                .tuple_windows()
                .map(|(i, j)| (i, &text[i..j])),
        }
    }
}

//! Character utilities for text assembly: CJK detection and spacing logic.

/// Returns true if the character is a CJK ideograph, Hiragana, Katakana, or Hangul.
pub(super) fn is_cjk_char(c: char) -> bool {
    let cp = c as u32;
    matches!(cp,
        0x4E00..=0x9FFF     // CJK Unified Ideographs
        | 0x3040..=0x309F   // Hiragana
        | 0x30A0..=0x30FF   // Katakana
        | 0xAC00..=0xD7AF   // Hangul Syllables
        | 0x3400..=0x4DBF   // CJK Extension A
        | 0xF900..=0xFAFF   // CJK Compatibility Ideographs
        | 0x20000..=0x2A6DF // CJK Extension B
        | 0x2A700..=0x2B73F // CJK Extension C
        | 0x2B740..=0x2B81F // CJK Extension D
        | 0x2B820..=0x2CEAF // CJK Extension E
        | 0x2CEB0..=0x2EBEF // CJK Extension F
        | 0x30000..=0x3134F // CJK Extension G
        | 0x31350..=0x323AF // CJK Extension H
        | 0x2F800..=0x2FA1F // CJK Compatibility Ideographs Supplement
    )
}

/// Returns true if a space should be inserted between two adjacent text chunks.
/// CJK text should not have spaces between them.
pub(super) fn needs_space_between(prev: &str, next: &str) -> bool {
    let prev_ends_cjk = prev.chars().last().is_some_and(is_cjk_char);
    let next_starts_cjk = next.chars().next().is_some_and(is_cjk_char);
    !(prev_ends_cjk && next_starts_cjk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cjk_char_basic() {
        assert!(is_cjk_char('\u{4E00}')); // CJK
        assert!(is_cjk_char('\u{3042}')); // Hiragana
        assert!(is_cjk_char('\u{30A2}')); // Katakana
        assert!(!is_cjk_char('A'));
        assert!(!is_cjk_char(' '));
    }

    #[test]
    fn test_needs_space_between() {
        assert!(needs_space_between("hello", "world"));
        assert!(!needs_space_between("\u{4E00}", "\u{4E01}"));
        assert!(needs_space_between("hello", "\u{4E00}"));
        assert!(needs_space_between("\u{4E00}", "hello"));
    }
}

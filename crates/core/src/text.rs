//! Text utilities: counting, normalization, and safe strings.

use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;

/// Count words in a string (Unicode-aware).
pub fn count_words(s: &str) -> usize {
    s.unicode_words().count()
}

/// Count characters (grapheme clusters) in a string.
pub fn count_chars(s: &str) -> usize {
    s.graphemes(true).count()
}

/// Count lines in a string.
pub fn count_lines(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }
    s.lines().count()
}

/// Normalize text to NFC form (canonical composition).
pub fn normalize_nfc(s: &str) -> String {
    s.nfc().collect()
}

/// Normalize text to NFD form (canonical decomposition).
pub fn normalize_nfd(s: &str) -> String {
    s.nfd().collect()
}

/// Collapse whitespace (including newlines) to single spaces and trim.
pub fn collapse_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A string that has been validated for safe storage.
///
/// Rejects strings containing NUL bytes or invalid UTF-8 (inherent in `String`).
pub fn validate_storage_string(s: &str) -> bool {
    !s.contains('\0')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_chars("hello"), 5);
        // Devanagari combining sequence counts as one grapheme
        assert_eq!(count_chars("\u{0915}\u{094D}\u{0937}"), 1);
        assert_eq!(count_lines("a\nb\nc"), 3);
    }

    #[test]
    fn whitespace_collapse() {
        assert_eq!(collapse_whitespace("  a   b\n\tc  "), "a b c");
    }

    #[test]
    fn storage_string_validation() {
        assert!(validate_storage_string("ok"));
        assert!(!validate_storage_string("bad\0string"));
    }
}

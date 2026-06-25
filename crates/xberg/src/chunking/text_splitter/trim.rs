//! Vendored from text-splitter v0.30.1 (MIT, © 2023 Benjamin Brandt). See ATTRIBUTIONS.md.
/*!
Different trimming behaviors for different splitter types.
*/

/// Out-of-the-box trim options.
/// If you need a custom trim behavior, you can implement the `Trim` trait.
#[derive(Clone, Copy, Debug)]
pub enum Trim {
    /// Will remove all leading and trailing whitespaces.
    All,
    /// Will remove all leading newlines and all trailing whitespace.
    /// If there are newlines within the text, then indentation will be preserved
    /// (leading spaces or tabs at the beginning of the text). If not, then all
    /// leading whitespace will be trimmed.
    /// Useful for text like Markdown or code, where indentation is important to
    /// the meaning of the text.
    PreserveIndentation,
    /// Apply no trimming
    None,
}

const NEWLINES: [char; 2] = ['\n', '\r'];

impl Trim {
    pub fn trim(self, offset: usize, chunk: &str) -> (usize, &str) {
        match self {
            Self::All => {
                // Figure out how many bytes we lose trimming the beginning
                let diff = chunk.len() - chunk.trim_start().len();
                (offset + diff, chunk.trim())
            }
            Self::PreserveIndentation => {
                // Preserve indentation if we have newlines inside the element
                if chunk.trim().contains(NEWLINES) {
                    let diff = chunk.len() - chunk.trim_start_matches(NEWLINES).len();
                    (offset + diff, chunk.trim_start_matches(NEWLINES).trim_end())
                } else {
                    Self::All.trim(offset, chunk)
                }
            }
            Self::None => (offset, chunk),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_all() {
        let chunk = "  hello world  ";
        let (offset, chunk) = Trim::All.trim(0, chunk);
        assert_eq!(offset, 2);
        assert_eq!(chunk, "hello world");
    }

    #[test]
    fn trim_indentation_fallback() {
        let chunk = "  hello world  ";
        let (offset, chunk) = Trim::PreserveIndentation.trim(0, chunk);
        assert_eq!(offset, 2);
        assert_eq!(chunk, "hello world");
    }

    #[test]
    fn trim_indentation_preserved() {
        let chunk = "\n  hello\n  world  ";
        let (offset, chunk) = Trim::PreserveIndentation.trim(0, chunk);
        assert_eq!(offset, 1);
        assert_eq!(chunk, "  hello\n  world");
    }
}

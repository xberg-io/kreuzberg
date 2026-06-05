//! LLM-driven abstractive summarisation.
//!
//! Wraps [`crate::llm::text_completion::complete_text`] with a prompt template
//! tuned for summary generation. Returns the prose summary plus the captured
//! LLM usage entry so the caller can append it to
//! [`crate::types::ExtractionResult::llm_usage`].

use crate::core::config::LlmConfig;
use crate::types::LlmUsage;

/// Source label embedded in the [`LlmUsage`] entry produced by this backend.
pub const USAGE_SOURCE: &str = "summarisation_abstractive";

const DEFAULT_MAX_TOKENS: u32 = 256;

/// Upper bound on input characters sent to the LLM. Roughly 32k tokens at the
/// common ~4 chars/token rule of thumb — comfortably below the context window
/// of every supported model and prevents pathological token-budget blowups for
/// large documents.
const MAX_PROMPT_INPUT_CHARS: usize = 128 * 1024;

/// Run abstractive summarisation against the configured LLM.
///
/// `text` is the document content to summarise (already extracted by the
/// pipeline). `max_tokens` softly bounds the requested summary length in
/// natural-language tokens; `None` uses [`DEFAULT_MAX_TOKENS`].
///
/// Returns the summary string and the (optional) usage record.
///
/// # Errors
///
/// Propagates any LLM client / request error returned by
/// [`crate::llm::text_completion::complete_text`].
#[cfg_attr(alef, alef(skip))]
pub async fn summarize_with_llm(
    text: &str,
    llm_config: &LlmConfig,
    max_tokens: Option<u32>,
) -> crate::Result<(String, Option<LlmUsage>)> {
    let target = max_tokens.unwrap_or(DEFAULT_MAX_TOKENS);
    let trimmed = truncate_input(text, MAX_PROMPT_INPUT_CHARS);
    let prompt = build_prompt(trimmed, target);
    crate::llm::text_completion::complete_text(llm_config, &prompt, USAGE_SOURCE).await
}

/// Slice `text` to at most `limit` characters on a UTF-8 boundary.
fn truncate_input(text: &str, limit: usize) -> &str {
    if text.len() <= limit {
        return text;
    }
    let mut end = limit;
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
}

fn build_prompt(text: &str, target_tokens: u32) -> String {
    format!(
        "Summarise the following document in approximately {target_tokens} tokens. \
         Produce a single concise prose paragraph. Do not include bullet lists, \
         markdown formatting, headings, or commentary about the summary itself. \
         Preserve named entities, numbers, and the document's original tone.\n\n\
         <document>\n{text}\n</document>\n\n\
         Summary:"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_prompt_embeds_text_and_budget() {
        let prompt = build_prompt("hello world", 32);
        assert!(prompt.contains("32 tokens"));
        assert!(prompt.contains("hello world"));
        assert!(prompt.contains("Summary:"));
    }

    #[test]
    fn truncate_input_passes_short_strings_through() {
        assert_eq!(truncate_input("hello", 100), "hello");
    }

    #[test]
    fn truncate_input_clips_long_strings_on_char_boundary() {
        let long = "a".repeat(MAX_PROMPT_INPUT_CHARS + 16);
        let truncated = truncate_input(&long, MAX_PROMPT_INPUT_CHARS);
        assert_eq!(truncated.len(), MAX_PROMPT_INPUT_CHARS);
    }

    #[test]
    fn truncate_input_respects_utf8_boundaries() {
        // Multi-byte chars must not be split.
        let s = "äöü".repeat(100); // each char is 2 bytes
        let truncated = truncate_input(&s, 5);
        // 5 bytes lands inside a 2-byte char, so we expect 4 bytes (2 full chars).
        assert!(truncated.len() <= 5);
        assert!(s.starts_with(truncated));
        // No partial char.
        assert!(std::str::from_utf8(truncated.as_bytes()).is_ok());
    }
}

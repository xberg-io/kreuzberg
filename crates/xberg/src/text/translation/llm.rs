//! LLM-driven translation backend.
//!
//! Builds a Minijinja prompt per text segment (whole content, formatted
//! content, chunks) and calls
//! [`crate::llm::text_completion::complete_text`] for each. Empty segments are
//! skipped so we do not waste tokens on whitespace.

use crate::core::config::TranslationConfig;
use crate::types::translation::Translation;
use crate::types::{ExtractedDocument, LlmUsage};

/// Default Jinja2 template for LLM translation. Receives `target_lang`,
/// `source_lang` (may be `"auto"`), `preserve_markup`, and `text` variables.
pub const DEFAULT_TRANSLATION_TEMPLATE: &str = "\
You are a precise translation engine. Translate the text below {% if source_lang and source_lang != 'auto' %}from {{ source_lang }} {% endif %}\
into {{ target_lang }}.

Rules:
- Preserve the original meaning exactly.
- Do not add commentary, explanations, or surrounding quotes.
{% if preserve_markup %}- Preserve Markdown formatting (headings, lists, emphasis, links, code blocks) and HTML tags exactly as they appear.\
{% else %}- Return plain text only.{% endif %}
- If the text is already in {{ target_lang }}, return it unchanged.
- If the text is empty, return an empty string.

Text:
{{ text }}";

/// Render the prompt for a single text segment.
fn render_prompt(config: &TranslationConfig, text: &str, preserve_markup: bool) -> crate::Result<String> {
    let ctx = minijinja::context! {
        target_lang => &config.target_lang,
        source_lang => config.source_lang.as_deref().unwrap_or("auto"),
        preserve_markup => preserve_markup,
        text => text,
    };
    crate::llm::prompts::render_template(DEFAULT_TRANSLATION_TEMPLATE, &ctx)
}

/// Translate a single segment, collecting any usage entry produced.
async fn translate_segment(
    config: &TranslationConfig,
    text: &str,
    preserve_markup: bool,
    source_label: &str,
    usages: &mut Vec<LlmUsage>,
) -> crate::Result<String> {
    if text.trim().is_empty() {
        return Ok(text.to_string());
    }
    let prompt = render_prompt(config, text, preserve_markup)?;
    let (translated, usage) = crate::llm::text_completion::complete_text(&config.llm, &prompt, source_label).await?;
    if let Some(u) = usage {
        usages.push(u);
    }
    Ok(translated)
}

/// Translate the extraction result in place.
///
/// Populates `result.translation` with the translated `content`, optionally the
/// translated `formatted_content` (when `preserve_markup = true`), and rewrites
/// every chunk's `content` field. Every LLM call's usage is appended to
/// `result.llm_usage`.
pub async fn translate_result(result: &mut ExtractedDocument, config: &TranslationConfig) -> crate::Result<()> {
    if config.target_lang.trim().is_empty() {
        return Err(crate::XbergError::validation(
            "TranslationConfig.target_lang must not be empty",
        ));
    }

    let mut usages: Vec<LlmUsage> = Vec::new();

    // Translate plain content.
    let translated_content =
        translate_segment(config, &result.content, false, "translation_content", &mut usages).await?;

    // Translate formatted_content (markup-preserving mode) when requested and present.
    let translated_formatted = if config.preserve_markup
        && let Some(formatted) = result.formatted_content.as_deref()
        && !formatted.trim().is_empty()
    {
        Some(translate_segment(config, formatted, true, "translation_formatted", &mut usages).await?)
    } else {
        None
    };

    // Translate chunks in place when present.
    if let Some(chunks) = result.chunks.as_mut() {
        for chunk in chunks.iter_mut() {
            let translated = translate_segment(config, &chunk.content, false, "translation_chunk", &mut usages).await?;
            chunk.content = translated;
        }
    }

    result.translation = Some(Translation {
        target_lang: config.target_lang.clone(),
        source_lang: config.source_lang.clone(),
        content: translated_content,
        formatted_content: translated_formatted,
    });

    if !usages.is_empty() {
        result.llm_usage.get_or_insert_with(Vec::new).extend(usages);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::LlmConfig;

    fn cfg() -> TranslationConfig {
        TranslationConfig {
            target_lang: "de".to_string(),
            source_lang: None,
            preserve_markup: false,
            llm: LlmConfig {
                model: "openai/gpt-4o-mini".to_string(),
                ..Default::default()
            },
        }
    }

    #[test]
    fn render_prompt_includes_target_lang() {
        let prompt = render_prompt(&cfg(), "Hello world", false).unwrap();
        assert!(prompt.contains("de"));
        assert!(prompt.contains("Hello world"));
    }

    #[test]
    fn render_prompt_includes_source_lang_when_set() {
        let mut c = cfg();
        c.source_lang = Some("en".to_string());
        let prompt = render_prompt(&c, "Hello", false).unwrap();
        assert!(prompt.contains("from en"));
    }

    #[test]
    fn render_prompt_preserves_markup_clause_when_enabled() {
        let prompt = render_prompt(&cfg(), "**hi**", true).unwrap();
        assert!(prompt.contains("Markdown"));
    }
}

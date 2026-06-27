//! Built-in Middle-stage post-processor that drives LLM translation.
//!
//! Activates when [`ExtractionConfig::translation`](crate::core::config::ExtractionConfig::translation)
//! is `Some`. Delegates to [`crate::text::translation::translate_result`].

use std::sync::Arc;

use async_trait::async_trait;

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::plugins::{Plugin, PostProcessor, ProcessingStage, register_post_processor};
use crate::types::ExtractedDocument;

/// Post-processor that translates content (and optionally formatted content and chunks)
/// into the requested target language via the configured LLM.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Copy, Default)]
pub struct TranslationProcessor;

impl Plugin for TranslationProcessor {
    fn name(&self) -> &str {
        "translation"
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn initialize(&self) -> Result<()> {
        Ok(())
    }

    fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl PostProcessor for TranslationProcessor {
    async fn process(&self, result: &mut ExtractedDocument, config: &ExtractionConfig) -> Result<()> {
        if let Some(t_config) = config.translation.as_ref() {
            tracing::info!(
                target: "xberg::translation",
                target_lang = %t_config.target_lang,
                source_lang = ?t_config.source_lang,
                preserve_markup = t_config.preserve_markup,
                model = %t_config.llm.model,
                "running translation"
            );
            crate::text::translation::translate_result(result, t_config).await?;
        }
        Ok(())
    }

    fn processing_stage(&self) -> ProcessingStage {
        ProcessingStage::Middle
    }

    fn should_process(&self, _result: &ExtractedDocument, config: &ExtractionConfig) -> bool {
        config.translation.is_some()
    }

    fn priority(&self) -> i32 {
        40
    }
}

/// Register the default translation post-processor with the global registry.
#[cfg_attr(alef, alef(skip))]
pub fn register() -> Result<()> {
    register_post_processor(Arc::new(TranslationProcessor))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{LlmConfig, TranslationConfig};
    use std::borrow::Cow;

    fn config_with_translation() -> ExtractionConfig {
        ExtractionConfig {
            translation: Some(TranslationConfig {
                target_lang: "de".to_string(),
                source_lang: None,
                preserve_markup: false,
                llm: LlmConfig {
                    model: "openai/gpt-4o-mini".to_string(),
                    ..Default::default()
                },
            }),
            ..Default::default()
        }
    }

    #[test]
    fn processor_metadata_is_correct() {
        let p = TranslationProcessor;
        assert_eq!(p.name(), "translation");
        assert_eq!(p.processing_stage(), ProcessingStage::Middle);
    }

    #[test]
    fn should_process_only_when_config_present() {
        let p = TranslationProcessor;
        let result = ExtractedDocument {
            content: "x".to_string(),
            mime_type: Cow::Borrowed("text/plain"),
            ..Default::default()
        };
        assert!(!p.should_process(&result, &ExtractionConfig::default()));
        assert!(p.should_process(&result, &config_with_translation()));
    }
}

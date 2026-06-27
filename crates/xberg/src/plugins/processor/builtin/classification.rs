//! Built-in Middle-stage post-processor that drives per-page LLM classification.
//!
//! Activates when [`ExtractionConfig::page_classification`](crate::core::config::ExtractionConfig::page_classification)
//! is `Some`. Delegates the heavy lifting to
//! [`crate::text::classification::classify_pages`].

use std::sync::Arc;

use async_trait::async_trait;

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::plugins::{Plugin, PostProcessor, ProcessingStage, register_post_processor};
use crate::types::ExtractedDocument;

/// Post-processor that asks an LLM to classify each page of the extracted content.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Copy, Default)]
pub struct PageClassificationProcessor;

impl Plugin for PageClassificationProcessor {
    fn name(&self) -> &str {
        "page-classification"
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
impl PostProcessor for PageClassificationProcessor {
    async fn process(&self, result: &mut ExtractedDocument, config: &ExtractionConfig) -> Result<()> {
        if let Some(pc_config) = config.page_classification.as_ref() {
            tracing::info!(
                target: "xberg::classification",
                labels = pc_config.labels.len(),
                multi_label = pc_config.multi_label,
                model = %pc_config.llm.model,
                "running per-page classification"
            );
            crate::text::classification::classify_pages(result, pc_config).await?;
        }
        Ok(())
    }

    fn processing_stage(&self) -> ProcessingStage {
        ProcessingStage::Middle
    }

    fn should_process(&self, _result: &ExtractedDocument, config: &ExtractionConfig) -> bool {
        config.page_classification.is_some()
    }

    fn priority(&self) -> i32 {
        50
    }
}

/// Register the default page-classification post-processor with the global registry.
#[cfg_attr(alef, alef(skip))]
pub fn register() -> Result<()> {
    register_post_processor(Arc::new(PageClassificationProcessor))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::{LlmConfig, PageClassificationConfig};
    use std::borrow::Cow;

    fn config_with_classification() -> ExtractionConfig {
        ExtractionConfig {
            page_classification: Some(PageClassificationConfig {
                prompt_template: None,
                labels: vec!["invoice".to_string(), "memo".to_string()],
                multi_label: false,
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
        let p = PageClassificationProcessor;
        assert_eq!(p.name(), "page-classification");
        assert_eq!(p.processing_stage(), ProcessingStage::Middle);
    }

    #[test]
    fn should_process_only_when_config_present() {
        let p = PageClassificationProcessor;
        let result = ExtractedDocument {
            content: "x".to_string(),
            mime_type: Cow::Borrowed("text/plain"),
            ..Default::default()
        };
        assert!(!p.should_process(&result, &ExtractionConfig::default()));
        assert!(p.should_process(&result, &config_with_classification()));
    }
}

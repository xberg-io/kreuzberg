//! Live smoke test for the abstractive summarisation path.
//!
//! Skipped automatically when no provider API key is present in the
//! workspace `.env`. Mirrors the skip-on-missing-env idiom used by
//! `llm_integration.rs`.
//!
//! Run with:
//!
//! ```text
//! cargo test -p xberg --features "summarization-llm,quality" --test llm_summarization_smoke -- --nocapture
//! ```

#![cfg(all(feature = "summarization-llm", not(target_os = "windows")))]

use std::borrow::Cow;

use xberg::core::config::{ExtractionConfig, LlmConfig, SummarizationConfig};
use xberg::plugins::PostProcessor;
use xberg::plugins::processor::builtin::summarization::SummarizationProcessor;
use xberg::types::ExtractionResult;
use xberg::types::summary::SummaryStrategy;

const FIXTURE_TEXT: &str = "Machine learning is a branch of artificial intelligence. \
It focuses on building systems that can learn from data without explicit programming. \
Deep learning is a subset of machine learning that uses neural networks with many layers. \
Recurrent neural networks are well-suited for sequence modelling. \
Convolutional neural networks excel at image recognition tasks.";

macro_rules! require_env {
    ($var:expr) => {
        match std::env::var($var) {
            Ok(val) if !val.is_empty() => val,
            _ => {
                eprintln!("SKIP: {} not set, skipping live abstractive summarisation test", $var);
                return;
            }
        }
    };
}

fn init() {
    let _ = dotenvy::dotenv();
}

#[tokio::test]
async fn abstractive_summary_runs_against_real_provider() {
    init();
    let api_key = require_env!("OPENAI_API_KEY");

    let llm = LlmConfig {
        model: "openai/gpt-4o-mini".to_string(),
        api_key: Some(api_key),
        timeout_secs: Some(120),
        max_retries: Some(2),
        ..Default::default()
    };

    let processor = SummarizationProcessor;
    let config = ExtractionConfig {
        summarization: Some(SummarizationConfig {
            strategy: SummaryStrategy::Abstractive,
            max_tokens: Some(120),
            llm: Some(llm),
        }),
        ..Default::default()
    };

    let mut result = ExtractionResult {
        content: FIXTURE_TEXT.to_string(),
        mime_type: Cow::Borrowed("text/plain"),
        detected_languages: Some(vec!["en".to_string()]),
        ..Default::default()
    };

    processor
        .process(&mut result, &config)
        .await
        .expect("abstractive summary succeeds");

    let summary = result.summary.as_ref().expect("summary populated");
    assert_eq!(summary.strategy, SummaryStrategy::Abstractive);
    assert!(!summary.text.is_empty(), "abstractive summary text should be non-empty");

    let usage = result
        .llm_usage
        .as_ref()
        .expect("LLM usage recorded")
        .iter()
        .find(|u| u.source == xberg::text::summarization::llm::USAGE_SOURCE)
        .expect("summarisation usage entry present");
    assert!(!usage.model.is_empty(), "usage entry must record the model id");
}

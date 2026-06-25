//! Deterministic integration tests for the TextRank extractive summariser.
//!
//! These tests do not touch the network and require no API keys. They lock
//! the public scoring behaviour so future refactors can spot regressions in
//! the PageRank weighting, stopword filtering, or sentence segmentation.

#![cfg(feature = "summarization")]

use std::borrow::Cow;

use xberg::core::config::{ExtractionConfig, SummarizationConfig};
use xberg::plugins::PostProcessor;
use xberg::plugins::processor::builtin::summarization::SummarizationProcessor;
use xberg::text::summarization::textrank;
use xberg::types::ExtractionResult;
use xberg::types::summary::SummaryStrategy;

const ML_PARAGRAPH: &str = "Machine learning is a branch of artificial intelligence. \
It focuses on building systems that can learn from data without explicit programming. \
Deep learning is a subset of machine learning that uses neural networks with many layers. \
Neural networks are inspired by the structure of the human brain. \
Convolutional neural networks excel at image recognition tasks. \
Recurrent neural networks are well-suited for sequence modelling. \
The chef carefully seasoned the soup before serving. \
The weather forecast predicts heavy rain tomorrow afternoon.";

#[test]
fn textrank_summarize_returns_some_for_well_formed_text() {
    let summary = textrank::summarize(ML_PARAGRAPH, Some("en"), Some(60));
    assert!(summary.is_some(), "summary should be produced");
    let text = summary.unwrap();
    assert!(!text.is_empty());
}

#[test]
fn textrank_summarize_is_deterministic() {
    let one = textrank::summarize(ML_PARAGRAPH, Some("en"), Some(60)).expect("summary");
    let two = textrank::summarize(ML_PARAGRAPH, Some("en"), Some(60)).expect("summary");
    assert_eq!(one, two, "TextRank must produce identical output across runs");
}

#[test]
fn textrank_summarize_prefers_thematically_central_sentences() {
    // The chef / weather sentences are off-topic outliers — they should NOT
    // dominate the summary when the budget is tight.
    let summary = textrank::summarize(ML_PARAGRAPH, Some("en"), Some(40)).expect("summary");
    let lower = summary.to_lowercase();

    let on_topic_hits = ["machine", "deep", "neural", "learning"]
        .iter()
        .filter(|kw| lower.contains(*kw))
        .count();
    let off_topic_hits = ["chef", "soup", "weather", "rain"]
        .iter()
        .filter(|kw| lower.contains(*kw))
        .count();

    assert!(
        on_topic_hits >= off_topic_hits,
        "expected on-topic sentences to dominate ({on_topic_hits} on-topic vs {off_topic_hits} off-topic) — summary: {summary}",
    );
}

#[test]
fn textrank_summarize_returns_none_for_empty_input() {
    assert!(textrank::summarize("", None, None).is_none());
    assert!(textrank::summarize("   \n\t  ", None, None).is_none());
}

#[test]
fn textrank_summarize_passes_single_sentence_through() {
    let single = "Just one sentence here.";
    let summary = textrank::summarize(single, Some("en"), Some(50)).unwrap();
    assert_eq!(summary, single);
}

#[test]
fn textrank_summarize_falls_back_to_english_for_unknown_language() {
    let summary = textrank::summarize(ML_PARAGRAPH, Some("zz"), Some(40)).expect("fallback succeeds");
    assert!(!summary.is_empty());
}

#[test]
fn textrank_summarize_handles_german_with_native_stopwords() {
    // German paragraph mixing machine-learning content with off-topic sentences.
    // The German stopword set should keep ML-related vocabulary signal high enough
    // for the central sentences to outrank the outliers.
    let de_paragraph = "Maschinelles Lernen ist ein Teilgebiet der künstlichen Intelligenz. \
        Es konzentriert sich auf den Bau von Systemen, die aus Daten lernen können. \
        Tiefes Lernen ist eine Untergruppe des maschinellen Lernens. \
        Neuronale Netze sind vom menschlichen Gehirn inspiriert. \
        Das Wetter ist heute ungewöhnlich warm. \
        Der Koch würzte die Suppe sorgfältig.";

    let summary = textrank::summarize(de_paragraph, Some("de"), Some(50)).expect("german summary");
    assert!(!summary.is_empty());

    let lower = summary.to_lowercase();
    let on_topic = ["lernen", "netz", "intelligenz", "maschinellen", "tiefes"]
        .iter()
        .filter(|kw| lower.contains(*kw))
        .count();
    let off_topic = ["wetter", "koch", "suppe"]
        .iter()
        .filter(|kw| lower.contains(*kw))
        .count();
    assert!(
        on_topic >= off_topic,
        "German summary should prefer ML content over outliers (on={on_topic}, off={off_topic}): {summary}"
    );
}

#[test]
fn textrank_summarize_handles_cjk_without_panicking() {
    // CJK scripts have no whitespace-delimited tokens. The algorithm must
    // degrade gracefully — at worst it returns the input back, but it must
    // never panic and the result must be non-empty for non-empty input.
    let zh = "机器学习是人工智能的一个分支。深度学习是机器学习的子集。神经网络受人脑启发。";
    let summary = textrank::summarize(zh, Some("zh"), Some(40));
    // Either Some (segmented somehow) or None (no usable tokens) — neither panics.
    if let Some(s) = summary {
        assert!(!s.is_empty());
    }
}

#[test]
fn textrank_token_count_matches_whitespace_split() {
    assert_eq!(textrank::token_count("alpha beta gamma"), 3);
    assert_eq!(textrank::token_count("  multiple   spaces  count\tcorrectly "), 4);
    assert_eq!(textrank::token_count(""), 0);
}

#[tokio::test]
async fn extractive_processor_populates_extractive_summary() {
    let processor = SummarizationProcessor;
    let config = ExtractionConfig {
        summarization: Some(SummarizationConfig {
            strategy: SummaryStrategy::Extractive,
            max_tokens: Some(60),
            llm: None,
        }),
        ..Default::default()
    };

    let mut result = ExtractionResult {
        content: ML_PARAGRAPH.to_string(),
        mime_type: Cow::Borrowed("text/plain"),
        detected_languages: Some(vec!["en".to_string()]),
        ..Default::default()
    };

    processor.process(&mut result, &config).await.unwrap();

    let summary = result.summary.as_ref().expect("summary populated");
    assert_eq!(summary.strategy, SummaryStrategy::Extractive);
    assert!(!summary.text.is_empty());
    let token_count = summary.token_count.expect("token count present");
    assert!(token_count > 0);
}

#[tokio::test]
async fn extractive_processor_uses_english_when_no_language_detected() {
    let processor = SummarizationProcessor;
    let config = ExtractionConfig {
        summarization: Some(SummarizationConfig {
            strategy: SummaryStrategy::Extractive,
            max_tokens: Some(60),
            llm: None,
        }),
        ..Default::default()
    };

    let mut result = ExtractionResult {
        content: ML_PARAGRAPH.to_string(),
        mime_type: Cow::Borrowed("text/plain"),
        // detected_languages intentionally left as None.
        ..Default::default()
    };

    processor.process(&mut result, &config).await.unwrap();
    assert!(result.summary.is_some());
}

#[tokio::test]
async fn processor_is_idempotent_when_summarization_disabled() {
    let processor = SummarizationProcessor;
    let config = ExtractionConfig::default();

    let mut result = ExtractionResult {
        content: ML_PARAGRAPH.to_string(),
        mime_type: Cow::Borrowed("text/plain"),
        ..Default::default()
    };

    processor.process(&mut result, &config).await.unwrap();
    assert!(result.summary.is_none(), "no summary when config absent");
}

//! Provider-hosted reranking via liter-llm.
//!
//! Sends `(query, documents)` to a provider-hosted rerank endpoint (e.g.
//! Cohere `rerank-english-v3.0`, Jina, Voyage) and returns scored documents
//! via the liter-llm client.
//!
//! Since v5.0.0.

#[cfg(feature = "reranker")]
use liter_llm::{LlmClient, RerankDocument, RerankRequest};

#[cfg(feature = "reranker")]
use crate::core::config::LlmConfig;
#[cfg(feature = "reranker")]
use crate::reranking::RerankedDocument;

/// Rerank documents using a provider-hosted model via liter-llm.
///
/// Returns results sorted by descending score with `top_k` truncation applied
/// if provided.
///
/// # Arguments
///
/// * `query` - The search query.
/// * `documents` - Document texts to rerank (must be non-empty).
/// * `config` - LLM provider/model configuration. The model ID must be a
///   rerank-capable model (e.g. `"cohere/rerank-english-v3.0"`).
/// * `top_k` - Optional truncation applied after sorting.
///
/// # Returns
///
/// `(results, usage)` where `results` is sorted descending by relevance score.
///
/// # Errors
///
/// - `XbergError::Reranking` if the API call fails or returns unexpected data.
/// - `XbergError::MissingDependency` if the liter-llm client cannot be created.
///
/// Since v5.0.0.
#[cfg(feature = "reranker")]
pub(crate) async fn rerank_via_llm(
    query: &str,
    documents: &[String],
    config: &LlmConfig,
    top_k: Option<usize>,
) -> crate::Result<(Vec<RerankedDocument>, Option<crate::types::LlmUsage>)> {
    if documents.is_empty() {
        return Ok((Vec::new(), None));
    }

    let client = super::client::create_client(config)?;

    let docs: Vec<RerankDocument> = documents.iter().map(|d| RerankDocument::Text(d.clone())).collect();

    let request = RerankRequest {
        model: config.model.clone(),
        query: query.to_string(),
        documents: docs,
        top_n: top_k.map(|k| k as u32),
        return_documents: Some(false),
    };

    let response = client.rerank(request).await.map_err(|e| {
        crate::XbergError::reranking(format!("LLM rerank request failed (model={}): {e}", config.model))
    })?;

    let usage = extract_rerank_usage(&response, &config.model);

    // Sort by relevance_score descending (providers may not guarantee order).
    let mut results_raw = response.results;
    results_raw.sort_by(|a, b| {
        b.relevance_score
            .partial_cmp(&a.relevance_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Apply top_k truncation (already applied via RerankRequest.top_n, but providers
    // may return more — truncate defensively).
    if let Some(k) = top_k {
        results_raw.truncate(k);
    }

    let results: Vec<RerankedDocument> = results_raw
        .into_iter()
        .map(|r| {
            let index = r.index as usize;
            let doc_text = documents.get(index).cloned().ok_or_else(|| {
                crate::XbergError::reranking(format!(
                    "LLM reranker returned out-of-range index {index} for a batch of {} documents \
                     (model={}). This is a provider bug.",
                    documents.len(),
                    config.model,
                ))
            })?;
            Ok(RerankedDocument {
                index,
                score: r.relevance_score as f32,
                document: doc_text,
            })
        })
        .collect::<crate::Result<Vec<_>>>()?;

    Ok((results, usage))
}

/// Extract usage metadata from a rerank response.
///
/// liter-llm's `RerankResponse` stores usage in `meta` as an opaque JSON value.
/// We attempt a best-effort parse; failures produce `None` rather than an error.
#[cfg(feature = "reranker")]
fn extract_rerank_usage(response: &liter_llm::RerankResponse, model: &str) -> Option<crate::types::LlmUsage> {
    // Extract token counts from meta if the provider includes them.
    let (input_tokens, total_tokens) = response
        .meta
        .as_ref()
        .and_then(|meta| {
            let billed = meta.get("billed_units").or_else(|| meta.get("usage"))?;
            let input = billed.get("input_tokens").and_then(|v| v.as_u64());
            let total = billed.get("total_tokens").and_then(|v| v.as_u64());
            Some((input, total))
        })
        .unwrap_or((None, None));

    Some(crate::types::LlmUsage {
        model: model.to_string(),
        source: "reranking".to_string(),
        input_tokens,
        output_tokens: None,
        total_tokens,
        estimated_cost: None,
        finish_reason: None,
    })
}

#[cfg(all(test, feature = "reranker"))]
mod tests {
    use super::*;

    #[test]
    fn extract_rerank_usage_with_no_meta_returns_some_with_nones() {
        let response = liter_llm::RerankResponse {
            id: None,
            results: vec![],
            meta: None,
        };
        let usage = extract_rerank_usage(&response, "cohere/rerank-english-v3.0");
        assert!(usage.is_some());
        let u = usage.unwrap();
        assert_eq!(u.model, "cohere/rerank-english-v3.0");
        assert_eq!(u.source, "reranking");
        assert!(u.input_tokens.is_none());
    }

    #[test]
    fn extract_rerank_usage_with_cohere_meta() {
        let meta = serde_json::json!({
            "billed_units": { "input_tokens": 42, "total_tokens": 42 }
        });
        let response = liter_llm::RerankResponse {
            id: None,
            results: vec![],
            meta: Some(meta),
        };
        let usage = extract_rerank_usage(&response, "cohere/rerank-v3.0");
        let u = usage.unwrap();
        assert_eq!(u.input_tokens, Some(42_u64));
        assert_eq!(u.total_tokens, Some(42_u64));
    }
}

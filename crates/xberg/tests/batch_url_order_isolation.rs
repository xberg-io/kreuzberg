#![cfg(feature = "url-ingestion")]

//! Shared-URL batch extraction: input-order preservation, duplicate-URL
//! mapping, per-URL error isolation, and the batch timeout nuance.
//!
//! The multi-URL batch path routes every http(s) URL that shares the batch's
//! base crawl config through ONE crawlberg engine via `batch_scrape` /
//! `batch_crawl`. crawlberg returns results paired with the seed URL in
//! COMPLETION order; the engine must remap them back to INPUT order and isolate
//! per-URL failures. These tests drive that path through local HTTP fixtures
//! (wiremock) and a dead loopback port — no real network access.

use serde_json::Value;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use xberg::engine::Engine;
use xberg::{ExtractInput, ExtractionConfig, ExtractionResult, UrlExtractionMode};

/// Build a batch base config identical in shape to the public URL ingestion
/// tests: a single-concurrency, robots-ignoring crawl that allows loopback so
/// the wiremock fixtures and the dead-port fixture are reachable.
fn url_config(mode: UrlExtractionMode) -> ExtractionConfig {
    let mut crawl = crawlberg::CrawlConfig::builder()
        .allow_private_networks(true)
        .max_concurrent(4)
        .max_depth(1)
        .max_pages(8)
        .respect_robots_txt(false)
        .build();
    crawl.download_documents = true;
    crawl.rate_limit_ms = Some(0);

    let mut config = ExtractionConfig::default();
    config.url.mode = mode;
    config.url.crawl = crawl;
    config
}

fn source_index(result: &xberg::types::ExtractedDocument) -> Option<u64> {
    result.metadata.additional.get("source_index").and_then(Value::as_u64)
}

fn source_uri(result: &xberg::types::ExtractedDocument) -> Option<&str> {
    result.metadata.additional.get("source_uri").and_then(Value::as_str)
}

/// Find the single result annotated with the given input index.
fn result_for_index(output: &ExtractionResult, index: u64) -> &xberg::types::ExtractedDocument {
    output
        .results
        .iter()
        .find(|result| source_index(result) == Some(index))
        .unwrap_or_else(|| panic!("no result for input index {index}: {:?}", output.results))
}

/// Mount an HTML page returning a unique marker on `path_segment`.
async fn mount_html(server: &MockServer, path_segment: &str, marker: &str) {
    Mock::given(method("GET"))
        .and(path(path_segment.to_string()))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(format!(
                    "<html><body><h1>{marker}</h1><p>{marker} body.</p></body></html>"
                ))
                .append_header("content-type", "text/html"),
        )
        .mount(server)
        .await;
}

#[tokio::test]
async fn batch_preserves_input_order_for_shared_urls() {
    let server = MockServer::start().await;
    mount_html(&server, "/a.html", "ALPHA").await;
    mount_html(&server, "/b.html", "BRAVO").await;
    mount_html(&server, "/c.html", "CHARLIE").await;

    let url_a = format!("{}/a.html", server.uri());
    let url_b = format!("{}/b.html", server.uri());
    let url_c = format!("{}/c.html", server.uri());

    let inputs = vec![
        ExtractInput::from_uri(url_a.clone()),
        ExtractInput::from_uri(url_b.clone()),
        ExtractInput::from_uri(url_c.clone()),
    ];

    let output = Engine::new_default()
        .extract_batch(inputs, &url_config(UrlExtractionMode::Document))
        .await
        .expect("shared-URL batch must succeed");

    assert_eq!(output.summary.inputs, 3);
    assert_eq!(output.summary.results, 3);
    assert_eq!(output.summary.errors, 0);
    assert!(output.errors.is_empty(), "unexpected errors: {:?}", output.errors);

    // Results land in INPUT order even though crawlberg returns completion order.
    assert_eq!(source_index(&output.results[0]), Some(0));
    assert_eq!(source_index(&output.results[1]), Some(1));
    assert_eq!(source_index(&output.results[2]), Some(2));
    assert_eq!(source_uri(&output.results[0]), Some(url_a.as_str()));
    assert_eq!(source_uri(&output.results[1]), Some(url_b.as_str()));
    assert_eq!(source_uri(&output.results[2]), Some(url_c.as_str()));
    assert!(output.results[0].content.contains("ALPHA"));
    assert!(output.results[1].content.contains("BRAVO"));
    assert!(output.results[2].content.contains("CHARLIE"));
}

#[tokio::test]
async fn batch_maps_duplicate_urls_to_correct_indices() {
    let server = MockServer::start().await;
    mount_html(&server, "/dup.html", "DUPLICATE").await;
    mount_html(&server, "/other.html", "OTHER").await;

    let dup = format!("{}/dup.html", server.uri());
    let other = format!("{}/other.html", server.uri());

    // Same URL at index 0 and index 2; a distinct URL at index 1.
    let inputs = vec![
        ExtractInput::from_uri(dup.clone()),
        ExtractInput::from_uri(other.clone()),
        ExtractInput::from_uri(dup.clone()),
    ];

    let output = Engine::new_default()
        .extract_batch(inputs, &url_config(UrlExtractionMode::Document))
        .await
        .expect("duplicate-URL batch must succeed");

    assert_eq!(output.summary.results, 3);
    assert_eq!(output.summary.errors, 0);

    // The duplicate URL maps to BOTH index 0 and index 2; the distinct URL to 1.
    assert_eq!(source_uri(result_for_index(&output, 0)), Some(dup.as_str()));
    assert_eq!(source_uri(result_for_index(&output, 1)), Some(other.as_str()));
    assert_eq!(source_uri(result_for_index(&output, 2)), Some(dup.as_str()));
    assert!(result_for_index(&output, 1).content.contains("OTHER"));

    // Output order is still input order.
    assert_eq!(source_index(&output.results[0]), Some(0));
    assert_eq!(source_index(&output.results[1]), Some(1));
    assert_eq!(source_index(&output.results[2]), Some(2));
}

#[tokio::test]
async fn batch_isolates_a_single_failing_url() {
    let server = MockServer::start().await;
    mount_html(&server, "/ok1.html", "FIRST").await;
    mount_html(&server, "/ok2.html", "SECOND").await;

    // A dead loopback port: bind, capture the address, drop the listener so the
    // port is closed -> connection refused -> a per-URL crawl error.
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let dead_addr = listener.local_addr().expect("read local addr");
    drop(listener);

    let ok1 = format!("{}/ok1.html", server.uri());
    let bad = format!("http://{dead_addr}/dead.html");
    let ok2 = format!("{}/ok2.html", server.uri());

    let inputs = vec![
        ExtractInput::from_uri(ok1.clone()),
        ExtractInput::from_uri(bad.clone()),
        ExtractInput::from_uri(ok2.clone()),
    ];

    let output = Engine::new_default()
        .extract_batch(inputs, &url_config(UrlExtractionMode::Document))
        .await
        .expect("batch with one failing URL must still succeed as a whole");

    // The failing URL is isolated to the errors slot at its input index; the
    // other two URLs still produce results.
    assert_eq!(output.summary.results, 2, "two URLs must still succeed");
    assert_eq!(output.errors.len(), 1, "exactly one URL must fail: {:?}", output.errors);
    assert_eq!(output.errors[0].index, 1, "failure must carry the failing input index");
    assert_eq!(output.errors[0].source, bad);

    assert_eq!(source_uri(result_for_index(&output, 0)), Some(ok1.as_str()));
    assert!(result_for_index(&output, 0).content.contains("FIRST"));
    assert_eq!(source_uri(result_for_index(&output, 2)), Some(ok2.as_str()));
    assert!(result_for_index(&output, 2).content.contains("SECOND"));
    // No result was produced for the failing index.
    assert!(output.results.iter().all(|result| source_index(result) != Some(1)));
}

/// Timeout nuance: in batch mode the network fetch is performed inside
/// crawlberg's `batch_scrape`, governed by the shared `CrawlConfig` (request
/// timeout / rate limit), NOT by the xberg per-item `extraction_timeout_secs`.
/// The per-item timeout instead bounds the CONVERSION stage (the `extract_bytes`
/// pipeline run after the fetch). This test asserts that a generous per-item
/// timeout leaves a shared-URL extraction successful and that the timeout-aware
/// finalize path ran (it stamps `extraction_duration_ms`).
#[tokio::test]
async fn batch_conversion_timeout_governs_shared_url_results() {
    let server = MockServer::start().await;
    mount_html(&server, "/timed.html", "TIMED").await;

    let url = format!("{}/timed.html", server.uri());
    let mut config = url_config(UrlExtractionMode::Document);
    // Per-item timeout that comfortably covers the conversion of a tiny page.
    config.extraction_timeout_secs = Some(30);

    let output = Engine::new_default()
        .extract_batch(vec![ExtractInput::from_uri(url.clone())], &config)
        .await
        .expect("shared-URL batch with generous timeout must succeed");

    assert_eq!(output.summary.results, 1);
    assert_eq!(output.summary.errors, 0);
    assert!(output.results[0].content.contains("TIMED"));
    // The timeout-aware finalize path stamps a duration on the converted result.
    assert!(
        output.results[0].metadata.extraction_duration_ms.is_some(),
        "shared-URL finalize must stamp extraction_duration_ms"
    );
}

#![cfg(feature = "url-ingestion")]

//! Public URL ingestion integration tests with local HTTP fixtures.

use crawlberg::CrawlConfig;
use serde_json::Value;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use xberg::{ExtractInput, ExtractionConfig, ExtractionResult, UrlExtractionMode, extract};

const DOCUMENT_BODY: &str = "Downloaded document fixture served from local HTTP.";

fn url_config(mode: UrlExtractionMode) -> ExtractionConfig {
    let mut crawl = CrawlConfig::builder()
        .allow_private_networks(true)
        .max_concurrent(1)
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

fn recursive_url_config() -> ExtractionConfig {
    let mut config = url_config(UrlExtractionMode::Document);
    config.url.crawl.follow_document_urls = true;
    config.url.crawl.document_url_depth = Some(1);
    config.url.max_total_urls = Some(4);
    config
}

fn source_kind(output: &ExtractionResult, index: usize) -> Option<&str> {
    output
        .results
        .get(index)?
        .metadata
        .additional
        .get("source_kind")
        .and_then(Value::as_str)
}

#[tokio::test]
async fn public_url_document_downloads_remote_bytes() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/payload.bin"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(DOCUMENT_BODY)
                .append_header("content-type", "application/octet-stream"),
        )
        .mount(&server)
        .await;

    let output = extract(
        ExtractInput::from_uri(format!("{}/payload.bin", server.uri())),
        &url_config(UrlExtractionMode::Document),
    )
    .await
    .expect("URL document extraction must succeed");

    assert_eq!(output.summary.inputs, 1);
    assert_eq!(output.summary.remote_urls, 1);
    assert_eq!(output.summary.documents_downloaded, 1);
    assert_eq!(output.summary.pages_crawled, 0);
    assert_eq!(output.summary.results, 1);
    assert_eq!(output.summary.errors, 0);
    assert!(output.errors.is_empty(), "unexpected URL errors: {:?}", output.errors);
    assert_eq!(source_kind(&output, 0), Some("url_document"));
    assert!(output.results[0].content.contains(DOCUMENT_BODY));
}

#[tokio::test]
async fn public_url_crawl_extracts_seed_and_linked_page() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/index.html"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(r#"<html><body><h1>Seed page</h1><a href="/guide.html">guide</a></body></html>"#)
                .append_header("content-type", "text/html"),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/guide.html"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("<html><body><h1>Guide page</h1><p>Linked crawl content.</p></body></html>")
                .append_header("content-type", "text/html"),
        )
        .mount(&server)
        .await;

    let output = extract(
        ExtractInput::from_uri(format!("{}/index.html", server.uri())),
        &url_config(UrlExtractionMode::Crawl),
    )
    .await
    .expect("URL crawl extraction must succeed");

    assert_eq!(output.summary.inputs, 1);
    assert_eq!(output.summary.remote_urls, 1);
    assert_eq!(output.summary.pages_crawled, 2);
    assert_eq!(output.summary.documents_downloaded, 0);
    assert_eq!(output.summary.results, 2);
    assert_eq!(output.summary.errors, 0);
    assert!(output.errors.is_empty(), "unexpected URL errors: {:?}", output.errors);
    assert!(
        output.results.iter().all(|result| {
            result.metadata.additional.get("source_kind").and_then(Value::as_str) == Some("url_page")
        })
    );
    assert!(output.results.iter().any(|result| result.content.contains("Seed page")));
    assert!(
        output
            .results
            .iter()
            .any(|result| result.content.contains("Guide page"))
    );
}

#[tokio::test]
async fn public_url_recursive_document_links_followed_from_page() {
    let server = MockServer::start().await;
    let document_url = format!("{}/payload.bin", server.uri());
    Mock::given(method("GET"))
        .and(path("/index.html"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(format!(
                    r#"<html><body><h1>Seed page</h1><a href="{document_url}">payload</a></body></html>"#
                ))
                .append_header("content-type", "text/html"),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/payload.bin"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(DOCUMENT_BODY)
                .append_header("content-type", "application/octet-stream"),
        )
        .mount(&server)
        .await;

    let output = extract(
        ExtractInput::from_uri(format!("{}/index.html", server.uri())),
        &recursive_url_config(),
    )
    .await
    .expect("recursive URL extraction must succeed");

    assert_eq!(output.summary.inputs, 2);
    assert_eq!(output.summary.remote_urls, 2);
    assert_eq!(output.summary.pages_crawled, 1);
    assert_eq!(output.summary.documents_downloaded, 1);
    assert_eq!(output.summary.results, 2);
    assert_eq!(output.summary.errors, 0);
    assert!(output.errors.is_empty(), "unexpected URL errors: {:?}", output.errors);
    assert!(output.results.iter().any(|result| result.content.contains("Seed page")));
    assert!(
        output
            .results
            .iter()
            .any(|result| result.content.contains(DOCUMENT_BODY))
    );
    assert!(output.results.iter().any(|result| {
        result.metadata.additional.get("source_kind").and_then(Value::as_str) == Some("url_document")
    }));
}

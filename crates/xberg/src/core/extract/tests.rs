use std::fs::File;
use std::io::Write;

use tempfile::tempdir;

use super::*;

#[tokio::test]
async fn extract_bytes_input_returns_envelope() {
    let config = ExtractionConfig::default();
    let output = extract(ExtractInput::from_bytes(b"hello".to_vec(), "text/plain", None), &config)
        .await
        .unwrap();

    assert_eq!(output.results.len(), 1);
    assert_eq!(output.summary.inputs, 1);
    assert_eq!(output.summary.results, 1);
    assert_eq!(output.results[0].content.trim(), "hello");
}

#[tokio::test]
async fn extract_local_uri_returns_envelope() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doc.txt");
    File::create(&path).unwrap().write_all(b"hello path").unwrap();

    let config = ExtractionConfig::default();
    let output = extract(ExtractInput::from_uri(path.to_string_lossy()), &config)
        .await
        .unwrap();

    assert_eq!(output.results.len(), 1);
    assert_eq!(output.results[0].content.trim(), "hello path");
}

#[tokio::test]
async fn extract_file_uri_returns_envelope() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doc.txt");
    File::create(&path).unwrap().write_all(b"hello file uri").unwrap();

    let config = ExtractionConfig::default();
    let output = extract(ExtractInput::from_uri(format!("file://{}", path.display())), &config)
        .await
        .unwrap();

    assert_eq!(output.results.len(), 1);
    assert_eq!(output.results[0].content.trim(), "hello file uri");
}

#[tokio::test]
async fn extract_rejects_local_path_when_policy_disallows_it() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doc.txt");
    File::create(&path).unwrap().write_all(b"hello local policy").unwrap();

    let mut config = ExtractionConfig::default();
    config.url.allow_local_file_inputs = false;
    let error = extract(ExtractInput::from_uri(path.to_string_lossy()), &config)
        .await
        .unwrap_err();

    assert!(error.to_string().contains("local filesystem path inputs are disabled"));
}

#[tokio::test]
async fn extract_rejects_non_local_file_uri_host() {
    let config = ExtractionConfig::default();
    let error = extract(ExtractInput::from_uri("file://evilhost/tmp/doc.txt"), &config)
        .await
        .unwrap_err();

    assert!(error.to_string().contains("unsupported non-local file URI host"));
}

#[tokio::test]
async fn extract_file_uri_accepts_localhost_host() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doc.txt");
    File::create(&path)
        .unwrap()
        .write_all(b"hello localhost file uri")
        .unwrap();

    let config = ExtractionConfig::default();
    let output = extract(
        ExtractInput::from_uri(format!("file://localhost{}", path.display())),
        &config,
    )
    .await
    .unwrap();

    assert_eq!(output.results.len(), 1);
    assert_eq!(output.results[0].content.trim(), "hello localhost file uri");
}

#[tokio::test]
async fn extract_rejects_unsupported_scheme() {
    let config = ExtractionConfig::default();
    let error = extract(ExtractInput::from_uri("s3://bucket/file.txt"), &config)
        .await
        .unwrap_err();

    assert!(error.to_string().contains("unsupported URI scheme"));
}

#[tokio::test]
async fn extract_batch_collects_mixed_inputs() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("doc.txt");
    File::create(&path).unwrap().write_all(b"hello batch path").unwrap();

    let config = ExtractionConfig::default();
    let output = extract_batch(
        vec![
            ExtractInput::from_bytes(b"hello batch bytes".to_vec(), "text/plain", None),
            ExtractInput::from_uri(path.to_string_lossy()),
        ],
        &config,
    )
    .await
    .unwrap();

    assert_eq!(output.results.len(), 2);
    assert_eq!(output.summary.inputs, 2);
    assert!(output.errors.is_empty());
}

#[tokio::test]
async fn extract_batch_collects_unsupported_scheme_error() {
    let config = ExtractionConfig::default();
    let output = extract_batch(
        vec![
            ExtractInput::from_bytes(b"hello batch bytes".to_vec(), "text/plain", None),
            ExtractInput::from_uri("s3://bucket/doc.txt"),
        ],
        &config,
    )
    .await
    .unwrap();

    assert_eq!(output.results.len(), 1);
    assert_eq!(output.errors.len(), 1);
    assert_eq!(output.summary.inputs, 2);
    assert_eq!(output.summary.results, 1);
    assert_eq!(output.summary.errors, 1);
    assert_eq!(output.errors[0].index, 1);
    assert_eq!(output.errors[0].code, 1003);
    assert_eq!(output.errors[0].error_type, "unsupported_format");
}

#[tokio::test]
async fn extract_batch_applies_item_timeout() {
    let item = run_batch_item(
        0,
        "<test>".to_string(),
        std::sync::Arc::new(tokio::sync::Semaphore::new(1)),
        Some(1),
        None,
        || async {
            std::future::pending::<()>().await;
            Ok(ExtractionResult::default())
        },
    )
    .await;

    let error = item.result.unwrap_err();
    assert_eq!(error_code(&error), 1004);
    assert_eq!(error_type(&error), "timeout");
}

#[cfg(feature = "url-ingestion")]
#[tokio::test]
async fn url_markdown_page_runs_through_pipeline() {
    let config = ExtractionConfig::default();
    let links = vec![ExtractedUri {
        url: "https://example.com/next".to_string(),
        label: Some("next".to_string()),
        page: None,
        kind: UriKind::Hyperlink,
    }];

    let result = run_url_page_pipeline(
        "alpha beta gamma delta epsilon zeta eta theta".to_string(),
        true,
        "text/html; charset=utf-8",
        links,
        &config,
    )
    .await
    .unwrap();

    assert_eq!(result.mime_type, "text/markdown");
    assert_eq!(result.metadata.output_format.as_deref(), Some("plain"));
    assert_eq!(result.uris.as_ref().map(Vec::len), Some(1));
}

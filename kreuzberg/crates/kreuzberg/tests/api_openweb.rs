//! Integration tests for the OpenWebUI compatibility endpoints.

#![cfg(feature = "api")]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use kreuzberg::{
    ExtractionConfig,
    api::{DoclingCompatResponse, OpenWebDocumentResponse, create_router},
};

// ---------------------------------------------------------------------------
// PUT /process — OpenWebUI "External" engine
// ---------------------------------------------------------------------------

/// Test successful extraction via the external engine endpoint.
#[tokio::test]
async fn test_openweb_process_text_file() {
    let app = create_router(ExtractionConfig::default());

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/process")
                .header("content-type", "text/plain")
                .header("X-Filename", "hello.txt")
                .body(Body::from("Hello, world!"))
                .expect("Failed to create HTTP request body"),
        )
        .await
        .expect("Failed to send HTTP request");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read HTTP response body");
    let doc: OpenWebDocumentResponse = serde_json::from_slice(&body).expect("Failed to deserialize JSON response");

    assert!(
        doc.page_content.contains("Hello, world"),
        "Expected extracted text to contain 'Hello, world', got: {}",
        doc.page_content
    );
    assert_eq!(doc.metadata.source, "hello.txt");
}

/// Test that a URL-encoded filename in X-Filename is decoded correctly.
#[tokio::test]
async fn test_openweb_process_url_encoded_filename() {
    let app = create_router(ExtractionConfig::default());

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/process")
                .header("content-type", "text/plain")
                .header("X-Filename", "my%20document%20%281%29.txt")
                .body(Body::from("content"))
                .expect("Failed to create HTTP request body"),
        )
        .await
        .expect("Failed to send HTTP request");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read HTTP response body");
    let doc: OpenWebDocumentResponse = serde_json::from_slice(&body).expect("Failed to deserialize JSON response");

    assert_eq!(doc.metadata.source, "my document (1).txt");
}

/// Test that the external endpoint returns 400 on empty body.
#[tokio::test]
async fn test_openweb_process_empty_body_returns_400() {
    let app = create_router(ExtractionConfig::default());

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/process")
                .header("content-type", "text/plain")
                .body(Body::empty())
                .expect("Failed to create HTTP request body"),
        )
        .await
        .expect("Failed to send HTTP request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

/// Test fallback when no X-Filename header is provided.
#[tokio::test]
async fn test_openweb_process_missing_filename_defaults_to_unknown() {
    let app = create_router(ExtractionConfig::default());

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/process")
                .header("content-type", "text/plain")
                .body(Body::from("some text"))
                .expect("Failed to create HTTP request body"),
        )
        .await
        .expect("Failed to send HTTP request");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read HTTP response body");
    let doc: OpenWebDocumentResponse = serde_json::from_slice(&body).expect("Failed to deserialize JSON response");

    assert_eq!(doc.metadata.source, "unknown");
}

/// Test MIME type detection from filename when Content-Type is octet-stream.
#[tokio::test]
async fn test_openweb_process_octet_stream_detects_mime_from_filename() {
    let app = create_router(ExtractionConfig::default());

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/process")
                .header("content-type", "application/octet-stream")
                .header("X-Filename", "readme.txt")
                .body(Body::from("Plain text content"))
                .expect("Failed to create HTTP request body"),
        )
        .await
        .expect("Failed to send HTTP request");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read HTTP response body");
    let doc: OpenWebDocumentResponse = serde_json::from_slice(&body).expect("Failed to deserialize JSON response");

    assert!(doc.page_content.contains("Plain text content"));
}

// ---------------------------------------------------------------------------
// POST /v1/convert/file — OpenWebUI "Docling" engine
// ---------------------------------------------------------------------------

/// Test successful extraction via the docling-compatible endpoint.
#[tokio::test]
async fn test_openweb_docling_text_file() {
    let app = create_router(ExtractionConfig::default());

    let boundary = "----boundary";
    let body_content = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"files\"; filename=\"test.txt\"\r\n\
         Content-Type: text/plain\r\n\
         \r\n\
         Hello from docling!\r\n\
         --{boundary}--\r\n"
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/convert/file")
                .header("content-type", format!("multipart/form-data; boundary={}", boundary))
                .body(Body::from(body_content))
                .expect("Failed to create HTTP request body"),
        )
        .await
        .expect("Failed to send HTTP request");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read HTTP response body");
    let resp: DoclingCompatResponse = serde_json::from_slice(&body).expect("Failed to deserialize JSON response");

    assert_eq!(resp.status, "success");
    assert!(
        resp.document.md_content.contains("Hello from docling"),
        "Expected md_content to contain 'Hello from docling', got: {}",
        resp.document.md_content
    );
}

/// Test that the docling endpoint returns 400 when no files field is provided.
#[tokio::test]
async fn test_openweb_docling_no_file_returns_400() {
    let app = create_router(ExtractionConfig::default());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/convert/file")
                .header("content-type", "multipart/form-data; boundary=testboundary")
                .body(Body::from("--testboundary--\r\n"))
                .expect("Failed to create HTTP request body"),
        )
        .await
        .expect("Failed to send HTTP request");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

/// Test that the docling endpoint detects MIME from filename when Content-Type is octet-stream.
#[tokio::test]
async fn test_openweb_docling_octet_stream_detects_mime() {
    let app = create_router(ExtractionConfig::default());

    let boundary = "----boundary";
    let body_content = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"files\"; filename=\"data.txt\"\r\n\
         Content-Type: application/octet-stream\r\n\
         \r\n\
         Some plain text\r\n\
         --{boundary}--\r\n"
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/convert/file")
                .header("content-type", format!("multipart/form-data; boundary={}", boundary))
                .body(Body::from(body_content))
                .expect("Failed to create HTTP request body"),
        )
        .await
        .expect("Failed to send HTTP request");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read HTTP response body");
    let resp: DoclingCompatResponse = serde_json::from_slice(&body).expect("Failed to deserialize JSON response");

    assert_eq!(resp.status, "success");
    assert!(resp.document.md_content.contains("Some plain text"));
}

/// Test that the response JSON structure matches what OpenWebUI expects.
#[tokio::test]
async fn test_openweb_docling_response_structure() {
    let app = create_router(ExtractionConfig::default());

    let boundary = "----boundary";
    let body_content = format!(
        "--{boundary}\r\n\
         Content-Disposition: form-data; name=\"files\"; filename=\"test.txt\"\r\n\
         Content-Type: text/plain\r\n\
         \r\n\
         content\r\n\
         --{boundary}--\r\n"
    );

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/convert/file")
                .header("content-type", format!("multipart/form-data; boundary={}", boundary))
                .body(Body::from(body_content))
                .expect("Failed to create HTTP request body"),
        )
        .await
        .expect("Failed to send HTTP request");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read HTTP response body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("Failed to deserialize JSON response");

    // OpenWebUI reads exactly these fields
    assert!(json["document"].is_object(), "Expected 'document' object");
    assert!(
        json["document"]["md_content"].is_string(),
        "Expected 'document.md_content' string"
    );
    assert!(json["status"].is_string(), "Expected 'status' string");
}

/// Test that the external engine response structure matches what OpenWebUI expects.
#[tokio::test]
async fn test_openweb_process_response_structure() {
    let app = create_router(ExtractionConfig::default());

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/process")
                .header("content-type", "text/plain")
                .header("X-Filename", "test.txt")
                .body(Body::from("content"))
                .expect("Failed to create HTTP request body"),
        )
        .await
        .expect("Failed to send HTTP request");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read HTTP response body");
    let json: serde_json::Value = serde_json::from_slice(&body).expect("Failed to deserialize JSON response");

    // OpenWebUI reads exactly these fields
    assert!(json["page_content"].is_string(), "Expected 'page_content' string");
    assert!(json["metadata"].is_object(), "Expected 'metadata' object");
    assert!(
        json["metadata"]["source"].is_string(),
        "Expected 'metadata.source' string"
    );
}

//! Integration tests for the /embed API endpoint.

#![cfg(all(feature = "api", feature = "embeddings"))]

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt;

use kreuzberg::{
    ExtractionConfig,
    api::{EmbedResponse, create_router},
};

/// Test embed endpoint with valid texts.
#[tokio::test]
async fn test_embed_valid_texts() {
    let app = create_router(ExtractionConfig::default());

    let request_body = json!({
        "texts": ["Hello world", "Second text"]
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&request_body).expect("Operation failed"),
                ))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to convert to bytes");
    let embed_response: EmbedResponse = serde_json::from_slice(&body).expect("Failed to deserialize");

    assert_eq!(embed_response.count, 2);
    assert_eq!(embed_response.embeddings.len(), 2);
    assert!(embed_response.dimensions > 0);
    assert!(!embed_response.model.is_empty());

    // Verify embeddings have correct dimensions
    for embedding in &embed_response.embeddings {
        assert_eq!(embedding.len(), embed_response.dimensions);
    }
}

/// Test embed endpoint with empty texts array returns 400.
#[tokio::test]
async fn test_embed_empty_texts() {
    let app = create_router(ExtractionConfig::default());

    let request_body = json!({
        "texts": []
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&request_body).expect("Operation failed"),
                ))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

/// Test embed endpoint with custom embedding configuration.
#[tokio::test]
#[cfg_attr(target_arch = "aarch64", ignore = "ONNX Runtime model loading unstable on ARM")]
async fn test_embed_with_custom_config() {
    let app = create_router(ExtractionConfig::default());

    let request_body = json!({
        "texts": ["Test embedding with custom config"],
        "config": {
            "model": {
                "type": "preset",
                "name": "fast"
            },
            "batch_size": 32
        }
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&request_body).expect("Operation failed"),
                ))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to convert to bytes");
    let embed_response: EmbedResponse = serde_json::from_slice(&body).expect("Failed to deserialize");

    assert_eq!(embed_response.count, 1);
    assert_eq!(embed_response.embeddings.len(), 1);
    assert_eq!(embed_response.model, "fast");
}

/// Test embed endpoint with single text.
#[tokio::test]
#[cfg_attr(target_arch = "aarch64", ignore = "ONNX Runtime model loading unstable on ARM")]
async fn test_embed_single_text() {
    let app = create_router(ExtractionConfig::default());

    let request_body = json!({
        "texts": ["Single text for embedding"]
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&request_body).expect("Operation failed"),
                ))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to convert to bytes");
    let embed_response: EmbedResponse = serde_json::from_slice(&body).expect("Failed to deserialize");

    assert_eq!(embed_response.count, 1);
    assert_eq!(embed_response.embeddings.len(), 1);
}

/// Test embed endpoint with multiple texts (batch).
#[tokio::test]
async fn test_embed_batch() {
    let app = create_router(ExtractionConfig::default());

    let texts: Vec<String> = (0..10).map(|i| format!("Test text number {}", i)).collect();

    let request_body = json!({
        "texts": texts
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&request_body).expect("Operation failed"),
                ))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to convert to bytes");
    let embed_response: EmbedResponse = serde_json::from_slice(&body).expect("Failed to deserialize");

    assert_eq!(embed_response.count, 10);
    assert_eq!(embed_response.embeddings.len(), 10);

    // Verify all embeddings have the same dimensions
    let first_dim = embed_response.embeddings[0].len();
    for embedding in &embed_response.embeddings {
        assert_eq!(embedding.len(), first_dim);
    }
}

/// Test embed endpoint with long text.
#[tokio::test]
#[cfg_attr(target_arch = "aarch64", ignore = "ONNX Runtime model loading unstable on ARM")]
async fn test_embed_long_text() {
    let app = create_router(ExtractionConfig::default());

    let long_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(100);

    let request_body = json!({
        "texts": [long_text]
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&request_body).expect("Operation failed"),
                ))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to convert to bytes");
    let embed_response: EmbedResponse = serde_json::from_slice(&body).expect("Failed to deserialize");

    assert_eq!(embed_response.count, 1);
    assert_eq!(embed_response.embeddings.len(), 1);
}

/// Test embed endpoint with malformed JSON returns 400.
#[tokio::test]
async fn test_embed_malformed_json() {
    let app = create_router(ExtractionConfig::default());

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from("{invalid json}"))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

/// Test embed endpoint rejects JSON array at root level.
#[tokio::test]
async fn test_embed_rejects_json_array() {
    let app = create_router(ExtractionConfig::default());

    // Send a JSON array instead of object
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(r#"[["text1"], {"texts": ["text2"]}]"#))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    // Should reject with 400 or 422, NOT 200
    assert!(
        response.status() == StatusCode::BAD_REQUEST || response.status() == StatusCode::UNPROCESSABLE_ENTITY,
        "Expected 400 or 422, got {}",
        response.status()
    );
}

/// Test embed endpoint rejects simple JSON array with strings.
#[tokio::test]
async fn test_embed_rejects_simple_json_array() {
    let app = create_router(ExtractionConfig::default());

    // Send a simple string array instead of object with texts field
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(r#"["text1", "text2", "text3"]"#))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    // Check that error response contains helpful message
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("Failed to read response body");
    let error_response: serde_json::Value = serde_json::from_slice(&body).expect("Failed to parse error response");

    assert!(
        error_response["message"]
            .as_str()
            .map(|msg| msg.contains("array") || msg.contains("object"))
            .unwrap_or(false)
    );
}

/// Test embed endpoint preserves embedding vector values across calls.
#[tokio::test]
#[cfg_attr(target_arch = "aarch64", ignore = "ONNX Runtime model loading unstable on ARM")]
async fn test_embed_deterministic() {
    let app = create_router(ExtractionConfig::default());

    let request_body = json!({
        "texts": ["Deterministic test"]
    });

    // First call
    let response1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&request_body).expect("Operation failed"),
                ))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response1.status(), StatusCode::OK);

    let body1 = axum::body::to_bytes(response1.into_body(), usize::MAX)
        .await
        .expect("Failed to convert to bytes");
    let embed_response1: EmbedResponse = serde_json::from_slice(&body1).expect("Failed to deserialize");

    // Second call with same text
    let response2 = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&request_body).expect("Operation failed"),
                ))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response2.status(), StatusCode::OK);

    let body2 = axum::body::to_bytes(response2.into_body(), usize::MAX)
        .await
        .expect("Failed to convert to bytes");
    let embed_response2: EmbedResponse = serde_json::from_slice(&body2).expect("Failed to deserialize");

    // Compare embeddings - they should be identical
    assert_eq!(embed_response1.embeddings.len(), embed_response2.embeddings.len());
    assert_eq!(embed_response1.embeddings[0], embed_response2.embeddings[0]);
}

/// Test embed endpoint with different embedding presets.
#[tokio::test]
#[cfg_attr(target_arch = "aarch64", ignore = "ONNX Runtime model loading unstable on ARM")]
async fn test_embed_different_presets() {
    let app = create_router(ExtractionConfig::default());

    // Test with "fast" preset
    let request_fast = json!({
        "texts": ["Test text"],
        "config": {
            "model": {
                "type": "preset",
                "name": "fast"
            }
        }
    });

    let response_fast = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&request_fast).expect("Operation failed"),
                ))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response_fast.status(), StatusCode::OK);

    let body_fast = axum::body::to_bytes(response_fast.into_body(), usize::MAX)
        .await
        .expect("Operation failed");
    let embed_fast: EmbedResponse = serde_json::from_slice(&body_fast).expect("Failed to deserialize");

    // Test with "balanced" preset
    let request_balanced = json!({
        "texts": ["Test text"],
        "config": {
            "model": {
                "type": "preset",
                "name": "balanced"
            }
        }
    });

    let response_balanced = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/embed")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&request_balanced).expect("Operation failed"),
                ))
                .expect("Operation failed"),
        )
        .await
        .expect("Operation failed");

    assert_eq!(response_balanced.status(), StatusCode::OK);

    let body_balanced = axum::body::to_bytes(response_balanced.into_body(), usize::MAX)
        .await
        .expect("Operation failed");
    let embed_balanced: EmbedResponse = serde_json::from_slice(&body_balanced).expect("Failed to deserialize");

    // Different presets should have different dimensions
    assert_ne!(embed_fast.dimensions, embed_balanced.dimensions);
    assert_eq!(embed_fast.model, "fast");
    assert_eq!(embed_balanced.model, "balanced");
}

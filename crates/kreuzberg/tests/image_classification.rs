//! Integration test for image classification and clustering.

use bytes::Bytes;
use kreuzberg::extraction::image_kind::{classify, cluster_tiles};
use kreuzberg::types::{ExtractedImage, ImageKind};

#[test]
fn test_classify_simple() {
    // Test basic classification works
    let (kind, conf) = classify(&[], "jpeg", Some(1000), Some(1000), None, None, false);
    assert_eq!(kind, ImageKind::Photograph);
    assert!(conf > 0.8, "confidence should be > 0.8 for large JPEG");
}

#[test]
fn test_cluster_tiles_basic() {
    // Test clustering works
    let mut images = vec![
        ExtractedImage {
            data: Bytes::new(),
            format: "png".into(),
            image_index: 0,
            page_number: Some(1),
            width: Some(100),
            height: Some(100),
            colorspace: None,
            bits_per_component: None,
            is_mask: false,
            description: None,
            ocr_result: None,
            bounding_box: None,
            source_path: None,
            image_kind: Some(ImageKind::Drawing),
            kind_confidence: Some(0.7),
            cluster_id: None,
            caption: None,
            qr_codes: None,
            data_base64: None,
        },
        ExtractedImage {
            data: Bytes::new(),
            format: "png".into(),
            image_index: 1,
            page_number: Some(1),
            width: Some(100),
            height: Some(100),
            colorspace: None,
            bits_per_component: None,
            is_mask: false,
            description: None,
            ocr_result: None,
            bounding_box: None,
            source_path: None,
            image_kind: Some(ImageKind::Drawing),
            kind_confidence: Some(0.7),
            cluster_id: None,
            caption: None,
            qr_codes: None,
            data_base64: None,
        },
    ];

    cluster_tiles(&mut images);

    // Both should be in the same cluster (same page, same kind, adjacent indices)
    assert_eq!(images[0].cluster_id, Some(1));
    assert_eq!(images[1].cluster_id, Some(1));
}

#[test]
fn test_image_kind_serde() {
    // Test that ImageKind serializes/deserializes correctly
    let kind = ImageKind::Photograph;
    let json = serde_json::to_string(&kind).unwrap();
    assert_eq!(json, "\"photograph\"");

    let deserialized: ImageKind = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, ImageKind::Photograph);
}

#[test]
fn test_extracted_image_with_classification() {
    // Test that ExtractedImage with classification fields roundtrips
    let image = ExtractedImage {
        data: Bytes::new(),
        format: "png".into(),
        image_index: 0,
        page_number: Some(1),
        width: Some(100),
        height: Some(100),
        colorspace: None,
        bits_per_component: None,
        is_mask: false,
        description: None,
        ocr_result: None,
        bounding_box: None,
        source_path: None,
        image_kind: Some(ImageKind::Icon),
        kind_confidence: Some(0.85),
        cluster_id: Some(1),
        caption: None,
        qr_codes: None,
        data_base64: None,
    };

    let json = serde_json::to_string(&image).unwrap();
    assert!(json.contains("\"image_kind\":\"icon\""));
    assert!(json.contains("\"kind_confidence\":0.85"));
    assert!(json.contains("\"cluster_id\":1"));

    let deserialized: ExtractedImage = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.image_kind, Some(ImageKind::Icon));
    assert_eq!(deserialized.kind_confidence, Some(0.85));
    assert_eq!(deserialized.cluster_id, Some(1));
}

#[test]
fn test_extracted_image_with_classification_none() {
    // Test that ExtractedImage without classification fields works
    let image = ExtractedImage {
        data: Bytes::new(),
        format: "png".into(),
        image_index: 0,
        page_number: Some(1),
        width: Some(100),
        height: Some(100),
        colorspace: None,
        bits_per_component: None,
        is_mask: false,
        description: None,
        ocr_result: None,
        bounding_box: None,
        source_path: None,
        image_kind: None,
        kind_confidence: None,
        cluster_id: None,
        caption: None,
        qr_codes: None,
        data_base64: None,
    };

    let json = serde_json::to_string(&image).unwrap();
    // Optional fields should not appear when None
    assert!(!json.contains("image_kind"));
    assert!(!json.contains("kind_confidence"));
    assert!(!json.contains("cluster_id"));

    let deserialized: ExtractedImage = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.image_kind, None);
    assert_eq!(deserialized.kind_confidence, None);
    assert_eq!(deserialized.cluster_id, None);
}

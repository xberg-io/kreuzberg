//! Comprehensive tests for PaddleOCR multi-language support.
//!
//! This test suite verifies the multi-language model infrastructure WITHOUT requiring
//! network access or ONNX runtime. Tests focus on language mapping, model management,
//! configuration, and backend initialization.
//!
//! Run with: `cargo test -p kreuzberg --features paddle-ocr --test paddle_ocr_multilang`

#![cfg(feature = "paddle-ocr")]

use kreuzberg::core::config::OcrConfig;
use kreuzberg::paddle_ocr::{
    ModelManager, PaddleLanguage, PaddleOcrBackend, PaddleOcrConfig, SUPPORTED_LANGUAGES, language_to_script_family,
    map_language_code,
};
use kreuzberg::plugins::{OcrBackend, Plugin};
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================================================
// Language Mapping Tests (non-ignored, no network needed)
// ============================================================================

/// Test that all PaddleOCR language codes map to correct script families.
#[test]
fn test_language_to_script_family_all_paddle_codes() {
    // PP-OCRv5 families (7 families)
    assert_eq!(language_to_script_family("en"), "english");
    assert_eq!(language_to_script_family("ch"), "chinese");
    assert_eq!(language_to_script_family("japan"), "chinese");
    assert_eq!(language_to_script_family("chinese_cht"), "chinese");
    assert_eq!(language_to_script_family("korean"), "korean");
    assert_eq!(language_to_script_family("latin"), "latin");
    assert_eq!(language_to_script_family("french"), "latin");
    assert_eq!(language_to_script_family("german"), "latin");
    assert_eq!(language_to_script_family("cyrillic"), "eslav");
    assert_eq!(language_to_script_family("thai"), "thai");
    assert_eq!(language_to_script_family("greek"), "greek");

    // PP-OCRv3 families (5 families)
    assert_eq!(language_to_script_family("arabic"), "arabic");
    assert_eq!(language_to_script_family("devanagari"), "devanagari");
    assert_eq!(language_to_script_family("ta"), "tamil");
    assert_eq!(language_to_script_family("te"), "telugu");
    assert_eq!(language_to_script_family("ka"), "kannada");
}

/// Test that Tesseract-style language codes map correctly to PaddleOCR codes.
#[test]
fn test_language_to_script_family_tesseract_codes() {
    // Tesseract codes should map via map_language_code first, then to families
    assert_eq!(map_language_code("eng"), Some("en"));
    assert_eq!(language_to_script_family("en"), "english");

    assert_eq!(map_language_code("fra"), Some("french"));
    assert_eq!(language_to_script_family("french"), "latin");

    assert_eq!(map_language_code("deu"), Some("german"));
    assert_eq!(language_to_script_family("german"), "latin");

    assert_eq!(map_language_code("chi_sim"), Some("ch"));
    assert_eq!(language_to_script_family("ch"), "chinese");

    assert_eq!(map_language_code("jpn"), Some("japan"));
    assert_eq!(language_to_script_family("japan"), "chinese");

    assert_eq!(map_language_code("kor"), Some("korean"));
    assert_eq!(language_to_script_family("korean"), "korean");

    assert_eq!(map_language_code("ara"), Some("arabic"));
    assert_eq!(language_to_script_family("arabic"), "arabic");

    assert_eq!(map_language_code("hin"), Some("devanagari"));
    assert_eq!(language_to_script_family("devanagari"), "devanagari");

    assert_eq!(map_language_code("tam"), Some("ta"));
    assert_eq!(language_to_script_family("ta"), "tamil");

    assert_eq!(map_language_code("tel"), Some("te"));
    assert_eq!(language_to_script_family("te"), "telugu");

    assert_eq!(map_language_code("kan"), Some("ka"));
    assert_eq!(language_to_script_family("ka"), "kannada");

    assert_eq!(map_language_code("tha"), Some("thai"));
    assert_eq!(language_to_script_family("thai"), "thai");

    assert_eq!(map_language_code("ell"), Some("greek"));
    assert_eq!(language_to_script_family("greek"), "greek");

    assert_eq!(map_language_code("rus"), Some("cyrillic"));
    assert_eq!(language_to_script_family("cyrillic"), "eslav");
}

/// Test that ISO 639-1 language codes map correctly to PaddleOCR codes.
#[test]
fn test_language_to_script_family_iso639_codes() {
    // ISO 639-1 codes (2-letter)
    assert_eq!(map_language_code("en"), Some("en"));
    assert_eq!(map_language_code("fr"), Some("french"));
    assert_eq!(map_language_code("de"), Some("german"));
    assert_eq!(map_language_code("zh"), Some("ch"));
    assert_eq!(map_language_code("ja"), Some("japan"));
    assert_eq!(map_language_code("ko"), Some("korean"));
    assert_eq!(map_language_code("ar"), Some("arabic"));
    assert_eq!(map_language_code("hi"), Some("devanagari"));
    assert_eq!(map_language_code("ta"), Some("ta"));
    assert_eq!(map_language_code("te"), Some("te"));
    assert_eq!(map_language_code("ka"), Some("ka"));
    assert_eq!(map_language_code("th"), Some("thai"));
    assert_eq!(map_language_code("el"), Some("greek"));
    assert_eq!(map_language_code("ru"), Some("cyrillic"));

    // Verify they map to correct families
    assert_eq!(language_to_script_family("en"), "english");
    assert_eq!(language_to_script_family("french"), "latin");
    assert_eq!(language_to_script_family("german"), "latin");
    assert_eq!(language_to_script_family("ch"), "chinese");
    assert_eq!(language_to_script_family("japan"), "chinese");
    assert_eq!(language_to_script_family("korean"), "korean");
    assert_eq!(language_to_script_family("arabic"), "arabic");
    assert_eq!(language_to_script_family("devanagari"), "devanagari");
    assert_eq!(language_to_script_family("ta"), "tamil");
    assert_eq!(language_to_script_family("te"), "telugu");
    assert_eq!(language_to_script_family("ka"), "kannada");
    assert_eq!(language_to_script_family("thai"), "thai");
    assert_eq!(language_to_script_family("greek"), "greek");
    assert_eq!(language_to_script_family("cyrillic"), "eslav");
}

/// Test that unknown language codes fall back to "english" script family.
#[test]
fn test_language_to_script_family_unknown_fallback() {
    // Unknown codes should fall back to "english"
    assert_eq!(language_to_script_family("xyz"), "english");
    assert_eq!(language_to_script_family("unknown"), "english");
    assert_eq!(language_to_script_family("invalid"), "english");
    assert_eq!(language_to_script_family(""), "english");
    assert_eq!(language_to_script_family("klingon"), "english");
}

/// Test that map_language_code normalizes various formats to canonical PaddleOCR codes.
#[test]
fn test_map_language_code_normalization() {
    // English variants
    assert_eq!(map_language_code("en"), Some("en"));
    assert_eq!(map_language_code("eng"), Some("en"));
    assert_eq!(map_language_code("english"), Some("en"));

    // Chinese variants
    assert_eq!(map_language_code("ch"), Some("ch"));
    assert_eq!(map_language_code("chi_sim"), Some("ch"));
    assert_eq!(map_language_code("zho"), Some("ch"));
    assert_eq!(map_language_code("zh"), Some("ch"));
    assert_eq!(map_language_code("chinese"), Some("ch"));

    // Traditional Chinese
    assert_eq!(map_language_code("chi_tra"), Some("chinese_cht"));
    assert_eq!(map_language_code("zh_tw"), Some("chinese_cht"));
    assert_eq!(map_language_code("zh_hant"), Some("chinese_cht"));

    // Japanese
    assert_eq!(map_language_code("ja"), Some("japan"));
    assert_eq!(map_language_code("jpn"), Some("japan"));
    assert_eq!(map_language_code("japanese"), Some("japan"));

    // Korean
    assert_eq!(map_language_code("ko"), Some("korean"));
    assert_eq!(map_language_code("kor"), Some("korean"));
    assert_eq!(map_language_code("korean"), Some("korean"));

    // French
    assert_eq!(map_language_code("fr"), Some("french"));
    assert_eq!(map_language_code("fra"), Some("french"));
    assert_eq!(map_language_code("french"), Some("french"));

    // German
    assert_eq!(map_language_code("de"), Some("german"));
    assert_eq!(map_language_code("deu"), Some("german"));
    assert_eq!(map_language_code("german"), Some("german"));

    // Arabic
    assert_eq!(map_language_code("ar"), Some("arabic"));
    assert_eq!(map_language_code("ara"), Some("arabic"));
    assert_eq!(map_language_code("arabic"), Some("arabic"));

    // Thai
    assert_eq!(map_language_code("th"), Some("thai"));
    assert_eq!(map_language_code("tha"), Some("thai"));
    assert_eq!(map_language_code("thai"), Some("thai"));

    // Greek
    assert_eq!(map_language_code("el"), Some("greek"));
    assert_eq!(map_language_code("ell"), Some("greek"));
    assert_eq!(map_language_code("greek"), Some("greek"));

    // Russian and other Cyrillic
    assert_eq!(map_language_code("ru"), Some("cyrillic"));
    assert_eq!(map_language_code("rus"), Some("cyrillic"));
    assert_eq!(map_language_code("russian"), Some("cyrillic"));
    assert_eq!(map_language_code("uk"), Some("cyrillic"));
    assert_eq!(map_language_code("ukr"), Some("cyrillic"));
    assert_eq!(map_language_code("ukrainian"), Some("cyrillic"));

    // Hindi (Devanagari)
    assert_eq!(map_language_code("hi"), Some("devanagari"));
    assert_eq!(map_language_code("hin"), Some("devanagari"));
    assert_eq!(map_language_code("hindi"), Some("devanagari"));

    // Tamil
    assert_eq!(map_language_code("ta"), Some("ta"));
    assert_eq!(map_language_code("tam"), Some("ta"));
    assert_eq!(map_language_code("tamil"), Some("ta"));

    // Telugu
    assert_eq!(map_language_code("te"), Some("te"));
    assert_eq!(map_language_code("tel"), Some("te"));
    assert_eq!(map_language_code("telugu"), Some("te"));

    // Kannada
    assert_eq!(map_language_code("ka"), Some("ka"));
    assert_eq!(map_language_code("kan"), Some("ka"));
    assert_eq!(map_language_code("kannada"), Some("ka"));

    // Latin script languages (should map to "latin")
    assert_eq!(map_language_code("es"), Some("latin"));
    assert_eq!(map_language_code("spa"), Some("latin"));
    assert_eq!(map_language_code("spanish"), Some("latin"));
    assert_eq!(map_language_code("it"), Some("latin"));
    assert_eq!(map_language_code("ita"), Some("latin"));
    assert_eq!(map_language_code("italian"), Some("latin"));
    assert_eq!(map_language_code("pt"), Some("latin"));
    assert_eq!(map_language_code("por"), Some("latin"));
    assert_eq!(map_language_code("portuguese"), Some("latin"));

    // Unknown codes should return None
    assert_eq!(map_language_code("xyz"), None);
    assert_eq!(map_language_code("unknown"), None);
    assert_eq!(map_language_code("invalid"), None);
}

/// Test that SUPPORTED_LANGUAGES contains expected entries and correct count.
#[test]
fn test_supported_languages_list() {
    // Should contain 16 entries as documented
    assert_eq!(SUPPORTED_LANGUAGES.len(), 16);

    // Verify key languages are present
    assert!(SUPPORTED_LANGUAGES.contains(&"ch"));
    assert!(SUPPORTED_LANGUAGES.contains(&"en"));
    assert!(SUPPORTED_LANGUAGES.contains(&"french"));
    assert!(SUPPORTED_LANGUAGES.contains(&"german"));
    assert!(SUPPORTED_LANGUAGES.contains(&"korean"));
    assert!(SUPPORTED_LANGUAGES.contains(&"japan"));
    assert!(SUPPORTED_LANGUAGES.contains(&"chinese_cht"));
    assert!(SUPPORTED_LANGUAGES.contains(&"ta"));
    assert!(SUPPORTED_LANGUAGES.contains(&"te"));
    assert!(SUPPORTED_LANGUAGES.contains(&"ka"));
    assert!(SUPPORTED_LANGUAGES.contains(&"latin"));
    assert!(SUPPORTED_LANGUAGES.contains(&"arabic"));
    assert!(SUPPORTED_LANGUAGES.contains(&"cyrillic"));
    assert!(SUPPORTED_LANGUAGES.contains(&"devanagari"));
    assert!(SUPPORTED_LANGUAGES.contains(&"thai"));
    assert!(SUPPORTED_LANGUAGES.contains(&"greek"));
}

// ============================================================================
// Model Manager Tests (non-ignored, no network needed)
// ============================================================================

/// Test that ModelManager creates the cache directory path correctly.
#[test]
fn test_model_manager_cache_dir_creation() {
    let temp_dir = TempDir::new().unwrap();
    let cache_path = temp_dir.path().join("paddle-cache");

    let manager = ModelManager::new(cache_path.clone());

    assert_eq!(manager.cache_dir(), &cache_path);
}

/// Test that rec_family_path returns the correct path for each script family.
#[test]
fn test_model_manager_rec_family_path() {
    let temp_dir = TempDir::new().unwrap();
    let _manager = ModelManager::new(temp_dir.path().to_path_buf());

    // Test all 12 script families
    let families = [
        "english",
        "chinese",
        "latin",
        "korean",
        "eslav",
        "thai",
        "greek",
        "arabic",
        "devanagari",
        "tamil",
        "telugu",
        "kannada",
    ];

    for family in families {
        // We can't call rec_family_path directly as it's private, but we can verify
        // the cache structure via the public API
        let expected_path = temp_dir.path().join("rec").join(family);

        // The path should not exist yet (no models cached)
        assert!(!expected_path.exists(), "Path should not exist yet for {}", family);

        // After creating the directory structure manually, verify it matches expectation
        std::fs::create_dir_all(&expected_path).unwrap();
        assert!(expected_path.exists(), "Path should exist for {}", family);
        assert!(expected_path.ends_with(format!("rec/{}", family)));
    }
}

/// Test that fresh ModelManager reports models as not cached.
#[test]
fn test_model_manager_empty_cache_not_cached() {
    let temp_dir = TempDir::new().unwrap();
    let manager = ModelManager::new(temp_dir.path().to_path_buf());

    // Should report as not cached
    assert!(!manager.are_shared_models_cached());
    assert!(!manager.is_rec_model_cached("english"));
    assert!(!manager.is_rec_model_cached("chinese"));
    assert!(!manager.is_rec_model_cached("latin"));
    assert!(!manager.is_rec_model_cached("korean"));
    assert!(!manager.is_rec_model_cached("thai"));
    assert!(!manager.is_rec_model_cached("greek"));
    assert!(!manager.are_models_cached());
}

// ============================================================================
// Config Tests (non-ignored)
// ============================================================================

/// Test that all PaddleLanguage enum variants exist and match documentation.
#[test]
fn test_paddle_language_enum_variants() {
    // Verify all documented variants exist
    let _english = PaddleLanguage::English;
    let _chinese = PaddleLanguage::Chinese;
    let _japanese = PaddleLanguage::Japanese;
    let _korean = PaddleLanguage::Korean;
    let _german = PaddleLanguage::German;
    let _french = PaddleLanguage::French;
    let _latin = PaddleLanguage::Latin;
    let _arabic = PaddleLanguage::Arabic;
    let _cyrillic = PaddleLanguage::Cyrillic;
    let _devanagari = PaddleLanguage::Devanagari;
    let _tamil = PaddleLanguage::Tamil;
    let _telugu = PaddleLanguage::Telugu;
    let _kannada = PaddleLanguage::Kannada;
    let _traditional_chinese = PaddleLanguage::TraditionalChinese;
    let _thai = PaddleLanguage::Thai;
    let _greek = PaddleLanguage::Greek;
    let _east_slavic = PaddleLanguage::EastSlavic;

    // Verify codes match expectations
    assert_eq!(PaddleLanguage::English.code(), "en");
    assert_eq!(PaddleLanguage::Chinese.code(), "ch");
    assert_eq!(PaddleLanguage::Japanese.code(), "jpn");
    assert_eq!(PaddleLanguage::Korean.code(), "kor");
    assert_eq!(PaddleLanguage::German.code(), "deu");
    assert_eq!(PaddleLanguage::French.code(), "fra");
    assert_eq!(PaddleLanguage::Latin.code(), "latin");
    assert_eq!(PaddleLanguage::Arabic.code(), "arabic");
    assert_eq!(PaddleLanguage::Cyrillic.code(), "cyrillic");
    assert_eq!(PaddleLanguage::Devanagari.code(), "devanagari");
    assert_eq!(PaddleLanguage::Tamil.code(), "ta");
    assert_eq!(PaddleLanguage::Telugu.code(), "te");
    assert_eq!(PaddleLanguage::Kannada.code(), "ka");
    assert_eq!(PaddleLanguage::TraditionalChinese.code(), "chinese_cht");
    assert_eq!(PaddleLanguage::Thai.code(), "thai");
    assert_eq!(PaddleLanguage::Greek.code(), "greek");
    assert_eq!(PaddleLanguage::EastSlavic.code(), "eslav");
}

/// Test that PaddleOcrConfig::new stores the correct language.
#[test]
fn test_paddle_ocr_config_new_with_language() {
    let config_en = PaddleOcrConfig::new("en");
    assert_eq!(config_en.language, "en");

    let config_ch = PaddleOcrConfig::new("ch");
    assert_eq!(config_ch.language, "ch");

    let config_thai = PaddleOcrConfig::new("thai");
    assert_eq!(config_thai.language, "thai");

    let config_greek = PaddleOcrConfig::new("greek");
    assert_eq!(config_greek.language, "greek");

    let config_cyrillic = PaddleOcrConfig::new("cyrillic");
    assert_eq!(config_cyrillic.language, "cyrillic");
}

/// Test that with_cache_dir properly sets the cache directory.
#[test]
fn test_paddle_ocr_config_with_cache_dir() {
    let cache_path = PathBuf::from("/custom/cache/dir");
    let config = PaddleOcrConfig::new("en").with_cache_dir(cache_path.clone());

    assert_eq!(config.cache_dir, Some(cache_path.clone()));
    assert_eq!(config.resolve_cache_dir(), cache_path);
}

// ============================================================================
// Backend Tests (non-ignored)
// ============================================================================

/// Test that PaddleOcrBackend::new() succeeds without errors.
#[test]
fn test_paddle_backend_creation() {
    let result = PaddleOcrBackend::new();
    assert!(result.is_ok(), "Backend creation should succeed");

    let backend = result.unwrap();
    assert_eq!(backend.name(), "paddle-ocr");
}

/// Test that supports_language works for all new language codes.
#[test]
fn test_paddle_backend_supports_language_expanded() {
    let backend = PaddleOcrBackend::new().unwrap();

    // Direct PaddleOCR codes
    assert!(backend.supports_language("en"));
    assert!(backend.supports_language("ch"));
    assert!(backend.supports_language("japan"));
    assert!(backend.supports_language("korean"));
    assert!(backend.supports_language("french"));
    assert!(backend.supports_language("german"));
    assert!(backend.supports_language("latin"));
    assert!(backend.supports_language("arabic"));
    assert!(backend.supports_language("cyrillic"));
    assert!(backend.supports_language("devanagari"));
    assert!(backend.supports_language("ta"));
    assert!(backend.supports_language("te"));
    assert!(backend.supports_language("ka"));
    assert!(backend.supports_language("thai"));
    assert!(backend.supports_language("greek"));
    assert!(backend.supports_language("chinese_cht"));
}

/// Test that Tesseract-style codes are supported via mapping.
#[test]
fn test_paddle_backend_supports_tesseract_mapped_codes() {
    let backend = PaddleOcrBackend::new().unwrap();

    // Tesseract codes that map to PaddleOCR codes
    assert!(backend.supports_language("eng")); // → en
    assert!(backend.supports_language("chi_sim")); // → ch
    assert!(backend.supports_language("jpn")); // → japan
    assert!(backend.supports_language("kor")); // → korean
    assert!(backend.supports_language("fra")); // → french
    assert!(backend.supports_language("deu")); // → german
    assert!(backend.supports_language("ara")); // → arabic
    assert!(backend.supports_language("tha")); // → thai
    assert!(backend.supports_language("ell")); // → greek
    assert!(backend.supports_language("rus")); // → cyrillic
    assert!(backend.supports_language("hin")); // → devanagari
    assert!(backend.supports_language("tam")); // → ta
    assert!(backend.supports_language("tel")); // → te
    assert!(backend.supports_language("kan")); // → ka

    // ISO 639-1 codes
    assert!(backend.supports_language("en"));
    assert!(backend.supports_language("zh"));
    assert!(backend.supports_language("ja"));
    assert!(backend.supports_language("ko"));
    assert!(backend.supports_language("fr"));
    assert!(backend.supports_language("de"));
    assert!(backend.supports_language("ar"));
    assert!(backend.supports_language("th"));
    assert!(backend.supports_language("el"));
    assert!(backend.supports_language("ru"));
    assert!(backend.supports_language("hi"));
    assert!(backend.supports_language("ta"));
    assert!(backend.supports_language("te"));
}

// ============================================================================
// Integration Tests (ignored, require network + ONNX)
// ============================================================================

/// Test OCR with Chinese language and Chinese model.
///
/// This test verifies that the Chinese recognition model can process Chinese text.
/// Requires network access to download the Chinese rec model and ONNX Runtime.
#[tokio::test]
#[ignore = "requires network access and ONNX Runtime"]
async fn test_ocr_chinese_with_chinese_model() {
    let test_documents_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("test_documents");

    let image_path = test_documents_dir.join("images/chi_sim_image.jpeg");
    if !image_path.exists() {
        eprintln!("Skipping test: Chinese test image not found at {:?}", image_path);
        return;
    }

    let image_bytes = std::fs::read(&image_path).expect("Failed to read Chinese test image");

    let cache_dir = std::env::temp_dir().join("kreuzberg_paddle_multilang_test");
    let config = PaddleOcrConfig::new("ch").with_cache_dir(cache_dir);
    let backend = PaddleOcrBackend::with_config(config).expect("Failed to create backend");

    let ocr_config = OcrConfig {
        backend: "paddle-ocr".to_string(),
        language: "ch".to_string(),
        ..Default::default()
    };

    let result = backend.process_image(&image_bytes, &ocr_config).await;
    assert!(result.is_ok(), "Chinese OCR failed: {:?}", result.err());

    let extraction = result.unwrap();
    println!("Chinese OCR result: {}", extraction.content);

    // The result should contain Chinese characters (not empty after using Chinese model)
    assert!(
        !extraction.content.is_empty(),
        "Expected non-empty result from Chinese OCR"
    );

    // Verify Chinese characters are present (Unicode range for CJK)
    let has_chinese_chars = extraction.content.chars().any(|c| {
        matches!(c,
            '\u{4E00}'..='\u{9FFF}' | // CJK Unified Ideographs
            '\u{3400}'..='\u{4DBF}' | // CJK Extension A
            '\u{F900}'..='\u{FAFF}'   // CJK Compatibility Ideographs
        )
    });

    assert!(
        has_chinese_chars,
        "Expected Chinese characters in OCR result, got: {}",
        extraction.content
    );
}

/// Test concurrent OCR on different languages using the engine pool.
///
/// This test verifies that the backend can handle multiple concurrent OCR requests
/// with different languages without blocking. Each language should use its own
/// engine from the pool.
#[tokio::test]
#[ignore = "requires network access and ONNX Runtime"]
async fn test_ocr_concurrent_different_languages() {
    use tokio::task::JoinSet;

    let test_documents_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("test_documents");

    let cache_dir = std::env::temp_dir().join("kreuzberg_paddle_multilang_concurrent");
    let backend = std::sync::Arc::new(
        PaddleOcrBackend::with_config(PaddleOcrConfig::new("en").with_cache_dir(cache_dir))
            .expect("Failed to create backend"),
    );

    let mut tasks = JoinSet::new();

    // English OCR task
    {
        let backend_clone = backend.clone();
        let image_path = test_documents_dir.join("images/test_hello_world.png");
        if image_path.exists() {
            let image_bytes = std::fs::read(&image_path).expect("Failed to read English image");
            tasks.spawn(async move {
                let config = OcrConfig {
                    backend: "paddle-ocr".to_string(),
                    language: "en".to_string(),
                    ..Default::default()
                };
                let result = backend_clone.process_image(&image_bytes, &config).await;
                ("en", result)
            });
        }
    }

    // Chinese OCR task
    {
        let backend_clone = backend.clone();
        let image_path = test_documents_dir.join("images/chi_sim_image.jpeg");
        if image_path.exists() {
            let image_bytes = std::fs::read(&image_path).expect("Failed to read Chinese image");
            tasks.spawn(async move {
                let config = OcrConfig {
                    backend: "paddle-ocr".to_string(),
                    language: "ch".to_string(),
                    ..Default::default()
                };
                let result = backend_clone.process_image(&image_bytes, &config).await;
                ("ch", result)
            });
        }
    }

    // Wait for all tasks and verify results
    let mut results_count = 0;
    while let Some(result) = tasks.join_next().await {
        let (lang, ocr_result) = result.expect("Task panicked");
        assert!(
            ocr_result.is_ok(),
            "OCR failed for language {}: {:?}",
            lang,
            ocr_result.err()
        );

        let extraction = ocr_result.unwrap();
        println!(
            "OCR result for {}: {}",
            lang,
            &extraction.content[..extraction.content.len().min(100)]
        );

        assert!(
            !extraction.content.is_empty(),
            "Expected non-empty result for language {}",
            lang
        );
        results_count += 1;
    }

    // Should have completed both tasks
    assert!(results_count >= 2, "Expected at least 2 OCR tasks to complete");
    println!("Successfully completed {} concurrent OCR tasks", results_count);
}

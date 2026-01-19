//! Main extraction entry points.
//!
//! This module provides the primary API for extracting content from files and byte arrays.
//! It orchestrates the entire extraction pipeline: cache checking, MIME detection,
//! extractor selection, extraction, post-processing, and cache storage.
//!
//! # Functions
//!
//! - [`extract_file`] - Extract content from a file path
//! - [`extract_bytes`] - Extract content from a byte array
//! - [`batch_extract_file`] - Extract content from multiple files concurrently
//! - [`batch_extract_bytes`] - Extract content from multiple byte arrays concurrently

mod bytes;
mod file;
mod helpers;
mod legacy;
mod sync;

#[cfg(feature = "tokio-runtime")]
mod batch;

// Re-export public API
pub use bytes::extract_bytes;
pub use file::extract_file;
pub use helpers::get_pool_sizing_hint;
pub use sync::{extract_bytes_sync, extract_file_sync};

#[cfg(feature = "tokio-runtime")]
pub use batch::{batch_extract_bytes, batch_extract_file};
#[cfg(feature = "tokio-runtime")]
pub use sync::{batch_extract_bytes_sync, batch_extract_file_sync};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::ExtractionConfig;
    use serial_test::serial;
    use std::fs::File;
    use std::io::Write;
    use std::sync::Arc;
    use tempfile::tempdir;

    fn assert_text_content(actual: &str, expected: &str) {
        assert_eq!(actual.trim_end_matches('\n'), expected);
    }

    #[tokio::test]
    async fn test_extract_file_basic() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Hello, world!").unwrap();

        let config = ExtractionConfig::default();
        let result = extract_file(&file_path, None, &config).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_text_content(&result.content, "Hello, world!");
        assert_eq!(result.mime_type, "text/plain");
    }

    #[tokio::test]
    async fn test_extract_file_with_mime_override() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.dat");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();

        let config = ExtractionConfig::default();
        let result = extract_file(&file_path, Some("text/plain"), &config).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.mime_type, "text/plain");
    }

    #[tokio::test]
    async fn test_extract_file_nonexistent() {
        let config = ExtractionConfig::default();
        let result = extract_file("/nonexistent/file.txt", None, &config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_extract_bytes_basic() {
        let config = ExtractionConfig::default();
        let result = extract_bytes(b"test content", "text/plain", &config).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_text_content(&result.content, "test content");
        assert_eq!(result.mime_type, "text/plain");
    }

    #[tokio::test]
    async fn test_extract_bytes_invalid_mime() {
        let config = ExtractionConfig::default();
        let result = extract_bytes(b"test", "invalid/mime", &config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_batch_extract_file() {
        let dir = tempdir().unwrap();

        let file1 = dir.path().join("test1.txt");
        let file2 = dir.path().join("test2.txt");

        File::create(&file1).unwrap().write_all(b"content 1").unwrap();
        File::create(&file2).unwrap().write_all(b"content 2").unwrap();

        let config = ExtractionConfig::default();
        let paths = vec![file1, file2];
        let results = batch_extract_file(paths, &config).await;

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
        assert_text_content(&results[0].content, "content 1");
        assert_text_content(&results[1].content, "content 2");
    }

    #[tokio::test]
    async fn test_batch_extract_file_empty() {
        let config = ExtractionConfig::default();
        let paths: Vec<std::path::PathBuf> = vec![];
        let results = batch_extract_file(paths, &config).await;

        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_batch_extract_bytes() {
        let config = ExtractionConfig::default();
        let contents = vec![
            (b"content 1".as_slice(), "text/plain"),
            (b"content 2".as_slice(), "text/plain"),
        ];
        let owned_contents: Vec<(Vec<u8>, String)> = contents
            .into_iter()
            .map(|(bytes, mime)| (bytes.to_vec(), mime.to_string()))
            .collect();
        let results = batch_extract_bytes(owned_contents, &config).await;

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
        assert_text_content(&results[0].content, "content 1");
        assert_text_content(&results[1].content, "content 2");
    }

    #[test]
    fn test_sync_wrappers() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap().write_all(b"sync test").unwrap();

        let config = ExtractionConfig::default();

        let result = extract_file_sync(&file_path, None, &config);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_text_content(&result.content, "sync test");

        let result = extract_bytes_sync(b"test", "text/plain", &config);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_extractor_cache() {
        let config = ExtractionConfig::default();

        let result1 = extract_bytes(b"test 1", "text/plain", &config).await;
        assert!(result1.is_ok());
        let result1 = result1.unwrap();

        let result2 = extract_bytes(b"test 2", "text/plain", &config).await;
        assert!(result2.is_ok());
        let result2 = result2.unwrap();

        assert_text_content(&result1.content, "test 1");
        assert_text_content(&result2.content, "test 2");

        let result3 = extract_bytes(b"# test 3", "text/markdown", &config).await;
        assert!(result3.is_ok());
    }

    #[tokio::test]
    async fn test_extract_file_empty() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.txt");
        File::create(&file_path).unwrap();

        let config = ExtractionConfig::default();
        let result = extract_file(&file_path, None, &config).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.content, "");
    }

    #[tokio::test]
    async fn test_extract_bytes_empty() {
        let config = ExtractionConfig::default();
        let result = extract_bytes(b"", "text/plain", &config).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.content, "");
    }

    #[tokio::test]
    async fn test_extract_file_whitespace_only() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("whitespace.txt");
        File::create(&file_path).unwrap().write_all(b"   \n\t  \n  ").unwrap();

        let config = ExtractionConfig::default();
        let result = extract_file(&file_path, None, &config).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_extract_file_very_long_path() {
        let dir = tempdir().unwrap();
        let long_name = "a".repeat(200);
        let file_path = dir.path().join(format!("{}.txt", long_name));

        if let Ok(mut f) = File::create(&file_path) {
            f.write_all(b"content").unwrap();
            let config = ExtractionConfig::default();
            let result = extract_file(&file_path, None, &config).await;
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[tokio::test]
    async fn test_extract_file_special_characters_in_path() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test with spaces & symbols!.txt");
        File::create(&file_path).unwrap().write_all(b"content").unwrap();

        let config = ExtractionConfig::default();
        let result = extract_file(&file_path, None, &config).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_text_content(&result.content, "content");
    }

    #[tokio::test]
    async fn test_extract_file_unicode_filename() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("测试文件名.txt");
        File::create(&file_path).unwrap().write_all(b"content").unwrap();

        let config = ExtractionConfig::default();
        let result = extract_file(&file_path, None, &config).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_extract_bytes_unsupported_mime() {
        let config = ExtractionConfig::default();
        let result = extract_bytes(b"test", "application/x-unknown-format", &config).await;

        assert!(result.is_err());
        use crate::KreuzbergError;
        assert!(matches!(result.unwrap_err(), KreuzbergError::UnsupportedFormat(_)));
    }

    #[tokio::test]
    async fn test_batch_extract_file_with_errors() {
        let dir = tempdir().unwrap();

        let valid_file = dir.path().join("valid.txt");
        File::create(&valid_file).unwrap().write_all(b"valid content").unwrap();

        let invalid_file = dir.path().join("nonexistent.txt");

        let config = ExtractionConfig::default();
        let paths = vec![valid_file, invalid_file];
        let results = batch_extract_file(paths, &config).await;

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
        assert_text_content(&results[0].content, "valid content");
        assert!(results[1].metadata.error.is_some());
    }

    #[tokio::test]
    async fn test_batch_extract_bytes_mixed_valid_invalid() {
        let config = ExtractionConfig::default();
        let contents = vec![
            (b"valid 1".as_slice(), "text/plain"),
            (b"invalid".as_slice(), "invalid/mime"),
            (b"valid 2".as_slice(), "text/plain"),
        ];
        let owned_contents: Vec<(Vec<u8>, String)> = contents
            .into_iter()
            .map(|(bytes, mime)| (bytes.to_vec(), mime.to_string()))
            .collect();
        let results = batch_extract_bytes(owned_contents, &config).await;

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 3);
        assert_text_content(&results[0].content, "valid 1");
        assert!(results[1].metadata.error.is_some());
        assert_text_content(&results[2].content, "valid 2");
    }

    #[tokio::test]
    async fn test_batch_extract_bytes_all_invalid() {
        let config = ExtractionConfig::default();
        let contents = vec![
            (b"test 1".as_slice(), "invalid/mime1"),
            (b"test 2".as_slice(), "invalid/mime2"),
        ];
        let owned_contents: Vec<(Vec<u8>, String)> = contents
            .into_iter()
            .map(|(bytes, mime)| (bytes.to_vec(), mime.to_string()))
            .collect();
        let results = batch_extract_bytes(owned_contents, &config).await;

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
        assert!(results[0].metadata.error.is_some());
        assert!(results[1].metadata.error.is_some());
    }

    #[tokio::test]
    async fn test_extract_bytes_very_large() {
        let large_content = vec![b'a'; 10_000_000];
        let config = ExtractionConfig::default();
        let result = extract_bytes(&large_content, "text/plain", &config).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        let trimmed_len = result.content.trim_end_matches('\n').len();
        assert_eq!(trimmed_len, 10_000_000);
    }

    #[tokio::test]
    async fn test_batch_extract_large_count() {
        let dir = tempdir().unwrap();
        let mut paths = Vec::new();

        for i in 0..100 {
            let file_path = dir.path().join(format!("file{}.txt", i));
            File::create(&file_path)
                .unwrap()
                .write_all(format!("content {}", i).as_bytes())
                .unwrap();
            paths.push(file_path);
        }

        let config = ExtractionConfig::default();
        let results = batch_extract_file(paths, &config).await;

        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 100);

        for (i, result) in results.iter().enumerate() {
            assert_text_content(&result.content, &format!("content {}", i));
        }
    }

    #[tokio::test]
    async fn test_extract_file_mime_detection_fallback() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("testfile");
        File::create(&file_path)
            .unwrap()
            .write_all(b"plain text content")
            .unwrap();

        let config = ExtractionConfig::default();
        let result = extract_file(&file_path, None, &config).await;

        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_extract_file_wrong_mime_override() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap().write_all(b"plain text").unwrap();

        let config = ExtractionConfig::default();
        let result = extract_file(&file_path, Some("application/pdf"), &config).await;

        assert!(result.is_err() || result.is_ok());
    }

    #[test]
    fn test_sync_wrapper_nonexistent_file() {
        let config = ExtractionConfig::default();
        let result = extract_file_sync("/nonexistent/path.txt", None, &config);

        assert!(result.is_err());
        use crate::KreuzbergError;
        assert!(matches!(result.unwrap_err(), KreuzbergError::Validation { .. }));
    }

    #[test]
    fn test_sync_wrapper_batch_empty() {
        let config = ExtractionConfig::default();
        let paths: Vec<std::path::PathBuf> = vec![];
        let results = batch_extract_file_sync(paths, &config);

        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 0);
    }

    #[test]
    fn test_sync_wrapper_batch_bytes_empty() {
        let config = ExtractionConfig::default();
        let contents: Vec<(Vec<u8>, String)> = vec![];
        let results = batch_extract_bytes_sync(contents, &config);

        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_concurrent_extractions_same_mime() {
        use tokio::task::JoinSet;

        let config = Arc::new(ExtractionConfig::default());
        let mut tasks = JoinSet::new();

        for i in 0..50 {
            let config_clone = Arc::clone(&config);
            tasks.spawn(async move {
                let content = format!("test content {}", i);
                extract_bytes(content.as_bytes(), "text/plain", &config_clone).await
            });
        }

        let mut success_count = 0;
        while let Some(task_result) = tasks.join_next().await {
            if let Ok(Ok(_)) = task_result {
                success_count += 1;
            }
        }

        assert_eq!(success_count, 50);
    }

    #[serial]
    #[tokio::test]
    async fn test_concurrent_extractions_different_mimes() {
        use tokio::task::JoinSet;

        let config = Arc::new(ExtractionConfig::default());
        let mut tasks = JoinSet::new();

        let mime_types = ["text/plain", "text/markdown"];

        for i in 0..30 {
            let config_clone = Arc::clone(&config);
            let mime = mime_types[i % mime_types.len()];
            tasks.spawn(async move {
                let content = format!("test {}", i);
                extract_bytes(content.as_bytes(), mime, &config_clone).await
            });
        }

        let mut success_count = 0;
        while let Some(task_result) = tasks.join_next().await {
            if let Ok(Ok(_)) = task_result {
                success_count += 1;
            }
        }

        assert_eq!(success_count, 30);
    }
}

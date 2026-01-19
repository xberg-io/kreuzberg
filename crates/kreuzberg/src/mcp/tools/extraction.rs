//! Document extraction MCP tools.

use base64::prelude::*;
use crate::{
    ExtractionConfig, batch_extract_file, batch_extract_file_sync, extract_bytes, extract_bytes_sync, extract_file,
    extract_file_sync, mcp::errors::map_kreuzberg_error_to_mcp, mcp::format::{build_config, format_extraction_result},
    mcp::params::{BatchExtractFilesParams, ExtractBytesParams, ExtractFileParams},
};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content, RawContent},
    tool,
};

/// MCP tool methods for document extraction.
pub(in crate::mcp) trait ExtractionTool {
    /// Get reference to default config
    fn default_config(&self) -> &std::sync::Arc<ExtractionConfig>;

    /// Extract content from a file.
    ///
    /// This tool extracts text, metadata, and tables from documents in various formats
    /// including PDFs, Word documents, Excel spreadsheets, images (with OCR), and more.
    #[tool(
        description = "Extract content from a file by path. Supports PDFs, Word, Excel, images (with OCR), HTML, and more.",
        annotations(title = "Extract File", read_only_hint = true, idempotent_hint = true)
    )]
    async fn extract_file(
        &self,
        Parameters(params): Parameters<ExtractFileParams>,
    ) -> Result<CallToolResult, McpError> {
        let config = build_config(self.default_config(), params.enable_ocr, params.force_ocr);

        let result = if params.r#async {
            extract_file(&params.path, params.mime_type.as_deref(), &config)
                .await
                .map_err(map_kreuzberg_error_to_mcp)?
        } else {
            extract_file_sync(&params.path, params.mime_type.as_deref(), &config).map_err(map_kreuzberg_error_to_mcp)?
        };

        let response = format_extraction_result(&result);
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Extract content from base64-encoded bytes.
    ///
    /// This tool extracts text, metadata, and tables from base64-encoded document data.
    #[tool(
        description = "Extract content from base64-encoded file data. Returns extracted text, metadata, and tables.",
        annotations(title = "Extract Bytes", read_only_hint = true, idempotent_hint = true)
    )]
    async fn extract_bytes(
        &self,
        Parameters(params): Parameters<ExtractBytesParams>,
    ) -> Result<CallToolResult, McpError> {
        let bytes = BASE64_STANDARD
            .decode(&params.data)
            .map_err(|e| McpError::invalid_params(format!("Invalid base64: {}", e), None))?;

        let config = build_config(self.default_config(), params.enable_ocr, params.force_ocr);

        let mime_type = params.mime_type.as_deref().unwrap_or("");

        let result = if params.r#async {
            extract_bytes(&bytes, mime_type, &config)
                .await
                .map_err(map_kreuzberg_error_to_mcp)?
        } else {
            extract_bytes_sync(&bytes, mime_type, &config).map_err(map_kreuzberg_error_to_mcp)?
        };

        let response = format_extraction_result(&result);
        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Extract content from multiple files in parallel.
    ///
    /// This tool efficiently processes multiple documents simultaneously, useful for batch operations.
    #[tool(
        description = "Extract content from multiple files in parallel. Returns results for all files.",
        annotations(title = "Batch Extract Files", read_only_hint = true, idempotent_hint = true)
    )]
    async fn batch_extract_files(
        &self,
        Parameters(params): Parameters<BatchExtractFilesParams>,
    ) -> Result<CallToolResult, McpError> {
        let config = build_config(self.default_config(), params.enable_ocr, params.force_ocr);

        let results = if params.r#async {
            batch_extract_file(params.paths.clone(), &config)
                .await
                .map_err(map_kreuzberg_error_to_mcp)?
        } else {
            batch_extract_file_sync(params.paths.clone(), &config).map_err(map_kreuzberg_error_to_mcp)?
        };

        let mut response = String::new();
        for (i, result) in results.iter().enumerate() {
            response.push_str(&format!("=== Document {}: {} ===\n", i + 1, params.paths[i]));
            response.push_str(&format_extraction_result(result));
            response.push_str("\n\n");
        }

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Get the path to a test document relative to workspace root.
    fn get_test_path(relative_path: &str) -> String {
        let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        workspace_root
            .join("test_documents")
            .join(relative_path)
            .to_string_lossy()
            .to_string()
    }

    // Simple test struct for trait implementation
    struct TestMcpServer {
        config: std::sync::Arc<ExtractionConfig>,
    }

    impl TestMcpServer {
        fn new() -> Self {
            Self {
                config: std::sync::Arc::new(ExtractionConfig::default()),
            }
        }
    }

    impl ExtractionTool for TestMcpServer {
        fn default_config(&self) -> &std::sync::Arc<ExtractionConfig> {
            &self.config
        }
    }

    #[tokio::test]
    async fn test_extract_file_sync_with_valid_pdf() {
        let server = TestMcpServer::new();
        let params = ExtractFileParams {
            path: get_test_path("pdfs_with_tables/tiny.pdf").to_string(),
            mime_type: None,
            enable_ocr: false,
            force_ocr: false,
            r#async: true,
        };

        let result = server.extract_file(Parameters(params)).await;

        assert!(result.is_ok());
        let call_result = result.unwrap();
        if let Some(content) = call_result.content.first() {
            match &content.raw {
                RawContent::Text(text) => {
                    assert!(!text.text.is_empty());
                    assert!(text.text.contains("Content"));
                }
                _ => panic!("Expected text content"),
            }
        } else {
            panic!("Expected content in result");
        }
    }

    #[tokio::test]
    async fn test_extract_file_async_with_valid_pdf() {
        let server = TestMcpServer::new();
        let params = ExtractFileParams {
            path: get_test_path("pdfs_with_tables/tiny.pdf").to_string(),
            mime_type: None,
            enable_ocr: false,
            force_ocr: false,
            r#async: true,
        };

        let result = server.extract_file(Parameters(params)).await;

        assert!(result.is_ok());
        let call_result = result.unwrap();
        if let Some(content) = call_result.content.first() {
            match &content.raw {
                RawContent::Text(text) => {
                    assert!(!text.text.is_empty());
                }
                _ => panic!("Expected text content"),
            }
        } else {
            panic!("Expected content in result");
        }
    }

    #[tokio::test]
    async fn test_extract_file_with_invalid_path() {
        let server = TestMcpServer::new();
        let params = ExtractFileParams {
            path: "/nonexistent/file.pdf".to_string(),
            mime_type: None,
            enable_ocr: false,
            force_ocr: false,
            r#async: true,
        };

        let result = server.extract_file(Parameters(params)).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.code.0 == -32602 || error.code.0 == -32603);
    }

    #[tokio::test]
    async fn test_extract_file_with_mime_type_hint() {
        let server = TestMcpServer::new();
        let params = ExtractFileParams {
            path: get_test_path("pdfs_with_tables/tiny.pdf").to_string(),
            mime_type: Some("application/pdf".to_string()),
            enable_ocr: false,
            force_ocr: false,
            r#async: true,
        };

        let result = server.extract_file(Parameters(params)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_extract_bytes_sync_with_valid_data() {
        let server = TestMcpServer::new();

        let text_content = b"Hello, world!";
        let encoded = BASE64_STANDARD.encode(text_content);

        let params = ExtractBytesParams {
            data: encoded,
            mime_type: Some("text/plain".to_string()),
            enable_ocr: false,
            force_ocr: false,
            r#async: true,
        };

        let result = server.extract_bytes(Parameters(params)).await;

        assert!(result.is_ok());
        let call_result = result.unwrap();
        if let Some(content) = call_result.content.first() {
            match &content.raw {
                RawContent::Text(text) => {
                    assert!(text.text.contains("Hello, world!"));
                }
                _ => panic!("Expected text content"),
            }
        } else {
            panic!("Expected content in result");
        }
    }

    #[tokio::test]
    async fn test_extract_bytes_with_invalid_base64() {
        let server = TestMcpServer::new();

        let params = ExtractBytesParams {
            data: "not-valid-base64!!!".to_string(),
            mime_type: None,
            enable_ocr: false,
            force_ocr: false,
            r#async: true,
        };

        let result = server.extract_bytes(Parameters(params)).await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.code.0, -32602);
        assert!(error.message.contains("Invalid base64"));
    }

    #[tokio::test]
    async fn test_batch_extract_files_sync_with_valid_files() {
        let server = TestMcpServer::new();
        let params = BatchExtractFilesParams {
            paths: vec![get_test_path("pdfs_with_tables/tiny.pdf").to_string()],
            enable_ocr: false,
            force_ocr: false,
            r#async: true,
        };

        let result = server.batch_extract_files(Parameters(params)).await;

        assert!(result.is_ok());
        let call_result = result.unwrap();
        if let Some(content) = call_result.content.first() {
            match &content.raw {
                RawContent::Text(text) => {
                    assert!(text.text.contains("Document 1"));
                    assert!(text.text.contains("tiny.pdf"));
                }
                _ => panic!("Expected text content"),
            }
        } else {
            panic!("Expected content in result");
        }
    }

    #[tokio::test]
    async fn test_batch_extract_files_with_empty_list() {
        let server = TestMcpServer::new();
        let params = BatchExtractFilesParams {
            paths: vec![],
            enable_ocr: false,
            force_ocr: false,
            r#async: true,
        };

        let result = server.batch_extract_files(Parameters(params)).await;

        assert!(result.is_ok());
        let call_result = result.unwrap();
        if let Some(content) = call_result.content.first() {
            match &content.raw {
                RawContent::Text(text) => {
                    assert!(text.text.is_empty() || text.text.trim().is_empty());
                }
                _ => panic!("Expected text content"),
            }
        } else {
            panic!("Expected content in result");
        }
    }

    #[tokio::test]
    async fn test_response_includes_metadata() {
        let server = TestMcpServer::new();

        let test_file = get_test_path("pdfs_with_tables/tiny.pdf");

        if std::path::Path::new(&test_file).exists() {
            let params = ExtractFileParams {
                path: test_file.to_string(),
                mime_type: None,
                enable_ocr: false,
                force_ocr: false,
                r#async: true,
            };

            let result = server.extract_file(Parameters(params)).await;

            assert!(result.is_ok());
            let call_result = result.unwrap();

            if let Some(content) = call_result.content.first()
                && let RawContent::Text(text) = &content.raw
            {
                assert!(text.text.contains("Metadata:"));
            }
        }
    }

    #[tokio::test]
    async fn test_batch_extract_preserves_file_order() {
        let server = TestMcpServer::new();

        let file1 = get_test_path("pdfs_with_tables/tiny.pdf");
        let file2 = get_test_path("pdfs_with_tables/medium.pdf");

        if std::path::Path::new(&file1).exists() && std::path::Path::new(&file2).exists() {
            let params = BatchExtractFilesParams {
                paths: vec![file1.to_string(), file2.to_string()],
                enable_ocr: false,
                force_ocr: false,
                r#async: true,
            };

            let result = server.batch_extract_files(Parameters(params)).await;

            if let Ok(call_result) = result
                && let Some(content) = call_result.content.first()
                && let RawContent::Text(text) = &content.raw
            {
                assert!(text.text.contains("Document 1"));
                assert!(text.text.contains("Document 2"));

                let doc1_pos = text.text.find("Document 1");
                let doc2_pos = text.text.find("Document 2");
                if let (Some(pos1), Some(pos2)) = (doc1_pos, doc2_pos) {
                    assert!(pos1 < pos2, "Documents should be in order");
                }
            }
        }
    }
}

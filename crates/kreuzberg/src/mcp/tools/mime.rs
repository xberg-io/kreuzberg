//! MIME type detection MCP tool.

use crate::{detect_mime_type, mcp::errors::map_kreuzberg_error_to_mcp, mcp::params::DetectMimeTypeParams};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content, RawContent},
    tool,
};

/// MCP tool methods for MIME type detection.
pub(in crate::mcp) trait MimeTypeTool {
    /// Detect the MIME type of a file.
    ///
    /// This tool identifies the file format, useful for determining which extractor to use.
    #[tool(
        description = "Detect the MIME type of a file. Returns the detected MIME type string.",
        annotations(title = "Detect MIME Type", read_only_hint = true, idempotent_hint = true)
    )]
    fn detect_mime_type(
        &self,
        Parameters(params): Parameters<DetectMimeTypeParams>,
    ) -> Result<CallToolResult, McpError> {
        let mime_type = detect_mime_type(&params.path, params.use_content).map_err(map_kreuzberg_error_to_mcp)?;

        Ok(CallToolResult::success(vec![Content::text(mime_type)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ExtractionConfig;
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
    struct TestMcpServer;

    impl MimeTypeTool for TestMcpServer {}

    #[tokio::test]
    async fn test_detect_mime_type_with_valid_file() {
        let server = TestMcpServer;
        let params = DetectMimeTypeParams {
            path: get_test_path("pdfs_with_tables/tiny.pdf").to_string(),
            use_content: true,
        };

        let result = server.detect_mime_type(Parameters(params));

        assert!(result.is_ok());
        let call_result = result.unwrap();
        if let Some(content) = call_result.content.first() {
            match &content.raw {
                RawContent::Text(text) => {
                    assert!(text.text.contains("application/pdf") || text.text.contains("pdf"));
                }
                _ => panic!("Expected text content"),
            }
        } else {
            panic!("Expected content in result");
        }
    }

    #[tokio::test]
    async fn test_detect_mime_type_without_content_detection() {
        let server = TestMcpServer;
        let params = DetectMimeTypeParams {
            path: get_test_path("pdfs_with_tables/tiny.pdf").to_string(),
            use_content: false,
        };

        let result = server.detect_mime_type(Parameters(params));

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_detect_mime_type_with_invalid_file() {
        let server = TestMcpServer;
        let params = DetectMimeTypeParams {
            path: "/nonexistent/file.pdf".to_string(),
            use_content: true,
        };

        let result = server.detect_mime_type(Parameters(params));

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.code.0 == -32602 || error.code.0 == -32603);
    }

    #[tokio::test]
    async fn test_detect_mime_type_with_extension_only() {
        let server = TestMcpServer;

        let test_file = get_test_path("pdfs_with_tables/tiny.pdf");

        if std::path::Path::new(&test_file).exists() {
            let params = DetectMimeTypeParams {
                path: test_file.to_string(),
                use_content: false,
            };

            let result = server.detect_mime_type(Parameters(params));

            assert!(result.is_ok());
            let call_result = result.unwrap();
            if let Some(content) = call_result.content.first()
                && let RawContent::Text(text) = &content.raw
            {
                assert!(text.text.contains("pdf") || text.text.contains("PDF"));
            }
        }
    }

    #[tokio::test]
    async fn test_detect_mime_type_with_content_analysis() {
        let server = TestMcpServer;

        let test_file = get_test_path("pdfs_with_tables/tiny.pdf");

        if std::path::Path::new(&test_file).exists() {
            let params = DetectMimeTypeParams {
                path: test_file.to_string(),
                use_content: true,
            };

            let result = server.detect_mime_type(Parameters(params));

            assert!(result.is_ok());
        }
    }
}

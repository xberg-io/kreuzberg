//! Cache management MCP tools.

use crate::{cache, mcp::errors::map_kreuzberg_error_to_mcp};
use rmcp::{
    ErrorData as McpError,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content, RawContent},
    tool,
};

/// MCP tool methods for cache management.
pub(in crate::mcp) trait CacheTool {
    /// Get cache statistics.
    ///
    /// This tool returns statistics about the cache including total files, size, and disk space.
    #[tool(
        description = "Get cache statistics including total files, size, and available disk space.",
        annotations(title = "Cache Stats", read_only_hint = true, idempotent_hint = true)
    )]
    fn cache_stats(&self, Parameters(_): Parameters<()>) -> Result<CallToolResult, McpError> {
        let cache_dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join(".kreuzberg");

        let stats = cache::get_cache_metadata(cache_dir.to_str().unwrap_or(".")).map_err(map_kreuzberg_error_to_mcp)?;

        let response = format!(
            "Cache Statistics\n\
             ================\n\
             Directory: {}\n\
             Total files: {}\n\
             Total size: {:.2} MB\n\
             Available space: {:.2} MB\n\
             Oldest file age: {:.2} days\n\
             Newest file age: {:.2} days",
            cache_dir.to_string_lossy(),
            stats.total_files,
            stats.total_size_mb,
            stats.available_space_mb,
            stats.oldest_file_age_days,
            stats.newest_file_age_days
        );

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }

    /// Clear the cache.
    ///
    /// This tool removes all cached files and returns the number of files removed and space freed.
    #[tool(
        description = "Clear all cached files. Returns the number of files removed and space freed in MB.",
        annotations(title = "Clear Cache", destructive_hint = true)
    )]
    fn cache_clear(&self, Parameters(_): Parameters<()>) -> Result<CallToolResult, McpError> {
        let cache_dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join(".kreuzberg");

        let (removed_files, freed_mb) =
            cache::clear_cache_directory(cache_dir.to_str().unwrap_or(".")).map_err(map_kreuzberg_error_to_mcp)?;

        let response = format!(
            "Cache cleared successfully\n\
             Directory: {}\n\
             Removed files: {}\n\
             Freed space: {:.2} MB",
            cache_dir.to_string_lossy(),
            removed_files,
            freed_mb
        );

        Ok(CallToolResult::success(vec![Content::text(response)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test struct for trait implementation
    struct TestMcpServer;

    impl CacheTool for TestMcpServer {}

    #[tokio::test]
    async fn test_cache_stats_returns_statistics() {
        let server = TestMcpServer;

        let result = server.cache_stats(Parameters(()));

        assert!(result.is_ok());
        let call_result = result.unwrap();
        if let Some(content) = call_result.content.first() {
            match &content.raw {
                RawContent::Text(text) => {
                    assert!(text.text.contains("Cache Statistics"));
                    assert!(text.text.contains("Directory:"));
                    assert!(text.text.contains("Total files:"));
                    assert!(text.text.contains("Total size:"));
                    assert!(text.text.contains("Available space:"));
                }
                _ => panic!("Expected text content"),
            }
        } else {
            panic!("Expected content in result");
        }
    }

    #[tokio::test]
    async fn test_cache_clear_returns_result() {
        let server = TestMcpServer;

        let result = server.cache_clear(Parameters(()));

        assert!(result.is_ok());
        let call_result = result.unwrap();
        if let Some(content) = call_result.content.first() {
            match &content.raw {
                RawContent::Text(text) => {
                    assert!(text.text.contains("Cache cleared"));
                    assert!(text.text.contains("Directory:"));
                    assert!(text.text.contains("Removed files:"));
                    assert!(text.text.contains("Freed space:"));
                }
                _ => panic!("Expected text content"),
            }
        } else {
            panic!("Expected content in result");
        }
    }

    #[tokio::test]
    async fn test_cache_clear_is_idempotent() {
        let server = TestMcpServer;

        let result1 = server.cache_clear(Parameters(()));
        assert!(result1.is_ok());

        let result2 = server.cache_clear(Parameters(()));
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_cache_clear_returns_metrics() {
        let server = TestMcpServer;

        let result = server.cache_clear(Parameters(()));

        assert!(result.is_ok());
        let call_result = result.unwrap();
        if let Some(content) = call_result.content.first()
            && let RawContent::Text(text) = &content.raw
        {
            assert!(text.text.contains("Removed files:"));
            assert!(text.text.contains("Freed space:"));
        }
    }

    #[tokio::test]
    async fn test_cache_stats_returns_valid_data() {
        let server = TestMcpServer;

        let result = server.cache_stats(Parameters(()));

        assert!(result.is_ok());
        let call_result = result.unwrap();
        if let Some(content) = call_result.content.first()
            && let RawContent::Text(text) = &content.raw
        {
            assert!(text.text.contains("Cache Statistics"));
            assert!(text.text.contains("Directory:"));
            assert!(text.text.contains("Total files:"));
            assert!(text.text.contains("Total size:"));
            assert!(text.text.contains("Available space:"));
            assert!(text.text.contains("Oldest file age:"));
            assert!(text.text.contains("Newest file age:"));
        }
    }
}

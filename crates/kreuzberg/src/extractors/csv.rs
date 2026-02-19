//! CSV and TSV extractor.
//!
//! Parses CSV/TSV files into structured table data and clean text output.
//! Handles RFC 4180 quoted fields with embedded commas and newlines.

use crate::Result;
use crate::core::config::ExtractionConfig;
use crate::plugins::{DocumentExtractor, Plugin};
use crate::types::{ExtractionResult, Metadata, Table};
use async_trait::async_trait;

/// CSV/TSV extractor with proper field parsing.
///
/// Replaces raw text passthrough with structured CSV parsing,
/// producing space-separated text output and populated `tables` field.
pub struct CsvExtractor;

impl CsvExtractor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CsvExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for CsvExtractor {
    fn name(&self) -> &str {
        "csv-extractor"
    }

    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    fn initialize(&self) -> Result<()> {
        Ok(())
    }

    fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    fn description(&self) -> &str {
        "CSV/TSV text extraction with table structure"
    }

    fn author(&self) -> &str {
        "Kreuzberg Team"
    }
}

#[async_trait]
impl DocumentExtractor for CsvExtractor {
    async fn extract_bytes(
        &self,
        content: &[u8],
        mime_type: &str,
        _config: &ExtractionConfig,
    ) -> Result<ExtractionResult> {
        let text = String::from_utf8_lossy(content);
        let delimiter = if mime_type == "text/tab-separated-values" {
            '\t'
        } else {
            detect_delimiter(&text)
        };

        let rows = parse_csv(&text, delimiter);

        // Build space-separated text (each row on its own line, cells separated by spaces)
        let content_text = rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.trim())
                    .filter(|cell| !cell.is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        // Build markdown table
        let markdown = build_markdown_table(&rows);

        let table = Table {
            cells: rows.clone(),
            markdown,
            page_number: 1,
            bounding_box: None,
        };

        let row_count = rows.len();
        let col_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);

        let mut additional = ahash::AHashMap::new();
        additional.insert(
            std::borrow::Cow::Borrowed("row_count"),
            serde_json::Value::Number(row_count.into()),
        );
        additional.insert(
            std::borrow::Cow::Borrowed("column_count"),
            serde_json::Value::Number(col_count.into()),
        );
        additional.insert(
            std::borrow::Cow::Borrowed("extraction_method"),
            serde_json::Value::String("native_csv".to_string()),
        );

        Ok(ExtractionResult {
            content: content_text,
            mime_type: mime_type.to_string().into(),
            metadata: Metadata {
                additional,
                ..Default::default()
            },
            pages: None,
            tables: vec![table],
            detected_languages: None,
            chunks: None,
            images: None,
            djot_content: None,
            elements: None,
            ocr_elements: None,
            document: None,
            #[cfg(any(feature = "keywords-yake", feature = "keywords-rake"))]
            extracted_keywords: None,
            quality_score: None,
            processing_warnings: Vec::new(),
            annotations: None,
        })
    }

    fn supported_mime_types(&self) -> &[&str] {
        &["text/csv", "text/tab-separated-values"]
    }

    fn priority(&self) -> i32 {
        60 // Higher than PlainTextExtractor (50) to take precedence
    }
}

/// Auto-detect CSV delimiter using consistency-based approach.
/// Tests each candidate delimiter and picks the one producing the most
/// consistent column count across sample lines.
fn detect_delimiter(text: &str) -> char {
    const CANDIDATES: &[char] = &[',', '\t', '|', ';'];
    let mut best_delimiter = ',';
    let mut best_score = 0usize;

    for &candidate in CANDIDATES {
        let sample: String = text.lines().take(10).collect::<Vec<_>>().join("\n");
        let rows = parse_csv(&sample, candidate);
        if rows.len() < 2 {
            continue;
        }
        let col_counts: Vec<usize> = rows.iter().map(|r| r.len()).collect();
        let first_count = col_counts[0];
        if first_count <= 1 {
            continue;
        }
        let consistent_rows = col_counts.iter().filter(|&&c| c == first_count).count();
        let score = consistent_rows * first_count;
        if score > best_score {
            best_score = score;
            best_delimiter = candidate;
        }
    }
    best_delimiter
}

/// Parse CSV text into rows of fields, handling RFC 4180 quoted fields.
fn parse_csv(text: &str, delimiter: char) -> Vec<Vec<String>> {
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                // Check for escaped quote ("")
                if chars.peek() == Some(&'"') {
                    current_field.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                current_field.push(c);
            }
        } else {
            match c {
                '"' => {
                    in_quotes = true;
                }
                c if c == delimiter => {
                    current_row.push(current_field.clone());
                    current_field.clear();
                }
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                    current_row.push(current_field.clone());
                    current_field.clear();
                    if !current_row.iter().all(|f| f.is_empty()) {
                        rows.push(current_row);
                    }
                    current_row = Vec::new();
                }
                '\n' => {
                    current_row.push(current_field.clone());
                    current_field.clear();
                    if !current_row.iter().all(|f| f.is_empty()) {
                        rows.push(current_row);
                    }
                    current_row = Vec::new();
                }
                _ => {
                    current_field.push(c);
                }
            }
        }
    }

    // Flush last field/row
    if !current_field.is_empty() || !current_row.is_empty() {
        current_row.push(current_field);
        if !current_row.iter().all(|f| f.is_empty()) {
            rows.push(current_row);
        }
    }

    rows
}

/// Build a Markdown table from parsed rows.
fn build_markdown_table(rows: &[Vec<String>]) -> String {
    if rows.is_empty() {
        return String::new();
    }

    let col_count = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    if col_count == 0 {
        return String::new();
    }

    let mut markdown = String::new();

    for (i, row) in rows.iter().enumerate() {
        markdown.push('|');
        for j in 0..col_count {
            let cell = row.get(j).map(|s| s.trim()).unwrap_or("");
            markdown.push(' ');
            markdown.push_str(cell);
            markdown.push_str(" |");
        }
        markdown.push('\n');

        // Add separator after first row (header)
        if i == 0 {
            markdown.push('|');
            for _ in 0..col_count {
                markdown.push_str(" --- |");
            }
            markdown.push('\n');
        }
    }

    markdown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_simple() {
        let rows = parse_csv("a,b,c\n1,2,3\n", ',');
        assert_eq!(rows, vec![vec!["a", "b", "c"], vec!["1", "2", "3"]]);
    }

    #[test]
    fn test_parse_csv_quoted() {
        let rows = parse_csv("\"hello, world\",b,c\n", ',');
        assert_eq!(rows, vec![vec!["hello, world", "b", "c"]]);
    }

    #[test]
    fn test_parse_csv_escaped_quotes() {
        let rows = parse_csv("\"say \"\"hello\"\"\",b\n", ',');
        assert_eq!(rows, vec![vec!["say \"hello\"", "b"]]);
    }

    #[test]
    fn test_parse_tsv() {
        let rows = parse_csv("a\tb\tc\n1\t2\t3\n", '\t');
        assert_eq!(rows, vec![vec!["a", "b", "c"], vec!["1", "2", "3"]]);
    }

    #[test]
    fn test_parse_csv_crlf() {
        let rows = parse_csv("a,b\r\n1,2\r\n", ',');
        assert_eq!(rows, vec![vec!["a", "b"], vec!["1", "2"]]);
    }

    #[test]
    fn test_parse_csv_empty_fields() {
        let rows = parse_csv("a,,c\n", ',');
        assert_eq!(rows, vec![vec!["a", "", "c"]]);
    }

    #[test]
    fn test_build_markdown_table() {
        let rows = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
        ];
        let md = build_markdown_table(&rows);
        assert!(md.contains("| Name | Age |"));
        assert!(md.contains("| --- | --- |"));
        assert!(md.contains("| Alice | 30 |"));
    }

    #[tokio::test]
    async fn test_csv_extractor_plugin_interface() {
        let extractor = CsvExtractor::new();
        assert_eq!(extractor.name(), "csv-extractor");
        assert_eq!(extractor.version(), env!("CARGO_PKG_VERSION"));
        assert_eq!(extractor.priority(), 60);
        assert_eq!(
            extractor.supported_mime_types(),
            &["text/csv", "text/tab-separated-values"]
        );
    }

    #[tokio::test]
    async fn test_csv_extractor_output() {
        let extractor = CsvExtractor::new();
        let config = ExtractionConfig::default();
        let csv_data = b"Name,Age,City\nAlice,30,NYC\nBob,25,LA\n";

        let result = extractor.extract_bytes(csv_data, "text/csv", &config).await.unwrap();

        // Content should be space-separated (not comma-separated)
        assert!(result.content.contains("Name Age City"));
        assert!(result.content.contains("Alice 30 NYC"));
        assert!(result.content.contains("Bob 25 LA"));
        assert!(!result.content.contains(','));

        // Tables should be populated
        assert_eq!(result.tables.len(), 1);
        assert_eq!(result.tables[0].cells.len(), 3);
        assert_eq!(result.tables[0].cells[0], vec!["Name", "Age", "City"]);
    }

    #[tokio::test]
    async fn test_csv_extractor_quoted_fields() {
        let extractor = CsvExtractor::new();
        let config = ExtractionConfig::default();
        let csv_data = b"Name,Description\n\"Smith, John\",\"Has a comma, inside\"\n";

        let result = extractor.extract_bytes(csv_data, "text/csv", &config).await.unwrap();

        // Quoted fields with commas should be preserved as single fields
        assert!(result.content.contains("Smith, John"));
        assert_eq!(result.tables[0].cells[1][0], "Smith, John");
    }

    #[test]
    fn test_detect_delimiter_comma() {
        assert_eq!(detect_delimiter("a,b,c\n1,2,3\n4,5,6"), ',');
    }

    #[test]
    fn test_detect_delimiter_semicolon() {
        assert_eq!(detect_delimiter("a;b;c\n1;2;3\n4;5;6"), ';');
    }

    #[test]
    fn test_detect_delimiter_pipe() {
        assert_eq!(detect_delimiter("a|b|c\n1|2|3\n4|5|6"), '|');
    }

    #[test]
    fn test_detect_delimiter_tab() {
        assert_eq!(detect_delimiter("a\tb\tc\n1\t2\t3\n4\t5\t6"), '\t');
    }

    #[test]
    fn test_detect_delimiter_semicolons_with_commas_in_values() {
        assert_eq!(
            detect_delimiter("\"last, first\";age;city\n\"doe, john\";30;NYC\n\"smith, jane\";25;LA"),
            ';'
        );
    }

    #[tokio::test]
    async fn test_csv_extractor_real_file() {
        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../test_documents/csv/data_table.csv");
        if !test_file.exists() {
            return;
        }
        let content = std::fs::read(&test_file).expect("Failed to read test CSV");
        let extractor = CsvExtractor::new();
        let config = ExtractionConfig::default();
        let result = extractor.extract_bytes(&content, "text/csv", &config).await.unwrap();

        assert!(!result.content.is_empty());
        assert!(result.content.contains("Alice Johnson"));
        assert!(result.content.contains("Engineering"));
        // Should not have comma delimiters in the content
        assert!(!result.content.lines().any(|line| line.contains(',')));
        // Tables should be populated
        assert_eq!(result.tables.len(), 1);
        assert_eq!(result.tables[0].cells.len(), 11); // header + 10 data rows
    }
}

//! Result type bindings
//!
//! Provides PHP-friendly wrappers around extraction result types.

use ext_php_rs::convert::IntoZval;
use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use std::collections::HashMap;

/// Convert serde_json::Value to PHP Zval (array/string/number/bool/null).
///
/// Recursively converts JSON structures to PHP arrays, preserving nested objects.
pub(crate) fn json_value_to_php(value: &serde_json::Value) -> PhpResult<Zval> {
    match value {
        serde_json::Value::Null => Ok(Zval::new()),
        serde_json::Value::Bool(b) => Ok(b.into_zval(false)?),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_zval(false)?)
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_zval(false)?)
            } else {
                Err("Invalid number in JSON".into())
            }
        }
        serde_json::Value::String(s) => Ok(s.as_str().into_zval(false)?),
        serde_json::Value::Array(arr) => {
            let mut php_arr = Vec::new();
            for item in arr {
                php_arr.push(json_value_to_php(item)?);
            }
            Ok(php_arr.into_zval(false)?)
        }
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::new();
            for (k, v) in obj {
                map.insert(k.clone(), json_value_to_php(v)?);
            }
            Ok(map.into_zval(false)?)
        }
    }
}

/// Extraction result containing content, metadata, and tables.
///
/// This is the primary return type for all extraction operations.
///
/// # Properties
///
/// - `content` (string): Extracted text content
/// - `mime_type` (string): MIME type of the extracted document
/// - `metadata` (array): Document metadata as key-value pairs
/// - `tables` (array): Array of ExtractedTable objects
/// - `detected_languages` (array|null): Detected languages with confidence scores
/// - `images` (array|null): Extracted images with their data
/// - `chunks` (array|null): Text chunks with optional embeddings
/// - `pages` (array|null): Per-page extraction results
///
/// # Example
///
/// ```php
/// $result = kreuzberg_extract_file("document.pdf");
/// echo $result->content;
/// print_r($result->metadata);
/// foreach ($result->tables as $table) {
///     echo $table->markdown;
/// }
/// ```
#[php_class]
#[php(name = "Kreuzberg\\Types\\ExtractionResult")]
#[derive(Clone)]
pub struct ExtractionResult {
    /// Extracted text content
    #[php(prop)]
    pub content: String,

    /// MIME type of the document
    #[php(prop)]
    pub mime_type: String,

    /// Document metadata (stored as serialized JSON, accessed via property getter)
    metadata_json: String,

    /// Extracted tables
    pub tables: Vec<ExtractedTable>,

    /// Detected languages
    #[php(prop)]
    pub detected_languages: Option<Vec<String>>,

    /// Extracted images
    pub images: Option<Vec<ExtractedImage>>,

    /// Text chunks
    pub chunks: Option<Vec<TextChunk>>,

    /// Per-page results
    pub pages: Option<Vec<PageResult>>,

    /// Extracted keywords
    pub keywords: Option<Vec<Keyword>>,
}

#[php_impl]
impl ExtractionResult {
    /// Get metadata as a Metadata object.
    ///
    /// Returns a Metadata object with common fields accessible as properties.
    pub fn get_metadata(&self) -> PhpResult<Metadata> {
        Metadata::from_json(&self.metadata_json)
    }

    /// Magic getter for accessing properties that are not directly exposed.
    ///
    /// Allows access like $result->metadata, $result->chunks, $result->images, $result->pages, $result->tables.
    pub fn __get(&self, name: &str) -> PhpResult<Option<Zval>> {
        match name {
            "metadata" => {
                let metadata = self.get_metadata()?;
                Ok(Some(metadata.into_zval(false)?))
            }
            "chunks" => {
                if let Some(chunks) = &self.chunks {
                    Ok(Some(chunks.clone().into_zval(false)?))
                } else {
                    Ok(None)
                }
            }
            "embeddings" => {
                // Extract embeddings from chunks if available
                if let Some(chunks) = &self.chunks {
                    let embeddings: Vec<Vec<f32>> = chunks.iter().filter_map(|chunk| chunk.embedding.clone()).collect();

                    if !embeddings.is_empty() {
                        Ok(Some(embeddings.into_zval(false)?))
                    } else {
                        Ok(None)
                    }
                } else {
                    Ok(None)
                }
            }
            "images" => {
                if let Some(images) = &self.images {
                    Ok(Some(images.clone().into_zval(false)?))
                } else {
                    Ok(None)
                }
            }
            "pages" => {
                if let Some(pages) = &self.pages {
                    Ok(Some(pages.clone().into_zval(false)?))
                } else {
                    Ok(None)
                }
            }
            "keywords" => {
                if let Some(keywords) = &self.keywords {
                    // Convert keywords to PHP array of associative arrays
                    let mut php_keywords = Vec::new();
                    for kw in keywords {
                        let mut kw_map = HashMap::new();
                        kw_map.insert("text", kw.text.as_str().into_zval(false)?);
                        kw_map.insert("score", kw.score.into_zval(false)?);
                        php_keywords.push(kw_map.into_zval(false)?);
                    }
                    Ok(Some(php_keywords.into_zval(false)?))
                } else {
                    Ok(None)
                }
            }
            "tables" => Ok(Some(self.tables.clone().into_zval(false)?)),
            _ => Ok(None),
        }
    }

    /// Get all extracted tables.
    #[php(name = "getTables")]
    pub fn get_tables(&self) -> Vec<ExtractedTable> {
        self.tables.clone()
    }

    /// Get all extracted images.
    #[php(name = "getImages")]
    pub fn get_images(&self) -> Option<Vec<ExtractedImage>> {
        self.images.clone()
    }

    /// Get all text chunks.
    #[php(name = "getChunks")]
    pub fn get_chunks(&self) -> Option<Vec<TextChunk>> {
        self.chunks.clone()
    }

    /// Get all page results.
    #[php(name = "getPages")]
    pub fn get_pages(&self) -> Option<Vec<PageResult>> {
        self.pages.clone()
    }

    /// Get the total number of pages in the document.
    pub fn get_page_count(&self) -> usize {
        self.pages.as_ref().map(|p| p.len()).unwrap_or(0)
    }

    /// Get the total number of chunks in the document.
    pub fn get_chunk_count(&self) -> usize {
        self.chunks.as_ref().map(|c| c.len()).unwrap_or(0)
    }

    /// Get the primary detected language.
    pub fn get_detected_language(&self) -> Option<String> {
        self.detected_languages
            .as_ref()
            .and_then(|langs| langs.first().cloned())
    }

    /// Get all extracted keywords.
    #[php(name = "getKeywords")]
    pub fn get_keywords(&self) -> Option<Vec<Keyword>> {
        self.keywords.clone()
    }

    /// Get all embeddings extracted from chunks.
    ///
    /// Extracts embeddings from chunks if available.
    #[php(name = "getEmbeddings")]
    pub fn get_embeddings(&self) -> Option<Vec<Vec<f32>>> {
        self.chunks
            .as_ref()
            .map(|chunks| chunks.iter().filter_map(|chunk| chunk.embedding.clone()).collect())
    }
}

impl ExtractionResult {
    /// Convert from Rust ExtractionResult to PHP ExtractionResult.
    pub fn from_rust(result: kreuzberg::ExtractionResult) -> PhpResult<Self> {
        use serde_json::json;

        let mut metadata_obj = serde_json::Map::new();

        // Add common metadata fields
        if let Some(title) = &result.metadata.title {
            metadata_obj.insert("title".to_string(), json!(title));
        }
        if let Some(subject) = &result.metadata.subject {
            metadata_obj.insert("subject".to_string(), json!(subject));
        }
        if let Some(authors) = &result.metadata.authors {
            metadata_obj.insert("authors".to_string(), json!(authors));
        }
        if let Some(keywords) = &result.metadata.keywords {
            metadata_obj.insert("keywords".to_string(), json!(keywords));
        }
        if let Some(language) = &result.metadata.language {
            metadata_obj.insert("language".to_string(), json!(language));
        }
        if let Some(created_at) = &result.metadata.created_at {
            metadata_obj.insert("created_at".to_string(), json!(created_at));
        }
        if let Some(modified_at) = &result.metadata.modified_at {
            metadata_obj.insert("modified_at".to_string(), json!(modified_at));
        }
        if let Some(created_by) = &result.metadata.created_by {
            metadata_obj.insert("created_by".to_string(), json!(created_by));
        }
        if let Some(modified_by) = &result.metadata.modified_by {
            metadata_obj.insert("modified_by".to_string(), json!(modified_by));
        }

        // Add page count - try multiple sources
        let page_count = if let Some(pages_meta) = &result.metadata.pages {
            // Prefer page structure metadata if available
            Some(pages_meta.total_count)
        } else {
            // Fallback to counting pages array
            result.pages.as_ref().map(|pages_array| pages_array.len())
        };

        if let Some(count) = page_count {
            metadata_obj.insert("pageCount".to_string(), json!(count));
        }

        // Add pages metadata structure if available
        if let Some(pages) = &result.metadata.pages {
            let pages_json = serde_json::to_value(pages).map_err(|e| format!("Failed to serialize pages: {}", e))?;
            metadata_obj.insert("pages".to_string(), pages_json);
        }

        // Add format metadata (both nested and flattened for compatibility)
        if let Some(format) = &result.metadata.format {
            let format_json = serde_json::to_value(format).map_err(|e| format!("Failed to serialize format: {}", e))?;

            // The format_json will have a "format_type" field due to the #[serde(tag = "format_type")] attribute
            // We flatten it into the root metadata object for fields like format_type, sheet_count, etc.
            if let serde_json::Value::Object(ref format_obj) = format_json {
                for (key, value) in format_obj {
                    metadata_obj.insert(key.clone(), value.clone());
                }
            }

            // Also add the full format object as a nested "format" key for paths like "format.format"
            metadata_obj.insert("format".to_string(), format_json);
        }

        // Extract keywords from additional metadata before adding all additional fields
        let keywords = if let Some(keywords_value) = result.metadata.additional.get("keywords") {
            if let Some(arr) = keywords_value.as_array() {
                let mut keywords_vec = Vec::new();
                for keyword_value in arr {
                    match Keyword::from_json(keyword_value) {
                        Ok(kw) => keywords_vec.push(kw),
                        Err(_e) => {
                            continue;
                        }
                    }
                }
                if !keywords_vec.is_empty() {
                    Some(keywords_vec)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        // Add additional metadata fields (from postprocessors)
        for (key, value) in &result.metadata.additional {
            metadata_obj.insert(key.clone(), value.clone());
        }

        // Serialize the metadata to JSON string
        let metadata_json =
            serde_json::to_string(&metadata_obj).map_err(|e| format!("Failed to serialize metadata: {}", e))?;

        let tables = result
            .tables
            .into_iter()
            .map(ExtractedTable::from_rust)
            .collect::<PhpResult<Vec<_>>>()?;

        let images = result
            .images
            .map(|imgs| {
                imgs.into_iter()
                    .map(ExtractedImage::from_rust)
                    .collect::<PhpResult<Vec<_>>>()
            })
            .transpose()?;

        let chunks = result
            .chunks
            .map(|chnks| {
                chnks
                    .into_iter()
                    .map(TextChunk::from_rust)
                    .collect::<PhpResult<Vec<_>>>()
            })
            .transpose()?;

        let pages = result
            .pages
            .map(|pgs| {
                pgs.into_iter()
                    .map(PageResult::from_rust)
                    .collect::<PhpResult<Vec<_>>>()
            })
            .transpose()?;

        Ok(Self {
            content: result.content,
            mime_type: result.mime_type,
            metadata_json,
            tables,
            detected_languages: result.detected_languages,
            images,
            chunks,
            pages,
            keywords,
        })
    }
}

/// Extracted table with cells and markdown representation.
///
/// # Properties
///
/// - `cells` (array): Table data as nested arrays (rows of columns)
/// - `markdown` (string): Markdown representation of the table
/// - `page_number` (int): Page number where table was found
///
/// # Example
///
/// ```php
/// foreach ($result->tables as $table) {
///     echo "Table on page {$table->page_number}:\n";
///     echo $table->markdown . "\n";
///     echo "Dimensions: " . count($table->cells) . " rows\n";
/// }
/// ```
#[php_class]
#[php(name = "Kreuzberg\\Types\\ExtractedTable")]
#[derive(Clone)]
pub struct ExtractedTable {
    /// Table cells as nested arrays
    #[php(prop)]
    pub cells: Vec<Vec<String>>,

    /// Markdown representation
    #[php(prop)]
    pub markdown: String,

    /// Page number
    #[php(prop)]
    pub page_number: usize,
}

#[php_impl]
impl ExtractedTable {
    /// Get the table cells as nested arrays.
    #[php(name = "getCells")]
    pub fn get_cells(&self) -> Vec<Vec<String>> {
        self.cells.clone()
    }
}

impl ExtractedTable {
    /// Convert from Rust Table to PHP ExtractedTable.
    pub fn from_rust(table: kreuzberg::Table) -> PhpResult<Self> {
        Ok(Self {
            cells: table.cells,
            markdown: table.markdown,
            page_number: table.page_number,
        })
    }
}

/// Extracted image with data and metadata.
///
/// # Properties
///
/// - `data` (string): Binary image data
/// - `format` (string): Image format (e.g., "png", "jpeg")
/// - `image_index` (int): Index of image in document
/// - `page_number` (int|null): Page number where image was found
/// - `width` (int|null): Image width in pixels
/// - `height` (int|null): Image height in pixels
#[php_class]
#[php(name = "Kreuzberg\\Types\\ExtractedImage")]
#[derive(Clone)]
pub struct ExtractedImage {
    #[php(prop)]
    pub data: Vec<u8>,
    #[php(prop)]
    pub format: String,
    #[php(prop)]
    pub image_index: usize,
    #[php(prop)]
    pub page_number: Option<usize>,
    #[php(prop)]
    pub width: Option<i32>,
    #[php(prop)]
    pub height: Option<i32>,
    #[php(prop)]
    pub colorspace: Option<String>,
    #[php(prop)]
    pub bits_per_component: Option<i32>,
    #[php(prop)]
    pub description: Option<String>,
    #[php(prop)]
    pub is_mask: bool,
}

#[php_impl]
impl ExtractedImage {
    /// Get the binary image data.
    #[php(name = "getData")]
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl ExtractedImage {
    pub fn from_rust(img: kreuzberg::ExtractedImage) -> PhpResult<Self> {
        Ok(Self {
            data: img.data,
            format: img.format,
            image_index: img.image_index,
            page_number: img.page_number,
            width: img.width.map(|w| w as i32),
            height: img.height.map(|h| h as i32),
            colorspace: img.colorspace,
            bits_per_component: img.bits_per_component.map(|b| b as i32),
            description: img.description,
            is_mask: img.is_mask,
        })
    }
}

/// Text chunk with optional embedding.
///
/// # Properties
///
/// - `content` (string): Chunk text content
/// - `embedding` (array|null): Embedding vector (if enabled)
/// - `byte_start` (int): Start byte offset
/// - `byte_end` (int): End byte offset
/// - `chunk_index` (int): Index of this chunk
/// - `total_chunks` (int): Total number of chunks
#[php_class]
#[php(name = "Kreuzberg\\Types\\TextChunk")]
#[derive(Clone)]
pub struct TextChunk {
    #[php(prop)]
    pub content: String,
    #[php(prop)]
    pub embedding: Option<Vec<f32>>,
    #[php(prop)]
    pub byte_start: usize,
    #[php(prop)]
    pub byte_end: usize,
    #[php(prop)]
    pub chunk_index: usize,
    #[php(prop)]
    pub total_chunks: usize,
    #[php(prop)]
    pub token_count: usize,
    #[php(prop)]
    pub first_page: Option<usize>,
    #[php(prop)]
    pub last_page: Option<usize>,
}

#[php_impl]
impl TextChunk {}

impl TextChunk {
    pub fn from_rust(chunk: kreuzberg::Chunk) -> PhpResult<Self> {
        Ok(Self {
            content: chunk.content,
            embedding: chunk.embedding,
            byte_start: chunk.metadata.byte_start,
            byte_end: chunk.metadata.byte_end,
            chunk_index: chunk.metadata.chunk_index,
            total_chunks: chunk.metadata.total_chunks,
            token_count: chunk.metadata.token_count.unwrap_or(0),
            first_page: chunk.metadata.first_page,
            last_page: chunk.metadata.last_page,
        })
    }
}

/// Per-page extraction result.
///
/// # Properties
///
/// - `page_number` (int): Page number (1-indexed)
/// - `content` (string): Extracted text for this page
/// - `tables` (array): Tables found on this page
/// - `images` (array): Images found on this page
#[php_class]
#[php(name = "Kreuzberg\\Types\\PageResult")]
#[derive(Clone)]
pub struct PageResult {
    #[php(prop)]
    pub page_number: usize,
    #[php(prop)]
    pub content: String,
    pub tables: Vec<ExtractedTable>,
    pub images: Vec<ExtractedImage>,
}

#[php_impl]
impl PageResult {
    /// Get all tables on this page.
    #[php(name = "getTables")]
    pub fn get_tables(&self) -> Vec<ExtractedTable> {
        self.tables.clone()
    }

    /// Get all images on this page.
    #[php(name = "getImages")]
    pub fn get_images(&self) -> Vec<ExtractedImage> {
        self.images.clone()
    }
}

impl PageResult {
    pub fn from_rust(page: kreuzberg::PageContent) -> PhpResult<Self> {
        let tables = page
            .tables
            .into_iter()
            .map(|arc_t| {
                let table: kreuzberg::Table = (*arc_t).clone();
                ExtractedTable::from_rust(table)
            })
            .collect::<PhpResult<Vec<_>>>()?;

        let images = page
            .images
            .into_iter()
            .map(|arc_img| {
                let img: kreuzberg::ExtractedImage = (*arc_img).clone();
                ExtractedImage::from_rust(img)
            })
            .collect::<PhpResult<Vec<_>>>()?;

        Ok(Self {
            page_number: page.page_number,
            content: page.content,
            tables,
            images,
        })
    }
}

/// Document metadata object.
///
/// Provides access to common metadata fields as properties.
/// Additional fields are accessible via the additional property.
#[php_class]
#[php(name = "Kreuzberg\\Types\\Metadata")]
#[derive(Clone)]
pub struct Metadata {
    /// Document title
    #[php(prop)]
    pub title: Option<String>,

    /// Document subject
    #[php(prop)]
    pub subject: Option<String>,

    /// Authors list
    #[php(prop)]
    pub authors: Option<Vec<String>>,

    /// Keywords list
    #[php(prop)]
    pub keywords: Option<Vec<String>>,

    /// Language code
    #[php(prop)]
    pub language: Option<String>,

    /// Creation timestamp
    #[php(prop)]
    pub created_at: Option<String>,

    /// Modification timestamp
    #[php(prop)]
    pub modified_at: Option<String>,

    /// Creator name
    #[php(prop)]
    pub created_by: Option<String>,

    /// Modifier name
    #[php(prop)]
    pub modified_by: Option<String>,

    /// Page count (convenience field)
    #[php(prop)]
    pub page_count: Option<usize>,

    /// Additional metadata fields (stored as JSON for now)
    additional_json: String,
}

#[php_impl]
impl Metadata {
    /// Create Metadata from JSON string.
    pub fn from_json(json: &str) -> PhpResult<Self> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("Failed to parse metadata JSON: {}", e))?;

        let obj = value.as_object().ok_or("Metadata must be a JSON object")?;

        Ok(Self {
            title: obj.get("title").and_then(|v| v.as_str()).map(String::from),
            subject: obj.get("subject").and_then(|v| v.as_str()).map(String::from),
            authors: obj.get("authors").and_then(|v| {
                v.as_array()
                    .map(|arr| arr.iter().filter_map(|item| item.as_str().map(String::from)).collect())
            }),
            keywords: obj.get("keywords").and_then(|v| {
                v.as_array()
                    .map(|arr| arr.iter().filter_map(|item| item.as_str().map(String::from)).collect())
            }),
            language: obj.get("language").and_then(|v| v.as_str()).map(String::from),
            created_at: obj.get("created_at").and_then(|v| v.as_str()).map(String::from),
            modified_at: obj.get("modified_at").and_then(|v| v.as_str()).map(String::from),
            created_by: obj.get("created_by").and_then(|v| v.as_str()).map(String::from),
            modified_by: obj.get("modified_by").and_then(|v| v.as_str()).map(String::from),
            page_count: obj.get("page_count").and_then(|v| v.as_u64()).map(|n| n as usize),
            additional_json: json.to_string(),
        })
    }

    /// Get additional metadata fields as associative array.
    pub fn get_additional(&self) -> PhpResult<HashMap<String, Zval>> {
        let value: serde_json::Value = serde_json::from_str(&self.additional_json)
            .map_err(|e| format!("Failed to parse additional JSON: {}", e))?;

        if let serde_json::Value::Object(obj) = value {
            let mut result = HashMap::new();
            // Exclude the common fields we already exposed
            let excluded = [
                "title",
                "subject",
                "authors",
                "keywords",
                "language",
                "created_at",
                "modified_at",
                "created_by",
                "modified_by",
                "page_count",
            ];
            for (k, v) in obj {
                if !excluded.contains(&k.as_str()) {
                    result.insert(k, json_value_to_php(&v)?);
                }
            }
            Ok(result)
        } else {
            Ok(HashMap::new())
        }
    }

    /// Get all metadata as associative array (for backwards compatibility).
    pub fn to_array(&self) -> PhpResult<HashMap<String, Zval>> {
        let value: serde_json::Value =
            serde_json::from_str(&self.additional_json).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        if let serde_json::Value::Object(obj) = value {
            let mut result = HashMap::new();
            for (k, v) in obj {
                result.insert(k, json_value_to_php(&v)?);
            }
            Ok(result)
        } else {
            Ok(HashMap::new())
        }
    }

    /// Check if a custom metadata field exists.
    pub fn has_custom(&self, key: String) -> PhpResult<bool> {
        let additional = self.get_additional()?;
        Ok(additional.contains_key(&key))
    }

    /// Get a custom metadata field value.
    pub fn get_custom(&self, key: String) -> PhpResult<Zval> {
        let additional = self.get_additional()?;
        Ok(additional
            .get(&key)
            .map(|v| v.shallow_clone())
            .unwrap_or_else(Zval::new))
    }
}

/// Extracted keyword with score and metadata.
///
/// # Properties
///
/// - `text` (string): The keyword text
/// - `score` (float): Keyword relevance score (0-1 range typically)
/// - `algorithm` (string|null): The algorithm used to extract this keyword
#[php_class]
#[php(name = "Kreuzberg\\Types\\Keyword")]
#[derive(Clone, Debug)]
pub struct Keyword {
    /// Keyword text
    #[php(prop)]
    pub text: String,

    /// Keyword score
    #[php(prop)]
    pub score: f32,

    /// Algorithm used
    #[php(prop)]
    pub algorithm: Option<String>,
}

impl Keyword {
    /// Convert from JSON value to Keyword.
    pub fn from_json(value: &serde_json::Value) -> PhpResult<Self> {
        let obj = value.as_object().ok_or("Keyword must be a JSON object")?;

        let text = obj
            .get("text")
            .and_then(|v| v.as_str())
            .ok_or("Keyword must have a 'text' field")?
            .to_string();

        let score = obj
            .get("score")
            .and_then(|v| v.as_f64())
            .ok_or("Keyword must have a 'score' field")? as f32;

        let algorithm = obj.get("algorithm").and_then(|v| v.as_str()).map(|s| s.to_string());

        Ok(Self { text, score, algorithm })
    }
}

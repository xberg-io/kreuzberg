# R API Reference <span class="version-badge unreleased">Unreleased</span>

Complete reference for the Kreuzberg R API.

## Installation

Install from the R-universe repository:

```r title="R"
install.packages("kreuzberg", repos = "https://kreuzberg-dev.r-universe.dev")
```

Or install from source using `remotes`:

```r title="R"
remotes::install_github("kreuzberg-dev/kreuzberg", subdir = "packages/r")
```

**System Requirements:**

- R >= 4.2
- Rust toolchain (cargo, rustc >= 1.91) for building from source
- Supported platforms: Linux (x64, arm64), macOS (Apple Silicon)

---

## Core Functions

### batch_extract_bytes()

Extract content from multiple raw byte arrays (asynchronous via Tokio runtime).

**Signature:**

```r title="R"
batch_extract_bytes(data_list, mime_types, config = NULL) -> list of kreuzberg_result
```

**Parameters:**

Same as [`batch_extract_bytes_sync()`](#batch_extract_bytes_sync).

**Returns:**

- List of `kreuzberg_result` objects

---

### batch_extract_bytes_sync()

Extract content from multiple raw byte arrays (synchronous).

**Signature:**

```r title="R"
batch_extract_bytes_sync(data_list, mime_types, config = NULL) -> list of kreuzberg_result
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `data_list` | list of raw | List of binary data (raw vectors) |
| `mime_types` | character | MIME types corresponding to each byte array |
| `config` | list, NULL | Extraction configuration |

**Returns:**

- List of `kreuzberg_result` objects

**Example:**

```r title="R"
library(kreuzberg)

pdf_data <- readBin("invoice.pdf", what = "raw", n = file.size("invoice.pdf"))
docx_data <- readBin("report.docx", what = "raw", n = file.size("report.docx"))

data_list <- list(pdf_data, docx_data)
mime_types <- c("application/pdf", "application/vnd.openxmlformats-officedocument.wordprocessingml.document")

results <- batch_extract_bytes_sync(data_list, mime_types)

for (i in seq_along(results)) {
  cat(sprintf("Document %d: %d characters\n", i, nchar(results[[i]]$content)))
}
```

---

### batch_extract_files()

Extract content from multiple files in parallel (asynchronous via Tokio runtime).

**Signature:**

```r title="R"
batch_extract_files(paths, config = NULL) -> list of kreuzberg_result
```

**Parameters:**

Same as [`batch_extract_files_sync()`](#batch_extract_files_sync).

**Returns:**

- List of `kreuzberg_result` objects

---

### batch_extract_files_sync()

Extract content from multiple files in parallel (synchronous).

**Signature:**

```r title="R"
batch_extract_files_sync(paths, config = NULL) -> list of kreuzberg_result
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `paths` | character | Vector of file paths to extract |
| `config` | list, NULL | Extraction configuration applied to all files |

**Returns:**

- List of `kreuzberg_result` objects

**Example:**

```r title="R"
library(kreuzberg)

paths <- c("doc1.pdf", "doc2.docx", "doc3.xlsx")
results <- batch_extract_files_sync(paths)

for (i in seq_along(results)) {
  cat(sprintf("%s: %d characters\n", paths[i], nchar(results[[i]]$content)))
}
```

---

### extract_bytes()

Extract content from raw bytes (asynchronous via Tokio runtime).

**Signature:**

```r title="R"
extract_bytes(data, mime_type, config = NULL) -> kreuzberg_result
```

**Parameters:**

Same as [`extract_bytes_sync()`](#extract_bytes_sync).

**Returns:**

- `kreuzberg_result`: Extraction result object

---

### extract_bytes_sync()

Extract content from raw bytes (synchronous).

**Signature:**

```r title="R"
extract_bytes_sync(data, mime_type, config = NULL) -> kreuzberg_result
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `data` | raw | Binary data to extract (raw vector) |
| `mime_type` | character | MIME type of the data (required for format detection) |
| `config` | list, NULL | Extraction configuration |

**Returns:**

- `kreuzberg_result`: Extraction result object

**Example:**

```r title="R"
library(kreuzberg)

data <- readBin("document.pdf", what = "raw", n = file.size("document.pdf"))
result <- extract_bytes_sync(data, "application/pdf")
cat(result$content)
```

---

### extract_file()

Extract content from a file (asynchronous via Tokio runtime).

**Note:** R does not have native async/await. This function internally uses a blocking Tokio runtime. For background processing, run in a separate R process or use a thread pool.

**Signature:**

```r title="R"
extract_file(path, mime_type = NULL, config = NULL) -> kreuzberg_result
```

**Parameters:**

Same as [`extract_file_sync()`](#extract_file_sync).

**Returns:**

- `kreuzberg_result`: Extraction result object

**Example:**

```r title="R"
library(kreuzberg)

# Equivalent to extract_file_sync in R
result <- extract_file("document.pdf")
cat(result$content)
```

---

### extract_file_sync()

Extract content from a file (synchronous).

**Signature:**

```r title="R"
extract_file_sync(path, mime_type = NULL, config = NULL) -> kreuzberg_result
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | character | Path to the file to extract |
| `mime_type` | character, NULL | Optional MIME type hint. If NULL, MIME type is auto-detected |
| `config` | list, NULL | Extraction configuration. Uses defaults if NULL |

**Returns:**

- `kreuzberg_result`: Extraction result object (S3 class inheriting from list)

**Raises:**

- `ValidationError`: Input validation failed
- `ParsingError`: Document parsing failed
- `FileNotFoundError`: File does not exist
- `UnsupportedFormatError`: Document format not supported
- `ExtractionError`: General extraction failure

**Example - Basic usage:**

```r title="R"
library(kreuzberg)

result <- extract_file_sync("document.pdf")
cat("Content:\n", result$content, "\n")
cat("Pages:", page_count(result), "\n")
```

**Example - With configuration:**

```r title="R"
library(kreuzberg)

config <- extraction_config(
  ocr = ocr_config(backend = "tesseract", language = "eng")
)
result <- extract_file_sync("scanned.pdf", config = config)
```

**Example - With explicit MIME type:**

```r title="R"
library(kreuzberg)

result <- extract_file_sync("document.pdf", mime_type = "application/pdf")
```

---

## Configuration

### chunking_config()

Create text chunking configuration.

**Signature:**

```r title="R"
chunking_config(max_characters = 1000L, overlap = 200L, ...) -> list
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `max_characters` | integer | Maximum characters per chunk. Default: 1000 |
| `overlap` | integer | Overlap between chunks. Default: 200 |
| ... | | Additional chunking options |

**Returns:**

- Named list with chunking configuration

**Example:**

```r title="R"
config <- extraction_config(
  chunking = chunking_config(max_characters = 2000L, overlap = 500L)
)
```

---

### discover()

Search for `kreuzberg.toml` configuration file in current and parent directories.

**Signature:**

```r title="R"
discover() -> list or NULL
```

**Returns:**

- Named list with configuration if found, NULL otherwise

**Example:**

```r title="R"
config <- discover()
if (!is.null(config)) {
  result <- extract_file_sync("document.pdf", config = config)
}
```

---

### extraction_config()

Create an extraction configuration object.

**Signature:**

```r title="R"
extraction_config(
  chunking = NULL,
  enable_quality_processing = NULL,
  force_ocr = FALSE,
  html_options = NULL,
  images = NULL,
  include_document_structure = NULL,
  keywords = NULL,
  language_detection = NULL,
  layout = NULL,
  max_concurrent_extractions = NULL,
  ocr = NULL,
  output_format = NULL,
  pages = NULL,
  pdf_options = NULL,
  postprocessor = NULL,
  result_format = NULL,
  security_limits = NULL,
  token_reduction = NULL,
  use_cache = NULL,
  ...
) -> list
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `chunking` | list, NULL | Text chunking options (see `chunking_config()`) |
| `enable_quality_processing` | logical, NULL | Enable quality processing enhancements |
| `force_ocr` | logical | Force OCR on all documents regardless of document type |
| `html_options` | list, NULL | HTML-specific options |
| `images` | list, NULL | Image extraction options |
| `include_document_structure` | logical, NULL | Include hierarchical document structure in results |
| `keywords` | list, NULL | Keyword extraction options |
| `language_detection` | list, NULL | Language detection options |
| `layout` | list, NULL | Layout detection options |
| `max_concurrent_extractions` | integer, NULL | Maximum concurrent extractions for batch operations |
| `ocr` | list, NULL | OCR configuration (see `ocr_config()`) |
| `output_format` | character, NULL | Output format for extracted content ('plain', 'markdown', 'djot', 'html') |
| `pages` | list, NULL | Page extraction options |
| `pdf_options` | list, NULL | PDF-specific options |
| `postprocessor` | character, NULL | Post-processor name |
| `result_format` | character, NULL | Result format ('unified', 'element_based') |
| `security_limits` | list, NULL | Security limit options |
| `token_reduction` | list, NULL | Token reduction options |
| `use_cache` | logical, NULL | Enable extraction result caching |
| Other options | | Additional configuration parameters |

**Returns:**

- Named list with configuration options

**Example:**

```r title="R"
config <- extraction_config(
  ocr = ocr_config(backend = "tesseract", language = "eng"),
  chunking = chunking_config(max_characters = 1000L, overlap = 200L),
  use_cache = TRUE
)

result <- extract_file_sync("document.pdf", config = config)
```

---

### from_file()

Load configuration from a TOML, YAML, or JSON file.

**Signature:**

```r title="R"
from_file(path) -> list
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | character | Path to configuration file (TOML, YAML, or JSON) |

**Returns:**

- Named list with configuration

**Example:**

```r title="R"
config <- from_file("kreuzberg.toml")
result <- extract_file_sync("document.pdf", config = config)
```

---

### layout_detection_config()

Create a layout detection configuration.

**Signature:**

```r title="R"
layout_detection_config(preset = "fast", confidence_threshold = NULL, apply_heuristics = TRUE, ...) -> list
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `apply_heuristics` | logical | Whether to apply heuristic post-processing to refine layout regions. Default: TRUE |
| `confidence_threshold` | numeric, NULL | Minimum confidence threshold for detected regions (0.0-1.0). Default: NULL |
| `preset` | character | Model preset controlling accuracy vs speed trade-off ("fast" or "accurate"). Default: "fast" |
| ... | | Additional layout detection options |

**Returns:**

- Named list with layout detection configuration

**Example:**

```r title="R"
config <- extraction_config(
  layout = layout_detection_config(preset = "accurate", apply_heuristics = TRUE)
)
```

---

### ocr_config()

Create OCR configuration.

**Signature:**

```r title="R"
ocr_config(backend = "tesseract", language = "eng", dpi = NULL, ...) -> list
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `backend` | character | OCR backend ("tesseract" or "paddle-ocr"). Default: "tesseract" |
| `dpi` | integer, NULL | DPI for OCR processing |
| `language` | character | Language code (ISO 639-3). Default: "eng" |
| `model_tier` | character, NULL | <span class="version-badge">v4.5.0</span> PaddleOCR model tier: "server" (high accuracy, ~88MB det model) or "mobile" (lightweight, ~4.5MB det model). Default: "server" |
| `padding` | integer, NULL | <span class="version-badge">v4.5.0</span> Padding in pixels (0-100) added around the image before PaddleOCR detection. Default: 10 |
| ... | | Additional OCR options |

**Returns:**

- Named list with OCR configuration

**Example:**

```r title="R"
config <- extraction_config(
  ocr = ocr_config(backend = "paddle-ocr", language = "eng")
)
```

---

## Results & Types

### kreuzberg_result

Result object returned by all extraction functions. Inherits from list with named fields.

**Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `annotations` | list, NULL | PDF annotations (links, highlights, notes) |
| `chunks` | list, NULL | Text chunks (if chunking enabled) |
| `content` | character | Extracted text content |
| `detected_language` | character, NULL | Detected language code (ISO 639-1) |
| `djot_content` | list, NULL | Structured Djot content |
| `document` | list, NULL | Hierarchical document structure |
| `elements` | list, NULL | Document semantic elements |
| `extracted_keywords` | list, NULL | Extracted keywords with scores |
| `images` | list, NULL | Extracted images |
| `metadata` | list | Document metadata |
| `mime_type` | character | MIME type of the processed document |
| `ocr_elements` | list, NULL | OCR elements with positioning and confidence |
| `pages` | list, NULL | Per-page extracted content (if page extraction enabled) |
| `processing_warnings` | list, NULL | Non-fatal processing warnings |
| `quality_score` | numeric, NULL | Quality score (0.0-1.0) |
| `tables` | list, NULL | Array of extracted tables |

**Example:**

```r title="R"
result <- extract_file_sync("document.pdf")

cat("Content:", result$content, "\n")
cat("MIME type:", result$mime_type, "\n")
cat("Pages:", page_count(result), "\n")
cat("Tables:", length(result$tables), "\n")
cat("Language:", detected_language(result), "\n")
```

---

### S3 Methods for kreuzberg_result

#### chunk_count()

Get the number of text chunks.

```r title="R"
chunk_count(x) -> integer
```

**Example:**

```r title="R"
result <- extract_file_sync("document.pdf", config = extraction_config(chunking = chunking_config()))
chunks <- chunk_count(result)
```

---

#### content()

Extract the text content.

```r title="R"
content(x) -> character
```

**Example:**

```r title="R"
result <- extract_file_sync("document.pdf")
text <- content(result)
```

---

#### detected_language()

Get the detected language code.

```r title="R"
detected_language(x) -> character or NULL
```

**Example:**

```r title="R"
result <- extract_file_sync("document.pdf")
lang <- detected_language(result)
if (!is.null(lang)) {
  cat("Language:", lang, "\n")
}
```

---

#### format()

Format the result as a string.

```r title="R"
format(x)
```

---

#### metadata_field()

Extract a specific metadata field by name.

```r title="R"
metadata_field(x, name) -> value or NULL
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `x` | kreuzberg_result | Result object |
| `name` | character | Field name |

**Returns:**

- Field value or NULL if not present

**Example:**

```r title="R"
result <- extract_file_sync("document.pdf")
title <- metadata_field(result, "title")
author <- metadata_field(result, "author")
```

---

#### mime_type()

Get the MIME type of the document.

```r title="R"
mime_type(x) -> character
```

**Example:**

```r title="R"
result <- extract_file_sync("document.pdf")
type <- mime_type(result)
```

---

#### page_count()

Get the number of pages in the document.

```r title="R"
page_count(x) -> integer
```

**Example:**

```r title="R"
result <- extract_file_sync("document.pdf")
pages <- page_count(result)
```

---

#### print()

Print a brief summary of the result.

```r title="R"
print(x)
```

**Example:**

```r title="R"
result <- extract_file_sync("document.pdf")
print(result)  # Displays summary
```

---

#### summary()

Summarize the extraction result.

```r title="R"
summary(object)
```

**Example:**

```r title="R"
result <- extract_file_sync("document.pdf")
summary(result)
```

---

### Metadata Hash

Document metadata with format-specific fields.

**Common Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `authors` | character | Document authors |
| `created_at` | character | Creation date (ISO 8601) |
| `created_by` | character | Creator/application name |
| `custom` | list | Additional custom metadata from postprocessors |
| `date` | character | Document date (ISO 8601 format) |
| `format_type` | character | Format discriminator ("pdf", "excel", "email", etc.) |
| `keywords` | character | Document keywords |
| `language` | character | Document language (ISO 639-1 code) |
| `modified_at` | character | Modification date (ISO 8601) |
| `page_count` | integer | Number of pages |
| `producer` | character | Producer/generator |
| `subject` | character | Document subject |
| `title` | character | Document title |

**Example:**

```r title="R"
result <- extract_file_sync("document.pdf")
metadata <- result$metadata

if (metadata$format_type == "pdf") {
  cat("Title:", metadata$title, "\n")
  cat("Author:", metadata$authors, "\n")
  cat("Pages:", metadata$page_count, "\n")
}
```

---

## Error Handling

Errors are raised as typed conditions with class hierarchy:
- `kreuzberg_error` (base)
  - `ValidationError`
  - `ParsingError`
  - `FileNotFoundError`
  - `UnsupportedFormatError`
  - `ExtractionError`

**Example - Basic error handling:**

```r title="R"
library(kreuzberg)

tryCatch(
  result <- extract_file_sync("document.pdf"),
  FileNotFoundError = function(e) {
    cat("File not found:", conditionMessage(e), "\n")
  },
  ValidationError = function(e) {
    cat("Validation error:", conditionMessage(e), "\n")
  },
  kreuzberg_error = function(e) {
    cat("Extraction error:", conditionMessage(e), "\n")
  }
)
```

**Example - Specific error handling:**

```r title="R"
tryCatch(
  {
    result <- extract_file_sync("scanned.pdf", config = extraction_config(
      ocr = ocr_config(backend = "unsupported-backend")
    ))
  },
  ValidationError = function(e) {
    cat("Invalid configuration:", conditionMessage(e), "\n")
  },
  error = function(e) {
    cat("Unexpected error:", conditionMessage(e), "\n")
  }
)
```

---

## Cache Management

## Cache Management

### cache_stats()

Get cache statistics.

**Signature:**

```r title="R"
cache_stats() -> list
```

**Returns:**

- Named list with:
  - `total_entries` (integer): Number of cached entries
  - `total_size_bytes` (integer): Total cache size in bytes

**Example:**

```r title="R"
library(kreuzberg)

stats <- cache_stats()
cat("Cache entries:", stats$total_entries, "\n")
cat("Cache size:", stats$total_size_bytes, "bytes\n")
```

---

### clear_cache()

Clear the extraction cache.

**Signature:**

```r title="R"
clear_cache() -> invisible(NULL)
```

**Example:**

```r title="R"
library(kreuzberg)

clear_cache()
```

---

## Validation

### validate_language_code()

Validate language code.

**Signature:**

```r title="R"
validate_language_code(code) -> logical
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `code` | character | Language code (ISO 639-3 or 639-1) |

**Returns:**

- Logical: TRUE if valid, FALSE otherwise

**Example:**

```r title="R"
library(kreuzberg)

is_valid <- validate_language_code("eng")
```

---

### validate_mime_type()

Validate MIME type.

**Signature:**

```r title="R"
validate_mime_type(mime_type) -> logical
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `mime_type` | character | MIME type to validate |

**Returns:**

- Logical: TRUE if valid, FALSE otherwise

**Example:**

```r title="R"
library(kreuzberg)

is_valid <- validate_mime_type("application/pdf")
```

---

### validate_ocr_backend_name()

Validate OCR backend name.

**Signature:**

```r title="R"
validate_ocr_backend_name(backend) -> logical
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `backend` | character | Backend name to validate |

**Returns:**

- Logical: TRUE if valid, FALSE otherwise

**Example:**

```r title="R"
library(kreuzberg)

is_valid <- validate_ocr_backend_name("tesseract")
if (!is_valid) {
  cat("Invalid OCR backend\n")
}
```

---

### validate_output_format()

Validate output format.

**Signature:**

```r title="R"
validate_output_format(format) -> logical
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `format` | character | Output format name |

**Returns:**

- Logical: TRUE if valid, FALSE otherwise

---

## Metadata Detection

### detect_mime_type()

Detect MIME type from raw bytes.

**Signature:**

```r title="R"
detect_mime_type(data) -> character
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `data` | raw | Binary data |

**Returns:**

- Character: Detected MIME type

**Example:**

```r title="R"
library(kreuzberg)

data <- readBin("document", what = "raw", n = file.size("document"))
mime_type <- detect_mime_type(data)
cat("Detected MIME type:", mime_type, "\n")
```

---

### detect_mime_type_from_path()

Detect MIME type from file path.

**Signature:**

```r title="R"
detect_mime_type_from_path(path) -> character
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `path` | character | File path |

**Returns:**

- Character: Detected MIME type

**Example:**

```r title="R"
library(kreuzberg)

mime_type <- detect_mime_type_from_path("document.pdf")
cat("MIME type:", mime_type, "\n")
```

---

### get_extensions_for_mime()

Get file extensions for a MIME type.

**Signature:**

```r title="R"
get_extensions_for_mime(mime_type) -> character
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `mime_type` | character | MIME type |

**Returns:**

- Character vector: File extensions for the MIME type

**Example:**

```r title="R"
library(kreuzberg)

extensions <- get_extensions_for_mime("application/pdf")
cat("PDF extensions:", paste(extensions, collapse = ", "), "\n")
```

---

## Plugins

### OCR Backends

#### clear_ocr_backends()

Clear all registered OCR backends.

**Signature:**

```r title="R"
clear_ocr_backends() -> invisible(NULL)
```

---

#### list_ocr_backends()

List all registered OCR backends.

**Signature:**

```r title="R"
list_ocr_backends() -> character
```

**Returns:**

- Character vector: Names of registered backends

**Example:**

```r title="R"
library(kreuzberg)

backends <- list_ocr_backends()
cat("Available OCR backends:", paste(backends, collapse = ", "), "\n")
```

---

#### register_ocr_backend()

Register a custom OCR backend.

**Signature:**

```r title="R"
register_ocr_backend(name, callback) -> invisible(NULL)
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | character | Backend name |
| `callback` | function | Backend implementation function |

---

#### unregister_ocr_backend()

Unregister an OCR backend.

**Signature:**

```r title="R"
unregister_ocr_backend(name) -> invisible(NULL)
```

---

### Post-Processors

#### clear_post_processors()

Clear all registered post-processors.

**Signature:**

```r title="R"
clear_post_processors() -> invisible(NULL)
```

---

#### list_post_processors()

List all registered post-processors.

**Signature:**

```r title="R"
list_post_processors() -> character
```

**Returns:**

- Character vector: Names of registered post-processors

---

#### register_post_processor()

Register a custom post-processor.

**Signature:**

```r title="R"
register_post_processor(name, callback) -> invisible(NULL)
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | character | Processor name |
| `callback` | function | Processor implementation function |

---

#### unregister_post_processor()

Unregister a post-processor.

**Signature:**

```r title="R"
unregister_post_processor(name) -> invisible(NULL)
```

---

### Validators

#### clear_validators()

Clear all registered validators.

**Signature:**

```r title="R"
clear_validators() -> invisible(NULL)
```

---

#### list_validators()

List all registered validators.

**Signature:**

```r title="R"
list_validators() -> character
```

**Returns:**

- Character vector: Names of registered validators

---

#### register_validator()

Register a custom validator.

**Signature:**

```r title="R"
register_validator(name, callback) -> invisible(NULL)
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | character | Validator name |
| `callback` | function | Validator implementation function |

---

#### unregister_validator()

Unregister a validator.

**Signature:**

```r title="R"
unregister_validator(name) -> invisible(NULL)
```

---

### Document Extractors

#### clear_document_extractors()

Clear all document extractors.

**Signature:**

```r title="R"
clear_document_extractors() -> invisible(NULL)
```

---

#### list_document_extractors()

List all available document extractors.

**Signature:**

```r title="R"
list_document_extractors() -> character
```

**Returns:**

- Character vector: Names of available document extractors

---

#### unregister_document_extractor()

Unregister a document extractor.

**Signature:**

```r title="R"
unregister_document_extractor(name) -> invisible(NULL)
```

**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `name` | character | Extractor name |

---

## Thread Safety

All Kreuzberg functions are thread-safe and can be called from multiple threads concurrently via R's parallel package or future framework.

**Example - Using parallel package:**

```r title="R"
library(kreuzberg)
library(parallel)

files <- c("doc1.pdf", "doc2.pdf", "doc3.pdf")

# Use parallel processing
results <- mclapply(files, function(file) {
  extract_file_sync(file)
}, mc.cores = 3)

for (i in seq_along(results)) {
  cat(sprintf("%s: %d characters\n", files[i], nchar(results[[i]]$content)))
}
```

**Example - Using future package:**

```r title="R"
library(kreuzberg)
library(future)

plan(multisession)

files <- c("doc1.pdf", "doc2.pdf", "doc3.pdf")

# Process files asynchronously
futures <- lapply(files, function(file) {
  future({
    extract_file_sync(file)
  })
})

# Collect results
results <- lapply(futures, value)

for (i in seq_along(results)) {
  cat(sprintf("%s: %d characters\n", files[i], nchar(results[[i]]$content)))
}
```

However, for better performance, use the batch API instead:

```r title="R"
library(kreuzberg)

files <- c("doc1.pdf", "doc2.pdf", "doc3.pdf")

# Better approach: use built-in batch processing
results <- batch_extract_files_sync(files)

for (i in seq_along(results)) {
  cat(sprintf("%s: %d characters\n", files[i], nchar(results[[i]]$content)))
}
```

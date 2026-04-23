---
title: "C# API Reference"
---

## C# API Reference <span class="version-badge">v4.9.2</span>

### Functions

#### ValidateBinarizationMethod()

Validate a binarization method string.

**Returns:**

`Ok(())` if the method is valid, or a `ValidationError` with details about valid options.

**Signature:**

```csharp
public static void ValidateBinarizationMethod(string method)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Method` | `string` | Yes | The binarization method to validate (e.g., "otsu", "adaptive", "sauvola") |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ValidateTokenReductionLevel()

Validate a token reduction level string.

**Returns:**

`Ok(())` if the level is valid, or a `ValidationError` with details about valid options.

**Signature:**

```csharp
public static void ValidateTokenReductionLevel(string level)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Level` | `string` | Yes | The token reduction level to validate (e.g., "off", "light", "moderate") |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ValidateOcrBackend()

Validate an OCR backend string.

**Returns:**

`Ok(())` if the backend is valid, or a `ValidationError` with details about valid options.

**Signature:**

```csharp
public static void ValidateOcrBackend(string backend)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Backend` | `string` | Yes | The OCR backend to validate (e.g., "tesseract", "easyocr", "paddleocr") |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ValidateLanguageCode()

Validate a language code (ISO 639-1 or 639-3 format).

Accepts both 2-letter ISO 639-1 codes (e.g., "en", "de") and
3-letter ISO 639-3 codes (e.g., "eng", "deu") for broader compatibility.

**Returns:**

`Ok(())` if the code is valid, or a `ValidationError` indicating an invalid language code.

**Signature:**

```csharp
public static void ValidateLanguageCode(string code)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Code` | `string` | Yes | The language code to validate |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ValidateTesseractPsm()

Validate a tesseract Page Segmentation Mode (PSM).

**Returns:**

`Ok(())` if the PSM is valid, or a `ValidationError` with details about valid ranges.

**Signature:**

```csharp
public static void ValidateTesseractPsm(int psm)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Psm` | `int` | Yes | The PSM value to validate (0-13) |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ValidateTesseractOem()

Validate a tesseract OCR Engine Mode (OEM).

**Returns:**

`Ok(())` if the OEM is valid, or a `ValidationError` with details about valid options.

**Signature:**

```csharp
public static void ValidateTesseractOem(int oem)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Oem` | `int` | Yes | The OEM value to validate (0-3) |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ValidateOutputFormat()

Validate a document extraction output format.

Accepts the following formats and aliases:
- "plain" or "text" for plain text output
- "markdown" or "md" for Markdown output
- "djot" for Djot markup format
- "html" for HTML output

**Returns:**

`Ok(())` if the format is valid, or a `ValidationError` with details about valid options.

**Signature:**

```csharp
public static void ValidateOutputFormat(string format)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Format` | `string` | Yes | The output format to validate |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ValidateConfidence()

Validate a confidence threshold value.

Confidence thresholds should be between 0.0 and 1.0 inclusive.

**Returns:**

`Ok(())` if the confidence is valid, or a `ValidationError` with details about valid ranges.

**Signature:**

```csharp
public static void ValidateConfidence(double confidence)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Confidence` | `double` | Yes | The confidence threshold to validate |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ValidateDpi()

Validate a DPI (dots per inch) value.

DPI should be a positive integer, typically 72-600.

**Returns:**

`Ok(())` if the DPI is valid, or a `ValidationError` with details about valid ranges.

**Signature:**

```csharp
public static void ValidateDpi(int dpi)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Dpi` | `int` | Yes | The DPI value to validate |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ValidateChunkingParams()

Validate chunk size parameters.

Checks that max_chars > 0 and max_overlap < max_chars.

**Returns:**

`Ok(())` if the parameters are valid, or a `ValidationError` with details about constraints.

**Signature:**

```csharp
public static void ValidateChunkingParams(nuint maxChars, nuint maxOverlap)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `MaxChars` | `nuint` | Yes | The maximum characters per chunk |
| `MaxOverlap` | `nuint` | Yes | The maximum overlap between chunks |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ValidateLlmConfigModel()

Validate that an `LlmConfig` has a non-empty model string.

**Returns:**

`Ok(())` if the model is non-empty, or a `ValidationError` otherwise.

**Signature:**

```csharp
public static void ValidateLlmConfigModel(string model)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Model` | `string` | Yes | The model string to validate |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ExtractBytes()

Extract content from a byte array.

This is the main entry point for in-memory extraction. It performs the following steps:
1. Validate MIME type
2. Handle legacy format conversion if needed
3. Select appropriate extractor from registry
4. Extract content
5. Run post-processing pipeline

**Returns:**

An `ExtractionResult` containing the extracted content and metadata.

**Errors:**

Returns `KreuzbergError.Validation` if MIME type is invalid.
Returns `KreuzbergError.UnsupportedFormat` if MIME type is not supported.

**Signature:**

```csharp
public static async Task<ExtractionResult> ExtractBytesAsync(byte[] content, string mimeType, ExtractionConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Content` | `byte[]` | Yes | The byte array to extract |
| `MimeType` | `string` | Yes | MIME type of the content |
| `Config` | `ExtractionConfig` | Yes | Extraction configuration |

**Returns:** `ExtractionResult`

**Errors:** Throws `Error`.


---

#### ExtractFile()

Extract content from a file.

This is the main entry point for file-based extraction. It performs the following steps:
1. Check cache for existing result (if caching enabled)
2. Detect or validate MIME type
3. Select appropriate extractor from registry
4. Extract content
5. Run post-processing pipeline
6. Store result in cache (if caching enabled)

**Returns:**

An `ExtractionResult` containing the extracted content and metadata.

**Errors:**

Returns `KreuzbergError.Io` if the file doesn't exist (NotFound) or for other file I/O errors.
Returns `KreuzbergError.UnsupportedFormat` if MIME type is not supported.

**Signature:**

```csharp
public static async Task<ExtractionResult> ExtractFileAsync(string path, string? mimeType = null, ExtractionConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Path` | `string` | Yes | Path to the file to extract |
| `MimeType` | `string?` | No | Optional MIME type override. If None, will be auto-detected |
| `Config` | `ExtractionConfig` | Yes | Extraction configuration |

**Returns:** `ExtractionResult`

**Errors:** Throws `Error`.


---

#### ExtractFileSync()

Synchronous wrapper for `extract_file`.

This is a convenience function that blocks the current thread until extraction completes.
For async code, use `extract_file` directly.

Uses the global Tokio runtime for 100x+ performance improvement over creating
a new runtime per call. Always uses the global runtime to avoid nested runtime issues.

This function is only available with the `tokio-runtime` feature. For WASM targets,
use a truly synchronous extraction approach instead.

**Signature:**

```csharp
public static ExtractionResult ExtractFileSync(string path, string? mimeType = null, ExtractionConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Path` | `string` | Yes | Path to the file |
| `MimeType` | `string?` | No | The mime type |
| `Config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `ExtractionResult`

**Errors:** Throws `Error`.


---

#### ExtractBytesSync()

Synchronous wrapper for `extract_bytes`.

Uses the global Tokio runtime for 100x+ performance improvement over creating
a new runtime per call.

With the `tokio-runtime` feature, this blocks the current thread using the global
Tokio runtime. Without it (WASM), this calls a truly synchronous implementation.

**Signature:**

```csharp
public static ExtractionResult ExtractBytesSync(byte[] content, string mimeType, ExtractionConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Content` | `byte[]` | Yes | The content to process |
| `MimeType` | `string` | Yes | The mime type |
| `Config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `ExtractionResult`

**Errors:** Throws `Error`.


---

#### BatchExtractFileSync()

Synchronous wrapper for `batch_extract_file`.

Uses the global Tokio runtime for optimal performance.
Only available with `tokio-runtime` (WASM has no filesystem).

**Signature:**

```csharp
public static List<ExtractionResult> BatchExtractFileSync(List<string> items, ExtractionConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Items` | `List<string>` | Yes | The items |
| `Config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `List<ExtractionResult>`

**Errors:** Throws `Error`.


---

#### BatchExtractBytesSync()

Synchronous wrapper for `batch_extract_bytes`.

Uses the global Tokio runtime for optimal performance.
With the `tokio-runtime` feature, this blocks the current thread using the global
Tokio runtime. Without it (WASM), this calls a truly synchronous implementation
that iterates through items and calls `extract_bytes_sync()`.

**Signature:**

```csharp
public static List<ExtractionResult> BatchExtractBytesSync(List<string> items, ExtractionConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Items` | `List<string>` | Yes | The items |
| `Config` | `ExtractionConfig` | Yes | The configuration options |

**Returns:** `List<ExtractionResult>`

**Errors:** Throws `Error`.


---

#### BatchExtractFile()

Extract content from multiple files concurrently.

This function processes multiple files in parallel, automatically managing
concurrency to prevent resource exhaustion. The concurrency limit can be
configured via `ExtractionConfig.max_concurrent_extractions` or defaults
to `(num_cpus * 1.5).ceil()`.

Each file can optionally specify a `FileExtractionConfig` that overrides specific
fields from the batch-level `config`. Pass `null` for a file to use the batch defaults.
Batch-level settings like `max_concurrent_extractions` and `use_cache` are always
taken from the batch-level `config`.

  config to use the batch-level defaults for that file.
* `config` - Batch-level extraction configuration (provides defaults and batch settings)

**Returns:**

A vector of `ExtractionResult` in the same order as the input items.

**Errors:**

Individual file errors are captured in the result metadata. System errors
(IO, RuntimeError equivalents) will bubble up and fail the entire batch.

Simple usage with no per-file overrides:


Per-file configuration overrides:

**Signature:**

```csharp
public static async Task<List<ExtractionResult>> BatchExtractFileAsync(List<string> items, ExtractionConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Items` | `List<string>` | Yes | Vector of `(path, optional_file_config)` tuples. Pass `None` as the |
| `Config` | `ExtractionConfig` | Yes | Batch-level extraction configuration (provides defaults and batch settings) |

**Returns:** `List<ExtractionResult>`

**Errors:** Throws `Error`.


---

#### BatchExtractBytes()

Extract content from multiple byte arrays concurrently.

This function processes multiple byte arrays in parallel, automatically managing
concurrency to prevent resource exhaustion. The concurrency limit can be
configured via `ExtractionConfig.max_concurrent_extractions` or defaults
to `(num_cpus * 1.5).ceil()`.

Each item can optionally specify a `FileExtractionConfig` that overrides specific
fields from the batch-level `config`. Pass `null` as the config to use
the batch-level defaults for that item.

**Returns:**

A vector of `ExtractionResult` in the same order as the input items.

Simple usage with no per-item overrides:


Per-item configuration overrides:

**Signature:**

```csharp
public static async Task<List<ExtractionResult>> BatchExtractBytesAsync(List<string> items, ExtractionConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Items` | `List<string>` | Yes | Vector of `(bytes, mime_type, optional_file_config)` tuples |
| `Config` | `ExtractionConfig` | Yes | Batch-level extraction configuration |

**Returns:** `List<ExtractionResult>`

**Errors:** Throws `Error`.


---

#### IsValidFormatField()

Validates whether a field name is in the known formats registry.

This uses a pre-built hash set for O(1) lookups instead of linear search,
providing significant performance improvements for repeated validations.

**Returns:**

`true` if the field is in KNOWN_FORMATS, `false` otherwise.

**Signature:**

```csharp
public static bool IsValidFormatField(string field)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Field` | `string` | Yes | The field name to validate |

**Returns:** `bool`


---

#### ValidateMimeType()

Validate that a MIME type is supported.

**Returns:**

The validated MIME type (may be normalized).

**Errors:**

Returns `KreuzbergError.UnsupportedFormat` if not supported.

**Signature:**

```csharp
public static string ValidateMimeType(string mimeType)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `MimeType` | `string` | Yes | The MIME type to validate |

**Returns:** `string`

**Errors:** Throws `Error`.


---

#### DetectOrValidate()

Detect or validate MIME type.

If `mime_type` is provided, validates it. Otherwise, detects from `path`.

**Returns:**

The validated MIME type string.

**Signature:**

```csharp
public static string DetectOrValidate(string? path = null, string? mimeType = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Path` | `string?` | No | Optional path to detect MIME type from |
| `MimeType` | `string?` | No | Optional explicit MIME type to validate |

**Returns:** `string`

**Errors:** Throws `Error`.


---

#### DetectMimeTypeFromBytes()

Detect MIME type from raw file bytes.

Uses magic byte signatures to detect file type from content.
Falls back to `infer` crate for comprehensive detection.

For ZIP-based files, inspects contents to distinguish Office Open XML
formats (DOCX, XLSX, PPTX) from plain ZIP archives.

**Returns:**

The detected MIME type string.

**Errors:**

Returns `KreuzbergError.UnsupportedFormat` if MIME type cannot be determined.

**Signature:**

```csharp
public static string DetectMimeTypeFromBytes(byte[] content)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Content` | `byte[]` | Yes | Raw file bytes |

**Returns:** `string`

**Errors:** Throws `Error`.


---

#### GetExtensionsForMime()

Get file extensions for a given MIME type.

Returns all known file extensions that map to the specified MIME type.

**Returns:**

A vector of file extensions (without leading dot) for the MIME type.

**Signature:**

```csharp
public static List<string> GetExtensionsForMime(string mimeType)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `MimeType` | `string` | Yes | The MIME type to look up |

**Returns:** `List<string>`

**Errors:** Throws `Error`.


---

#### ListSupportedFormats()

List all supported document formats.

Returns a list of all file extensions and their corresponding MIME types
that Kreuzberg can process. Derived from the centralized `FORMATS` registry.

The list is sorted alphabetically by file extension.

**Signature:**

```csharp
public static List<SupportedFormat> ListSupportedFormats()
```

**Returns:** `List<SupportedFormat>`


---

#### TransformExtractionResultToElements()

Transform an extraction result into semantic elements.

This function takes a reference to an ExtractionResult and generates
a vector of Element structs representing semantic blocks in the document.
It detects content sections, list items, page breaks, and other structural
elements to create an Unstructured-compatible element-based output.

Handles:
- PDF hierarchy → Title/Heading elements
- Multi-page documents with correct page numbers
- Table and Image extraction
- PageBreak interleaving
- Bounding box coordinates
- Paragraph detection for NarrativeText

**Returns:**

A vector of Elements with proper semantic types and metadata.

**Signature:**

```csharp
public static List<Element> TransformExtractionResultToElements(ExtractionResult result)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Result` | `ExtractionResult` | Yes | Reference to the ExtractionResult to transform |

**Returns:** `List<Element>`


---

#### ExtractEmailContent()

Extract email content from either .eml or .msg format

**Signature:**

```csharp
public static EmailExtractionResult ExtractEmailContent(byte[] data, string mimeType, uint? fallbackCodepage = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Data` | `byte[]` | Yes | The data |
| `MimeType` | `string` | Yes | The mime type |
| `FallbackCodepage` | `uint?` | No | The fallback codepage |

**Returns:** `EmailExtractionResult`

**Errors:** Throws `Error`.


---

#### CellsToText()

Converts a 2D vector of cell strings into a GitHub-Flavored Markdown table.

# Behavior

- The first row is treated as the header row
- A separator row is inserted after the header
- Pipe characters (`|`) in cell content are automatically escaped with backslash
- Irregular tables (rows with varying column counts) are padded with empty cells to match the header
- Returns an empty string for empty input

**Returns:**

A `String` containing the GFM markdown table representation

Converts a 2D vector of cell strings into plain text with tab-separated columns.

# Behavior

- Rows are separated by newlines
- Cells within a row are separated by tab characters
- No pipe delimiters or separator rows (unlike markdown tables)
- Returns an empty string for empty input

**Returns:**

A `String` containing the plain text table representation

**Signature:**

```csharp
public static string CellsToText(List<List<string>> cells)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Cells` | `List<List<string>>` | Yes | A slice of vectors representing table rows, where each inner vector contains cell values |

**Returns:** `string`


---

#### CellsToMarkdown()

**Signature:**

```csharp
public static string CellsToMarkdown(List<List<string>> cells)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Cells` | `List<List<string>>` | Yes | The cells |

**Returns:** `string`


---

#### DjotToHtml()

Render djot content to HTML.

This function takes djot source text and renders it to HTML using jotdown's
built-in HTML renderer.

**Returns:**

A `Result` containing the rendered HTML string

**Signature:**

```csharp
public static string DjotToHtml(string djotSource)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `DjotSource` | `string` | Yes | The djot markup text to render |

**Returns:** `string`

**Errors:** Throws `Error`.


---

#### DedupText()

Deduplicate a list of text strings while preserving order.
Adjacent duplicates and near-duplicates are removed.

**Signature:**

```csharp
public static List<string> DedupText(List<string> texts)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Texts` | `List<string>` | Yes | The texts |

**Returns:** `List<string>`


---

#### NormalizeWhitespace()

Normalize whitespace in a string.

- Collapses multiple consecutive spaces/tabs into a single space
- Preserves single newlines (paragraph breaks from \par)
- Collapses multiple consecutive newlines into a double newline
- Trims leading/trailing whitespace from each line
- Trims leading/trailing blank lines

**Signature:**

```csharp
public static string NormalizeWhitespace(string s)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `S` | `string` | Yes | The s |

**Returns:** `string`


---

#### RegisterDefaultExtractors()

Register all built-in extractors with the global registry.

This function should be called once at application startup to register
the default extractors (PlainText, Markdown, XML, etc.).

**Note:** This is called automatically on first extraction operation.
Explicit calling is optional.

**Signature:**

```csharp
public static void RegisterDefaultExtractors()
```

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### ListPostProcessors()

List all registered post-processor names.

Returns a vector of all post-processor names currently registered in the
global registry.

**Returns:**

- `Ok(Vec<String>)` - Vector of post-processor names
- `Err(...)` if the registry lock is poisoned

**Signature:**

```csharp
public static List<string> ListPostProcessors()
```

**Returns:** `List<string>`

**Errors:** Throws `Error`.


---

#### SanitizeFilename()

Sanitize a file path to return only the filename (no directory).

Prevents PII from appearing in traces.

**Signature:**

```csharp
public static string SanitizeFilename(string path)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Path` | `string` | Yes | Path to the file |

**Returns:** `string`


---

#### SanitizePath()

Sanitize a file path to return only the filename.

Prevents PII (personally identifiable information) from appearing in
traces by only recording filenames instead of full paths.

**Signature:**

```csharp
public static string SanitizePath(string path)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Path` | `string` | Yes | Path to the file |

**Returns:** `string`


---

#### IsValidUtf8()

Validates bytes as UTF-8 without conversion to string slice.

Returns `true` if the bytes represent valid UTF-8, `false` otherwise.
This is useful when you only need to check validity without constructing a string.

**Returns:**

`true` if valid UTF-8, `false` otherwise.

# Performance

This function is optimized for early exit on invalid sequences.

**Signature:**

```csharp
public static bool IsValidUtf8(byte[] bytes)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Bytes` | `byte[]` | Yes | The byte slice to validate |

**Returns:** `bool`


---

#### CleanExtractedText()

**Signature:**

```csharp
public static string CleanExtractedText(string text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text |

**Returns:** `string`


---

#### ReduceTokens()

Reduces token count in text while preserving meaning and structure.

This function removes stopwords, redundancy, and applies compression techniques
based on the specified reduction level. Supports 64 languages with automatic
stopword removal and optional semantic clustering.

**Returns:**

Returns the reduced text with preserved structure (markdown, code blocks).

**Errors:**

Returns an error if the language hint is invalid or stopwords cannot be loaded.

**Signature:**

```csharp
public static string ReduceTokens(string text, TokenReductionConfig config, string? languageHint = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The input text to reduce |
| `Config` | `TokenReductionConfig` | Yes | Configuration specifying reduction level and options |
| `LanguageHint` | `string?` | No | Optional ISO 639-3 language code (e.g., "eng", "spa") |

**Returns:** `string`

**Errors:** Throws `Error`.


---

#### BatchReduceTokens()

Reduces token count for multiple texts efficiently using parallel processing.

This function processes multiple texts in parallel using Rayon, providing
significant performance improvements for batch operations. All texts use the
same configuration and language hint for consistency.

**Returns:**

Returns a vector of reduced texts in the same order as the input.

**Errors:**

Returns an error if the language hint is invalid or stopwords cannot be loaded.

**Signature:**

```csharp
public static List<string> BatchReduceTokens(List<string> texts, TokenReductionConfig config, string? languageHint = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Texts` | `List<string>` | Yes | Slice of text references to reduce |
| `Config` | `TokenReductionConfig` | Yes | Configuration specifying reduction level and options |
| `LanguageHint` | `string?` | No | Optional ISO 639-3 language code (e.g., "eng", "spa") |

**Returns:** `List<string>`

**Errors:** Throws `Error`.


---

#### Bold()

Create a bold annotation for the given byte range.

**Signature:**

```csharp
public static TextAnnotation Bold(uint start, uint end)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Start` | `uint` | Yes | The start |
| `End` | `uint` | Yes | The end |

**Returns:** `TextAnnotation`


---

#### Italic()

Create an italic annotation for the given byte range.

**Signature:**

```csharp
public static TextAnnotation Italic(uint start, uint end)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Start` | `uint` | Yes | The start |
| `End` | `uint` | Yes | The end |

**Returns:** `TextAnnotation`


---

#### Underline()

Create an underline annotation for the given byte range.

**Signature:**

```csharp
public static TextAnnotation Underline(uint start, uint end)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Start` | `uint` | Yes | The start |
| `End` | `uint` | Yes | The end |

**Returns:** `TextAnnotation`


---

#### Link()

Create a link annotation for the given byte range.

**Signature:**

```csharp
public static TextAnnotation Link(uint start, uint end, string url, string? title = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Start` | `uint` | Yes | The start |
| `End` | `uint` | Yes | The end |
| `Url` | `string` | Yes | The URL to fetch |
| `Title` | `string?` | No | The title |

**Returns:** `TextAnnotation`


---

#### Code()

Create a code (inline) annotation for the given byte range.

**Signature:**

```csharp
public static TextAnnotation Code(uint start, uint end)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Start` | `uint` | Yes | The start |
| `End` | `uint` | Yes | The end |

**Returns:** `TextAnnotation`


---

#### Strikethrough()

Create a strikethrough annotation for the given byte range.

**Signature:**

```csharp
public static TextAnnotation Strikethrough(uint start, uint end)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Start` | `uint` | Yes | The start |
| `End` | `uint` | Yes | The end |

**Returns:** `TextAnnotation`


---

#### Subscript()

Create a subscript annotation for the given byte range.

**Signature:**

```csharp
public static TextAnnotation Subscript(uint start, uint end)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Start` | `uint` | Yes | The start |
| `End` | `uint` | Yes | The end |

**Returns:** `TextAnnotation`


---

#### Superscript()

Create a superscript annotation for the given byte range.

**Signature:**

```csharp
public static TextAnnotation Superscript(uint start, uint end)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Start` | `uint` | Yes | The start |
| `End` | `uint` | Yes | The end |

**Returns:** `TextAnnotation`


---

#### FontSize()

Create a font size annotation for the given byte range.

**Signature:**

```csharp
public static TextAnnotation FontSize(uint start, uint end, string value)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Start` | `uint` | Yes | The start |
| `End` | `uint` | Yes | The end |
| `Value` | `string` | Yes | The value |

**Returns:** `TextAnnotation`


---

#### Color()

Create a color annotation for the given byte range.

**Signature:**

```csharp
public static TextAnnotation Color(uint start, uint end, string value)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Start` | `uint` | Yes | The start |
| `End` | `uint` | Yes | The end |
| `Value` | `string` | Yes | The value |

**Returns:** `TextAnnotation`


---

#### Highlight()

Create a highlight annotation for the given byte range.

**Signature:**

```csharp
public static TextAnnotation Highlight(uint start, uint end)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Start` | `uint` | Yes | The start |
| `End` | `uint` | Yes | The end |

**Returns:** `TextAnnotation`


---

#### ClassifyUri()

Classify a URL string into the appropriate `UriKind`.

- `mailto:` → `Email`
- `#` prefix → `Anchor`
- everything else → `Hyperlink`

**Signature:**

```csharp
public static UriKind ClassifyUri(string url)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Url` | `string` | Yes | The URL to fetch |

**Returns:** `UriKind`


---

#### SafeDecode()

Decode raw bytes into UTF-8, using heuristics and fallback encodings when necessary.

The function prefers an explicit `encoding`, falls back to the cached guess, probes
an encoding detector, and finally tries a small curated list before returning a
mojibake-cleaned string.

**Signature:**

```csharp
public static string SafeDecode(byte[] byteData, string? encoding = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ByteData` | `byte[]` | Yes | The byte data |
| `Encoding` | `string?` | No | The encoding |

**Returns:** `string`


---

#### CalculateTextConfidence()

Estimate how trustworthy a decoded string is on a 0.0–1.0 scale.

Scores close to 1.0 indicate mostly printable characters, whereas lower scores
point to mojibake, control characters, or suspicious character mixes.

**Signature:**

```csharp
public static double CalculateTextConfidence(string text)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text |

**Returns:** `double`


---

#### ChunkText()

Split text into chunks with optional page boundary tracking.

This is the primary API function for chunking text. It supports both plain text
and Markdown with configurable chunk size, overlap, and page boundary mapping.

**Returns:**

A ChunkingResult containing all chunks and their metadata.

**Signature:**

```csharp
public static ChunkingResult ChunkText(string text, ChunkingConfig config, List<PageBoundary>? pageBoundaries = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text to split into chunks |
| `Config` | `ChunkingConfig` | Yes | Chunking configuration (max size, overlap, type) |
| `PageBoundaries` | `List<PageBoundary>?` | No | Optional page boundary markers for mapping chunks to pages |

**Returns:** `ChunkingResult`

**Errors:** Throws `Error`.


---

#### ChunkTextWithHeadingSource()

Chunk text with an optional separate markdown source for heading context resolution.

When `heading_source` is provided, it is used instead of `text` for building the
heading map. This is needed when `text` is plain text (no markdown headings) but
the original document had headings that were stripped during rendering.

**Signature:**

```csharp
public static ChunkingResult ChunkTextWithHeadingSource(string text, ChunkingConfig config, List<PageBoundary>? pageBoundaries = null, string? headingSource = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text |
| `Config` | `ChunkingConfig` | Yes | The configuration options |
| `PageBoundaries` | `List<PageBoundary>?` | No | The page boundaries |
| `HeadingSource` | `string?` | No | The heading source |

**Returns:** `ChunkingResult`

**Errors:** Throws `Error`.


---

#### ChunkTextsBatch()

Batch process multiple texts with the same configuration.

This convenience function applies the same chunking configuration to multiple
texts in sequence.

**Returns:**

A vector of ChunkingResult objects, one per input text.

**Errors:**

Returns an error if chunking any individual text fails.

**Signature:**

```csharp
public static List<ChunkingResult> ChunkTextsBatch(List<string> texts, ChunkingConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Texts` | `List<string>` | Yes | Slice of text strings to chunk |
| `Config` | `ChunkingConfig` | Yes | Chunking configuration to apply to all texts |

**Returns:** `List<ChunkingResult>`

**Errors:** Throws `Error`.


---

#### ChunkSemantic()

Split text into semantically coherent chunks.

Splits text into fine-grained segments, detects structural (and optionally
embedding-based) topic boundaries, then merges segments into chunks that
respect those boundaries and the configured size budget.

**Signature:**

```csharp
public static ChunkingResult ChunkSemantic(string text, ChunkingConfig config, List<PageBoundary>? pageBoundaries = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text |
| `Config` | `ChunkingConfig` | Yes | The configuration options |
| `PageBoundaries` | `List<PageBoundary>?` | No | The page boundaries |

**Returns:** `ChunkingResult`

**Errors:** Throws `Error`.


---

#### Normalize()

L2-normalize a vector.

**Signature:**

```csharp
public static List<float> Normalize(List<float> v)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `V` | `List<float>` | Yes | The v |

**Returns:** `List<float>`


---

#### GetPreset()

Get a preset by name.

**Signature:**

```csharp
public static string? GetPreset(string name)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Name` | `string` | Yes | The name |

**Returns:** `string?`


---

#### ListPresets()

List all available preset names.

**Signature:**

```csharp
public static List<string> ListPresets()
```

**Returns:** `List<string>`


---

#### WarmModel()

Eagerly download and cache an embedding model without returning the handle.

This triggers the same download and initialization as `get_or_init_engine`
but discards the result, making it suitable for cache-warming scenarios
where the caller doesn't need to use the model immediately.

**Note**: This function downloads AND initializes the ONNX model, which
requires ONNX Runtime and uses significant memory. For download-only
scenarios (e.g., init containers), use `download_model` instead.

**Signature:**

```csharp
public static void WarmModel(EmbeddingModelType modelType, string? cacheDir = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ModelType` | `EmbeddingModelType` | Yes | The embedding model type |
| `CacheDir` | `string?` | No | The cache dir |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### DownloadModel()

Download an embedding model's files without initializing ONNX Runtime.

Downloads the model files (ONNX model, tokenizer, config) from HuggingFace
to the cache directory. Subsequent calls to `warm_model` or
`get_or_init_engine` will find the files cached and skip the download step.

This is ideal for init containers or CI environments where you want to
pre-populate the cache without loading models into memory.

**Signature:**

```csharp
public static void DownloadModel(EmbeddingModelType modelType, string? cacheDir = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `ModelType` | `EmbeddingModelType` | Yes | The embedding model type |
| `CacheDir` | `string?` | No | The cache dir |

**Returns:** `void`

**Errors:** Throws `Error`.


---

#### CalculateOptimalDpi()

Calculate optimal DPI with min/max constraints

**Signature:**

```csharp
public static int CalculateOptimalDpi(double pageWidth, double pageHeight, int targetDpi, int maxDimension, int minDpi, int maxDpi)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `PageWidth` | `double` | Yes | The page width |
| `PageHeight` | `double` | Yes | The page height |
| `TargetDpi` | `int` | Yes | The target dpi |
| `MaxDimension` | `int` | Yes | The max dimension |
| `MinDpi` | `int` | Yes | The min dpi |
| `MaxDpi` | `int` | Yes | The max dpi |

**Returns:** `int`


---

#### DetectLanguages()

Detect languages in text using whatlang.

Returns a list of detected language codes (ISO 639-3 format).
Returns `null` if no languages could be detected with sufficient confidence.

**Signature:**

```csharp
public static List<string>? DetectLanguages(string text, LanguageDetectionConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text to analyze for language detection |
| `Config` | `LanguageDetectionConfig` | Yes | Optional configuration for language detection |

**Returns:** `List<string>?`

**Errors:** Throws `Error`.


---

#### ExtractKeywords()

Extract keywords from text using the specified algorithm.

This is the unified entry point for keyword extraction. The algorithm
used is determined by `config.algorithm`.

**Returns:**

A vector of keywords sorted by relevance (highest score first).

**Errors:**

Returns an error if:
- The specified algorithm feature is not enabled
- Keyword extraction fails

**Signature:**

```csharp
public static List<Keyword> ExtractKeywords(string text, KeywordConfig config)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Text` | `string` | Yes | The text to extract keywords from |
| `Config` | `KeywordConfig` | Yes | Keyword extraction configuration |

**Returns:** `List<Keyword>`

**Errors:** Throws `Error`.


---

#### ComputeHash()

Compute a blake3 hash string from input data.

Returns a 32-character hex string (128 bits of blake3 output).

**Signature:**

```csharp
public static string ComputeHash(string data)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Data` | `string` | Yes | The data |

**Returns:** `string`


---

#### RenderPdfPageToPng()

Render a single PDF page to a PNG-encoded byte buffer.

**Errors:**

Returns an error if the PDF is invalid, the page index is out of bounds,
or if the page fails to render.

**Signature:**

```csharp
public static byte[] RenderPdfPageToPng(byte[] pdfBytes, nuint pageIndex, int? dpi = null, string? password = null)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `PdfBytes` | `byte[]` | Yes | The pdf bytes |
| `PageIndex` | `nuint` | Yes | The page index |
| `Dpi` | `int?` | No | The dpi |
| `Password` | `string?` | No | The password |

**Returns:** `byte[]`

**Errors:** Throws `Error`.


---

#### ExtractTextFromPdf()

**Signature:**

```csharp
public static string ExtractTextFromPdf(byte[] pdfBytes)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `PdfBytes` | `byte[]` | Yes | The pdf bytes |

**Returns:** `string`

**Errors:** Throws `Error`.


---

#### SerializeToToon()

Serialize an `ExtractionResult` to TOON (Token-Oriented Object Notation).

TOON is a token-efficient alternative to JSON for LLM prompts.
Losslessly convertible to/from JSON but uses fewer tokens.

**Signature:**

```csharp
public static string SerializeToToon(ExtractionResult result)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Result` | `ExtractionResult` | Yes | The extraction result |

**Returns:** `string`

**Errors:** Throws `Error`.


---

#### SerializeToJson()

Serialize an `ExtractionResult` to pretty-printed JSON.

**Signature:**

```csharp
public static string SerializeToJson(ExtractionResult result)
```

**Parameters:**

| Name | Type | Required | Description |
|------|------|----------|-------------|
| `Result` | `ExtractionResult` | Yes | The extraction result |

**Returns:** `string`

**Errors:** Throws `Error`.


---

### Types

#### AccelerationConfig

Hardware acceleration configuration for ONNX Runtime models.

Controls which execution provider (CPU, CoreML, CUDA, TensorRT) is used
for inference in layout detection and embedding generation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Provider` | `ExecutionProviderType` | `ExecutionProviderType.Auto` | Execution provider to use for ONNX inference. |
| `DeviceId` | `uint` | — | GPU device ID (for CUDA/TensorRT). Ignored for CPU/CoreML/Auto. |


---

#### AnchorProperties

Properties for anchored drawings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `BehindDoc` | `bool` | — | Behind doc |
| `LayoutInCell` | `bool` | — | Layout in cell |
| `RelativeHeight` | `long?` | `null` | Relative height |
| `PositionH` | `string?` | `null` | Position h |
| `PositionV` | `string?` | `null` | Position v |
| `WrapType` | `string` | — | Wrap type |


---

#### ApiDoc

OpenAPI documentation structure.

Defines all endpoints, request/response schemas, and examples
for the Kreuzberg document extraction API.


---

#### ApiState

API server state.

Holds the default extraction configuration loaded from config file
(via discovery or explicit path). Per-request configs override these defaults.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `DefaultConfig` | `ExtractionConfig` | — | Default extraction configuration |
| `ExtractionService` | `string` | — | Tower service for extraction requests. Wrapped in `Arc<Mutex>` because `BoxCloneService` is `Send` but not `Sync`, while `ApiState` must be `Clone + Sync` for Axum's state requirement. The lock is held only long enough to clone the service. |


---

#### ArchiveEntry

A single file extracted from an archive.

When archives (ZIP, TAR, 7Z, GZIP) are extracted with recursive extraction
enabled, each processable file produces its own full `ExtractionResult`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Path` | `string` | — | Archive-relative file path (e.g. "folder/document.pdf"). |
| `MimeType` | `string` | — | Detected MIME type of the file. |
| `Result` | `ExtractionResult` | — | Full extraction result for this file. |


---

#### ArchiveMetadata

Archive (ZIP/TAR/7Z) metadata.

Extracted from compressed archive files containing file lists and size information.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Format` | `string` | — | Archive format ("ZIP", "TAR", "7Z", etc.) |
| `FileCount` | `nuint` | — | Total number of files in the archive |
| `FileList` | `List<string>` | `new List<string>()` | List of file paths within the archive |
| `TotalSize` | `nuint` | — | Total uncompressed size in bytes |
| `CompressedSize` | `nuint?` | `null` | Compressed size in bytes (if available) |


---

#### BBox

Bounding box in original image coordinates (x1, y1) top-left, (x2, y2) bottom-right.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `X1` | `float` | — | X1 |
| `Y1` | `float` | — | Y1 |
| `X2` | `float` | — | X2 |
| `Y2` | `float` | — | Y2 |


---

#### BatchExtractFilesParams

Request parameters for batch file extraction.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Paths` | `List<string>` | — | Paths to files to extract |
| `Config` | `object?` | `null` | Extraction configuration (JSON object) |
| `PdfPassword` | `string?` | `null` | Password for encrypted PDFs |
| `FileConfigs` | `List<object?>?` | `null` | Per-file extraction configuration overrides (parallel array to paths). Each entry is either null (use default) or a FileExtractionConfig JSON object. |
| `ResponseFormat` | `string?` | `null` | Wire format for the response: "json" (default) or "toon" |


---

#### BibtexMetadata

BibTeX bibliography metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `EntryCount` | `nuint` | — | Number of entrys |
| `CitationKeys` | `List<string>` | `new List<string>()` | Citation keys |
| `Authors` | `List<string>` | `new List<string>()` | Authors |
| `YearRange` | `YearRange?` | `null` | Year range (year range) |
| `EntryTypes` | `Dictionary<string, nuint>?` | `new Dictionary<string, nuint>()` | Entry types |


---

#### ByteBufferPool

Convenience type alias for a pooled Vec<u8>.


---

#### CacheClearResponse

Cache clear response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Directory` | `string` | — | Cache directory path |
| `RemovedFiles` | `nuint` | — | Number of files removed |
| `FreedMb` | `double` | — | Space freed in MB |


---

#### CacheStatsResponse

Cache statistics response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Directory` | `string` | — | Cache directory path |
| `TotalFiles` | `nuint` | — | Total number of cache files |
| `TotalSizeMb` | `double` | — | Total cache size in MB |
| `AvailableSpaceMb` | `double` | — | Available disk space in MB |
| `OldestFileAgeDays` | `double` | — | Age of oldest file in days |
| `NewestFileAgeDays` | `double` | — | Age of newest file in days |


---

#### CacheWarmParams

Request parameters for cache warm (model download).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `AllEmbeddings` | `bool` | — | Download all embedding model presets |
| `EmbeddingModel` | `string?` | `null` | Specific embedding preset name to download (e.g. "balanced", "speed", "quality") |


---

#### CharData

Character information extracted from PDF with font metrics.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | The character text content |
| `X` | `float` | — | X position in PDF units |
| `Y` | `float` | — | Y position in PDF units |
| `FontSize` | `float` | — | Font size in points |
| `Width` | `float` | — | Character width in PDF units |
| `Height` | `float` | — | Character height in PDF units |
| `IsBold` | `bool` | — | Whether the font is bold (from pdfium force-bold flag) |
| `IsItalic` | `bool` | — | Whether the font is italic |
| `BaselineY` | `float` | — | Baseline Y position (from character origin, falls back to bounds bottom) |


---

#### Chunk

A text chunk with optional embedding and metadata.

Chunks are created when chunking is enabled in `ExtractionConfig`. Each chunk
contains the text content, optional embedding vector (if embedding generation
is configured), and metadata about its position in the document.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The text content of this chunk. |
| `ChunkType` | `ChunkType` | — | Semantic structural classification of this chunk. Assigned by the heuristic classifier based on content patterns and heading context. Defaults to `ChunkType.Unknown` when no rule matches. |
| `Embedding` | `List<float>?` | `null` | Optional embedding vector for this chunk. Only populated when `EmbeddingConfig` is provided in chunking configuration. The dimensionality depends on the chosen embedding model. |
| `Metadata` | `ChunkMetadata` | — | Metadata about this chunk's position and properties. |


---

#### ChunkMetadata

Metadata about a chunk's position in the original document.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ByteStart` | `nuint` | — | Byte offset where this chunk starts in the original text (UTF-8 valid boundary). |
| `ByteEnd` | `nuint` | — | Byte offset where this chunk ends in the original text (UTF-8 valid boundary). |
| `TokenCount` | `nuint?` | `null` | Number of tokens in this chunk (if available). This is calculated by the embedding model's tokenizer if embeddings are enabled. |
| `ChunkIndex` | `nuint` | — | Zero-based index of this chunk in the document. |
| `TotalChunks` | `nuint` | — | Total number of chunks in the document. |
| `FirstPage` | `nuint?` | `null` | First page number this chunk spans (1-indexed). Only populated when page tracking is enabled in extraction configuration. |
| `LastPage` | `nuint?` | `null` | Last page number this chunk spans (1-indexed, equal to first_page for single-page chunks). Only populated when page tracking is enabled in extraction configuration. |
| `HeadingContext` | `HeadingContext?` | `null` | Heading context when using Markdown chunker. Contains the heading hierarchy this chunk falls under. Only populated when `ChunkerType.Markdown` is used. |


---

#### ChunkRequest

Chunk request with text and configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | Text to chunk (must not be empty) |
| `Config` | `string?` | `null` | Optional chunking configuration |
| `ChunkerType` | `string` | — | Chunker type (text, markdown, yaml, or semantic) |


---

#### ChunkResponse

Chunk response with chunks and metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Chunks` | `List<string>` | — | List of chunks |
| `ChunkCount` | `nuint` | — | Total number of chunks |
| `Config` | `string` | — | Configuration used for chunking |
| `InputSizeBytes` | `nuint` | — | Input text size in bytes |
| `ChunkerType` | `string` | — | Chunker type used for chunking |


---

#### ChunkTextParams

Request parameters for text chunking.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | Text content to split into chunks |
| `MaxCharacters` | `nuint?` | `null` | Maximum characters per chunk (default: 2000) |
| `Overlap` | `nuint?` | `null` | Number of overlapping characters between chunks (default: 100) |
| `ChunkerType` | `string?` | `null` | Chunker type: "text", "markdown", "yaml", or "semantic" (default: "text") |
| `TopicThreshold` | `float?` | `null` | Topic threshold for semantic chunking (0.0-1.0, default: 0.75) |


---

#### ChunkingConfig

Chunking configuration.

Configures text chunking for document content, including chunk size,
overlap, trimming behavior, and optional embeddings.

Use `..the default constructor` when constructing to allow for future field additions:

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MaxCharacters` | `nuint` | `1000` | Maximum size per chunk (in units determined by `sizing`). When `sizing` is `Characters` (default), this is the max character count. When using token-based sizing, this is the max token count. Default: 1000 |
| `Overlap` | `nuint` | `200` | Overlap between chunks (in units determined by `sizing`). Default: 200 |
| `Trim` | `bool` | `true` | Whether to trim whitespace from chunk boundaries. Default: true |
| `ChunkerType` | `ChunkerType` | `ChunkerType.Text` | Type of chunker to use (Text or Markdown). Default: Text |
| `Embedding` | `EmbeddingConfig?` | `null` | Optional embedding configuration for chunk embeddings. |
| `Preset` | `string?` | `null` | Use a preset configuration (overrides individual settings if provided). |
| `Sizing` | `ChunkSizing` | `ChunkSizing.Characters` | How to measure chunk size. Default: `Characters` (Unicode character count). Enable `chunking-tiktoken` or `chunking-tokenizers` features for token-based sizing. |
| `PrependHeadingContext` | `bool` | `false` | When `true` and `chunker_type` is `Markdown`, prepend the heading hierarchy path (e.g. `"# Title > ## Section\n\n"`) to each chunk's content string. This is useful for RAG pipelines where each chunk needs self-contained context about its position in the document structure. Default: `false` |
| `TopicThreshold` | `float?` | `null` | Optional cosine similarity threshold for semantic topic boundary detection. Only used when `chunker_type` is `Semantic` and an `EmbeddingConfig` is provided. You almost never need to set this. When omitted, defaults to `0.75` which works well for most documents. Lower values detect more topic boundaries (more, smaller chunks); higher values detect fewer. Range: `0.0..=1.0`. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public ChunkingConfig CreateDefault()
```


---

#### ChunkingResult

Result of a text chunking operation.

Contains the generated chunks and metadata about the chunking.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Chunks` | `List<Chunk>` | — | List of text chunks |
| `ChunkCount` | `nuint` | — | Total number of chunks generated |


---

#### CitationMetadata

Citation file metadata (RIS, PubMed, EndNote).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `CitationCount` | `nuint` | — | Number of citations |
| `Format` | `string?` | `null` | Format |
| `Authors` | `List<string>` | `new List<string>()` | Authors |
| `YearRange` | `YearRange?` | `null` | Year range (year range) |
| `Dois` | `List<string>` | `new List<string>()` | Dois |
| `Keywords` | `List<string>` | `new List<string>()` | Keywords |


---

#### CommonPdfMetadata

Common metadata fields extracted from a PDF.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Title` | `string?` | `null` | Title |
| `Subject` | `string?` | `null` | Subject |
| `Authors` | `List<string>?` | `null` | Authors |
| `Keywords` | `List<string>?` | `null` | Keywords |
| `CreatedAt` | `string?` | `null` | Created at |
| `ModifiedAt` | `string?` | `null` | Modified at |
| `CreatedBy` | `string?` | `null` | Created by |


---

#### ContentFilterConfig

Cross-extractor content filtering configuration.

Controls whether "furniture" content (headers, footers, page numbers,
watermarks, repeating text) is included in or stripped from extraction
results. Applies across all extractors (PDF, DOCX, RTF, ODT, HTML, etc.)
with format-specific implementation.

When `null` on `ExtractionConfig`, each extractor uses its current
default behavior unchanged.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IncludeHeaders` | `bool` | `false` | Include running headers in extraction output. - PDF: Disables top-margin furniture stripping and prevents the layout model from treating `PageHeader`-classified regions as furniture. - DOCX: Includes document headers in text output. - RTF/ODT: Headers already included; this is a no-op when true. - HTML/EPUB: Keeps `<header>` element content. Default: `false` (headers are stripped or excluded). |
| `IncludeFooters` | `bool` | `false` | Include running footers in extraction output. - PDF: Disables bottom-margin furniture stripping and prevents the layout model from treating `PageFooter`-classified regions as furniture. - DOCX: Includes document footers in text output. - RTF/ODT: Footers already included; this is a no-op when true. - HTML/EPUB: Keeps `<footer>` element content. Default: `false` (footers are stripped or excluded). |
| `StripRepeatingText` | `bool` | `true` | Enable the heuristic cross-page repeating text detector. When `true` (default), text that repeats verbatim across a supermajority of pages is classified as furniture and stripped.  Disable this if brand names or repeated headings are being incorrectly removed by the heuristic. Note: when a layout-detection model is active, the model may independently classify page-header / page-footer regions as furniture on a per-page basis. To preserve those regions, set `include_headers = true` and/or `include_footers = true` in addition to disabling this flag. Primarily affects PDF extraction. Default: `true`. |
| `IncludeWatermarks` | `bool` | `false` | Include watermark text in extraction output. - PDF: Keeps watermark artifacts and arXiv identifiers. - Other formats: No effect currently. Default: `false` (watermarks are stripped). |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public ContentFilterConfig CreateDefault()
```


---

#### ContributorRole

JATS contributor with role.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `Role` | `string?` | `null` | Role |


---

#### CsvMetadata

CSV/TSV file metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `RowCount` | `nuint` | — | Number of rows |
| `ColumnCount` | `nuint` | — | Number of columns |
| `Delimiter` | `string?` | `null` | Delimiter |
| `HasHeader` | `bool` | — | Whether header |
| `ColumnTypes` | `List<string>?` | `new List<string>()` | Column types |


---

#### CustomProperties

Custom properties from docProps/custom.xml

Maps property names to their values. Values are converted to JSON types
based on the VT (Variant Type) specified in the XML.


---

#### DbfFieldInfo

dBASE field information.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The name |
| `FieldType` | `string` | — | Field type |


---

#### DbfMetadata

dBASE (DBF) file metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `RecordCount` | `nuint` | — | Number of records |
| `FieldCount` | `nuint` | — | Number of fields |
| `Fields` | `List<DbfFieldInfo>` | `new List<DbfFieldInfo>()` | Fields |


---

#### DepthValidator

Helper struct for validating nesting depth.


---

#### DetectMimeTypeParams

Request parameters for MIME type detection.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Path` | `string` | — | Path to the file |
| `UseContent` | `bool` | — | Use content-based detection (default: true) |


---

#### DetectResponse

MIME type detection response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MimeType` | `string` | — | Detected MIME type |
| `Filename` | `string?` | `null` | Original filename (if provided) |


---

#### DetectedBoundary

A detected structural boundary in the text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ByteOffset` | `nuint` | — | Byte offset of the start of the line in the original text. |
| `IsHeader` | `bool` | — | Whether this boundary looks like a header/section title. |


---

#### DetectionResult

Page-level detection result containing all detections and page metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PageWidth` | `uint` | — | Page width |
| `PageHeight` | `uint` | — | Page height |
| `Detections` | `List<LayoutDetection>` | — | Detections |


---

#### DjotContent

Comprehensive Djot document structure with semantic preservation.

This type captures the full richness of Djot markup, including:
- Block-level structures (headings, lists, blockquotes, code blocks, etc.)
- Inline formatting (emphasis, strong, highlight, subscript, superscript, etc.)
- Attributes (classes, IDs, key-value pairs)
- Links, images, footnotes
- Math expressions (inline and display)
- Tables with full structure

Available when the `djot` feature is enabled.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PlainText` | `string` | — | Plain text representation for backwards compatibility |
| `Blocks` | `List<FormattedBlock>` | — | Structured block-level content |
| `Metadata` | `Metadata` | — | Metadata from YAML frontmatter |
| `Tables` | `List<string>` | — | Extracted tables as structured data |
| `Images` | `List<DjotImage>` | — | Extracted images with metadata |
| `Links` | `List<DjotLink>` | — | Extracted links with URLs |
| `Footnotes` | `List<Footnote>` | — | Footnote definitions |
| `Attributes` | `List<string>` | — | Attributes mapped by element identifier (if present) |


---

#### DjotImage

Image element in Djot.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Src` | `string` | — | Image source URL or path |
| `Alt` | `string` | — | Alternative text |
| `Title` | `string?` | `null` | Optional title |
| `Attributes` | `string?` | `null` | Element attributes |


---

#### DjotLink

Link element in Djot.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | Link URL |
| `Text` | `string` | — | Link text content |
| `Title` | `string?` | `null` | Optional title |
| `Attributes` | `string?` | `null` | Element attributes |


---

#### DoclingCompatResponse

OpenWebUI "Docling" engine response format.

Returned by `POST /v1/convert/file` for docling-serve compatibility.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Document` | `string` | — | Converted document content |
| `Status` | `string` | — | Processing status |


---

#### DocumentNode

A single node in the document tree.

Each node has deterministic `id`, typed `content`, optional `parent`/`children`
for tree structure, and metadata like page number, bounding box, and content layer.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Deterministic identifier (hash of content + position). |
| `Content` | `NodeContent` | — | Node content — tagged enum, type-specific data only. |
| `Parent` | `uint?` | `null` | Parent node index (`null` = root-level node). |
| `Children` | `List<uint>` | — | Child node indices in reading order. |
| `ContentLayer` | `ContentLayer` | — | Content layer classification. |
| `Page` | `uint?` | `null` | Page number where this node starts (1-indexed). |
| `PageEnd` | `uint?` | `null` | Page number where this node ends (for multi-page tables/sections). |
| `Bbox` | `string?` | `null` | Bounding box in document coordinates. |
| `Annotations` | `List<TextAnnotation>` | — | Inline annotations (formatting, links) on this node's text content. Only meaningful for text-carrying nodes; empty for containers. |
| `Attributes` | `Dictionary<string, string>?` | `null` | Format-specific key-value attributes. Extensible bag for data that doesn't warrant a typed field: CSS classes, LaTeX environment names, Excel cell formulas, slide layout names, etc. |


---

#### DocumentRelationship

A resolved relationship between two nodes in the document tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Source` | `uint` | — | Source node index (the referencing node). |
| `Target` | `uint` | — | Target node index (the referenced node). |
| `Kind` | `RelationshipKind` | — | Semantic kind of the relationship. |


---

#### DocumentStructure

Top-level structured document representation.

A flat array of nodes with index-based parent/child references forming a tree.
Root-level nodes have `parent: None`. Use `body_roots()` and `furniture_roots()`
to iterate over top-level content by layer.

# Validation

Call `validate()` after construction to verify all node indices are in bounds
and parent-child relationships are bidirectionally consistent.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Nodes` | `List<DocumentNode>` | `new List<DocumentNode>()` | All nodes in document/reading order. |
| `SourceFormat` | `string?` | `null` | Origin format identifier (e.g. "docx", "pptx", "html", "pdf"). Allows renderers to apply format-aware heuristics when converting the document tree to output formats. |
| `Relationships` | `List<DocumentRelationship>` | `new List<DocumentRelationship>()` | Resolved relationships between nodes (footnote refs, citations, anchor links, etc.). Populated during derivation from the internal document representation. Empty when no relationships are detected. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public DocumentStructure CreateDefault()
```


---

#### DocxMetadata

Word document metadata.

Extracted from DOCX files using shared Office Open XML metadata extraction.
Integrates with `office_metadata` module for core/app/custom properties.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `CoreProperties` | `string?` | `null` | Core properties from docProps/core.xml (Dublin Core metadata) Contains title, creator, subject, keywords, dates, etc. Shared format across DOCX/PPTX/XLSX documents. |
| `AppProperties` | `string?` | `null` | Application properties from docProps/app.xml (Word-specific statistics) Contains word count, page count, paragraph count, editing time, etc. DOCX-specific variant of Office application properties. |
| `CustomProperties` | `Dictionary<string, object>?` | `new Dictionary<string, object>()` | Custom properties from docProps/custom.xml (user-defined properties) Contains key-value pairs defined by users or applications. Values can be strings, numbers, booleans, or dates. |


---

#### Drawing

A drawing object extracted from `<w:drawing>`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `DrawingType` | `string` | — | Drawing type |
| `Extent` | `string?` | `null` | Extent |
| `DocProperties` | `string?` | `null` | Doc properties |
| `ImageRef` | `string?` | `null` | Image ref |


---

#### Element

Semantic element extracted from document.

Represents a logical unit of content with semantic classification,
unique identifier, and metadata for tracking origin and position.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ElementId` | `string` | — | Unique element identifier |
| `ElementType` | `ElementType` | — | Semantic type of this element |
| `Text` | `string` | — | Text content of the element |
| `Metadata` | `ElementMetadata` | — | Metadata about the element |


---

#### ElementMetadata

Metadata for a semantic element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PageNumber` | `nuint?` | `null` | Page number (1-indexed) |
| `Filename` | `string?` | `null` | Source filename or document name |
| `Coordinates` | `string?` | `null` | Bounding box coordinates if available |
| `ElementIndex` | `nuint?` | `null` | Position index in the element sequence |
| `Additional` | `Dictionary<string, string>` | — | Additional custom metadata |


---

#### EmailAttachment

Email attachment representation.

Contains metadata and optionally the content of an email attachment.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string?` | `null` | Attachment name (from Content-Disposition header) |
| `Filename` | `string?` | `null` | Filename of the attachment |
| `MimeType` | `string?` | `null` | MIME type of the attachment |
| `Size` | `nuint?` | `null` | Size in bytes |
| `IsImage` | `bool` | — | Whether this attachment is an image |
| `Data` | `byte[]?` | `null` | Attachment data (if extracted). Uses `bytes.Bytes` for cheap cloning of large buffers. |


---

#### EmailConfig

Configuration for email extraction.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MsgFallbackCodepage` | `uint?` | `null` | Windows codepage number to use when an MSG file contains no codepage property. Defaults to `null`, which falls back to windows-1252. If an unrecognized or invalid codepage number is supplied (including 0), the behavior silently falls back to windows-1252 — the same as when the MSG file itself contains an unrecognized codepage. No error or warning is emitted. Users should verify output when supplying unusual values. Common values: - 1250: Central European (Polish, Czech, Hungarian, etc.) - 1251: Cyrillic (Russian, Ukrainian, Bulgarian, etc.) - 1252: Western European (default) - 1253: Greek - 1254: Turkish - 1255: Hebrew - 1256: Arabic - 932:  Japanese (Shift-JIS) - 936:  Simplified Chinese (GBK) |


---

#### EmailExtractionResult

Email extraction result.

Complete representation of an extracted email message (.eml or .msg)
including headers, body content, and attachments.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Subject` | `string?` | `null` | Email subject line |
| `FromEmail` | `string?` | `null` | Sender email address |
| `ToEmails` | `List<string>` | — | Primary recipient email addresses |
| `CcEmails` | `List<string>` | — | CC recipient email addresses |
| `BccEmails` | `List<string>` | — | BCC recipient email addresses |
| `Date` | `string?` | `null` | Email date/timestamp |
| `MessageId` | `string?` | `null` | Message-ID header value |
| `PlainText` | `string?` | `null` | Plain text version of the email body |
| `HtmlContent` | `string?` | `null` | HTML version of the email body |
| `CleanedText` | `string` | — | Cleaned/processed text content |
| `Attachments` | `List<EmailAttachment>` | — | List of email attachments |
| `Metadata` | `Dictionary<string, string>` | — | Additional email headers and metadata |


---

#### EmailMetadata

Email metadata extracted from .eml and .msg files.

Includes sender/recipient information, message ID, and attachment list.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `FromEmail` | `string?` | `null` | Sender's email address |
| `FromName` | `string?` | `null` | Sender's display name |
| `ToEmails` | `List<string>` | `new List<string>()` | Primary recipients |
| `CcEmails` | `List<string>` | `new List<string>()` | CC recipients |
| `BccEmails` | `List<string>` | `new List<string>()` | BCC recipients |
| `MessageId` | `string?` | `null` | Message-ID header value |
| `Attachments` | `List<string>` | `new List<string>()` | List of attachment filenames |


---

#### EmbedRequest

Embedding request for generating embeddings from text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Texts` | `List<string>` | — | Text strings to generate embeddings for (at least one non-empty string required) |
| `Config` | `EmbeddingConfig?` | `null` | Optional embedding configuration (model, batch size, etc.) |


---

#### EmbedResponse

Embedding response containing generated embeddings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Embeddings` | `List<List<float>>` | — | Generated embeddings (one per input text) |
| `Model` | `string` | — | Model used for embedding generation |
| `Dimensions` | `nuint` | — | Dimensionality of the embeddings |
| `Count` | `nuint` | — | Number of embeddings generated |


---

#### EmbedTextParams

Request parameters for embedding generation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Texts` | `List<string>` | — | List of text strings to generate embeddings for |
| `Preset` | `string?` | `null` | Embedding preset name (default: "balanced"). Available: "speed", "balanced", "quality" |
| `Model` | `string?` | `null` | LLM model for provider-hosted embeddings (e.g., "openai/text-embedding-3-small"). When set, overrides preset and uses liter-llm for embedding generation. |
| `ApiKey` | `string?` | `null` | API key for the LLM provider (optional, falls back to env). |


---

#### EmbeddedFile

Embedded file descriptor extracted from the PDF name tree.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | The filename as stored in the PDF name tree. |
| `Data` | `byte[]` | — | Raw file bytes from the embedded stream. |
| `MimeType` | `string?` | `null` | MIME type if specified in the filespec, otherwise `null`. |


---

#### EmbeddingConfig

Embedding configuration for text chunks.

Configures embedding generation using ONNX models via the vendored embedding engine.
Requires the `embeddings` feature to be enabled.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `EmbeddingModelType` | `EmbeddingModelType.Preset` | The embedding model to use (defaults to "balanced" preset if not specified) |
| `Normalize` | `bool` | `true` | Whether to normalize embedding vectors (recommended for cosine similarity) |
| `BatchSize` | `nuint` | `32` | Batch size for embedding generation |
| `ShowDownloadProgress` | `bool` | `false` | Show model download progress |
| `CacheDir` | `string?` | `null` | Custom cache directory for model files Defaults to `~/.cache/kreuzberg/embeddings/` if not specified. Allows full customization of model download location. |
| `Acceleration` | `AccelerationConfig?` | `null` | Hardware acceleration for the embedding ONNX model. When set, controls which execution provider (CPU, CUDA, CoreML, TensorRT) is used for inference. Defaults to `null` (auto-select per platform). |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public EmbeddingConfig CreateDefault()
```


---

#### EntityValidator

Helper struct for validating entity/string length.


---

#### EpubMetadata

EPUB metadata (Dublin Core extensions).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Coverage` | `string?` | `null` | Coverage |
| `DcFormat` | `string?` | `null` | Dc format |
| `Relation` | `string?` | `null` | Relation |
| `Source` | `string?` | `null` | Source |
| `DcType` | `string?` | `null` | Dc type |
| `CoverImage` | `string?` | `null` | Cover image |


---

#### ErrorMetadata

Error metadata (for batch operations).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ErrorType` | `string` | — | Error type |
| `Message` | `string` | — | Message |


---

#### ExcelMetadata

Excel/spreadsheet metadata.

Contains information about sheets in Excel, OpenDocument Calc, and other
spreadsheet formats (.xlsx, .xls, .ods, etc.).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `SheetCount` | `nuint` | — | Total number of sheets in the workbook |
| `SheetNames` | `List<string>` | `new List<string>()` | Names of all sheets in order |


---

#### ExcelSheet

Single Excel worksheet.

Represents one sheet from an Excel workbook with its content
converted to Markdown format and dimensional statistics.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Name` | `string` | — | Sheet name as it appears in Excel |
| `Markdown` | `string` | — | Sheet content converted to Markdown tables |
| `RowCount` | `nuint` | — | Number of rows |
| `ColCount` | `nuint` | — | Number of columns |
| `CellCount` | `nuint` | — | Total number of non-empty cells |
| `TableCells` | `List<List<string>>?` | `null` | Pre-extracted table cells (2D vector of cell values) Populated during markdown generation to avoid re-parsing markdown. None for empty sheets. |


---

#### ExcelWorkbook

Excel workbook representation.

Contains all sheets from an Excel file (.xlsx, .xls, etc.) with
extracted content and metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Sheets` | `List<ExcelSheet>` | — | All sheets in the workbook |
| `Metadata` | `Dictionary<string, string>` | — | Workbook-level metadata (author, creation date, etc.) |


---

#### ExtractBytesParams

Request parameters for bytes extraction.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Data` | `string` | — | Base64-encoded file content |
| `MimeType` | `string?` | `null` | Optional MIME type hint (auto-detected if not provided) |
| `Config` | `object?` | `null` | Extraction configuration (JSON object) |
| `PdfPassword` | `string?` | `null` | Password for encrypted PDFs |
| `ResponseFormat` | `string?` | `null` | Wire format for the response: "json" (default) or "toon" |


---

#### ExtractFileParams

Request parameters for file extraction.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Path` | `string` | — | Path to the file to extract |
| `MimeType` | `string?` | `null` | Optional MIME type hint (auto-detected if not provided) |
| `Config` | `object?` | `null` | Extraction configuration (JSON object) |
| `PdfPassword` | `string?` | `null` | Password for encrypted PDFs |
| `ResponseFormat` | `string?` | `null` | Wire format for the response: "json" (default) or "toon" |


---

#### ExtractResponse

Extraction response (list of results).


---

#### ExtractStructuredParams

Request parameters for LLM-based structured extraction.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Path` | `string` | — | File path to extract from |
| `Schema` | `object` | — | JSON schema for structured output |
| `Model` | `string` | — | LLM model (e.g., "openai/gpt-4o") |
| `SchemaName` | `string` | — | Schema name (default: "extraction") |
| `SchemaDescription` | `string?` | `null` | Schema description for the LLM |
| `Prompt` | `string?` | `null` | Custom Jinja2 prompt template |
| `ApiKey` | `string?` | `null` | API key (optional, falls back to env) |
| `Strict` | `bool` | — | Enable strict mode |


---

#### ExtractedImage

Extracted image from a document.

Contains raw image data, metadata, and optional nested OCR results.
Raw bytes allow cross-language compatibility - users can convert to
PIL.Image (Python), Sharp (Node.js), or other formats as needed.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Data` | `byte[]` | — | Raw image data (PNG, JPEG, WebP, etc. bytes). Uses `bytes.Bytes` for cheap cloning of large buffers. |
| `Format` | `string` | — | Image format (e.g., "jpeg", "png", "webp") Uses Cow<'static, str> to avoid allocation for static literals. |
| `ImageIndex` | `nuint` | — | Zero-indexed position of this image in the document/page |
| `PageNumber` | `nuint?` | `null` | Page/slide number where image was found (1-indexed) |
| `Width` | `uint?` | `null` | Image width in pixels |
| `Height` | `uint?` | `null` | Image height in pixels |
| `Colorspace` | `string?` | `null` | Colorspace information (e.g., "RGB", "CMYK", "Gray") |
| `BitsPerComponent` | `uint?` | `null` | Bits per color component (e.g., 8, 16) |
| `IsMask` | `bool` | — | Whether this image is a mask image |
| `Description` | `string?` | `null` | Optional description of the image |
| `OcrResult` | `ExtractionResult?` | `null` | Nested OCR extraction result (if image was OCRed) When OCR is performed on this image, the result is embedded here rather than in a separate collection, making the relationship explicit. |
| `BoundingBox` | `string?` | `null` | Bounding box of the image on the page (PDF coordinates: x0=left, y0=bottom, x1=right, y1=top). Only populated for PDF-extracted images when position data is available from pdfium. |
| `SourcePath` | `string?` | `null` | Original source path of the image within the document archive (e.g., "media/image1.png" in DOCX). Used for rendering image references when the binary data is not extracted. |


---

#### ExtractedInlineImage

Extracted inline image with metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Data` | `byte[]` | — | Uses `bytes.Bytes` for cheap cloning of large buffers. |
| `Format` | `string` | — | Format |
| `Filename` | `string?` | `null` | Filename |
| `Description` | `string?` | `null` | Human-readable description |
| `Dimensions` | `string?` | `null` | Dimensions |
| `Attributes` | `List<string>` | — | Attributes |


---

#### ExtractionConfig

Main extraction configuration.

This struct contains all configuration options for the extraction process.
It can be loaded from TOML, YAML, or JSON files, or created programmatically.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `UseCache` | `bool` | `true` | Enable caching of extraction results |
| `EnableQualityProcessing` | `bool` | `true` | Enable quality post-processing |
| `Ocr` | `OcrConfig?` | `null` | OCR configuration (None = OCR disabled) |
| `ForceOcr` | `bool` | `false` | Force OCR even for searchable PDFs |
| `ForceOcrPages` | `List<nuint>?` | `null` | Force OCR on specific pages only (1-indexed page numbers, must be >= 1). When set, only the listed pages are OCR'd regardless of text layer quality. Unlisted pages use native text extraction. Ignored when `force_ocr` is `true`. Only applies to PDF documents. Duplicates are automatically deduplicated. An `ocr` config is recommended for backend/language selection; defaults are used if absent. |
| `DisableOcr` | `bool` | `false` | Disable OCR entirely, even for images. When `true`, OCR is skipped for all document types. Images return metadata only (dimensions, format, EXIF) without text extraction. PDFs use only native text extraction without OCR fallback. Cannot be `true` simultaneously with `force_ocr`. *Added in v4.7.0.* |
| `Chunking` | `ChunkingConfig?` | `null` | Text chunking configuration (None = chunking disabled) |
| `ContentFilter` | `ContentFilterConfig?` | `null` | Content filtering configuration (None = use extractor defaults). Controls whether document "furniture" (headers, footers, watermarks, repeating text) is included in or stripped from extraction results. See `ContentFilterConfig` for per-field documentation. |
| `Images` | `ImageExtractionConfig?` | `null` | Image extraction configuration (None = no image extraction) |
| `PdfOptions` | `PdfConfig?` | `null` | PDF-specific options (None = use defaults) |
| `TokenReduction` | `TokenReductionOptions?` | `null` | Token reduction configuration (None = no token reduction) |
| `LanguageDetection` | `LanguageDetectionConfig?` | `null` | Language detection configuration (None = no language detection) |
| `Pages` | `PageConfig?` | `null` | Page extraction configuration (None = no page tracking) |
| `Postprocessor` | `PostProcessorConfig?` | `null` | Post-processor configuration (None = use defaults) |
| `HtmlOptions` | `string?` | `null` | HTML to Markdown conversion options (None = use defaults) Configure how HTML documents are converted to Markdown, including heading styles, list formatting, code block styles, and preprocessing options. |
| `HtmlOutput` | `HtmlOutputConfig?` | `null` | Styled HTML output configuration. When set alongside `output_format = OutputFormat.Html`, the extraction pipeline uses `StyledHtmlRenderer` which emits stable `kb-*` CSS class hooks on every structural element and optionally embeds theme CSS or user-supplied CSS in a `<style>` block. When `null`, the existing plain comrak-based HTML renderer is used. |
| `ExtractionTimeoutSecs` | `ulong?` | `null` | Default per-file timeout in seconds for batch extraction. When set, each file in a batch will be canceled after this duration unless overridden by `FileExtractionConfig.timeout_secs`. `null` means no timeout (unbounded extraction time). |
| `MaxConcurrentExtractions` | `nuint?` | `null` | Maximum concurrent extractions in batch operations (None = (num_cpus × 1.5).ceil()). Limits parallelism to prevent resource exhaustion when processing large batches. Defaults to (num_cpus × 1.5).ceil() when not set. |
| `ResultFormat` | `string` | — | Result structure format Controls whether results are returned in unified format (default) with all content in the `content` field, or element-based format with semantic elements (for Unstructured-compatible output). |
| `SecurityLimits` | `string?` | `null` | Security limits for archive extraction. Controls maximum archive size, compression ratio, file count, and other security thresholds to prevent decompression bomb attacks. When `null`, default limits are used (500MB archive, 100:1 ratio, 10K files). |
| `OutputFormat` | `string` | `Plain` | Content text format (default: Plain). Controls the format of the extracted content: - `Plain`: Raw extracted text (default) - `Markdown`: Markdown formatted output - `Djot`: Djot markup format (requires djot feature) - `Html`: HTML formatted output When set to a structured format, extraction results will include formatted output. The `formatted_content` field may be populated when format conversion is applied. |
| `Layout` | `LayoutDetectionConfig?` | `null` | Layout detection configuration (None = layout detection disabled). When set, PDF pages and images are analyzed for document structure (headings, code, formulas, tables, figures, etc.) using RT-DETR models via ONNX Runtime. For PDFs, layout hints override paragraph classification in the markdown pipeline. For images, per-region OCR is performed with markdown formatting based on detected layout classes. Requires the `layout-detection` feature. |
| `IncludeDocumentStructure` | `bool` | `false` | Enable structured document tree output. When true, populates the `document` field on `ExtractionResult` with a hierarchical `DocumentStructure` containing heading-driven section nesting, table grids, content layer classification, and inline annotations. Independent of `result_format` — can be combined with Unified or ElementBased. |
| `Acceleration` | `AccelerationConfig?` | `null` | Hardware acceleration configuration for ONNX Runtime models. Controls execution provider selection for layout detection and embedding models. When `null`, uses platform defaults (CoreML on macOS, CUDA on Linux, CPU on Windows). |
| `CacheNamespace` | `string?` | `null` | Cache namespace for tenant isolation. When set, cache entries are stored under `{cache_dir}/{namespace}/`. Must be alphanumeric, hyphens, or underscores only (max 64 chars). Different namespaces have isolated cache spaces on the same filesystem. |
| `CacheTtlSecs` | `ulong?` | `null` | Per-request cache TTL in seconds. Overrides the global `max_age_days` for this specific extraction. When `0`, caching is completely skipped (no read or write). When `null`, the global TTL applies. |
| `Email` | `EmailConfig?` | `null` | Email extraction configuration (None = use defaults). Currently supports configuring the fallback codepage for MSG files that do not specify one. See `crate.core.config.EmailConfig` for details. |
| `Concurrency` | `string?` | `null` | Concurrency limits for constrained environments (None = use defaults). Controls Rayon thread pool size, ONNX Runtime intra-op threads, and (when `max_concurrent_extractions` is unset) the batch concurrency semaphore. See `crate.core.config.ConcurrencyConfig` for details. |
| `MaxArchiveDepth` | `nuint` | — | Maximum recursion depth for archive extraction (default: 3). Set to 0 to disable recursive extraction (legacy behavior). |
| `TreeSitter` | `TreeSitterConfig?` | `null` | Tree-sitter language pack configuration (None = tree-sitter disabled). When set, enables code file extraction using tree-sitter parsers. Controls grammar download behavior and code analysis options. |
| `StructuredExtraction` | `StructuredExtractionConfig?` | `null` | Structured extraction via LLM (None = disabled). When set, the extracted document content is sent to an LLM with the provided JSON schema. The structured response is stored in `ExtractionResult.structured_output`. |
| `CancelToken` | `string?` | `null` | Cancellation token for this extraction (None = no external cancellation). Pass a `CancellationToken` clone here and call `CancellationToken.cancel` from another thread / task to abort the extraction in progress. The extractor checks the token at safe checkpoints (before lock acquisition, between pages, between batch items) and returns `KreuzbergError.Cancelled` when set. The field is excluded from serialization because `CancellationToken` is a runtime handle, not a configuration value. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public ExtractionConfig CreateDefault()
```


---

#### ExtractionResult

General extraction result used by the core extraction API.

This is the main result type returned by all extraction functions.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The extracted text content |
| `MimeType` | `string` | — | The detected MIME type |
| `Metadata` | `Metadata` | — | Document metadata |
| `Tables` | `List<string>` | `new List<string>()` | Tables extracted from the document |
| `DetectedLanguages` | `List<string>?` | `new List<string>()` | Detected languages |
| `Chunks` | `List<Chunk>?` | `new List<Chunk>()` | Text chunks when chunking is enabled. When chunking configuration is provided, the content is split into overlapping chunks for efficient processing. Each chunk contains the text, optional embeddings (if enabled), and metadata about its position. |
| `Images` | `List<ExtractedImage>?` | `new List<ExtractedImage>()` | Extracted images from the document. When image extraction is enabled via `ImageExtractionConfig`, this field contains all images found in the document with their raw data and metadata. Each image may optionally contain a nested `ocr_result` if OCR was performed. |
| `Pages` | `List<PageContent>?` | `new List<PageContent>()` | Per-page content when page extraction is enabled. When page extraction is configured, the document is split into per-page content with tables and images mapped to their respective pages. |
| `Elements` | `List<Element>?` | `new List<Element>()` | Semantic elements when element-based result format is enabled. When result_format is set to ElementBased, this field contains semantic elements with type classification, unique identifiers, and metadata for Unstructured-compatible element-based processing. |
| `DjotContent` | `DjotContent?` | `null` | Rich Djot content structure (when extracting Djot documents). When extracting Djot documents with structured extraction enabled, this field contains the full semantic structure including: - Block-level elements with nesting - Inline formatting with attributes - Links, images, footnotes - Math expressions - Complete attribute information The `content` field still contains plain text for backward compatibility. Always `null` for non-Djot documents. |
| `OcrElements` | `List<OcrElement>?` | `new List<OcrElement>()` | OCR elements with full spatial and confidence metadata. When OCR is performed with element extraction enabled, this field contains the structured representation of detected text including: - Bounding geometry (rectangles or quadrilaterals) - Confidence scores (detection and recognition) - Rotation information - Hierarchical relationships (Tesseract only) This field preserves all metadata that would otherwise be lost when converting to plain text or markdown output formats. Only populated when `OcrElementConfig.include_elements` is true. |
| `Document` | `DocumentStructure?` | `null` | Structured document tree (when document structure extraction is enabled). When `include_document_structure` is true in `ExtractionConfig`, this field contains the full hierarchical representation of the document including: - Heading-driven section nesting - Table grids with cell-level metadata - Content layer classification (body, header, footer, footnote) - Inline text annotations (formatting, links) - Bounding boxes and page numbers Independent of `result_format` — can be combined with Unified or ElementBased. |
| `QualityScore` | `double?` | `null` | Document quality score from quality analysis. A value between 0.0 and 1.0 indicating the overall text quality. Previously stored in `metadata.additional["quality_score"]`. |
| `ProcessingWarnings` | `List<ProcessingWarning>` | `new List<ProcessingWarning>()` | Non-fatal warnings collected during processing pipeline stages. Captures errors from optional pipeline features (embedding, chunking, language detection, output formatting) that don't prevent extraction but may indicate degraded results. Previously stored as individual keys in `metadata.additional`. |
| `Annotations` | `List<PdfAnnotation>?` | `new List<PdfAnnotation>()` | PDF annotations extracted from the document. When annotation extraction is enabled via `PdfConfig.extract_annotations`, this field contains text notes, highlights, links, stamps, and other annotations found in PDF documents. |
| `Children` | `List<ArchiveEntry>?` | `new List<ArchiveEntry>()` | Nested extraction results from archive contents. When extracting archives, each processable file inside produces its own full extraction result. Set to `null` for non-archive formats. Use `max_archive_depth` in config to control recursion depth. |
| `Uris` | `List<Uri>?` | `new List<Uri>()` | URIs/links discovered during document extraction. Contains hyperlinks, image references, citations, email addresses, and other URI-like references found in the document. Always extracted when present in the source document. |
| `StructuredOutput` | `object?` | `null` | Structured extraction output from LLM-based JSON schema extraction. When `structured_extraction` is configured in `ExtractionConfig`, the extracted document content is sent to a VLM with the provided JSON schema. The response is parsed and stored here as a JSON value matching the schema. |
| `CodeIntelligence` | `string?` | `null` | Code intelligence results from tree-sitter analysis. Populated when extracting source code files with the `tree-sitter` feature. Contains metrics, structural analysis, imports/exports, comments, docstrings, symbols, diagnostics, and optionally chunked code segments. |
| `LlmUsage` | `List<LlmUsage>?` | `new List<LlmUsage>()` | LLM token usage and cost data for all LLM calls made during this extraction. Contains one entry per LLM call. Multiple entries are produced when VLM OCR, structured extraction, and/or LLM embeddings all run during the same extraction. `null` when no LLM was used. |
| `FormattedContent` | `string?` | `null` | Pre-rendered content in the requested output format. Populated during `derive_extraction_result` before tree derivation consumes element data. `apply_output_format` swaps this into `content` at the end of the pipeline, after post-processors have operated on plain text. |
| `OcrInternalDocument` | `string?` | `null` | Structured hOCR document for the OCR+layout pipeline. When tesseract produces hOCR output, the parsed `InternalDocument` carries paragraph structure with bounding boxes and confidence scores. The layout classification step enriches these elements before final rendering. |


---

#### FictionBookMetadata

FictionBook (FB2) metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Genres` | `List<string>` | `new List<string>()` | Genres |
| `Sequences` | `List<string>` | `new List<string>()` | Sequences |
| `Annotation` | `string?` | `null` | Annotation |


---

#### FileExtractionConfig

Per-file extraction configuration overrides for batch processing.

All fields are `Option<T>` — `null` means "use the batch-level default."
This type is used with `crate.batch_extract_file` and
`crate.batch_extract_bytes` to allow heterogeneous
extraction settings within a single batch.

# Excluded Fields

The following `super.ExtractionConfig` fields are batch-level only and
cannot be overridden per file:
- `max_concurrent_extractions` — controls batch parallelism
- `use_cache` — global caching policy
- `acceleration` — shared ONNX execution provider
- `security_limits` — global archive security policy

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `EnableQualityProcessing` | `bool?` | `null` | Override quality post-processing for this file. |
| `Ocr` | `OcrConfig?` | `null` | Override OCR configuration for this file (None in the Option = use batch default). |
| `ForceOcr` | `bool?` | `null` | Override force OCR for this file. |
| `ForceOcrPages` | `List<nuint>?` | `new List<nuint>()` | Override force OCR pages for this file (1-indexed page numbers). |
| `DisableOcr` | `bool?` | `null` | Override disable OCR for this file. |
| `Chunking` | `ChunkingConfig?` | `null` | Override chunking configuration for this file. |
| `ContentFilter` | `ContentFilterConfig?` | `null` | Override content filtering configuration for this file. |
| `Images` | `ImageExtractionConfig?` | `null` | Override image extraction configuration for this file. |
| `PdfOptions` | `PdfConfig?` | `null` | Override PDF options for this file. |
| `TokenReduction` | `TokenReductionOptions?` | `null` | Override token reduction for this file. |
| `LanguageDetection` | `LanguageDetectionConfig?` | `null` | Override language detection for this file. |
| `Pages` | `PageConfig?` | `null` | Override page extraction for this file. |
| `Postprocessor` | `PostProcessorConfig?` | `null` | Override post-processor for this file. |
| `HtmlOptions` | `string?` | `null` | Override HTML conversion options for this file. |
| `ResultFormat` | `string?` | `null` | Override result format for this file. |
| `OutputFormat` | `string?` | `null` | Override output content format for this file. |
| `IncludeDocumentStructure` | `bool?` | `null` | Override document structure output for this file. |
| `Layout` | `LayoutDetectionConfig?` | `null` | Override layout detection for this file. |
| `TimeoutSecs` | `ulong?` | `null` | Override per-file extraction timeout in seconds. When set, the extraction for this file will be canceled after the specified duration. A timed-out file produces an error result without affecting other files in the batch. |
| `TreeSitter` | `TreeSitterConfig?` | `null` | Override tree-sitter configuration for this file. |
| `StructuredExtraction` | `StructuredExtractionConfig?` | `null` | Override structured extraction configuration for this file. When set, enables LLM-based structured extraction with a JSON schema for this specific file. The extracted content is sent to a VLM/LLM and the response is parsed according to the provided schema. |


---

#### FontSizeCluster

A cluster of text blocks with the same font size characteristics.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Centroid` | `float` | — | The centroid (mean) font size of this cluster |
| `Members` | `List<string>` | — | The text blocks that belong to this cluster |


---

#### Footnote

Footnote in Djot.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Label` | `string` | — | Footnote label |
| `Content` | `List<FormattedBlock>` | — | Footnote content blocks |


---

#### FormattedBlock

Block-level element in a Djot document.

Represents structural elements like headings, paragraphs, lists, code blocks, etc.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `BlockType` | `BlockType` | — | Type of block element |
| `Level` | `nuint?` | `null` | Heading level (1-6) for headings, or nesting level for lists |
| `InlineContent` | `List<InlineElement>` | — | Inline content within the block |
| `Attributes` | `string?` | `null` | Element attributes (classes, IDs, key-value pairs) |
| `Language` | `string?` | `null` | Language identifier for code blocks |
| `Code` | `string?` | `null` | Raw code content for code blocks |
| `Children` | `List<FormattedBlock>` | — | Nested blocks for containers (blockquotes, list items, divs) |


---

#### GridCell

Individual grid cell with position and span metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | Cell text content. |
| `Row` | `uint` | — | Zero-indexed row position. |
| `Col` | `uint` | — | Zero-indexed column position. |
| `RowSpan` | `uint` | — | Number of rows this cell spans. |
| `ColSpan` | `uint` | — | Number of columns this cell spans. |
| `IsHeader` | `bool` | — | Whether this is a header cell. |
| `Bbox` | `string?` | `null` | Bounding box for this cell (if available). |


---

#### HeaderFooter

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Paragraphs` | `List<string>` | `new List<string>()` | Paragraphs |
| `Tables` | `List<string>` | `new List<string>()` | Tables extracted from the document |
| `HeaderType` | `string` | — | Header type |


---

#### HeaderMetadata

Header/heading element metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Level` | `byte` | — | Header level: 1 (h1) through 6 (h6) |
| `Text` | `string` | — | Normalized text content of the header |
| `Id` | `string?` | `null` | HTML id attribute if present |
| `Depth` | `nuint` | — | Document tree depth at the header element |
| `HtmlOffset` | `nuint` | — | Byte offset in original HTML document |


---

#### HeadingContext

Heading context for a chunk within a Markdown document.

Contains the heading hierarchy from document root to this chunk's section.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Headings` | `List<HeadingLevel>` | — | The heading hierarchy from document root to this chunk's section. Index 0 is the outermost (h1), last element is the most specific. |


---

#### HeadingLevel

A single heading in the hierarchy.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Level` | `byte` | — | Heading depth (1 = h1, 2 = h2, etc.) |
| `Text` | `string` | — | The text content of the heading. |


---

#### HealthResponse

Health check response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Status` | `string` | — | Health status |
| `Version` | `string` | — | API version |
| `Plugins` | `string?` | `null` | Plugin status (optional) |


---

#### HierarchicalBlock

A text block with hierarchy level assignment.

Represents a block of text with semantic heading information extracted from
font size clustering and hierarchical analysis.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | The text content of this block |
| `FontSize` | `float` | — | The font size of the text in this block |
| `Level` | `string` | — | The hierarchy level of this block (H1-H6 or Body) Levels correspond to HTML heading tags: - "h1": Top-level heading - "h2": Secondary heading - "h3": Tertiary heading - "h4": Quaternary heading - "h5": Quinary heading - "h6": Senary heading - "body": Body text (no heading level) |
| `Bbox` | `string?` | `null` | Bounding box information for the block Contains coordinates as (left, top, right, bottom) in PDF units. |


---

#### HierarchyBlock

A TextBlock with hierarchy level assignment.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | The text content |
| `Bbox` | `string` | — | The bounding box of the block |
| `FontSize` | `float` | — | The font size of the text in this block |
| `HierarchyLevel` | `string` | — | The hierarchy level of this block (H1-H6 or Body) |


---

#### HierarchyConfig

Hierarchy extraction configuration for PDF text structure analysis.

Enables extraction of document hierarchy levels (H1-H6) based on font size
clustering and semantic analysis. When enabled, hierarchical blocks are
included in page content.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | `true` | Enable hierarchy extraction |
| `KClusters` | `nuint` | `3` | Number of font size clusters to use for hierarchy levels (1-7) Default: 6, which provides H1-H6 heading levels with body text. Larger values create more fine-grained hierarchy levels. |
| `IncludeBbox` | `bool` | `true` | Include bounding box information in hierarchy blocks |
| `OcrCoverageThreshold` | `float?` | `null` | OCR coverage threshold for smart OCR triggering (0.0-1.0) Determines when OCR should be triggered based on text block coverage. OCR is triggered when text blocks cover less than this fraction of the page. Default: 0.5 (trigger OCR if less than 50% of page has text) |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public HierarchyConfig CreateDefault()
```


---

#### HtmlExtractionResult

Result of HTML extraction with optional images and warnings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Markdown` | `string` | — | Markdown |
| `Images` | `List<ExtractedInlineImage>` | — | Images extracted from the document |
| `Warnings` | `List<string>` | — | Warnings |


---

#### HtmlMetadata

HTML metadata extracted from HTML documents.

Includes document-level metadata, Open Graph data, Twitter Card metadata,
and extracted structural elements (headers, links, images, structured data).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Title` | `string?` | `null` | Document title from `<title>` tag |
| `Description` | `string?` | `null` | Document description from `<meta name="description">` tag |
| `Keywords` | `List<string>` | `new List<string>()` | Document keywords from `<meta name="keywords">` tag, split on commas |
| `Author` | `string?` | `null` | Document author from `<meta name="author">` tag |
| `CanonicalUrl` | `string?` | `null` | Canonical URL from `<link rel="canonical">` tag |
| `BaseHref` | `string?` | `null` | Base URL from `<base href="">` tag for resolving relative URLs |
| `Language` | `string?` | `null` | Document language from `lang` attribute |
| `TextDirection` | `TextDirection?` | `null` | Document text direction from `dir` attribute |
| `OpenGraph` | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Open Graph metadata (og:* properties) for social media Keys like "title", "description", "image", "url", etc. |
| `TwitterCard` | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Twitter Card metadata (twitter:* properties) Keys like "card", "site", "creator", "title", "description", "image", etc. |
| `MetaTags` | `Dictionary<string, string>` | `new Dictionary<string, string>()` | Additional meta tags not covered by specific fields Keys are meta name/property attributes, values are content |
| `Headers` | `List<HeaderMetadata>` | `new List<HeaderMetadata>()` | Extracted header elements with hierarchy |
| `Links` | `List<LinkMetadata>` | `new List<LinkMetadata>()` | Extracted hyperlinks with type classification |
| `Images` | `List<ImageMetadataType>` | `new List<ImageMetadataType>()` | Extracted images with source and dimensions |
| `StructuredData` | `List<StructuredData>` | `new List<StructuredData>()` | Extracted structured data blocks |

##### Methods

###### From()

**Signature:**

```csharp
public HtmlMetadata From(HtmlMetadata metadata)
```


---

#### HtmlOutputConfig

Configuration for styled HTML output.

When set on `ExtractionConfig.html_output` alongside
`output_format = OutputFormat.Html`, the pipeline builds a
`StyledHtmlRenderer` instead of
the plain comrak-based renderer.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Css` | `string?` | `null` | Inline CSS string injected into the output after the theme stylesheet. Concatenated after `css_file` content when both are set. |
| `CssFile` | `string?` | `null` | Path to a CSS file loaded once at renderer construction time. Concatenated before `css` when both are set. |
| `Theme` | `HtmlTheme` | `HtmlTheme.Unstyled` | Built-in colour/typography theme. Default: `HtmlTheme.Unstyled`. |
| `ClassPrefix` | `string` | — | CSS class prefix applied to every emitted class name. Default: `"kb-"`. Change this if your host application already uses classes that start with `kb-`. |
| `EmbedCss` | `bool` | `true` | When `true` (default), write the resolved CSS into a `<style>` block immediately after the opening `<div class="{prefix}doc">`. Set to `false` to emit only the structural markup and wire up your own stylesheet targeting the `kb-*` class names. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public HtmlOutputConfig CreateDefault()
```


---

#### ImageExtractionConfig

Image extraction configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ExtractImages` | `bool` | — | Extract images from documents |
| `TargetDpi` | `int` | — | Target DPI for image normalization |
| `MaxImageDimension` | `int` | — | Maximum dimension for images (width or height) |
| `InjectPlaceholders` | `bool` | — | Whether to inject image reference placeholders into markdown output. When `true` (default), image references like `![Image 1](embedded:p1_i0)` are appended to the markdown. Set to `false` to extract images as data without polluting the markdown output. |
| `AutoAdjustDpi` | `bool` | — | Automatically adjust DPI based on image content |
| `MinDpi` | `int` | — | Minimum DPI threshold |
| `MaxDpi` | `int` | — | Maximum DPI threshold |


---

#### ImageMetadataType

Image element metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Src` | `string` | — | Image source (URL, data URI, or SVG content) |
| `Alt` | `string?` | `null` | Alternative text from alt attribute |
| `Title` | `string?` | `null` | Title attribute |
| `Dimensions` | `string?` | `null` | Image dimensions as (width, height) if available |
| `ImageType` | `ImageType` | — | Image type classification |
| `Attributes` | `List<string>` | — | Additional attributes as key-value pairs |


---

#### ImageOcrResult

Result of OCR extraction from an image with optional page tracking.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | Extracted text content |
| `Boundaries` | `List<PageBoundary>?` | `null` | Character byte boundaries per frame (for multi-frame TIFFs) |
| `PageContents` | `List<PageContent>?` | `null` | Per-frame content information |


---

#### ImagePreprocessingConfig

Image preprocessing configuration for OCR.

These settings control how images are preprocessed before OCR to improve
text recognition quality. Different preprocessing strategies work better
for different document types.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `TargetDpi` | `int` | `300` | Target DPI for the image (300 is standard, 600 for small text). |
| `AutoRotate` | `bool` | `true` | Auto-detect and correct image rotation. |
| `Deskew` | `bool` | `true` | Correct skew (tilted images). |
| `Denoise` | `bool` | `false` | Remove noise from the image. |
| `ContrastEnhance` | `bool` | `false` | Enhance contrast for better text visibility. |
| `BinarizationMethod` | `string` | `"otsu"` | Binarization method: "otsu", "sauvola", "adaptive". |
| `InvertColors` | `bool` | `false` | Invert colors (white text on black → black on white). |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public ImagePreprocessingConfig CreateDefault()
```


---

#### ImagePreprocessingMetadata

Image preprocessing metadata.

Tracks the transformations applied to an image during OCR preprocessing,
including DPI normalization, resizing, and resampling.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `OriginalDimensions` | `string` | — | Original image dimensions (width, height) in pixels |
| `OriginalDpi` | `string` | — | Original image DPI (horizontal, vertical) |
| `TargetDpi` | `int` | — | Target DPI from configuration |
| `ScaleFactor` | `double` | — | Scaling factor applied to the image |
| `AutoAdjusted` | `bool` | — | Whether DPI was auto-adjusted based on content |
| `FinalDpi` | `int` | — | Final DPI after processing |
| `NewDimensions` | `string?` | `null` | New dimensions after resizing (if resized) |
| `ResampleMethod` | `string` | — | Resampling algorithm used ("LANCZOS3", "CATMULLROM", etc.) |
| `DimensionClamped` | `bool` | — | Whether dimensions were clamped to max_image_dimension |
| `CalculatedDpi` | `int?` | `null` | Calculated optimal DPI (if auto_adjust_dpi enabled) |
| `SkippedResize` | `bool` | — | Whether resize was skipped (dimensions already optimal) |
| `ResizeError` | `string?` | `null` | Error message if resize failed |


---

#### InfoResponse

Server information response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Version` | `string` | — | API version |
| `RustBackend` | `bool` | — | Whether using Rust backend |


---

#### InlineElement

Inline element within a block.

Represents text with formatting, links, images, etc.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ElementType` | `InlineType` | — | Type of inline element |
| `Content` | `string` | — | Text content |
| `Attributes` | `string?` | `null` | Element attributes |
| `Metadata` | `Dictionary<string, string>?` | `null` | Additional metadata (e.g., href for links, src/alt for images) |


---

#### IterationValidator

Helper struct for validating iteration counts.


---

#### JatsMetadata

JATS (Journal Article Tag Suite) metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Copyright` | `string?` | `null` | Copyright |
| `License` | `string?` | `null` | License |
| `HistoryDates` | `Dictionary<string, string>` | `new Dictionary<string, string>()` | History dates |
| `ContributorRoles` | `List<ContributorRole>` | `new List<ContributorRole>()` | Contributor roles |


---

#### Keyword

Extracted keyword with metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | The keyword text. |
| `Score` | `float` | — | Relevance score (higher is better, algorithm-specific range). |
| `Algorithm` | `KeywordAlgorithm` | — | Algorithm that extracted this keyword. |
| `Positions` | `List<nuint>?` | `null` | Optional positions where keyword appears in text (character offsets). |


---

#### KeywordConfig

Keyword extraction configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Algorithm` | `KeywordAlgorithm` | `KeywordAlgorithm.Yake` | Algorithm to use for extraction. |
| `MaxKeywords` | `nuint` | `10` | Maximum number of keywords to extract (default: 10). |
| `MinScore` | `float` | `0` | Minimum score threshold (0.0-1.0, default: 0.0). Keywords with scores below this threshold are filtered out. Note: Score ranges differ between algorithms. |
| `NgramRange` | `string` | — | N-gram range for keyword extraction (min, max). (1, 1) = unigrams only (1, 2) = unigrams and bigrams (1, 3) = unigrams, bigrams, and trigrams (default) |
| `Language` | `string?` | `null` | Language code for stopword filtering (e.g., "en", "de", "fr"). If None, no stopword filtering is applied. |
| `YakeParams` | `YakeParams?` | `null` | YAKE-specific tuning parameters. |
| `RakeParams` | `RakeParams?` | `null` | RAKE-specific tuning parameters. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public KeywordConfig CreateDefault()
```


---

#### LanguageDetectionConfig

Language detection configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | — | Enable language detection |
| `MinConfidence` | `double` | — | Minimum confidence threshold (0.0-1.0) |
| `DetectMultiple` | `bool` | — | Detect multiple languages in the document |


---

#### LayoutDetection

A single layout detection result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Class` | `LayoutClass` | — | Class (layout class) |
| `Confidence` | `float` | — | Confidence |
| `Bbox` | `BBox` | — | Bbox (b box) |


---

#### LayoutDetectionConfig

Layout detection configuration.

Controls layout detection behavior in the extraction pipeline.
When set on `ExtractionConfig`, layout detection
is enabled for PDF extraction.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ConfidenceThreshold` | `float?` | `null` | Confidence threshold override (None = use model default). |
| `ApplyHeuristics` | `bool` | `true` | Whether to apply postprocessing heuristics (default: true). |
| `TableModel` | `TableModel` | `TableModel.Tatr` | Table structure recognition model. Controls which model is used for table cell detection within layout-detected table regions. Defaults to `TableModel.Tatr`. |
| `Acceleration` | `AccelerationConfig?` | `null` | Hardware acceleration for ONNX models (layout detection + table structure). When set, controls which execution provider (CPU, CUDA, CoreML, TensorRT) is used for inference. Defaults to `null` (auto-select per platform). |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public LayoutDetectionConfig CreateDefault()
```


---

#### LayoutRegion

A detected layout region on a page.

When layout detection is enabled, each page may have layout regions
identifying different content types (text, pictures, tables, etc.)
with confidence scores and spatial positions.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Class` | `string` | — | Layout class name (e.g. "picture", "table", "text", "section_header"). |
| `Confidence` | `double` | — | Confidence score from the layout detection model (0.0 to 1.0). |
| `BoundingBox` | `string` | — | Bounding box in document coordinate space. |
| `AreaFraction` | `double` | — | Fraction of the page area covered by this region (0.0 to 1.0). |


---

#### LinkMetadata

Link element metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Href` | `string` | — | The href URL value |
| `Text` | `string` | — | Link text content (normalized) |
| `Title` | `string?` | `null` | Optional title attribute |
| `LinkType` | `LinkType` | — | Link type classification |
| `Rel` | `List<string>` | — | Rel attribute values |
| `Attributes` | `List<string>` | — | Additional attributes as key-value pairs |


---

#### LlmConfig

Configuration for an LLM provider/model via liter-llm.

Each feature (VLM OCR, VLM embeddings, structured extraction) carries
its own `LlmConfig`, allowing different providers per feature.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | Provider/model string using liter-llm routing format. Examples: `"openai/gpt-4o"`, `"anthropic/claude-sonnet-4-20250514"`, `"groq/llama-3.1-70b-versatile"`. |
| `ApiKey` | `string?` | `null` | API key for the provider. When `null`, liter-llm falls back to the provider's standard environment variable (e.g., `OPENAI_API_KEY`). |
| `BaseUrl` | `string?` | `null` | Custom base URL override for the provider endpoint. |
| `TimeoutSecs` | `ulong?` | `null` | Request timeout in seconds (default: 60). |
| `MaxRetries` | `uint?` | `null` | Maximum retry attempts (default: 3). |
| `Temperature` | `double?` | `null` | Sampling temperature for generation tasks. |
| `MaxTokens` | `ulong?` | `null` | Maximum tokens to generate. |


---

#### LlmUsage

Token usage and cost data for a single LLM call made during extraction.

Populated when VLM OCR, structured extraction, or LLM-based embeddings
are used. Multiple entries may be present when multiple LLM calls occur
within one extraction (e.g. VLM OCR + structured extraction).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Model` | `string` | — | The LLM model identifier (e.g. "openai/gpt-4o", "anthropic/claude-sonnet-4-20250514"). |
| `Source` | `string` | — | The pipeline stage that triggered this LLM call (e.g. "vlm_ocr", "structured_extraction", "embeddings"). |
| `InputTokens` | `ulong?` | `null` | Number of input/prompt tokens consumed. |
| `OutputTokens` | `ulong?` | `null` | Number of output/completion tokens generated. |
| `TotalTokens` | `ulong?` | `null` | Total tokens (input + output). |
| `EstimatedCost` | `double?` | `null` | Estimated cost in USD based on the provider's published pricing. |
| `FinishReason` | `string?` | `null` | Why the model stopped generating (e.g. "stop", "length", "content_filter"). |


---

#### ManifestEntryResponse

Model manifest entry for cache management.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `RelativePath` | `string` | — | Relative path within the cache directory |
| `Sha256` | `string` | — | SHA256 checksum of the model file |
| `SizeBytes` | `ulong` | — | Expected file size in bytes |
| `SourceUrl` | `string` | — | HuggingFace source URL for downloading |


---

#### ManifestResponse

Model manifest response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `KreuzbergVersion` | `string` | — | Kreuzberg version |
| `TotalSizeBytes` | `ulong` | — | Total size of all models in bytes |
| `ModelCount` | `nuint` | — | Number of models in the manifest |
| `Models` | `List<ManifestEntryResponse>` | — | Individual model entries |


---

#### MergedChunk

A merged chunk produced by `merge_segments`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | Text |
| `ByteStart` | `nuint` | — | Byte start |
| `ByteEnd` | `nuint` | — | Byte end |


---

#### Metadata

Extraction result metadata.

Contains common fields applicable to all formats, format-specific metadata
via a discriminated union, and additional custom fields from postprocessors.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Title` | `string?` | `null` | Document title |
| `Subject` | `string?` | `null` | Document subject or description |
| `Authors` | `List<string>?` | `new List<string>()` | Primary author(s) - always Vec for consistency |
| `Keywords` | `List<string>?` | `new List<string>()` | Keywords/tags - always Vec for consistency |
| `Language` | `string?` | `null` | Primary language (ISO 639 code) |
| `CreatedAt` | `string?` | `null` | Creation timestamp (ISO 8601 format) |
| `ModifiedAt` | `string?` | `null` | Last modification timestamp (ISO 8601 format) |
| `CreatedBy` | `string?` | `null` | User who created the document |
| `ModifiedBy` | `string?` | `null` | User who last modified the document |
| `Pages` | `PageStructure?` | `null` | Page/slide/sheet structure with boundaries |
| `Format` | `FormatMetadata?` | `null` | Format-specific metadata (discriminated union) Contains detailed metadata specific to the document format. Serializes with a `format_type` discriminator field. |
| `ImagePreprocessing` | `ImagePreprocessingMetadata?` | `null` | Image preprocessing metadata (when OCR preprocessing was applied) |
| `JsonSchema` | `object?` | `null` | JSON schema (for structured data extraction) |
| `Error` | `ErrorMetadata?` | `null` | Error metadata (for batch operations) |
| `ExtractionDurationMs` | `ulong?` | `null` | Extraction duration in milliseconds (for benchmarking). This field is populated by batch extraction to provide per-file timing information. It's `null` for single-file extraction (which uses external timing). |
| `Category` | `string?` | `null` | Document category (from frontmatter or classification). |
| `Tags` | `List<string>?` | `new List<string>()` | Document tags (from frontmatter). |
| `DocumentVersion` | `string?` | `null` | Document version string (from frontmatter). |
| `AbstractText` | `string?` | `null` | Abstract or summary text (from frontmatter). |
| `OutputFormat` | `string?` | `null` | Output format identifier (e.g., "markdown", "html", "text"). Set by the output format pipeline stage when format conversion is applied. Previously stored in `metadata.additional["output_format"]`. |
| `Additional` | `string` | — | Additional custom fields from postprocessors. **Deprecated**: Prefer using typed fields on `ExtractionResult` and `Metadata` instead of inserting into this map. Typed fields provide better cross-language compatibility and type safety. This field will be removed in a future major version. This flattened map allows Python/TypeScript postprocessors to add arbitrary fields (entity extraction, keyword extraction, etc.). Fields are merged at the root level during serialization. Uses `Cow<'static, str>` keys so static string keys avoid allocation. |


---

#### ModelPaths

Combined paths to all models needed for OCR (backward compatibility).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `DetModel` | `string` | — | Path to the detection model directory. |
| `ClsModel` | `string` | — | Path to the classification model directory. |
| `RecModel` | `string` | — | Path to the recognition model directory. |
| `DictFile` | `string` | — | Path to the character dictionary file. |


---

#### Note

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | Unique identifier |
| `NoteType` | `string` | — | Note type |
| `Paragraphs` | `List<string>` | — | Paragraphs |


---

#### OcrCacheStats

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `TotalFiles` | `nuint` | — | Total files |
| `TotalSizeMb` | `double` | — | Total size mb |


---

#### OcrConfidence

Confidence scores for an OCR element.

Separates detection confidence (how confident that text exists at this location)
from recognition confidence (how confident about the actual text content).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Detection` | `double?` | `null` | Detection confidence: how confident the OCR engine is that text exists here. PaddleOCR provides this as `box_score`, Tesseract doesn't have a direct equivalent. Range: 0.0 to 1.0 (or None if not available). |
| `Recognition` | `double` | — | Recognition confidence: how confident about the text content. Range: 0.0 to 1.0. |


---

#### OcrConfig

OCR configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | `true` | Whether OCR is enabled. Setting `enabled: false` is a shorthand for `disable_ocr: true` on the parent `ExtractionConfig`. Images return metadata only; PDFs use native text extraction without OCR fallback. Defaults to `true`. When `false`, all other OCR settings are ignored. |
| `Backend` | `string` | — | OCR backend: tesseract, easyocr, paddleocr |
| `Language` | `string` | — | Language code (e.g., "eng", "deu") |
| `TesseractConfig` | `TesseractConfig?` | `null` | Tesseract-specific configuration (optional) |
| `OutputFormat` | `string?` | `null` | Output format for OCR results (optional, for format conversion) |
| `PaddleOcrConfig` | `object?` | `null` | PaddleOCR-specific configuration (optional, JSON passthrough) |
| `ElementConfig` | `OcrElementConfig?` | `null` | OCR element extraction configuration |
| `QualityThresholds` | `OcrQualityThresholds?` | `null` | Quality thresholds for the native-text-to-OCR fallback decision. When None, uses compiled defaults (matching previous hardcoded behavior). |
| `Pipeline` | `OcrPipelineConfig?` | `null` | Multi-backend OCR pipeline configuration. When set, enables weighted fallback across multiple OCR backends based on output quality. When None, uses the single `backend` field (same as today). |
| `AutoRotate` | `bool` | `false` | Enable automatic page rotation based on orientation detection. When enabled, uses Tesseract's `DetectOrientationScript()` to detect page orientation (0/90/180/270 degrees) before OCR. If the page is rotated with high confidence, the image is corrected before recognition. This is critical for handling rotated scanned documents. |
| `VlmConfig` | `LlmConfig?` | `null` | VLM (Vision Language Model) OCR configuration. Required when `backend` is `"vlm"`. Uses liter-llm to send page images to a vision model for text extraction. |
| `VlmPrompt` | `string?` | `null` | Custom Jinja2 prompt template for VLM OCR. When `null`, uses the default template. Available variables: - `{{ language }}` — The document language code (e.g., "eng", "deu"). |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public OcrConfig CreateDefault()
```


---

#### OcrElement

A unified OCR element representing detected text with full metadata.

This is the primary type for structured OCR output, preserving all information
from both Tesseract and PaddleOCR backends.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Text` | `string` | — | The recognized text content. |
| `Geometry` | `OcrBoundingGeometry` | `OcrBoundingGeometry.Rectangle` | Bounding geometry (rectangle or quadrilateral). |
| `Confidence` | `OcrConfidence` | — | Confidence scores for detection and recognition. |
| `Level` | `OcrElementLevel` | `OcrElementLevel.Line` | Hierarchical level (word, line, block, page). |
| `Rotation` | `OcrRotation?` | `null` | Rotation information (if detected). |
| `PageNumber` | `nuint` | — | Page number (1-indexed). |
| `ParentId` | `string?` | `null` | Parent element ID for hierarchical relationships. Only used for Tesseract output which has word -> line -> block hierarchy. |
| `BackendMetadata` | `Dictionary<string, object>` | `new Dictionary<string, object>()` | Backend-specific metadata that doesn't fit the unified schema. |


---

#### OcrElementConfig

Configuration for OCR element extraction.

Controls how OCR elements are extracted and filtered.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `IncludeElements` | `bool` | — | Whether to include OCR elements in the extraction result. When true, the `ocr_elements` field in `ExtractionResult` will be populated. |
| `MinLevel` | `OcrElementLevel` | `OcrElementLevel.Line` | Minimum hierarchical level to include. Elements below this level (e.g., words when min_level is Line) will be excluded. |
| `MinConfidence` | `double` | — | Minimum recognition confidence threshold (0.0-1.0). Elements with confidence below this threshold will be filtered out. |
| `BuildHierarchy` | `bool` | — | Whether to build hierarchical relationships between elements. When true, `parent_id` fields will be populated based on spatial containment. Only meaningful for Tesseract output. |


---

#### OcrExtractionResult

OCR extraction result.

Result of performing OCR on an image or scanned document,
including recognized text and detected tables.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | Recognized text content |
| `MimeType` | `string` | — | Original MIME type of the processed image |
| `Metadata` | `Dictionary<string, object>` | — | OCR processing metadata (confidence scores, language, etc.) |
| `Tables` | `List<OcrTable>` | — | Tables detected and extracted via OCR |
| `OcrElements` | `List<OcrElement>?` | `null` | Structured OCR elements with bounding boxes and confidence scores. Available when TSV output is requested or table detection is enabled. |
| `InternalDocument` | `string?` | `null` | Structured document produced from hOCR parsing. Carries paragraph structure, bounding boxes, and confidence scores that the flattened `content` string discards. |


---

#### OcrFallbackDecision

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Stats` | `string` | — | Stats |
| `AvgNonWhitespace` | `double` | — | Avg non whitespace |
| `AvgAlnum` | `double` | — | Avg alnum |
| `Fallback` | `bool` | — | Fallback |


---

#### OcrMetadata

OCR processing metadata.

Captures information about OCR processing configuration and results.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Language` | `string` | — | OCR language code(s) used |
| `Psm` | `int` | — | Tesseract Page Segmentation Mode (PSM) |
| `OutputFormat` | `string` | — | Output format (e.g., "text", "hocr") |
| `TableCount` | `nuint` | — | Number of tables detected |
| `TableRows` | `nuint?` | `null` | Table rows |
| `TableCols` | `nuint?` | `null` | Table cols |


---

#### OcrPipelineConfig

Multi-backend OCR pipeline with quality-based fallback.

Backends are tried in priority order (highest first). After each backend
produces output, quality is evaluated. If it meets `quality_thresholds.pipeline_min_quality`,
the result is accepted. Otherwise the next backend is tried.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Stages` | `List<OcrPipelineStage>` | — | Ordered list of backends to try. Sorted by priority (descending) at runtime. |
| `QualityThresholds` | `OcrQualityThresholds` | — | Quality thresholds for deciding whether to accept a result or try the next backend. |


---

#### OcrPipelineStage

A single backend stage in the OCR pipeline.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Backend` | `string` | — | Backend name: "tesseract", "paddleocr", "easyocr", or a custom registered name. |
| `Priority` | `uint` | — | Priority weight (higher = tried first). Stages are sorted by priority descending. |
| `Language` | `string?` | `null` | Language override for this stage (None = use parent OcrConfig.language). |
| `TesseractConfig` | `TesseractConfig?` | `null` | Tesseract-specific config override for this stage. |
| `PaddleOcrConfig` | `object?` | `null` | PaddleOCR-specific config for this stage. |
| `VlmConfig` | `LlmConfig?` | `null` | VLM config override for this pipeline stage. |


---

#### OcrQualityThresholds

Quality thresholds for OCR fallback decisions and pipeline quality gating.

All fields default to the values that match the previous hardcoded behavior,
so `OcrQualityThresholds.default()` preserves existing semantics exactly.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MinTotalNonWhitespace` | `nuint` | `64` | Minimum total non-whitespace characters to consider text substantive. |
| `MinNonWhitespacePerPage` | `double` | `32` | Minimum non-whitespace characters per page on average. |
| `MinMeaningfulWordLen` | `nuint` | `4` | Minimum character count for a word to be "meaningful". |
| `MinMeaningfulWords` | `nuint` | `3` | Minimum count of meaningful words before text is accepted. |
| `MinAlnumRatio` | `double` | `0.3` | Minimum alphanumeric ratio (non-whitespace chars that are alphanumeric). |
| `MinGarbageChars` | `nuint` | `5` | Minimum Unicode replacement characters (U+FFFD) to trigger OCR fallback. |
| `MaxFragmentedWordRatio` | `double` | `0.6` | Maximum fraction of short (1-2 char) words before text is considered fragmented. |
| `CriticalFragmentedWordRatio` | `double` | `0.8` | Critical fragmentation threshold — triggers OCR regardless of meaningful words. Normal English text has ~20-30% short words. 80%+ is definitive garbage. |
| `MinAvgWordLength` | `double` | `2` | Minimum average word length. Below this with enough words indicates garbled extraction. |
| `MinWordsForAvgLengthCheck` | `nuint` | `50` | Minimum word count before average word length check applies. |
| `MinConsecutiveRepeatRatio` | `double` | `0.08` | Minimum consecutive word repetition ratio to detect column scrambling. |
| `MinWordsForRepeatCheck` | `nuint` | `50` | Minimum word count before consecutive repetition check is applied. |
| `SubstantiveMinChars` | `nuint` | `100` | Minimum character count for "substantive markdown" OCR skip gate. |
| `NonTextMinChars` | `nuint` | `20` | Minimum character count for "non-text content" OCR skip gate. |
| `AlnumWsRatioThreshold` | `double` | `0.4` | Alphanumeric+whitespace ratio threshold for skip decisions. |
| `PipelineMinQuality` | `double` | `0.5` | Minimum quality score (0.0-1.0) for a pipeline stage result to be accepted. If the result from a backend scores below this, try the next backend. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public OcrQualityThresholds CreateDefault()
```


---

#### OcrRotation

Rotation information for an OCR element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `AngleDegrees` | `double` | — | Rotation angle in degrees (0, 90, 180, 270 for PaddleOCR). |
| `Confidence` | `double?` | `null` | Confidence score for the rotation detection. |


---

#### OcrTable

Table detected via OCR.

Represents a table structure recognized during OCR processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Cells` | `List<List<string>>` | — | Table cells as a 2D vector (rows × columns) |
| `Markdown` | `string` | — | Markdown representation of the table |
| `PageNumber` | `nuint` | — | Page number where the table was found (1-indexed) |
| `BoundingBox` | `OcrTableBoundingBox?` | `null` | Bounding box of the table in pixel coordinates (from OCR word positions). |


---

#### OcrTableBoundingBox

Bounding box for an OCR-detected table in pixel coordinates.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Left` | `uint` | — | Left x-coordinate (pixels) |
| `Top` | `uint` | — | Top y-coordinate (pixels) |
| `Right` | `uint` | — | Right x-coordinate (pixels) |
| `Bottom` | `uint` | — | Bottom y-coordinate (pixels) |


---

#### OdtProperties

OpenDocument metadata from meta.xml

Contains metadata fields defined by the OASIS OpenDocument Format standard.
Uses Dublin Core elements (dc:) and OpenDocument meta elements (meta:).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Title` | `string?` | `null` | Document title (dc:title) |
| `Subject` | `string?` | `null` | Document subject/topic (dc:subject) |
| `Creator` | `string?` | `null` | Current document creator/author (dc:creator) |
| `InitialCreator` | `string?` | `null` | Initial creator of the document (meta:initial-creator) |
| `Keywords` | `string?` | `null` | Keywords or tags (meta:keyword) |
| `Description` | `string?` | `null` | Document description (dc:description) |
| `Date` | `string?` | `null` | Current modification date (dc:date) |
| `CreationDate` | `string?` | `null` | Initial creation date (meta:creation-date) |
| `Language` | `string?` | `null` | Document language (dc:language) |
| `Generator` | `string?` | `null` | Generator/application that created the document (meta:generator) |
| `EditingDuration` | `string?` | `null` | Editing duration in ISO 8601 format (meta:editing-duration) |
| `EditingCycles` | `string?` | `null` | Number of edits/revisions (meta:editing-cycles) |
| `PageCount` | `int?` | `null` | Document statistics - page count (meta:page-count) |
| `WordCount` | `int?` | `null` | Document statistics - word count (meta:word-count) |
| `CharacterCount` | `int?` | `null` | Document statistics - character count (meta:character-count) |
| `ParagraphCount` | `int?` | `null` | Document statistics - paragraph count (meta:paragraph-count) |
| `TableCount` | `int?` | `null` | Document statistics - table count (meta:table-count) |
| `ImageCount` | `int?` | `null` | Document statistics - image count (meta:image-count) |


---

#### OpenWebDocumentResponse

OpenWebUI "External" engine response format.

Returned by `PUT /process` for the OpenWebUI external document loader.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PageContent` | `string` | — | Extracted text content |
| `Metadata` | `string` | — | Document metadata |


---

#### OrientationResult

Document orientation detection result.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Degrees` | `uint` | — | Detected orientation in degrees (0, 90, 180, or 270). |
| `Confidence` | `float` | — | Confidence score (0.0-1.0). |


---

#### PaddleOcrConfig

Configuration for PaddleOCR backend.

Configures PaddleOCR text detection and recognition with multi-language support.
Uses a builder pattern for convenient configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Language` | `string` | — | Language code (e.g., "en", "ch", "jpn", "kor", "deu", "fra") |
| `CacheDir` | `string?` | `null` | Optional custom cache directory for model files |
| `UseAngleCls` | `bool` | — | Enable angle classification for rotated text (default: false). Can misfire on short text regions, rotating crops incorrectly before recognition. |
| `EnableTableDetection` | `bool` | — | Enable table structure detection (default: false) |
| `DetDbThresh` | `float` | — | Database threshold for text detection (default: 0.3) Range: 0.0-1.0, higher values require more confident detections |
| `DetDbBoxThresh` | `float` | — | Box threshold for text bounding box refinement (default: 0.5) Range: 0.0-1.0 |
| `DetDbUnclipRatio` | `float` | — | Unclip ratio for expanding text bounding boxes (default: 1.6) Controls the expansion of detected text regions |
| `DetLimitSideLen` | `uint` | — | Maximum side length for detection image (default: 960) Larger images may be resized to this limit for faster inference |
| `RecBatchNum` | `uint` | — | Batch size for recognition inference (default: 6) Number of text regions to process simultaneously |
| `Padding` | `uint` | — | Padding in pixels added around the image before detection (default: 10). Large values can include surrounding content like table gridlines. |
| `DropScore` | `float` | — | Minimum recognition confidence score for text lines (default: 0.5). Text regions with recognition confidence below this threshold are discarded. Matches PaddleOCR Python's `drop_score` parameter. Range: 0.0-1.0 |
| `ModelTier` | `string` | — | Model tier controlling detection/recognition model size and accuracy trade-off. - `"mobile"` (default): Lightweight models (~4.5MB detection, ~16.5MB recognition), fast download and inference - `"server"`: Large, high-accuracy models (~88MB detection, ~84MB recognition), best for GPU or complex documents |

##### Methods

###### CreateDefault()

Creates a default configuration with English language support.

**Signature:**

```csharp
public PaddleOcrConfig CreateDefault()
```


---

#### PageBoundary

Byte offset boundary for a page.

Tracks where a specific page's content starts and ends in the main content string,
enabling mapping from byte positions to page numbers. Offsets are guaranteed to be
at valid UTF-8 character boundaries when using standard String methods (push_str, push, etc.).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ByteStart` | `nuint` | — | Byte offset where this page starts in the content string (UTF-8 valid boundary, inclusive) |
| `ByteEnd` | `nuint` | — | Byte offset where this page ends in the content string (UTF-8 valid boundary, exclusive) |
| `PageNumber` | `nuint` | — | Page number (1-indexed) |


---

#### PageConfig

Page extraction and tracking configuration.

Controls how pages are extracted, tracked, and represented in the extraction results.
When `null`, page tracking is disabled.

Page range tracking in chunk metadata (first_page/last_page) is automatically enabled
when page boundaries are available and chunking is configured.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ExtractPages` | `bool` | `false` | Extract pages as separate array (ExtractionResult.pages) |
| `InsertPageMarkers` | `bool` | `false` | Insert page markers in main content string |
| `MarkerFormat` | `string` | `"

<!-- PAGE {page_num} -->

"` | Page marker format (use {page_num} placeholder) Default: "\n\n<!-- PAGE {page_num} -->\n\n" |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public PageConfig CreateDefault()
```


---

#### PageContent

Content for a single page/slide.

When page extraction is enabled, documents are split into per-page content
with associated tables and images mapped to each page.

# Performance

Uses Arc-wrapped tables and images for memory efficiency:
- `Vec<Arc<Table>>` enables zero-copy sharing of table data
- `Vec<Arc<ExtractedImage>>` enables zero-copy sharing of image data
- Maintains exact JSON compatibility via custom Serialize/Deserialize

This reduces memory overhead for documents with shared tables/images
by avoiding redundant copies during serialization.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PageNumber` | `nuint` | — | Page number (1-indexed) |
| `Content` | `string` | — | Text content for this page |
| `Tables` | `List<string>` | — | Tables found on this page (uses Arc for memory efficiency) Serializes as Vec<Table> for JSON compatibility while maintaining Arc semantics in-memory for zero-copy sharing. |
| `Images` | `List<ExtractedImage>` | — | Images found on this page (uses Arc for memory efficiency) Serializes as Vec<ExtractedImage> for JSON compatibility while maintaining Arc semantics in-memory for zero-copy sharing. |
| `Hierarchy` | `PageHierarchy?` | `null` | Hierarchy information for the page (when hierarchy extraction is enabled) Contains text hierarchy levels (H1-H6) extracted from the page content. |
| `IsBlank` | `bool?` | `null` | Whether this page is blank (no meaningful text content) Determined during extraction based on text content analysis. A page is blank if it has fewer than 3 non-whitespace characters and contains no tables or images. |
| `LayoutRegions` | `List<LayoutRegion>?` | `null` | Layout detection regions for this page (when layout detection is enabled). Contains detected layout regions with class, confidence, bounding box, and area fraction. Only populated when layout detection is configured. |


---

#### PageHierarchy

Page hierarchy structure containing heading levels and block information.

Used when PDF text hierarchy extraction is enabled. Contains hierarchical
blocks with heading levels (H1-H6) for semantic document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `BlockCount` | `nuint` | — | Number of hierarchy blocks on this page |
| `Blocks` | `List<HierarchicalBlock>` | — | Hierarchical blocks with heading levels |


---

#### PageInfo

Metadata for individual page/slide/sheet.

Captures per-page information including dimensions, content counts,
and visibility state (for presentations).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Number` | `nuint` | — | Page number (1-indexed) |
| `Title` | `string?` | `null` | Page title (usually for presentations) |
| `Dimensions` | `string?` | `null` | Dimensions in points (PDF) or pixels (images): (width, height) |
| `ImageCount` | `nuint?` | `null` | Number of images on this page |
| `TableCount` | `nuint?` | `null` | Number of tables on this page |
| `Hidden` | `bool?` | `null` | Whether this page is hidden (e.g., in presentations) |
| `IsBlank` | `bool?` | `null` | Whether this page is blank (no meaningful text, no images, no tables) A page is considered blank if it has fewer than 3 non-whitespace characters and contains no tables or images. This is useful for filtering out empty pages in scanned documents or PDFs with blank separator pages. |


---

#### PageLayoutResult

Layout detection results for a single page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PageIndex` | `nuint` | — | Page index |
| `Regions` | `List<string>` | — | Regions |
| `PageWidthPts` | `float` | — | Page width pts |
| `PageHeightPts` | `float` | — | Page height pts |
| `RenderWidthPx` | `uint` | — | Width of the rendered image used for layout detection (pixels). |
| `RenderHeightPx` | `uint` | — | Height of the rendered image used for layout detection (pixels). |


---

#### PageMarginsPoints

Page margins converted to points (1/72 inch).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Top` | `double?` | `null` | Top |
| `Right` | `double?` | `null` | Right |
| `Bottom` | `double?` | `null` | Bottom |
| `Left` | `double?` | `null` | Left |
| `Header` | `double?` | `null` | Header |
| `Footer` | `double?` | `null` | Footer |
| `Gutter` | `double?` | `null` | Gutter |


---

#### PageStructure

Unified page structure for documents.

Supports different page types (PDF pages, PPTX slides, Excel sheets)
with character offset boundaries for chunk-to-page mapping.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `TotalCount` | `nuint` | — | Total number of pages/slides/sheets |
| `UnitType` | `PageUnitType` | — | Type of paginated unit |
| `Boundaries` | `List<PageBoundary>?` | `null` | Character offset boundaries for each page Maps character ranges in the extracted content to page numbers. Used for chunk page range calculation. |
| `Pages` | `List<PageInfo>?` | `null` | Detailed per-page metadata (optional, only when needed) |


---

#### PageTiming

Timing breakdown for a single page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `RenderMs` | `double` | — | Time to render the PDF page to a raster image (amortized from batch render). |
| `PreprocessMs` | `double` | — | Time spent in image preprocessing (resize, normalize, tensor construction). |
| `OnnxMs` | `double` | — | Time for the ONNX model session.run() call (actual neural network inference). |
| `InferenceMs` | `double` | — | Total model inference time (preprocess + onnx), as measured by the engine. |
| `PostprocessMs` | `double` | — | Time spent in postprocessing (confidence filtering, overlap resolution). |
| `MappingMs` | `double` | — | Time to map pixel-space bounding boxes to PDF coordinate space. |


---

#### PdfAnnotation

A PDF annotation extracted from a document page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `AnnotationType` | `PdfAnnotationType` | — | The type of annotation. |
| `Content` | `string?` | `null` | Text content of the annotation (e.g., comment text, link URL). |
| `PageNumber` | `nuint` | — | Page number where the annotation appears (1-indexed). |
| `BoundingBox` | `string?` | `null` | Bounding box of the annotation on the page. |


---

#### PdfConfig

PDF-specific configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Backend` | `PdfBackend` | `PdfBackend.Pdfium` | PDF extraction backend. Default: `Pdfium`. |
| `ExtractImages` | `bool` | `false` | Extract images from PDF |
| `Passwords` | `List<string>?` | `null` | List of passwords to try when opening encrypted PDFs |
| `ExtractMetadata` | `bool` | `true` | Extract PDF metadata |
| `Hierarchy` | `HierarchyConfig?` | `null` | Hierarchy extraction configuration (None = hierarchy extraction disabled) |
| `ExtractAnnotations` | `bool` | `false` | Extract PDF annotations (text notes, highlights, links, stamps). Default: false |
| `TopMarginFraction` | `float?` | `null` | Top margin fraction (0.0–1.0) of page height to exclude headers/running heads. Default: 0.06 (6%) |
| `BottomMarginFraction` | `float?` | `null` | Bottom margin fraction (0.0–1.0) of page height to exclude footers/page numbers. Default: 0.05 (5%) |
| `AllowSingleColumnTables` | `bool` | `false` | Allow single-column pseudo tables in extraction results. By default, tables with fewer than 2 columns (layout-guided) or 3 columns (heuristic) are rejected. When `true`, the minimum column count is relaxed to 1, allowing single-column structured data (glossaries, itemized lists) to be emitted as tables. Other quality filters (density, sparsity, prose detection) still apply. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public PdfConfig CreateDefault()
```


---

#### PdfImage

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `PageNumber` | `nuint` | — | Page number |
| `ImageIndex` | `nuint` | — | Image index |
| `Width` | `long` | — | Width |
| `Height` | `long` | — | Height |
| `ColorSpace` | `string?` | `null` | Color space |
| `BitsPerComponent` | `long?` | `null` | Bits per component |
| `Filters` | `List<string>` | — | Original PDF stream filters (e.g. `["FlateDecode"]`, `["DCTDecode"]`). |
| `Data` | `byte[]` | — | The decoded image bytes in a standard format (JPEG, PNG, etc.). |
| `DecodedFormat` | `string` | — | The format of `data` after decoding: `"jpeg"`, `"png"`, `"jpeg2000"`, `"ccitt"`, or `"raw"`. |


---

#### PdfUnifiedExtractionResult

Result type for unified PDF text and metadata extraction.

Contains text, optional page boundaries, optional per-page content, and metadata.


---

#### Plugin

Base trait that all plugins must implement.

This trait provides common functionality for plugin lifecycle management,
identification, and metadata.

# Thread Safety

All plugins must be `Send + Sync` to support concurrent usage across threads.

##### Methods

###### Name()

Returns the unique name/identifier for this plugin.

The name should be:
- Unique across all plugins
- Lowercase with hyphens (e.g., "my-custom-plugin")
- URL-safe characters only

**Signature:**

```csharp
public string Name()
```

###### Version()

Returns the semantic version of this plugin.

Should follow semver format: `MAJOR.MINOR.PATCH`

**Signature:**

```csharp
public string Version()
```

###### Initialize()

Initialize the plugin.

Called once when the plugin is registered. Use this to:
- Load configuration
- Initialize resources (connections, caches, etc.)
- Validate dependencies

# Thread Safety

This method takes `&self` instead of `&mut self` to work with `Arc<dyn Plugin>`.
Plugins needing mutable state during initialization should use interior mutability
patterns (Mutex, RwLock, OnceCell, etc.).

**Errors:**

Should return an error if initialization fails. The plugin will not be
registered if this method returns an error.

**Signature:**

```csharp
public void Initialize()
```

###### Shutdown()

Shutdown the plugin.

Called when the plugin is being unregistered or the application is shutting down.
Use this to:
- Close connections
- Flush caches
- Release resources

# Thread Safety

This method takes `&self` instead of `&mut self` to work with `Arc<dyn Plugin>`.
Plugins needing mutable state during shutdown should use interior mutability
patterns (Mutex, RwLock, etc.).

**Errors:**

Errors during shutdown are logged but don't prevent the shutdown process.

**Signature:**

```csharp
public void Shutdown()
```

###### Description()

Optional plugin description for debugging and logging.

Defaults to empty string if not overridden.

**Signature:**

```csharp
public string Description()
```

###### Author()

Optional plugin author information.

Defaults to empty string if not overridden.

**Signature:**

```csharp
public string Author()
```


---

#### PostProcessorConfig

Post-processor configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | `true` | Enable post-processors |
| `EnabledProcessors` | `List<string>?` | `null` | Whitelist of processor names to run (None = all enabled) |
| `DisabledProcessors` | `List<string>?` | `null` | Blacklist of processor names to skip (None = none disabled) |
| `EnabledSet` | `string?` | `null` | Pre-computed AHashSet for O(1) enabled processor lookup |
| `DisabledSet` | `string?` | `null` | Pre-computed AHashSet for O(1) disabled processor lookup |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public PostProcessorConfig CreateDefault()
```


---

#### PptxAppProperties

Application properties from docProps/app.xml for PPTX

Contains PowerPoint-specific document metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Application` | `string?` | `null` | Application name (e.g., "Microsoft Office PowerPoint") |
| `AppVersion` | `string?` | `null` | Application version |
| `TotalTime` | `int?` | `null` | Total editing time in minutes |
| `Company` | `string?` | `null` | Company name |
| `DocSecurity` | `int?` | `null` | Document security level |
| `ScaleCrop` | `bool?` | `null` | Scale crop flag |
| `LinksUpToDate` | `bool?` | `null` | Links up to date flag |
| `SharedDoc` | `bool?` | `null` | Shared document flag |
| `HyperlinksChanged` | `bool?` | `null` | Hyperlinks changed flag |
| `Slides` | `int?` | `null` | Number of slides |
| `Notes` | `int?` | `null` | Number of notes |
| `HiddenSlides` | `int?` | `null` | Number of hidden slides |
| `MultimediaClips` | `int?` | `null` | Number of multimedia clips |
| `PresentationFormat` | `string?` | `null` | Presentation format (e.g., "Widescreen", "Standard") |
| `SlideTitles` | `List<string>` | `new List<string>()` | Slide titles |


---

#### PptxExtractionResult

PowerPoint (PPTX) extraction result.

Contains extracted slide content, metadata, and embedded images/tables.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | Extracted text content from all slides |
| `Metadata` | `PptxMetadata` | — | Presentation metadata |
| `SlideCount` | `nuint` | — | Total number of slides |
| `ImageCount` | `nuint` | — | Total number of embedded images |
| `TableCount` | `nuint` | — | Total number of tables |
| `Images` | `List<ExtractedImage>` | — | Extracted images from the presentation |
| `PageStructure` | `PageStructure?` | `null` | Slide structure with boundaries (when page tracking is enabled) |
| `PageContents` | `List<PageContent>?` | `null` | Per-slide content (when page tracking is enabled) |
| `Document` | `DocumentStructure?` | `null` | Structured document representation |
| `Hyperlinks` | `List<string>` | — | Hyperlinks discovered in slides as (url, optional_label) pairs. |
| `OfficeMetadata` | `Dictionary<string, string>` | — | Office metadata extracted from docProps/core.xml and docProps/app.xml. Contains keys like "title", "author", "created_by", "subject", "keywords", "modified_by", "created_at", "modified_at", etc. |


---

#### PptxMetadata

PowerPoint presentation metadata.

Extracted from PPTX files containing slide counts and presentation details.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `SlideCount` | `nuint` | — | Total number of slides in the presentation |
| `SlideNames` | `List<string>` | `new List<string>()` | Names of slides (if available) |
| `ImageCount` | `nuint?` | `null` | Number of embedded images |
| `TableCount` | `nuint?` | `null` | Number of tables |


---

#### ProcessingWarning

A non-fatal warning from a processing pipeline stage.

Captures errors from optional features that don't prevent extraction
but may indicate degraded results.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Source` | `string` | — | The pipeline stage or feature that produced this warning (e.g., "embedding", "chunking", "language_detection", "output_format"). |
| `Message` | `string` | — | Human-readable description of what went wrong. |


---

#### PstMetadata

Outlook PST archive metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MessageCount` | `nuint` | — | Number of messages |


---

#### RakeParams

RAKE-specific parameters.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `MinWordLength` | `nuint` | `1` | Minimum word length to consider (default: 1). |
| `MaxWordsPerPhrase` | `nuint` | `3` | Maximum words in a keyword phrase (default: 3). |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public RakeParams CreateDefault()
```


---

#### RecognizedTable

Pre-computed table markdown for a table detection region.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `DetectionBbox` | `BBox` | — | Detection bbox that this table corresponds to (for matching). |
| `Cells` | `List<List<string>>` | — | Table cells as a 2D vector (rows x columns). |
| `Markdown` | `string` | — | Rendered markdown table. |


---

#### Recyclable

Trait for types that can be pooled and reused.

Implementing this trait allows a type to be used with `Pool<T>`.
The `reset()` method should clear the object's state for reuse.

##### Methods

###### Reset()

Reset the object to a reusable state.

This is called when returning an object to the pool.
Should clear any internal data while preserving capacity.

**Signature:**

```csharp
public void Reset()
```


---

#### ResolvedStyle

Fully resolved (flattened) style after walking the inheritance chain.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ParagraphProperties` | `string` | — | Paragraph properties |
| `RunProperties` | `string` | — | Run properties |


---

#### ServerConfig

API server configuration.

This struct holds all configuration options for the Kreuzberg API server,
including host/port settings, CORS configuration, and upload limits.

# Defaults

- `host`: "127.0.0.1" (localhost only)
- `port`: 8000
- `cors_origins`: empty vector (allows all origins)
- `max_request_body_bytes`: 104_857_600 (100 MB)
- `max_multipart_field_bytes`: 104_857_600 (100 MB)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Host` | `string` | — | Server host address (e.g., "127.0.0.1", "0.0.0.0") |
| `Port` | `ushort` | — | Server port number |
| `CorsOrigins` | `List<string>` | `new List<string>()` | CORS allowed origins. Empty vector means allow all origins. If this is an empty vector, the server will accept requests from any origin. If populated with specific origins (e.g., ["<https://example.com">]), only those origins will be allowed. |
| `MaxRequestBodyBytes` | `nuint` | — | Maximum size of request body in bytes (default: 100 MB) |
| `MaxMultipartFieldBytes` | `nuint` | — | Maximum size of multipart fields in bytes (default: 100 MB) |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public ServerConfig CreateDefault()
```

###### ListenAddr()

Get the server listen address (host:port).

**Signature:**

```csharp
public string ListenAddr()
```

###### CorsAllowsAll()

Check if CORS allows all origins.

Returns `true` if the `cors_origins` vector is empty, meaning all origins
are allowed. Returns `false` if specific origins are configured.

**Signature:**

```csharp
public bool CorsAllowsAll()
```

###### IsOriginAllowed()

Check if a given origin is allowed by CORS configuration.

Returns `true` if:
- CORS allows all origins (empty origins list), or
- The given origin is in the allowed origins list

**Signature:**

```csharp
public bool IsOriginAllowed(string origin)
```

###### MaxRequestBodyMb()

Get maximum request body size in megabytes (rounded up).

**Signature:**

```csharp
public nuint MaxRequestBodyMb()
```

###### MaxMultipartFieldMb()

Get maximum multipart field size in megabytes (rounded up).

**Signature:**

```csharp
public nuint MaxMultipartFieldMb()
```


---

#### StreamReader


---

#### StringBufferPool

Convenience type alias for a pooled String.


---

#### StringGrowthValidator

Helper struct for tracking and validating string growth.


---

#### StructuredData

Structured data (Schema.org, microdata, RDFa) block.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `DataType` | `StructuredDataType` | — | Type of structured data |
| `RawJson` | `string` | — | Raw JSON string representation |
| `SchemaType` | `string?` | `null` | Schema type if detectable (e.g., "Article", "Event", "Product") |


---

#### StructuredDataResult

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | The extracted text content |
| `Format` | `string` | — | Format |
| `Metadata` | `Dictionary<string, string>` | — | Document metadata |
| `TextFields` | `List<string>` | — | Text fields |


---

#### StructuredExtractionConfig

Configuration for LLM-based structured data extraction.

Sends extracted document content to a VLM with a JSON schema,
returning structured data that conforms to the schema.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Schema` | `object` | — | JSON Schema defining the desired output structure. |
| `SchemaName` | `string` | — | Schema name passed to the LLM's structured output mode. |
| `SchemaDescription` | `string?` | `null` | Optional schema description for the LLM. |
| `Strict` | `bool` | — | Enable strict mode — output must exactly match the schema. |
| `Prompt` | `string?` | `null` | Custom Jinja2 extraction prompt template. When `null`, a default template is used. Available template variables: - `{{ content }}` — The extracted document text. - `{{ schema }}` — The JSON schema as a formatted string. - `{{ schema_name }}` — The schema name. - `{{ schema_description }}` — The schema description (may be empty). |
| `Llm` | `LlmConfig` | — | LLM configuration for the extraction. |


---

#### StructuredExtractionResponse

Response from structured extraction endpoint.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `StructuredOutput` | `object` | — | Structured data conforming to the provided JSON schema |
| `Content` | `string` | — | Extracted document text content |
| `MimeType` | `string` | — | Detected MIME type of the input file |


---

#### StyleDefinition

A single style definition parsed from `<w:style>` in `word/styles.xml`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Id` | `string` | — | The style ID (`w:styleId` attribute). |
| `Name` | `string?` | `null` | Human-readable name (`<w:name w:val="..."/>`). |
| `StyleType` | `string` | — | Style type: paragraph, character, table, or numbering. |
| `BasedOn` | `string?` | `null` | ID of the parent style (`<w:basedOn w:val="..."/>`). |
| `NextStyle` | `string?` | `null` | ID of the style to apply to the next paragraph (`<w:next w:val="..."/>`). |
| `IsDefault` | `bool` | — | Whether this is the default style for its type. |
| `ParagraphProperties` | `string` | — | Paragraph properties defined directly on this style. |
| `RunProperties` | `string` | — | Run properties defined directly on this style. |


---

#### SupportedFormat

A supported document format entry.

Represents a file extension and its corresponding MIME type that Kreuzberg can process.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Extension` | `string` | — | File extension (without leading dot), e.g., "pdf", "docx" |
| `MimeType` | `string` | — | MIME type string, e.g., "application/pdf" |


---

#### SyncExtractor

Trait for extractors that can work synchronously (WASM-compatible).

This trait defines the synchronous extraction interface for WASM targets and other
environments where async/tokio runtimes are not available or desirable.

# Implementation

Extractors that need to support WASM should implement this trait in addition to
the async `DocumentExtractor` trait. This allows the same extractor to work in both
environments by delegating to the sync implementation.

# MIME Type Validation

The `mime_type` parameter is guaranteed to be already validated.

##### Methods

###### ExtractSync()

Extract content from a byte array synchronously.

This method performs extraction without requiring an async runtime.
It is called by `extract_bytes_sync()` when the `tokio-runtime` feature is disabled.

**Returns:**

An `InternalDocument` containing the extracted elements, metadata, and tables.

**Signature:**

```csharp
public string ExtractSync(byte[] content, string mimeType, ExtractionConfig config)
```


---

#### TableProperties

Table-level properties from `<w:tblPr>`.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `StyleId` | `string?` | `null` | Style id |
| `Width` | `string?` | `null` | Width |
| `Alignment` | `string?` | `null` | Alignment |
| `Layout` | `string?` | `null` | Layout |
| `Look` | `string?` | `null` | Look |
| `Borders` | `string?` | `null` | Borders |
| `CellMargins` | `string?` | `null` | Cell margins |
| `Indent` | `string?` | `null` | Indent |
| `Caption` | `string?` | `null` | Caption |


---

#### TableValidator

Helper struct for validating table cell counts.


---

#### TessdataManager

Manages tessdata file downloading, caching, and manifest generation.


---

#### TesseractConfig

Tesseract OCR configuration.

Provides fine-grained control over Tesseract OCR engine parameters.
Most users can use the defaults, but these settings allow optimization
for specific document types (invoices, handwriting, etc.).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Language` | `string` | `"eng"` | Language code (e.g., "eng", "deu", "fra") |
| `Psm` | `int` | `3` | Page Segmentation Mode (0-13). Common values: - 3: Fully automatic page segmentation (default) - 6: Assume a single uniform block of text - 11: Sparse text with no particular order |
| `OutputFormat` | `string` | `"markdown"` | Output format ("text" or "markdown") |
| `Oem` | `int` | `3` | OCR Engine Mode (0-3). - 0: Legacy engine only - 1: Neural nets (LSTM) only (usually best) - 2: Legacy + LSTM - 3: Default (based on what's available) |
| `MinConfidence` | `double` | `0` | Minimum confidence threshold (0.0-100.0). Words with confidence below this threshold may be rejected or flagged. |
| `Preprocessing` | `ImagePreprocessingConfig?` | `null` | Image preprocessing configuration. Controls how images are preprocessed before OCR. Can significantly improve quality for scanned documents or low-quality images. |
| `EnableTableDetection` | `bool` | `true` | Enable automatic table detection and reconstruction |
| `TableMinConfidence` | `double` | `0` | Minimum confidence threshold for table detection (0.0-1.0) |
| `TableColumnThreshold` | `int` | `50` | Column threshold for table detection (pixels) |
| `TableRowThresholdRatio` | `double` | `0.5` | Row threshold ratio for table detection (0.0-1.0) |
| `UseCache` | `bool` | `true` | Enable OCR result caching |
| `ClassifyUsePreAdaptedTemplates` | `bool` | `true` | Use pre-adapted templates for character classification |
| `LanguageModelNgramOn` | `bool` | `false` | Enable N-gram language model |
| `TesseditDontBlkrejGoodWds` | `bool` | `true` | Don't reject good words during block-level processing |
| `TesseditDontRowrejGoodWds` | `bool` | `true` | Don't reject good words during row-level processing |
| `TesseditEnableDictCorrection` | `bool` | `true` | Enable dictionary correction |
| `TesseditCharWhitelist` | `string` | `""` | Whitelist of allowed characters (empty = all allowed) |
| `TesseditCharBlacklist` | `string` | `""` | Blacklist of forbidden characters (empty = none forbidden) |
| `TesseditUsePrimaryParamsModel` | `bool` | `true` | Use primary language params model |
| `TextordSpaceSizeIsVariable` | `bool` | `true` | Variable-width space detection |
| `ThresholdingMethod` | `bool` | `false` | Use adaptive thresholding method |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public TesseractConfig CreateDefault()
```


---

#### TextAnnotation

Inline text annotation — byte-range based formatting and links.

Annotations reference byte offsets into the node's text content,
enabling precise identification of formatted regions.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Start` | `uint` | — | Start byte offset in the node's text content (inclusive). |
| `End` | `uint` | — | End byte offset in the node's text content (exclusive). |
| `Kind` | `AnnotationKind` | — | Annotation type. |


---

#### TextExtractionResult

Plain text and Markdown extraction result.

Contains the extracted text along with statistics and,
for Markdown files, structural elements like headers and links.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | Extracted text content |
| `LineCount` | `nuint` | — | Number of lines |
| `WordCount` | `nuint` | — | Number of words |
| `CharacterCount` | `nuint` | — | Number of characters |
| `Headers` | `List<string>?` | `null` | Markdown headers (text only, Markdown files only) |
| `Links` | `List<string>?` | `null` | Markdown links as (text, URL) tuples (Markdown files only) |
| `CodeBlocks` | `List<string>?` | `null` | Code blocks as (language, code) tuples (Markdown files only) |


---

#### TextMetadata

Text/Markdown metadata.

Extracted from plain text and Markdown files. Includes word counts and,
for Markdown, structural elements like headers and links.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `LineCount` | `nuint` | — | Number of lines in the document |
| `WordCount` | `nuint` | — | Number of words |
| `CharacterCount` | `nuint` | — | Number of characters |
| `Headers` | `List<string>?` | `new List<string>()` | Markdown headers (headings text only, for Markdown files) |
| `Links` | `List<string>?` | `new List<string>()` | Markdown links as (text, url) tuples (for Markdown files) |
| `CodeBlocks` | `List<string>?` | `new List<string>()` | Code blocks as (language, code) tuples (for Markdown files) |


---

#### TokenReductionConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Level` | `ReductionLevel` | `ReductionLevel.Moderate` | Level (reduction level) |
| `LanguageHint` | `string?` | `null` | Language hint |
| `PreserveMarkdown` | `bool` | `false` | Preserve markdown |
| `PreserveCode` | `bool` | `true` | Preserve code |
| `SemanticThreshold` | `float` | `0.3` | Semantic threshold |
| `EnableParallel` | `bool` | `true` | Enable parallel |
| `UseSimd` | `bool` | `true` | Use simd |
| `CustomStopwords` | `Dictionary<string, List<string>>?` | `null` | Custom stopwords |
| `PreservePatterns` | `List<string>` | `new List<string>()` | Preserve patterns |
| `TargetReduction` | `float?` | `null` | Target reduction |
| `EnableSemanticClustering` | `bool` | `false` | Enable semantic clustering |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public TokenReductionConfig CreateDefault()
```


---

#### TokenReductionOptions

Token reduction configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Mode` | `string` | — | Reduction mode: "off", "light", "moderate", "aggressive", "maximum" |
| `PreserveImportantWords` | `bool` | — | Preserve important words (capitalized, technical terms) |


---

#### TracingLayer

A `tower.Layer` that wraps each extraction in a semantic tracing span.


---

#### TreeSitterConfig

Configuration for tree-sitter language pack integration.

Controls grammar download behavior and code analysis options.

# Example (TOML)

```toml
[tree_sitter]
languages = ["python", "rust"]
groups = ["web"]

[tree_sitter.process]
structure = true
comments = true
docstrings = true
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Enabled` | `bool` | `true` | Enable code intelligence processing (default: true). When `false`, tree-sitter analysis is completely skipped even if the config section is present. |
| `CacheDir` | `string?` | `null` | Custom cache directory for downloaded grammars. When `null`, uses the default: `~/.cache/tree-sitter-language-pack/v{version}/libs/`. |
| `Languages` | `List<string>?` | `null` | Languages to pre-download on init (e.g., `["python", "rust"]`). |
| `Groups` | `List<string>?` | `null` | Language groups to pre-download (e.g., `["web", "systems", "scripting"]`). |
| `Process` | `TreeSitterProcessConfig` | — | Processing options for code analysis. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public TreeSitterConfig CreateDefault()
```


---

#### TreeSitterProcessConfig

Processing options for tree-sitter code analysis.

Controls which analysis features are enabled when extracting code files.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Structure` | `bool` | `true` | Extract structural items (functions, classes, structs, etc.). Default: true. |
| `Imports` | `bool` | `true` | Extract import statements. Default: true. |
| `Exports` | `bool` | `true` | Extract export statements. Default: true. |
| `Comments` | `bool` | `false` | Extract comments. Default: false. |
| `Docstrings` | `bool` | `false` | Extract docstrings. Default: false. |
| `Symbols` | `bool` | `false` | Extract symbol definitions. Default: false. |
| `Diagnostics` | `bool` | `false` | Include parse diagnostics. Default: false. |
| `ChunkMaxSize` | `nuint?` | `null` | Maximum chunk size in bytes. `null` disables chunking. |
| `ContentMode` | `CodeContentMode` | `CodeContentMode.Chunks` | Content rendering mode for code extraction. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public TreeSitterProcessConfig CreateDefault()
```


---

#### Uri

A URI extracted from a document.

Represents any link, reference, or resource pointer found during extraction.
The `kind` field classifies the URI semantically, while `label` carries
optional human-readable display text.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Url` | `string` | — | The URL or path string. |
| `Label` | `string?` | `null` | Optional display text / label for the link. |
| `Page` | `uint?` | `null` | Optional page number where the URI was found (1-indexed). |
| `Kind` | `UriKind` | — | Semantic classification of the URI. |


---

#### VersionResponse

Version response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Version` | `string` | — | Kreuzberg version string |


---

#### WarmRequest

Cache warm request.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `AllEmbeddings` | `bool` | — | Download all embedding model presets |
| `EmbeddingModel` | `string?` | `null` | Specific embedding model preset to download |


---

#### WarmResponse

Cache warm response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `CacheDir` | `string` | — | Cache directory used |
| `Downloaded` | `List<string>` | — | Models that were downloaded |
| `AlreadyCached` | `List<string>` | — | Models that were already cached |


---

#### XlsxAppProperties

Application properties from docProps/app.xml for XLSX

Contains Excel-specific document metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Application` | `string?` | `null` | Application name (e.g., "Microsoft Excel") |
| `AppVersion` | `string?` | `null` | Application version |
| `DocSecurity` | `int?` | `null` | Document security level |
| `ScaleCrop` | `bool?` | `null` | Scale crop flag |
| `LinksUpToDate` | `bool?` | `null` | Links up to date flag |
| `SharedDoc` | `bool?` | `null` | Shared document flag |
| `HyperlinksChanged` | `bool?` | `null` | Hyperlinks changed flag |
| `Company` | `string?` | `null` | Company name |
| `WorksheetNames` | `List<string>` | `new List<string>()` | Worksheet names |


---

#### XmlExtractionResult

XML extraction result.

Contains extracted text content from XML files along with
structural statistics about the XML document.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Content` | `string` | — | Extracted text content (XML structure filtered out) |
| `ElementCount` | `nuint` | — | Total number of XML elements processed |
| `UniqueElements` | `List<string>` | — | List of unique element names found (sorted) |


---

#### XmlMetadata

XML metadata extracted during XML parsing.

Provides statistics about XML document structure.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `ElementCount` | `nuint` | — | Total number of XML elements processed |
| `UniqueElements` | `List<string>` | `new List<string>()` | List of unique element tag names (sorted) |


---

#### YakeParams

YAKE-specific parameters.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `WindowSize` | `nuint` | `2` | Window size for co-occurrence analysis (default: 2). Controls the context window for computing co-occurrence statistics. |

##### Methods

###### CreateDefault()

**Signature:**

```csharp
public YakeParams CreateDefault()
```


---

#### YearRange

Year range for bibliographic metadata.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `Min` | `uint?` | `null` | Min |
| `Max` | `uint?` | `null` | Max |
| `Years` | `List<uint>` | — | Years |


---

#### ZipBombValidator

Helper struct for validating ZIP archives for security issues.


---

### Enums

#### ExecutionProviderType

ONNX Runtime execution provider type.

Determines which hardware backend is used for model inference.
`Auto` (default) selects the best available provider per platform.

| Value | Description |
|-------|-------------|
| `Auto` | Auto-select: CoreML on macOS, CUDA on Linux, CPU elsewhere. |
| `Cpu` | CPU execution provider (always available). |
| `CoreMl` | Apple CoreML (macOS/iOS Neural Engine + GPU). |
| `Cuda` | NVIDIA CUDA GPU acceleration. |
| `TensorRt` | NVIDIA TensorRT (optimized CUDA inference). |


---

#### HtmlTheme

Built-in HTML theme selection.

| Value | Description |
|-------|-------------|
| `Default` | Sensible defaults: system font stack, neutral colours, readable line measure. CSS custom properties (`--kb-*`) are all defined so user CSS can override individual values. |
| `GitHub` | GitHub Markdown-inspired palette and spacing. |
| `Dark` | Dark background, light text. |
| `Light` | Minimal light theme with generous whitespace. |
| `Unstyled` | No built-in stylesheet emitted. CSS custom properties are still defined on `:root` so user stylesheets can reference `var(--kb-*)` tokens. |


---

#### TableModel

Which table structure recognition model to use.

Controls the model used for table cell detection within layout-detected
table regions.

| Value | Description |
|-------|-------------|
| `Tatr` | TATR (Table Transformer) -- default, 30MB, DETR-based row/column detection. |
| `SlanetWired` | SLANeXT wired variant -- 365MB, optimized for bordered tables. |
| `SlanetWireless` | SLANeXT wireless variant -- 365MB, optimized for borderless tables. |
| `SlanetPlus` | SLANet-plus -- 7.78MB, lightweight general-purpose. |
| `SlanetAuto` | Classifier-routed SLANeXT: auto-select wired/wireless per table. Uses PP-LCNet classifier (6.78MB) + both SLANeXT variants (730MB total). |
| `Disabled` | Disable table structure model inference entirely; use heuristic path only. |


---

#### PdfBackend

PDF extraction backend selection.

Controls which PDF library is used for text extraction:
- `Pdfium`: pdfium-render (default, C++ based, mature)
- `PdfOxide`: pdf_oxide (pure Rust, faster, requires `pdf-oxide` feature)
- `Auto`: automatically select based on available features

| Value | Description |
|-------|-------------|
| `Pdfium` | Use pdfium-render backend (default). |
| `PdfOxide` | Use pdf_oxide backend (pure Rust). Requires `pdf-oxide` feature. |
| `Auto` | Automatically select the best available backend. |


---

#### ChunkerType

Type of text chunker to use.

# Variants

* `Text` - Generic text splitter, splits on whitespace and punctuation
* `Markdown` - Markdown-aware splitter, preserves formatting and structure
* `Yaml` - YAML-aware splitter, creates one chunk per top-level key
* `Semantic` - Topic-aware chunker. With an `EmbeddingConfig`, splits at
  embedding-based topic shifts tuned by `topic_threshold` (default 0.75,
  lower = more splits). Without an embedding, falls back to a
  structural-boundary heuristic (ALL-CAPS headers, numbered sections,
  blank-line paragraphs) and merges groups into chunks capped at
  `max_characters` (default 1000). `topic_threshold` has no effect in the
  fallback path. For best results, pair with an embedding model.

| Value | Description |
|-------|-------------|
| `Text` | Text format |
| `Markdown` | Markdown format |
| `Yaml` | Yaml format |
| `Semantic` | Semantic |


---

#### ChunkSizing

How chunk size is measured.

Defaults to `Characters` (Unicode character count). When using token-based sizing,
chunks are sized by token count according to the specified tokenizer.

Token-based sizing uses HuggingFace tokenizers loaded at runtime. Any tokenizer
available on HuggingFace Hub can be used, including OpenAI-compatible tokenizers
(e.g., `Xenova/gpt-4o`, `Xenova/cl100k_base`).

| Value | Description |
|-------|-------------|
| `Characters` | Size measured in Unicode characters (default). |
| `Tokenizer` | Size measured in tokens from a HuggingFace tokenizer. — Fields: `Model`: `string`, `CacheDir`: `string` |


---

#### EmbeddingModelType

Embedding model types supported by Kreuzberg.

| Value | Description |
|-------|-------------|
| `Preset` | Use a preset model configuration (recommended) — Fields: `Name`: `string` |
| `Custom` | Use a custom ONNX model from HuggingFace — Fields: `ModelId`: `string`, `Dimensions`: `nuint` |
| `Llm` | Provider-hosted embedding model via liter-llm. Uses the model specified in the nested `LlmConfig` (e.g., `"openai/text-embedding-3-small"`). — Fields: `Llm`: `LlmConfig` |


---

#### CodeContentMode

Content rendering mode for code extraction.

Controls how extracted code content is represented in the `content` field
of `ExtractionResult`.

| Value | Description |
|-------|-------------|
| `Chunks` | Use TSLP semantic chunks as content (default). |
| `Raw` | Use raw source code as content. |
| `Structure` | Emit function/class headings + docstrings (no code bodies). |


---

#### FracType

| Value | Description |
|-------|-------------|
| `Bar` | Bar |
| `NoBar` | No bar |
| `Linear` | Linear |
| `Skewed` | Skewed |


---

#### ReductionLevel

| Value | Description |
|-------|-------------|
| `Off` | Off |
| `Light` | Light |
| `Moderate` | Moderate |
| `Aggressive` | Aggressive |
| `Maximum` | Maximum |


---

#### PdfAnnotationType

Type of PDF annotation.

| Value | Description |
|-------|-------------|
| `Text` | Sticky note / text annotation |
| `Highlight` | Highlighted text region |
| `Link` | Hyperlink annotation |
| `Stamp` | Rubber stamp annotation |
| `Underline` | Underline text markup |
| `StrikeOut` | Strikeout text markup |
| `Other` | Any other annotation type |


---

#### BlockType

Types of block-level elements in Djot.

| Value | Description |
|-------|-------------|
| `Paragraph` | Paragraph element |
| `Heading` | Heading element |
| `Blockquote` | Blockquote element |
| `CodeBlock` | Code block |
| `ListItem` | List item |
| `OrderedList` | Ordered list |
| `BulletList` | Bullet list |
| `TaskList` | Task list |
| `DefinitionList` | Definition list |
| `DefinitionTerm` | Definition term |
| `DefinitionDescription` | Definition description |
| `Div` | Div |
| `Section` | Section element |
| `ThematicBreak` | Thematic break |
| `RawBlock` | Raw block |
| `MathDisplay` | Math display |


---

#### InlineType

Types of inline elements in Djot.

| Value | Description |
|-------|-------------|
| `Text` | Text format |
| `Strong` | Strong |
| `Emphasis` | Emphasis |
| `Highlight` | Highlight |
| `Subscript` | Subscript |
| `Superscript` | Superscript |
| `Insert` | Insert |
| `Delete` | Delete |
| `Code` | Code |
| `Link` | Link |
| `Image` | Image element |
| `Span` | Span |
| `Math` | Math |
| `RawInline` | Raw inline |
| `FootnoteRef` | Footnote ref |
| `Symbol` | Symbol |


---

#### RelationshipKind

Semantic kind of a relationship between document elements.

| Value | Description |
|-------|-------------|
| `FootnoteReference` | Footnote marker -> footnote definition. |
| `CitationReference` | Citation marker -> bibliography entry. |
| `InternalLink` | Internal anchor link (`#id`) -> target heading/element. |
| `Caption` | Caption paragraph -> figure/table it describes. |
| `Label` | Label -> labeled element (HTML `<label for>`, LaTeX `\label{}`). |
| `TocEntry` | TOC entry -> target section. |
| `CrossReference` | Cross-reference (LaTeX `\ref{}`, DOCX cross-reference field). |


---

#### ContentLayer

Content layer classification for document nodes.

Replaces separate body/furniture arrays with per-node granularity.

| Value | Description |
|-------|-------------|
| `Body` | Main document body content. |
| `Header` | Page/section header (running header). |
| `Footer` | Page/section footer (running footer). |
| `Footnote` | Footnote content. |


---

#### NodeContent

Tagged enum for node content. Each variant carries only type-specific data.

Uses `#[serde(tag = "node_type")]` to avoid "type" keyword collision in
Go/Java/TypeScript bindings.

| Value | Description |
|-------|-------------|
| `Title` | Document title. — Fields: `Text`: `string` |
| `Heading` | Section heading with level (1-6). — Fields: `Level`: `byte`, `Text`: `string` |
| `Paragraph` | Body text paragraph. — Fields: `Text`: `string` |
| `List` | List container — children are `ListItem` nodes. — Fields: `Ordered`: `bool` |
| `ListItem` | Individual list item. — Fields: `Text`: `string` |
| `Table` | Table with structured cell grid. — Fields: `Grid`: `string` |
| `Image` | Image reference. — Fields: `Description`: `string`, `ImageIndex`: `uint`, `Src`: `string` |
| `Code` | Code block. — Fields: `Text`: `string`, `Language`: `string` |
| `Quote` | Block quote — container, children carry the quoted content. |
| `Formula` | Mathematical formula / equation. — Fields: `Text`: `string` |
| `Footnote` | Footnote reference content. — Fields: `Text`: `string` |
| `Group` | Logical grouping container (section, key-value area). `heading_level` + `heading_text` capture the section heading directly rather than relying on a first-child positional convention. — Fields: `Label`: `string`, `HeadingLevel`: `byte`, `HeadingText`: `string` |
| `PageBreak` | Page break marker. |
| `Slide` | Presentation slide container — children are the slide's content nodes. — Fields: `Number`: `uint`, `Title`: `string` |
| `DefinitionList` | Definition list container — children are `DefinitionItem` nodes. |
| `DefinitionItem` | Individual definition list entry with term and definition. — Fields: `Term`: `string`, `Definition`: `string` |
| `Citation` | Citation or bibliographic reference. — Fields: `Key`: `string`, `Text`: `string` |
| `Admonition` | Admonition / callout container (note, warning, tip, etc.). Children carry the admonition body content. — Fields: `Kind`: `string`, `Title`: `string` |
| `RawBlock` | Raw block preserved verbatim from the source format. Used for content that cannot be mapped to a semantic node type (e.g. JSX in MDX, raw LaTeX in markdown, embedded HTML). — Fields: `Format`: `string`, `Content`: `string` |
| `MetadataBlock` | Structured metadata block (email headers, YAML frontmatter, etc.). — Fields: `Entries`: `List<string>` |


---

#### AnnotationKind

Types of inline text annotations.

| Value | Description |
|-------|-------------|
| `Bold` | Bold |
| `Italic` | Italic |
| `Underline` | Underline |
| `Strikethrough` | Strikethrough |
| `Code` | Code |
| `Subscript` | Subscript |
| `Superscript` | Superscript |
| `Link` | Link — Fields: `Url`: `string`, `Title`: `string` |
| `Highlight` | Highlighted text (PDF highlights, HTML `<mark>`). |
| `Color` | Text color (CSS-compatible value, e.g. "#ff0000", "red"). — Fields: `Value`: `string` |
| `FontSize` | Font size with units (e.g. "12pt", "1.2em", "16px"). — Fields: `Value`: `string` |
| `Custom` | Extensible annotation for format-specific styling. — Fields: `Name`: `string`, `Value`: `string` |


---

#### ChunkType

Semantic structural classification of a text chunk.

Assigned by the heuristic classifier in `chunking.classifier`.
Defaults to `Unknown` when no rule matches.
Designed to be extended in future versions without breaking changes.

| Value | Description |
|-------|-------------|
| `Heading` | Section heading or document title. |
| `PartyList` | Party list: names, addresses, and signatories. |
| `Definitions` | Definition clause ("X means…", "X shall mean…"). |
| `OperativeClause` | Operative clause containing legal/contractual action verbs. |
| `SignatureBlock` | Signature block with signatures, names, and dates. |
| `Schedule` | Schedule, annex, appendix, or exhibit section. |
| `TableLike` | Table-like content with aligned columns or repeated patterns. |
| `Formula` | Mathematical formula or equation. |
| `CodeBlock` | Code block or preformatted content. |
| `Image` | Embedded or referenced image content. |
| `OrgChart` | Organizational chart or hierarchy diagram. |
| `Diagram` | Diagram, figure, or visual illustration. |
| `Unknown` | Unclassified or mixed content. |


---

#### ElementType

Semantic element type classification.

Categorizes text content into semantic units for downstream processing.
Supports the element types commonly found in Unstructured documents.

| Value | Description |
|-------|-------------|
| `Title` | Document title |
| `NarrativeText` | Main narrative text body |
| `Heading` | Section heading |
| `ListItem` | List item (bullet, numbered, etc.) |
| `Table` | Table element |
| `Image` | Image element |
| `PageBreak` | Page break marker |
| `CodeBlock` | Code block |
| `BlockQuote` | Block quote |
| `Footer` | Footer text |
| `Header` | Header text |


---

#### FormatMetadata

Format-specific metadata (discriminated union).

Only one format type can exist per extraction result. This provides
type-safe, clean metadata without nested optionals.

| Value | Description |
|-------|-------------|
| `Pdf` | Pdf format — Fields: `0`: `string` |
| `Docx` | Docx format — Fields: `0`: `DocxMetadata` |
| `Excel` | Excel — Fields: `0`: `ExcelMetadata` |
| `Email` | Email — Fields: `0`: `EmailMetadata` |
| `Pptx` | Pptx format — Fields: `0`: `PptxMetadata` |
| `Archive` | Archive — Fields: `0`: `ArchiveMetadata` |
| `Image` | Image element — Fields: `0`: `string` |
| `Xml` | Xml format — Fields: `0`: `XmlMetadata` |
| `Text` | Text format — Fields: `0`: `TextMetadata` |
| `Html` | Preserve as HTML `<mark>` tags — Fields: `0`: `HtmlMetadata` |
| `Ocr` | Ocr — Fields: `0`: `OcrMetadata` |
| `Csv` | Csv format — Fields: `0`: `CsvMetadata` |
| `Bibtex` | Bibtex — Fields: `0`: `BibtexMetadata` |
| `Citation` | Citation — Fields: `0`: `CitationMetadata` |
| `FictionBook` | Fiction book — Fields: `0`: `FictionBookMetadata` |
| `Dbf` | Dbf — Fields: `0`: `DbfMetadata` |
| `Jats` | Jats — Fields: `0`: `JatsMetadata` |
| `Epub` | Epub format — Fields: `0`: `EpubMetadata` |
| `Pst` | Pst — Fields: `0`: `PstMetadata` |
| `Code` | Code — Fields: `0`: `string` |


---

#### TextDirection

Text direction enumeration for HTML documents.

| Value | Description |
|-------|-------------|
| `LeftToRight` | Left-to-right text direction |
| `RightToLeft` | Right-to-left text direction |
| `Auto` | Automatic text direction detection |


---

#### LinkType

Link type classification.

| Value | Description |
|-------|-------------|
| `Anchor` | Anchor link (#section) |
| `Internal` | Internal link (same domain) |
| `External` | External link (different domain) |
| `Email` | Email link (mailto:) |
| `Phone` | Phone link (tel:) |
| `Other` | Other link type |


---

#### ImageType

Image type classification.

| Value | Description |
|-------|-------------|
| `DataUri` | Data URI image |
| `InlineSvg` | Inline SVG |
| `External` | External image URL |
| `Relative` | Relative path image |


---

#### StructuredDataType

Structured data type classification.

| Value | Description |
|-------|-------------|
| `JsonLd` | JSON-LD structured data |
| `Microdata` | Microdata |
| `RDFa` | RDFa |


---

#### OcrBoundingGeometry

Bounding geometry for an OCR element.

Supports both axis-aligned rectangles (from Tesseract) and 4-point quadrilaterals
(from PaddleOCR and rotated text detection).

| Value | Description |
|-------|-------------|
| `Rectangle` | Axis-aligned bounding box (typical for Tesseract output). — Fields: `Left`: `uint`, `Top`: `uint`, `Width`: `uint`, `Height`: `uint` |
| `Quadrilateral` | 4-point quadrilateral for rotated/skewed text (PaddleOCR). Points are in clockwise order starting from top-left: `[top_left, top_right, bottom_right, bottom_left]` — Fields: `Points`: `string` |


---

#### OcrElementLevel

Hierarchical level of an OCR element.

Maps to Tesseract's page segmentation hierarchy and provides
equivalent semantics for PaddleOCR.

| Value | Description |
|-------|-------------|
| `Word` | Individual word |
| `Line` | Line of text (default for PaddleOCR) |
| `Block` | Paragraph or text block |
| `Page` | Page-level element |


---

#### PageUnitType

Type of paginated unit in a document.

Distinguishes between different types of "pages" (PDF pages, presentation slides, spreadsheet sheets).

| Value | Description |
|-------|-------------|
| `Page` | Standard document pages (PDF, DOCX, images) |
| `Slide` | Presentation slides (PPTX, ODP) |
| `Sheet` | Spreadsheet sheets (XLSX, ODS) |


---

#### UriKind

Semantic classification of an extracted URI.

| Value | Description |
|-------|-------------|
| `Hyperlink` | A clickable hyperlink (web URL, file link). |
| `Image` | An image or media resource reference. |
| `Anchor` | An internal anchor or cross-reference target. |
| `Citation` | A citation or bibliographic reference (DOI, academic ref). |
| `Reference` | A general reference (e.g. `\ref{}` in LaTeX, `:ref:` in RST). |
| `Email` | An email address (`mailto:` link or bare email). |


---

#### PoolError

Error type for pool operations.

| Value | Description |
|-------|-------------|
| `LockPoisoned` | The pool's internal mutex was poisoned. This indicates a panic occurred while holding the lock. The pool is in a locked state and cannot be recovered. |


---

#### KeywordAlgorithm

Keyword algorithm selection.

| Value | Description |
|-------|-------------|
| `Yake` | YAKE (Yet Another Keyword Extractor) - statistical approach |
| `Rake` | RAKE (Rapid Automatic Keyword Extraction) - co-occurrence based |


---

#### PsmMode

Page Segmentation Mode for Tesseract OCR

| Value | Description |
|-------|-------------|
| `OsdOnly` | Osd only |
| `AutoOsd` | Auto osd |
| `AutoOnly` | Auto only |
| `Auto` | Auto |
| `SingleColumn` | Single column |
| `SingleBlockVertical` | Single block vertical |
| `SingleBlock` | Single block |
| `SingleLine` | Single line |
| `SingleWord` | Single word |
| `CircleWord` | Circle word |
| `SingleChar` | Single char |


---

#### PaddleLanguage

Supported languages in PaddleOCR.

Maps user-friendly language codes to paddle-ocr-rs language identifiers.

| Value | Description |
|-------|-------------|
| `English` | English |
| `Chinese` | Simplified Chinese |
| `Japanese` | Japanese |
| `Korean` | Korean |
| `German` | German |
| `French` | French |
| `Latin` | Latin script (covers most European languages) |
| `Cyrillic` | Cyrillic (Russian and related) |
| `TraditionalChinese` | Traditional Chinese |
| `Thai` | Thai |
| `Greek` | Greek |
| `EastSlavic` | East Slavic (Russian, Ukrainian, Belarusian) |
| `Arabic` | Arabic (Arabic, Persian, Urdu) |
| `Devanagari` | Devanagari (Hindi, Marathi, Sanskrit, Nepali) |
| `Tamil` | Tamil |
| `Telugu` | Telugu |


---

#### LayoutClass

The 17 canonical document layout classes.

All model backends (RT-DETR, YOLO, etc.) map their native class IDs
to this shared set. Models with fewer classes (DocLayNet: 11, PubLayNet: 5)
map to the closest equivalent.

| Value | Description |
|-------|-------------|
| `Caption` | Caption element |
| `Footnote` | Footnote element |
| `Formula` | Formula |
| `ListItem` | List item |
| `PageFooter` | Page footer |
| `PageHeader` | Page header |
| `Picture` | Picture |
| `SectionHeader` | Section header |
| `Table` | Table element |
| `Text` | Text format |
| `Title` | Title element |
| `DocumentIndex` | Document index |
| `Code` | Code |
| `CheckboxSelected` | Checkbox selected |
| `CheckboxUnselected` | Checkbox unselected |
| `Form` | Form |
| `KeyValueRegion` | Key value region |


---

### Errors

#### KreuzbergError

Main error type for all Kreuzberg operations.

All errors in Kreuzberg use this enum, which preserves error chains
and provides context for debugging.

# Variants

- `Io` - File system and I/O errors (always bubble up)
- `Parsing` - Document parsing errors (corrupt files, unsupported features)
- `Ocr` - OCR processing errors
- `Validation` - Input validation errors (invalid paths, config, parameters)
- `Cache` - Cache operation errors (non-fatal, can be ignored)
- `ImageProcessing` - Image manipulation errors
- `Serialization` - JSON/MessagePack serialization errors
- `MissingDependency` - Missing optional dependencies (tesseract, etc.)
- `Plugin` - Plugin-specific errors
- `LockPoisoned` - Mutex/RwLock poisoning (should not happen in normal operation)
- `UnsupportedFormat` - Unsupported MIME type or file format
- `Other` - Catch-all for uncommon errors

| Variant | Description |
|---------|-------------|
| `Io` | IO error: {0} |
| `Parsing` | Parsing error: {message} |
| `Ocr` | OCR error: {message} |
| `Validation` | Validation error: {message} |
| `Cache` | Cache error: {message} |
| `ImageProcessing` | Image processing error: {message} |
| `Serialization` | Serialization error: {message} |
| `MissingDependency` | Missing dependency: {0} |
| `Plugin` | Plugin error in '{plugin_name}': {message} |
| `LockPoisoned` | Lock poisoned: {0} |
| `UnsupportedFormat` | Unsupported format: {0} |
| `Embedding` | Embedding error: {message} |
| `Timeout` | Extraction timed out after {elapsed_ms}ms (limit: {limit_ms}ms) |
| `Cancelled` | Extraction cancelled |
| `Other` | {0} |


---


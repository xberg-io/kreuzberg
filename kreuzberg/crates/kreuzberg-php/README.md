# Kreuzberg PHP Bindings

This crate provides PHP bindings for the Kreuzberg document intelligence library using [ext-php-rs](https://github.com/davidcole1340/ext-php-rs).

## Structure

The PHP bindings follow the same architecture as the Python bindings (PyO3):

- **lib.rs**: Main module entry point with `#[php_module]` macro
- **config.rs**: Configuration type conversions (ExtractionConfig, OcrConfig, etc.)
- **extraction.rs**: Core extraction function implementations
- **types.rs**: Result type conversions (ExtractionResult, ExtractedTable, etc.)
- **error.rs**: Exception handling and error conversions

## Building

### Prerequisites

- Rust 1.91 or later
- PHP 8.0 or later with development headers
- Clang (required by ext-php-rs)

### Compile the extension

```bash
cargo build --release -p kreuzberg-php
```

The compiled extension will be at:

```text
target/release/libkreuzberg.so    # Linux
target/release/libkreuzberg.dylib  # macOS
target/release/kreuzberg.dll       # Windows
```

### Install the extension

1. Copy the compiled library to your PHP extension directory
2. Add to php.ini:

   ```ini
   extension=kreuzberg
   ```

## Usage

### Basic extraction

```php
<?php

// Extract a PDF file
$result = kreuzberg_extract_file("document.pdf");
echo $result->content;
print_r($result->metadata);

// Extract with custom configuration
$config = new ExtractionConfig();
$config->use_cache = false;
$config->force_ocr = true;

$result = kreuzberg_extract_file("scanned.pdf", null, $config);
```

### OCR configuration

```php
<?php

$config = new ExtractionConfig();
$config->ocr = new OcrConfig();
$config->ocr->language = "deu";  // German
$config->ocr->backend = "tesseract";

$result = kreuzberg_extract_file("german.pdf", null, $config);
```

### Batch processing

```php
<?php

$paths = ["doc1.pdf", "doc2.docx", "doc3.txt"];
$results = kreuzberg_batch_extract_files($paths);

foreach ($results as $i => $result) {
    echo "Document {$i}: {$result->mime_type}\n";
    echo substr($result->content, 0, 100) . "...\n\n";
}
```

### Extract from bytes

```php
<?php

$data = file_get_contents("document.pdf");
$result = kreuzberg_extract_bytes($data, "application/pdf");
echo $result->content;
```

### Working with tables

```php
<?php

$result = kreuzberg_extract_file("spreadsheet.xlsx");

foreach ($result->tables as $table) {
    echo "Table on page {$table->page_number}:\n";
    echo $table->markdown . "\n\n";

    // Access raw cells
    foreach ($table->cells as $row) {
        foreach ($row as $cell) {
            echo $cell . "\t";
        }
        echo "\n";
    }
}
```

### MIME type detection

```php
<?php

// Detect from file
$mime_type = kreuzberg_detect_mime_type_from_path("unknown_file");
echo "Detected: $mime_type\n";

// Detect from bytes
$data = file_get_contents("unknown_file");
$mime_type = kreuzberg_detect_mime_type_from_bytes($data);
echo "Detected: $mime_type\n";

// Validate MIME type
try {
    $normalized = kreuzberg_validate_mime_type("application/pdf");
    echo "Valid: $normalized\n";
} catch (Exception $e) {
    echo "Invalid MIME type\n";
}

// Get extensions for MIME type
$extensions = kreuzberg_get_extensions_for_mime("application/pdf");
print_r($extensions); // ["pdf"]
```

## API Reference

### Functions

#### kreuzberg_extract_file

```php
function kreuzberg_extract_file(
    string $path,
    ?string $mime_type = null,
    ?ExtractionConfig $config = null
): ExtractionResult
```

#### kreuzberg_extract_bytes

```php
function kreuzberg_extract_bytes(
    string $data,
    string $mime_type,
    ?ExtractionConfig $config = null
): ExtractionResult
```

#### kreuzberg_batch_extract_files

```php
function kreuzberg_batch_extract_files(
    array $paths,
    ?ExtractionConfig $config = null
): array
```

#### kreuzberg_batch_extract_bytes

```php
function kreuzberg_batch_extract_bytes(
    array $data_list,
    array $mime_types,
    ?ExtractionConfig $config = null
): array
```

#### kreuzberg_version

```php
function kreuzberg_version(): string
```

#### kreuzberg_detect_mime_type_from_bytes

```php
function kreuzberg_detect_mime_type_from_bytes(string $data): string
```

#### kreuzberg_detect_mime_type_from_path

```php
function kreuzberg_detect_mime_type_from_path(string $path): string
```

#### kreuzberg_validate_mime_type

```php
function kreuzberg_validate_mime_type(string $mime_type): string
```

#### kreuzberg_get_extensions_for_mime

```php
function kreuzberg_get_extensions_for_mime(string $mime_type): array
```

### Classes

#### ExtractionResult

```php
class ExtractionResult {
    public string $content;
    public string $mime_type;
    public array $metadata;
    public array $tables;
    public ?array $detected_languages;
    public ?array $images;
    public ?array $chunks;
    public ?array $pages;

    public function get_page_count(): int;
    public function get_chunk_count(): int;
    public function get_detected_language(): ?string;
}
```

#### ExtractedTable

```php
class ExtractedTable {
    public array $cells;
    public string $markdown;
    public int $page_number;
}
```

#### ExtractionConfig

```php
class ExtractionConfig {
    public bool $use_cache;
    public bool $enable_quality_processing;
    public bool $force_ocr;
    public ?OcrConfig $ocr;
    public ?PdfConfig $pdf_options;
    public ?ChunkingConfig $chunking;
    public ?ImageExtractionConfig $images;
    public ?TokenReductionConfig $token_reduction;
    public ?LanguageDetectionConfig $language_detection;
    public ?KeywordConfig $keywords;
    public ?PostProcessorConfig $postprocessor;
    public ?int $max_concurrent_extractions;
    public ?PageConfig $pages;

    public static function from_file(string $path): ExtractionConfig;
    public static function discover(): ExtractionConfig;
}
```

#### OcrConfig

```php
class OcrConfig {
    public string $backend;
    public string $language;
    public ?TesseractConfig $tesseract_config;
}
```

See other configuration classes in the source code.

### Exceptions

- **ValidationException**: Invalid configuration or parameters
- **ParsingException**: Document parsing failed
- **OcrException**: OCR processing failed
- **MissingDependencyException**: Required dependency missing
- **CacheException**: Cache operation failed
- **ImageProcessingException**: Image processing failed
- **PluginException**: Plugin operation failed

## Implementation Notes

### Architecture

The PHP bindings follow the same pattern as the Python bindings:

1. **Thin wrapper**: All core logic is in the Rust `kreuzberg` crate
2. **Zero duplication**: No reimplementation of extraction logic
3. **Type conversions**: Clean conversion between Rust and PHP types
4. **Error mapping**: Proper exception handling with error context

### Type Mapping

| Rust Type | PHP Type |
|-----------|----------|
| String | string |
| Vec<T> | array |
| Option<T> | T or null |
| HashMap<K,V> | array |
| bool | bool |
| i32, i64, usize | int |
| f32, f64 | float |
| Vec<u8> | string (binary) |

### Performance

- Uses synchronous extraction (no async support in ext-php-rs)
- Batch operations for processing multiple documents
- Efficient memory handling with ext-php-rs smart pointers

## Development

### Running tests

```bash
cargo test -p kreuzberg-php
```

### Building documentation

```bash
cargo doc -p kreuzberg-php --open
```

### Code style

Follow Rust 2024 edition conventions and ext-php-rs best practices.

## License

MIT License - see LICENSE file for details.

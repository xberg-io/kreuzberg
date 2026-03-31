# Kreuzberg PHP Bindings - Comprehensive Test Suite

This directory contains a comprehensive test application for the Kreuzberg PHP bindings (v4.3.8).

## What This Test Suite Covers

### Test Organization

The test suite (`main.php`) is organized into 17 sections, each testing a different aspect of the public API:

1. **Extension & Version** - Verifies the Kreuzberg class, VERSION constant, and version() method
2. **Configuration Classes** - Tests ExtractionConfig, OcrConfig, PdfConfig, ChunkingConfig with defaults and custom values
3. **Configuration Builder** - Tests the fluent builder pattern via ExtractionConfig::builder()
4. **Configuration Serialization** - Tests toArray, toJson, fromArray, fromJson roundtrips
5. **Exception Hierarchy** - Validates KreuzbergException and all factory methods (validation, parsing, ocr, io, plugin, unsupportedFormat)
6. **MIME Type Functions** - Tests MIME detection from bytes and file paths, extension mapping
7. **Plugin Registry - Validators** - Tests validator listing and clearing
8. **Plugin Registry - Post-Processors** - Tests post-processor listing and clearing
9. **Plugin Registry - OCR Backends** - Tests OCR backend listing and clearing
10. **Plugin Registry - Document Extractors** - Tests document extractor listing and clearing
11. **Kreuzberg Class Instance API** - Tests instance creation with and without config
12. **Extraction - File (Sync)** - Tests synchronous file extraction with various options
13. **Extraction - Bytes (Sync)** - Tests synchronous byte extraction
14. **Batch Extraction (Sync)** - Tests batch file and byte extraction
15. **Error Handling** - Tests error conditions (missing files, invalid JSON, missing config files)
16. **Static vs Instance API** - Verifies static and instance methods produce equivalent results
17. **ExtractionResult Structure** - Validates result object properties and types

## Running the Tests

### Prerequisites

- PHP 8.4 or higher
- Composer
- The kreuzberg PHP extension (native) or the mock fallback (automatic)

### Installation

```bash
# Install PHP package dependencies
cd ../../../packages/php
composer install

# Or if using the kreuzberg extension, build it first
cargo build --release -p kreuzberg-php
```

### Running Tests

```bash
# Using the test runner script
bash run_tests.sh

# Or directly with PHP
php main.php
```

### Expected Output

```text
========================================================================
  TEST SUMMARY
========================================================================
  Total:   70+
  Passed:  70+
  Failed:  0
  Skipped: 0

  ALL TESTS PASSED
========================================================================
```

## API Surface Verified

### Core Classes

- `Kreuzberg\Kreuzberg` - Main API class (instance and static methods)
- `Kreuzberg\Config\ExtractionConfig` - Master configuration
- `Kreuzberg\Config\ExtractionConfigBuilder` - Fluent builder
- `Kreuzberg\Config\OcrConfig` - OCR configuration
- `Kreuzberg\Config\PdfConfig` - PDF extraction configuration
- `Kreuzberg\Config\ChunkingConfig` - Text chunking configuration
- `Kreuzberg\Exceptions\KreuzbergException` - Exception hierarchy
- `Kreuzberg\Types\ExtractionResult` - Extraction result type

### Core Functions

```php
// Extraction (instance)
$kreuzberg->extractFile($path, $mimeType, $config)
$kreuzberg->extractBytes($data, $mimeType, $config)
$kreuzberg->batchExtractFiles($paths, $config)
$kreuzberg->batchExtractBytes($dataList, $mimeTypes, $config)

// Extraction (static)
Kreuzberg::extractFileSync($path, $mimeType, $config)
Kreuzberg::extractBytesSync($data, $mimeType, $config)
Kreuzberg::batchExtractFilesSync($paths, $config)
Kreuzberg::batchExtractBytesSync($dataList, $mimeTypes, $config)

// MIME Type
Kreuzberg::detectMimeType($bytes)
Kreuzberg::detectMimeTypeFromPath($path)
Kreuzberg::getExtensionsForMime($mimeType)

// Plugin Registry
Kreuzberg::listValidators()
Kreuzberg::clearValidators()
Kreuzberg::listPostProcessors()
Kreuzberg::clearPostProcessors()
Kreuzberg::listOcrBackends()
Kreuzberg::clearOcrBackends()
Kreuzberg::listDocumentExtractors()
Kreuzberg::clearDocumentExtractors()
```

### Configuration Classes

- `ExtractionConfig` - 18 parameters including useCache, forceOcr, outputFormat, resultFormat
- `OcrConfig` - backend, language, tesseractConfig, paddleOcrConfig, imagePreprocessing
- `PdfConfig` - extractImages, passwords, extractMetadata, extractAnnotations
- `ChunkingConfig` - maxChars, maxOverlap, respectSentences, respectParagraphs

### Exception Factory Methods

All created via `KreuzbergException::` static factory:

- `validation(message)` - code 1
- `parsing(message)` - code 2
- `ocr(message)` - code 3
- `missingDependency(message)` - code 4
- `io(message)` - code 5
- `plugin(message)` - code 6
- `unsupportedFormat(message)` - code 7

## Troubleshooting

### "Could not find Composer autoloader"

Run `composer install` in the `packages/php` directory:

```bash
cd packages/php
composer install
```

### Extension not loaded

The test suite automatically falls back to the mock extension. To use the native extension:

```bash
cargo build --release -p kreuzberg-php
php -d extension=/path/to/kreuzberg.so main.php
```

### Test documents not found

Ensure you are running from within the kreuzberg repository and the `test_documents/` directory exists at the repository root.

## Architecture Notes

The Kreuzberg PHP bindings use **ext-php-rs** (a Rust FFI framework for PHP) to call the core Rust library. The test suite verifies:

1. **FFI Layer** - Extension function availability and correct return types
2. **PHP Wrapper** - Idiomatic PHP API wrapping the extension functions
3. **Type Safety** - Readonly classes, strict types, proper exception handling
4. **Configuration** - Constructor promotion, builder pattern, JSON/array serialization
5. **Plugin System** - Registry listing and clearing operations

## See Also

- [Kreuzberg PHP Package](../../../packages/php/)
- [Kreuzberg E2E PHP Tests](../../../e2e/php/)
- [Kreuzberg Documentation](https://kreuzberg.dev)

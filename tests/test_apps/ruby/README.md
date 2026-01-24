# Kreuzberg Ruby Bindings - Comprehensive Test Suite

This directory contains a comprehensive test application for the Kreuzberg Ruby bindings (v4.0.0-rc.16).

## What This Test Suite Covers

### Test Organization

The test suite (`main_test.rb`) is organized into 15 sections, each testing a different aspect of the public API:

1. **Module Imports & Setup** - Verifies all modules and constants are accessible
2. **Configuration Classes** - Tests all config class creation and serialization
3. **Error Classes & Exception Hierarchy** - Validates error structure and inheritance
4. **MIME Type Functions** - Tests MIME detection and validation
5. **Plugin Registry - Validators** - Tests validator registration system
6. **Plugin Registry - Post-Processors** - Tests post-processor registration
7. **Plugin Registry - OCR Backends** - Tests OCR backend management
8. **Embedding Presets** - Tests embedding preset functions
9. **Cache API** - Tests cache management functions
10. **Result Object Structure** - Validates Result and nested struct definitions
11. **Extraction Functions - File-based (Sync)** - Tests file extraction
12. **Extraction Functions - Bytes-based (Sync)** - Tests byte extraction
13. **Batch Extraction (Sync)** - Tests batch operations
14. **Module Functions & API Aliases** - Verifies aliases and accessibility
15. **Error Context** - Tests error context and panic information

## Running the Tests

### Prerequisites

- Ruby 3.2.0 or higher (including Ruby 4.x)
- RubyGems
- The `kreuzberg` gem (4.0.0-rc.16) installed from RubyGems

### Installation

```bash
# Using bundler (recommended)
bundle install

# Or install the gem directly
gem install kreuzberg --pre
```

### Running Tests

```bash
# Using the test runner script
ruby main_test.rb

# Expected output shows 100+ test cases with pass/fail/skip results
```

## Test Coverage by Category

### Configuration Classes (18 tests)

Tests the following configuration classes:

- `Kreuzberg::Config::OCR`
  - Backend selection (tesseract, paddleocr)
  - Language specification
  - Tesseract configuration

- `Kreuzberg::Config::Chunking`
  - Chunk size configuration
  - Overlap settings
  - Preset-based chunking
  - Embedding configuration
  - Enable/disable flag

- `Kreuzberg::Config::ImagePreprocessing`
  - Preprocessing options
  - Serialization

- `Kreuzberg::Config::Tesseract`
  - OCR engine configuration
  - Nested preprocessing

- `Kreuzberg::Config::PDF`
  - Text extraction settings
  - Custom DPI

- `Kreuzberg::Config::ImageExtraction`
  - Image extraction settings
  - Dimension filtering

- `Kreuzberg::Config::PageConfig`
  - Page-specific settings

- `Kreuzberg::Config::Extraction` (ExtractionConfig)
  - Master configuration combining all options
  - Force OCR setting
  - Custom component configs

### Error Classes (11 tests)

Tests error handling with:

- `Kreuzberg::Errors::ValidationError`
- `Kreuzberg::Errors::ParsingError` (with context)
- `Kreuzberg::Errors::OCRError` (with context)
- `Kreuzberg::Errors::MissingDependencyError`
- `Kreuzberg::Errors::IOError`
- `Kreuzberg::Errors::PluginError`
- `Kreuzberg::Errors::UnsupportedFormatError`

### MIME Type Operations (7 tests)

- Detecting MIME type from bytes (magic number detection)
- Detecting MIME type from file path
- Validating MIME type strings
- Getting file extensions for MIME types

### Plugin Registry (11 tests)

- Validator registration/unregistration
- Post-processor registration/unregistration
- OCR backend management
- Plugin listing and clearing

### Extraction Functions (4 tests)

Tests both synchronous extraction variants:

- `extract_file_sync(path, mime_type, config)`
- `extract_bytes_sync(data, mime_type, config)`
- Batch extraction operations

### Result Object Structure (6 tests)

Validates the Result class and nested structures:

- `Result::Table` - Table extraction with cells, markdown, page number
- `Result::Chunk` - Text chunks with byte offsets and token counts
- `Result::Image` - Extracted images with metadata
- `Result::Page` - Page information (dimensions, rotation)
- `Result#to_h` - Serialization to Hash

## API Surface Verified

### Core Functions (21 module functions)

```ruby
# Extraction
Kreuzberg.extract_file_sync(path, mime_type: nil, config: nil)
Kreuzberg.extract_bytes_sync(data, mime_type, config: nil)
Kreuzberg.batch_extract_files_sync(paths, config: nil)
Kreuzberg.batch_extract_bytes_sync(data_list, mime_types, config: nil)

# MIME Type
Kreuzberg.detect_mime_type(bytes)
Kreuzberg.detect_mime_type_from_path(path)
Kreuzberg.validate_mime_type(mime_type)
Kreuzberg.get_extensions_for_mime(mime_type)

# Plugin Registry - Validators
Kreuzberg.register_validator(name, callable)
Kreuzberg.unregister_validator(name)
Kreuzberg.clear_validators()
Kreuzberg.list_validators()

# Plugin Registry - Post-Processors
Kreuzberg.register_post_processor(name, callable)
Kreuzberg.unregister_post_processor(name)
Kreuzberg.clear_post_processors()
Kreuzberg.list_post_processors()

# Plugin Registry - OCR Backends
Kreuzberg.register_ocr_backend(name, backend)
Kreuzberg.unregister_ocr_backend(name)
Kreuzberg.list_ocr_backends()

# Embeddings
Kreuzberg.list_embedding_presets()
Kreuzberg.get_embedding_preset(name)

# Cache
Kreuzberg.clear_cache()
Kreuzberg.cache_stats()
```

### Configuration Classes (9 classes)

- `Kreuzberg::Config::OCR`
- `Kreuzberg::Config::Chunking`
- `Kreuzberg::Config::ImagePreprocessing`
- `Kreuzberg::Config::Tesseract`
- `Kreuzberg::Config::PDF`
- `Kreuzberg::Config::ImageExtraction`
- `Kreuzberg::Config::PageConfig`
- `Kreuzberg::Config::Extraction` (ExtractionConfig)
- `Kreuzberg::Config::KeywordConfig`

### Error Classes (7 classes)

All inherit from `Kreuzberg::Errors::Error` which inherits from `StandardError`:

- `ValidationError`
- `ParsingError` (with context attribute)
- `OCRError` (with context attribute)
- `MissingDependencyError` (with dependency attribute)
- `IOError`
- `PluginError`
- `UnsupportedFormatError`

### Result Objects (5 structs)

- `Result` - Main extraction result
- `Result::Table` - Table with cells and markdown
- `Result::Chunk` - Text chunk with byte offsets
- `Result::Image` - Extracted image with metadata
- `Result::Page` - Page information

### Constants & Enums

- `Kreuzberg::KeywordAlgorithm::YAKE`
- `Kreuzberg::KeywordAlgorithm::RAKE`

### Module Setup

- `Kreuzberg::PostProcessorProtocol`
- `Kreuzberg::ValidatorProtocol`
- `Kreuzberg::OcrBackendProtocol`
- `Kreuzberg::ErrorContext`
- `Kreuzberg::Errors::PanicContext`

## Installation Notes

### Building from RubyGems

The Kreuzberg gem includes native extensions (Magnus/Rust bindings) that must be compiled:

```bash
gem install kreuzberg --pre
# Will trigger compilation of native extensions
```

### Requirements

- Rust compiler (cargo) - required to build native extensions
- C/C++ compiler (clang/gcc)
- GNU Make

## Test Results Interpretation

The test runner produces three types of output:

- **✓** - Test passed
- **✗** - Test failed with error details
- **⊘** - Test skipped (feature not available)

Final summary shows:
- Total number of tests
- Number passed
- Number failed
- Number skipped

Exit code is 0 if all tests pass, 1 if any fail.

## Known Limitations

### Async Functions

The current test suite only covers synchronous extraction functions. Async variants (`extract_file`, `extract_bytes`, `batch_extract_files`, `batch_extract_bytes`) are available but not tested here due to complexity of async testing in plain Ruby.

### Document Fixtures

Tests that require actual document files (PDF, DOCX, etc.) for extraction are skipped to avoid dependency on test files. The non-existent file test verifies error handling works.

### Native Library Path

On some systems, the native library path may need to be set:

```bash
export DYLD_LIBRARY_PATH=/path/to/kreuzberg/lib:$DYLD_LIBRARY_PATH
```

## Troubleshooting

### "cannot load such file -- kreuzberg_rb"

The native extension hasn't been built. Rebuild:

```bash
gem pristine kreuzberg --version 4.0.0.pre.rc.16
```

### Cargo manifest errors

If you see workspace root errors, the pre-release gem may have incomplete workspace configuration. Install the latest rc version:

```bash
gem install kreuzberg:4.0.0.pre.rc.16 --pre
```

### Permission errors

If you get permission errors during native extension compilation:

```bash
gem install kreuzberg --pre --user-install
```

## Architecture Notes

The Kreuzberg Ruby bindings use **Magnus** (a Rust FFI framework) to call the core Rust library. The test suite verifies:

1. **FFI Layer** - Native method availability and correct signatures
2. **Ruby Wrapper** - Idiomatic Ruby APIs wrapping the FFI
3. **Type Safety** - Proper error handling and type conversions
4. **Configuration** - Builder pattern and configuration serialization
5. **Plugin System** - Registration/unregistration of Ruby callbacks

## See Also

- [Kreuzberg GitHub](https://github.com/namurian/kreuzberg)
- [Kreuzberg RubyGems](https://rubygems.org/gems/kreuzberg)
- [Magnus Documentation](https://docs.rs/magnus/)

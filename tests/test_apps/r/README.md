# Kreuzberg R Bindings - Comprehensive Test Suite

Comprehensive standalone test suite for the Kreuzberg R package (kreuzberg v4.3.8).

## Test Suite Overview

The test suite (`main_test.R`) is organized into **14 sections** covering the full public API surface:

1. **Package Loading and Smoke Test** (9 tests) - Verifies all exported extraction functions exist
2. **Configuration Builders** (14 tests) - Tests `extraction_config()`, `ocr_config()`, `chunking_config()` creation and serialization
3. **Configuration Validation** (6 tests) - Error handling for invalid config parameters (negative dpi, zero max_characters, etc.)
4. **MIME Type Functions** (8 tests) - `detect_mime_type()`, `detect_mime_type_from_path()`, `validate_mime_type()`, `get_extensions_for_mime()`
5. **Validation Functions** (11 tests) - `validate_ocr_backend_name()`, `validate_language_code()`, `validate_output_format()`
6. **Plugin Registry Functions** (12 tests) - List, clear, and unregister operations for post-processors, validators, OCR backends, document extractors
7. **Cache Functions** (4 tests) - `clear_cache()`, `cache_stats()`
8. **Extraction - Plain Text (Sync)** (6 tests) - `extract_file_sync()`, `extract_file()`, `extract_bytes_sync()`, `extract_bytes()` with text content
9. **Extraction - Error Handling** (6 tests) - Non-existent files, invalid types, unsupported formats, typed kreuzberg_error conditions
10. **Result Object (S3 Methods)** (10 tests) - `content()`, `mime_type()`, `page_count()`, `chunk_count()`, `detected_language()`, `metadata_field()`, `print()`, `summary()`, `format()`
11. **Batch Extraction** (6 tests) - `batch_extract_files_sync()`, `batch_extract_files()`, `batch_extract_bytes_sync()`, `batch_extract_bytes()`, mismatched lengths
12. **File Extraction with Test Documents** (10 tests) - Real document extraction (TXT, HTML, PDF, DOCX, XLSX, JSON) with content assertions
13. **Extraction with Configuration Options** (3 tests) - Extraction with `force_ocr`, `ocr_config`, and `chunking_config`
14. **Config Discovery** (5 tests) - `discover()`, `from_file()` functions and input validation

## Requirements

- R >= 4.2
- The `kreuzberg` R package installed (with native extendr extension compiled)
- The `jsonlite` package (dependency of kreuzberg)
- Rust toolchain for building the native extension: `cargo build --release -p kreuzberg-r`

## Running the Tests

### Using the convenience script

```bash
cd tests/test_apps/r
bash run_tests.sh
```

### Manual execution

```bash
# Set library paths (macOS)
export DYLD_LIBRARY_PATH=/path/to/kreuzberg/target/release:$DYLD_LIBRARY_PATH

# Set library paths (Linux)
export LD_LIBRARY_PATH=/path/to/kreuzberg/target/release:$LD_LIBRARY_PATH

# Run the tests
Rscript main_test.R
```

### Expected Output

```text
================================================================================
KREUZBERG R BINDINGS COMPREHENSIVE TEST SUITE
================================================================================

[SECTION 1] Package Loading and Smoke Test
--------------------------------------------------------------------------------
  PASS  kreuzberg package is loaded
  PASS  extract_file_sync function exists
  ...

================================================================================
TEST SUMMARY
================================================================================
Total Tests: 112
  Passed:  112
  Failed:  0
  Skipped: 0
================================================================================

ALL TESTS PASSED
```

Exit code is 0 if all tests pass, 1 if any fail.

## Test Documents

Tests in Section 12 use real documents from `test_documents/` at the repository root:

- `text/fake_text.txt` - Plain text file
- `html/simple_table.html` - HTML with table content
- `pdf/fake_memo.pdf` - PDF memo document
- `docx/fake.docx` - Word document
- `xlsx/stanley_cups.xlsx` - Excel spreadsheet
- `json/simple.json` - JSON file

If the `test_documents/` directory is not found, these tests are automatically skipped.

## API Surface Verified

### Extraction Functions

- `extract_file_sync(path, mime_type, config)`
- `extract_file(path, mime_type, config)`
- `extract_bytes_sync(data, mime_type, config)`
- `extract_bytes(data, mime_type, config)`
- `batch_extract_files_sync(paths, config)`
- `batch_extract_files(paths, config)`
- `batch_extract_bytes_sync(data_list, mime_types, config)`
- `batch_extract_bytes(data_list, mime_types, config)`

### Configuration Functions

- `extraction_config(...)` - Main config builder
- `ocr_config(backend, language, dpi, ...)` - OCR settings
- `chunking_config(max_characters, overlap, ...)` - Chunking settings
- `discover()` - Auto-discover config from kreuzberg.toml
- `from_file(path)` - Load config from file

### MIME Type Functions

- `detect_mime_type(data)` - From raw bytes
- `detect_mime_type_from_path(path)` - From file path
- `validate_mime_type(mime_type)` - Validate MIME string
- `get_extensions_for_mime(mime_type)` - Get file extensions

### Validation Functions

- `validate_ocr_backend_name(backend)`
- `validate_language_code(code)`
- `validate_output_format(format)`

### Plugin Registry

- `list_post_processors()` / `clear_post_processors()` / `unregister_post_processor(name)`
- `list_validators()` / `clear_validators()` / `unregister_validator(name)`
- `list_ocr_backends()` / `clear_ocr_backends()` / `unregister_ocr_backend(name)`
- `list_document_extractors()` / `clear_document_extractors()` / `unregister_document_extractor(name)`

### Cache

- `clear_cache()`
- `cache_stats()`

### Result S3 Object

- `content(result)`, `mime_type(result)`, `page_count(result)`, `chunk_count(result)`
- `detected_language(result)`, `metadata_field(result, name)`
- `print()`, `summary()`, `format()` methods

## File Structure

```text
tests/test_apps/r/
  main_test.R      # Comprehensive test script (112 tests)
  run_tests.sh     # Convenience runner with library path setup
  README.md        # This file
```

## Troubleshooting

### "there is no package called 'kreuzberg'"

The kreuzberg R package needs to be installed. Build and install from source:

```bash
cd packages/r
R CMD INSTALL .
```

### Native library not found

Set the library path to include the compiled Rust library:

```bash
# macOS
export DYLD_LIBRARY_PATH=/path/to/kreuzberg/target/release:$DYLD_LIBRARY_PATH

# Linux
export LD_LIBRARY_PATH=/path/to/kreuzberg/target/release:$LD_LIBRARY_PATH
```

### Test documents not found

Ensure you are running from within the kreuzberg repository. The test script walks up the directory tree looking for `test_documents/`. Tests that require documents are skipped if not found.

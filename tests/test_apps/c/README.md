# Kreuzberg C FFI Test App

Comprehensive test suite for the Kreuzberg C FFI API (kreuzberg-ffi).

## Quick Start

```bash
# Build the FFI library first
cargo build --release -p kreuzberg-ffi

# Build and run the test suite
make test
```

## Prerequisites

- C11-compatible compiler (gcc or clang)
- Kreuzberg FFI library compiled: `cargo build --release -p kreuzberg-ffi`
- Test documents in `test_documents/` (symlink or copy from another test app)

## Test Coverage

The test suite (`main.c`) validates 15 sections of the C FFI API:

1. **Library Info** - Version, last error, panic context
2. **Error Code Functions** - All 8 error codes, names, descriptions, classification
3. **Configuration** - JSON parsing, serialization, validation, merging
4. **Config Builder** - Builder pattern, setters, build/free lifecycle
5. **MIME Type Functions** - Detection from path/bytes, validation, extensions
6. **Validation Functions** - Language codes, PSM/OEM, confidence, DPI, binarization
7. **Enum Parsing** - Heading style, code block style, whitespace mode, preprocessing
8. **File Extraction** - PDF, DOCX, XLSX extraction with/without config
9. **Bytes Extraction** - In-memory extraction with MIME type
10. **Error Handling** - Missing file errors, error details, NULL safety
11. **Batch Extraction** - Multi-file batch extraction
12. **Plugin Registry** - OCR backends, post-processors, validators, document extractors
13. **Embedding Presets** - List and get presets
14. **Result Structure** - All CExtractionResult fields
15. **String Operations** - Clone, intern, stats

## File Structure

```text
tests/test_apps/c/
├── main.c              # Comprehensive test suite
├── Makefile            # Build instructions
├── README.md           # This file
└── test_documents/     # Test documents (symlink or copy)
    ├── tiny.pdf
    ├── lorem_ipsum.docx
    ├── stanley_cups.xlsx
    ├── ocr_image.jpg
    └── test_hello_world.png
```

## Build Options

```bash
# Debug build
make BUILD_MODE=debug

# Use specific compiler
make CC=clang

# Custom repo root
make KREUZBERG_ROOT=/path/to/kreuzberg
```

## Expected Output

```text
================================================================================
KREUZBERG C FFI COMPREHENSIVE TEST SUITE
================================================================================
Library version: 4.3.8

[SECTION 1] Library Info
--------------------------------------------------------------------------------
  PASS  kreuzberg_version() returns "4.3.8"
  PASS  KREUZBERG_VERSION_MAJOR >= 4
  ...

================================================================================
TEST SUMMARY
================================================================================
Total Tests: 80+
  Passed:  80+
  Failed:  0
  Skipped: 0

ALL TESTS PASSED
```

Exit codes: `0` = all passed, `1` = failures detected.

## Troubleshooting

### Library not found at runtime

```bash
# macOS
export DYLD_LIBRARY_PATH=/path/to/kreuzberg/target/release:$DYLD_LIBRARY_PATH

# Linux
export LD_LIBRARY_PATH=/path/to/kreuzberg/target/release:$LD_LIBRARY_PATH
```

### Header not found

Ensure `KREUZBERG_ROOT` points to the kreuzberg repository root:

```bash
make KREUZBERG_ROOT=/path/to/kreuzberg test
```

### Test documents missing

Copy or symlink from another test app:

```bash
ln -s ../go/test_documents test_documents
# or
cp -r ../go/test_documents test_documents
```

## Environment

- **C Standard:** C11
- **Compilers:** gcc, clang
- **Kreuzberg:** 4.3.8
- **Platforms:** macOS (Intel/ARM), Linux (x86_64/aarch64)

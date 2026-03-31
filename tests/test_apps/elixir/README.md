# Kreuzberg Elixir Test App

Comprehensive test suite for the Kreuzberg Elixir binding.

## Overview

This test application validates the Kreuzberg Elixir binding across multiple document formats and features:

- **Smoke Tests**: Basic functionality verification
- **Extraction Tests**: Comprehensive document extraction across formats
- **Async Operations**: Task-based async extraction
- **Configuration**: Custom extraction configs
- **Batch Operations**: Multiple file processing
- **OCR**: Image text extraction
- **Chunking**: Content segmentation for RAG
- **Metadata**: Format-specific metadata extraction
- **Tables**: Structured table extraction
- **Cache**: Cache management operations

## Requirements

- Elixir 1.14 or later
- OTP 25 or later
- Kreuzberg 4.3.6

## Installation

```bash
cd tests/test_apps/elixir
mix deps.get
```

## Running Tests

```bash
# Run all tests
mix test

# Run with verbose output
mix test --trace

# Run specific test file
mix test test/smoke_test.exs

# Run tests excluding OCR (if OCR not available)
mix test --exclude ocr
```

## Test Structure

```text
test/
├── test_helper.exs          # Test configuration
├── support/
│   └── test_helpers.ex      # Helper functions
├── smoke_test.exs           # Basic smoke tests
└── extraction_test.exs      # Comprehensive extraction tests
```

## Test Documents

Test documents are symlinked from the main test_documents directory:

```text
test_documents -> ../../kreuzberg/test_documents
```

## Coverage

The test suite covers:

- ✓ PDF extraction
- ✓ Office documents (DOCX, XLSX)
- ✓ Plain text
- ✓ Image OCR (with `@tag :ocr`)
- ✓ Async operations
- ✓ Batch processing
- ✓ Chunking configuration
- ✓ Metadata extraction
- ✓ Table extraction
- ✓ Cache operations
- ✓ Error handling

## CI Integration

This test app is executed in the CI pipeline via:

```bash
task elixir:test:app
```

## License

MIT

# Kreuzberg WASM Test Suite

Comprehensive test suite for Kreuzberg WASM bindings (v4.3.6). This test suite validates all major functionality of the `@kreuzberg/wasm` package with 79 test cases covering synchronous and asynchronous extraction, batch operations, configuration handling, and error scenarios.

## Overview

The WASM test suite provides comprehensive coverage of:

- **Type Verification** (8 tests) - Verify all exported types are accessible
- **Synchronous File Extraction** (7 tests) - PDF, DOCX, XLSX, images, ODT, Markdown
- **Asynchronous File Extraction** (7 tests) - Same file types with async API
- **Byte Extraction** (4 tests) - Sync/async from in-memory byte arrays
- **Batch Extraction APIs** (6 tests) - `batchExtractBytes`, `batchExtractBytesSync`, `batchExtractFiles`
- **MIME Type Detection** (7 tests) - From bytes and file paths
- **Configuration Handling** (8 tests) - ExtractionConfig creation and usage
- **Result Structure Validation** (6 tests) - Verify extraction results
- **Error Handling** (5 tests) - Graceful error scenarios
- **Adapter Functions** (5 tests) - Helper utilities
- **Concurrent Operations** (3 tests) - Parallel and sequential operations
- **Large Document Handling** (4 tests) - Multi-page, complex files
- **Content Quality Checks** (5 tests) - Special characters, RTL text

## Installation

### From Local Development Build (Current Setup)

```bash
cd test_apps/wasm
pnpm install
# WASM binaries are linked from kreuzberg/crates/kreuzberg-wasm/dist/pkg/
```

### From Published npm Package

When using the published `@kreuzberg/wasm@4.3.6` from npm:

```bash
cd test_apps/wasm
# Update package.json to reference npm version:
# "@kreuzberg/wasm": "4.3.6"
pnpm install
```

Note: The published npm package requires proper distribution of WASM binaries in the package tarball.

## Running Tests

```bash
# Run all tests once
npm test

# Watch mode for development
npm run test:watch

# UI mode for interactive testing
npm run test:ui

# Generate coverage report
npm run test:coverage
```

## Test Structure

Tests are organized by functionality:

```text
tests/
└── wasm-extraction.spec.ts    # Main test suite with 40+ tests
```

### Test Categories

#### 1. Type Verification (8 tests)

Validates that all TypeScript types are properly exported and accessible:

- ExtractionConfig, ExtractionResult
- OcrConfig, ChunkingConfig, ImageExtractionConfig
- PdfConfig, Table, Metadata

#### 2. File Extraction - Synchronous (7 tests)

Tests synchronous extraction from various file types:

- PDF files (`pdfs/fake_memo.pdf`)
- DOCX documents (`documents/lorem_ipsum.docx`)
- XLSX spreadsheets (`spreadsheets/test_01.xlsx`)
- ODT documents (`documents/simple.odt`)
- PNG/JPG images
- Markdown files

#### 3. File Extraction - Asynchronous (7 tests)

Tests asynchronous extraction with same file types:

- Validates async/await pattern works correctly
- Tests concurrent extraction capabilities
- Verifies promise-based API

#### 4. Byte Extraction (4 tests)

Tests extraction from in-memory byte arrays:

- PDF bytes sync and async
- DOCX bytes consistency
- Empty byte array handling
- Large byte array support

#### 5. Batch Operations (6 tests)

Tests batch extraction APIs:

- `batchExtractBytes()` - async batch from byte arrays
- `batchExtractBytesSync()` - sync batch from byte arrays
- `batchExtractFiles()` - async batch from File objects
- Mixed document types in single batch
- Order preservation
- Configuration application to batches

#### 6. MIME Type Detection (7 tests)

Validates correct MIME type identification:

- PDF, DOCX, XLSX, PNG, JPG, ODT
- Custom MIME types

#### 7. Configuration Handling (8 tests)

Tests ExtractionConfig usage:

- Null configuration handling
- OCR configuration
- Chunking configuration
- Image extraction settings
- PDF extraction settings
- Combined configurations
- `configToJS()` utility

#### 8. Result Structure (6 tests)

Validates extraction result structure:

- Required fields present
- Metadata handling
- Tables extraction
- Chunks generation
- Type consistency sync vs async

#### 9. Error Handling (5 tests)

Tests graceful error scenarios:

- Invalid PDF handling
- Corrupted data
- Error wrapping with context
- Empty file handling
- Large file support

#### 10. Adapter Functions (5 tests)

Tests utility functions:

- `fileToUint8Array()`
- `configToJS()`
- `isValidExtractionResult()`
- `wrapWasmError()`

#### 11. Concurrent Operations (3 tests)

Tests concurrent extraction:

- Parallel extractions with Promise.all()
- Rapid sequential extractions
- Mix of sync and async operations

#### 12. Large Document Handling (4 tests)

Tests complex documents:

- Large PDF files (1MB+)
- Multi-page PDFs
- Complex DOCX with tables
- Multi-sheet XLSX

#### 13. Content Quality (5 tests)

Validates extraction quality:

- Meaningful content extraction
- Structure preservation
- Special character handling
- Right-to-left text
- Content length validation

## Test Documents

Tests use real documents from `test_documents/`:

### PDFs

- `fake_memo.pdf` - Simple searchable PDF
- `multi_page.pdf` - Multi-page document
- `searchable.pdf` - Text-searchable PDF
- `embedded_images_tables.pdf` - Contains images and tables
- `pdfs_with_tables/*.pdf` - Various table formats
- Large academic PDFs for size testing
- Special text PDFs (RTL, non-ASCII)

### Office Documents

- `documents/lorem_ipsum.docx` - Standard DOCX
- `documents/docx_tables.docx` - DOCX with tables
- `documents/unit_test_*.docx` - Formatted documents
- `spreadsheets/*.xlsx` - Excel files (single/multi-sheet)

### Images

- `images/sample.png` - PNG image
- `images/flower_no_text.jpg` - JPG without text
- `images/ocr_image.jpg` - Image with text
- Table images for layout detection

### Other Formats

- `documents/simple.odt` - ODT document
- `documents/markdown.md` - Markdown file
- `text/*.md` - Various markdown files

## Configuration Examples

### OCR Configuration

```typescript
const config: ExtractionConfig = {
  ocr: {
    backend: 'tesseract',
    language: 'eng'
  }
};
const result = await extractBytes(bytes, 'application/pdf', config);
```

### Chunking Configuration

```typescript
const config: ExtractionConfig = {
  chunking: {
    maxChars: 500,
    chunkOverlap: 50
  }
};
const result = await extractBytes(bytes, 'application/pdf', config);
```

### Image Extraction

```typescript
const config: ExtractionConfig = {
  images: {
    extractImages: true,
    targetDpi: 150
  }
};
const result = await extractBytes(bytes, 'application/pdf', config);
```

### Combined Configuration

```typescript
const config: ExtractionConfig = {
  ocr: { backend: 'tesseract', language: 'eng' },
  chunking: { maxChars: 1000, chunkOverlap: 100 },
  images: { extractImages: true, targetDpi: 200 }
};
const result = await extractBytes(bytes, 'application/pdf', config);
```

## API Coverage

### Core Functions

- `initWasm()` - Initialize WASM module
- `isInitialized()` - Check initialization status
- `getVersion()` - Get WASM module version

### Extraction Functions

- `extractBytes()` - Async extraction from bytes
- `extractBytesSync()` - Sync extraction from bytes
- `extractFile()` - Async extraction from file path
- `extractFromFile()` - Async from File/Blob objects

### Batch Operations

- `batchExtractBytes()` - Async batch from byte arrays
- `batchExtractBytesSync()` - Sync batch from byte arrays
- `batchExtractFiles()` - Async batch from File objects

### OCR

- `enableOcr()` - Enable OCR with tesseract-wasm

### Utilities

- `fileToUint8Array()` - Convert File to Uint8Array
- `configToJS()` - Convert config object
- `isValidExtractionResult()` - Validate result structure
- `wrapWasmError()` - Wrap errors with context

## Supported File Types

- **PDF** - `application/pdf`
- **DOCX** - `application/vnd.openxmlformats-officedocument.wordprocessingml.document`
- **XLSX** - `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`
- **ODT** - `application/vnd.oasis.opendocument.text`
- **Images** - `image/png`, `image/jpeg`, `image/webp`
- **Markdown** - `text/markdown`
- **Plain Text** - `text/plain`

## TypeScript Configuration

Tests use strictest TypeScript settings:

- `strict: true` - All strict type checks
- `noUncheckedIndexedAccess: true` - Index safety
- `exactOptionalPropertyTypes: true` - Exact optional properties
- `noUnusedLocals: true` - Unused variable detection
- `noImplicitAny: true` - No implicit any types

## Coverage Goals

- Core extraction API: 100%
- Type definitions: 100%
- Error handling: 95%+
- Edge cases: 90%+

## Performance Characteristics

Tests validate:

- No memory leaks on repeated operations
- Efficient batch processing
- Concurrent operation support
- Handling of large documents (1MB+)

## CI/CD Integration

For CI/CD pipelines:

```bash
# Install dependencies
npm install

# Run tests with coverage
npm run test:coverage

# Or with specific options
vitest run --reporter=junit --outputFile=results.xml
```

## Troubleshooting

### WASM Module Not Loading

Ensure `@kreuzberg/wasm` is properly installed:

```bash
npm install @kreuzberg/wasm
```

### Test Timeouts

Some large document tests may need extended timeouts. Vitest is configured with:

- `testTimeout: 60000` - 60 second test timeout
- `hookTimeout: 60000` - 60 second hook timeout

### Missing Test Documents

Verify test_documents directory exists at `../../../../test_documents/` relative to test file.

## Test Metrics

Total Tests: 79

- Type Verification: 10
- Synchronous Extraction: 7
- Asynchronous Extraction: 7
- Byte Extraction: 4
- Batch Operations: 6
- MIME Detection: 7
- Configuration: 8
- Result Validation: 6
- Error Handling: 5
- Adapter Functions: 5
- Concurrent Operations: 3
- Large Document Handling: 4
- Content Quality: 5
- Memory/Performance: 2

## Notes

- Tests require the full test_documents directory from the kreuzberg repository
- WASM module must be properly built before running tests
- All tests are isolated and can run in parallel
- Both Node.js and browser-like environments are supported

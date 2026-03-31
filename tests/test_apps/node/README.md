# Kreuzberg TypeScript/Node.js Test App

Comprehensive test application for `@kreuzberg/node@4.3.6` published on npm.

## Quick Start

```bash
cd test_apps/node
pnpm install
pnpm test              # Run all tests
pnpm typecheck         # TypeScript type checking
```

## Overview

This test app validates the **entire public API surface** of the TypeScript/Node.js bindings against the published npm package. It includes two test suites:

1. **main.test.ts** (77 tests) - Comprehensive API coverage, documents what works/breaks
2. **api-corrections.test.ts** (31 tests) - Corrected tests using actual API signatures

## Test Results

- **Total Tests**: 108 tests
- **Passing**: 69 (64%)
- **Failing**: 39 (36%)

Key findings documented in [TEST_REPORT.md](./TEST_REPORT.md)

## Requirements

- Node.js 22+
- pnpm package manager
- @kreuzberg/node 4.3.6 (from npm)

## Running Tests

```bash
# Install dependencies from npm
pnpm install

# Run all tests
pnpm test

# Run specific test suite
pnpm test tests/main.test.ts
pnpm test tests/api-corrections.test.ts

# Watch mode
pnpm test:watch

# Type checking only
pnpm typecheck
```

## API Coverage

### Fully Working (100%)

- ✓ Version info (`__version__`)
- ✓ MIME type detection (`detectMimeType()`, `validateMimeType()`, `getExtensionsForMime()`)
- ✓ Error handling (error codes, classification, all error types)
- ✓ TypeScript type system (all types importable)
- ✓ Embeddings API (`listEmbeddingPresets()`, `getEmbeddingPreset()`)
- ✓ Plugin listing (`listPostProcessors()`, `listValidators()`, `listOcrBackends()`)

### Partially Working (40-80%)

- ⚠ Extraction functions (basic usage works, configuration complex)
- ⚠ Batch operations (works with proper mimeTypes array)
- ⚠ Plugin registry (listing works, registration broken)

### Known Issues

- ✗ ExtractionConfig builder pattern missing (use `.fromFile()` or `.discover()`)
- ✗ Plugin registration fails (validator, OCR backend registration broken)
- ✗ `detectMimeTypeFromPath()` not exported from main module

See [TEST_REPORT.md](./TEST_REPORT.md) for detailed findings.

## Project Structure

```text
.
├── package.json                  # npm dependencies
├── tsconfig.json               # Strict TypeScript config
├── vitest.config.ts            # Test runner config
├── README.md                   # This file
├── TEST_REPORT.md              # Detailed test findings
└── tests/
    ├── main.test.ts            # 77 comprehensive tests
    ├── api-corrections.test.ts # 31 corrected API tests
    └── extraction.test.mjs      # Legacy test file
```

## Key Findings

### Working API Example

```typescript
import { extractFile, batchExtractFiles } from '@kreuzberg/node';

// Single file extraction
const result = await extractFile('document.pdf');
console.log(result.content);

// Batch processing
const results = await batchExtractFiles(['doc1.pdf', 'doc2.pdf']);

// MIME detection
import { detectMimeType, validateMimeType } from '@kreuzberg/node';
const mime = detectMimeType(Buffer.from('%PDF-1.4'));
```

### Configuration

```typescript
// Load from file (recommended)
import { ExtractionConfig } from '@kreuzberg/node';
const config = ExtractionConfig.fromFile('kreuzberg.toml');
const result = await extractFile('document.pdf', null, config);

// Discover config automatically
const config = ExtractionConfig.discover();

// Or use inline config object
await extractFile('document.pdf', null, {
  ocr: { backend: 'tesseract', language: 'eng' },
  chunking: { maxChars: 2048 }
});
```

## Test Files Included

### main.test.ts

Comprehensive test suite with 77 tests covering:

- Version information
- MIME type detection and validation
- Extraction configuration
- Single document extraction (sync/async)
- Batch document extraction
- Plugin registry operations
- Error handling and classification
- Type system validation
- Plugin protocol validation

**Purpose**: Document actual API capabilities and limitations

### api-corrections.test.ts

Corrected test suite with 31 tests using actual API signatures:

- Verified extraction function signatures
- File-based configuration loading
- Batch processing with mimeTypes arrays
- Plugin listing operations
- Error handling validation
- Result structure verification

**Purpose**: Validate working parts of the API with correct usage patterns

## Version Info

- **Package**: @kreuzberg/node@4.3.6
- **Node.js**: 22+
- **TypeScript**: 5.9.3
- **Test Framework**: vitest 4.0.16
- **Created**: 2025-12-22

## Documentation

- [TEST_REPORT.md](./TEST_REPORT.md) - Detailed test results and findings
- [CLAUDE.md](../../CLAUDE.md) - Development guidelines for this project

## License

MIT - Same as Kreuzberg project

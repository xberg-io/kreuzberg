# PHP E2E Tests

Auto-generated end-to-end tests for the Kreuzberg PHP bindings.

## Overview

This directory contains PHPUnit-based E2E tests that are automatically generated from fixture definitions in the `fixtures/` directory. The tests verify document extraction functionality and plugin APIs across various document formats.

## Directory Structure

```text
e2e/php/
├── bootstrap.php          # PHPUnit bootstrap file
├── phpunit.xml           # PHPUnit configuration
├── tests/                # Generated test files
│   ├── Helpers.php       # Test helper functions
│   ├── EmailTest.php     # Email fixture tests
│   ├── HtmlTest.php      # HTML fixture tests
│   ├── PdfTest.php       # PDF fixture tests
│   └── ...               # Other category tests
└── README.md            # This file
```

## Generated Files

All PHP test files in the `tests/` directory are auto-generated. **Do not edit these files directly.**

To regenerate the tests:

```bash
cargo run -p kreuzberg-e2e-generator -- generate --lang php
```

## Running Tests

### Prerequisites

1. Build and install the Kreuzberg PHP extension:

   ```bash
   cd packages/php
   composer install
   composer build
   ```

2. Ensure the `test_documents/` directory exists in the workspace root

### Run All Tests

```bash
cd e2e/php
phpunit
```

### Run Specific Test Suite

```bash
phpunit tests/PdfTest.php
```

### Run Specific Test

```bash
phpunit --filter test_pdf_simple_extraction
```

## Test Structure

### Document Extraction Tests

Each test follows this pattern:

```php
public function test_pdf_simple_extraction(): void
{
    $documentPath = Helpers::resolveDocument('pdf/simple.pdf');
    if (!file_exists($documentPath)) {
        $this->markTestSkipped('Skipping: missing document');
    }

    $config = Helpers::buildConfig(null);
    $kreuzberg = new Kreuzberg($config);
    $result = $kreuzberg->extractFile($documentPath);

    Helpers::assertExpectedMime($result, ['application/pdf']);
    Helpers::assertMinContentLength($result, 100);
}
```

### Helper Functions

The `Helpers` class provides assertion methods:

- `assertExpectedMime()` - Verify MIME type
- `assertMinContentLength()` - Check minimum content length
- `assertMaxContentLength()` - Check maximum content length
- `assertContentContainsAny()` - Verify content contains any of the snippets
- `assertContentContainsAll()` - Verify content contains all snippets
- `assertTableCount()` - Verify table extraction count
- `assertDetectedLanguages()` - Verify language detection
- `assertMetadataExpectation()` - Verify metadata values

## Configuration

The `phpunit.xml` file configures:

- Test suite discovery in `tests/` directory
- Bootstrap file for environment setup
- PHP ini settings (memory limit, error reporting)
- Output formatting (colors, verbosity)

## Extension Verification

The bootstrap file automatically verifies:

1. Kreuzberg PHP extension is loaded
2. Composer dependencies are installed
3. Test documents directory exists

If any check fails, tests will abort with a descriptive error message.

## Adding New Tests

To add new E2E tests:

1. Create a fixture JSON file in `fixtures/` (e.g., `fixtures/pdf/my_test.json`)
2. Regenerate tests: `cargo run -p kreuzberg-e2e-generator -- generate --lang php`
3. Run tests to verify: `phpunit`

Example fixture structure:

```json
{
  "id": "pdf_my_test",
  "description": "Test description",
  "document": {
    "path": "pdf/my_document.pdf"
  },
  "assertions": {
    "expected_mime": ["application/pdf"],
    "min_content_length": 100,
    "content_contains_any": ["expected text"]
  }
}
```

## Troubleshooting

### Extension Not Loaded

```text
Error: Kreuzberg PHP extension is not loaded.
```

**Solution**: Build and install the extension:

```bash
cd packages/php
composer build
```

### Missing Test Documents

```text
Error: test_documents directory not found
```

**Solution**: Ensure you're running from the workspace root or the test_documents directory exists.

### Memory Limit Errors

If tests fail with memory errors, increase the limit in `phpunit.xml`:

```xml
<php>
    <ini name="memory_limit" value="1G"/>
</php>
```

## CI Integration

These tests can be integrated into CI pipelines. Ensure:

1. Kreuzberg extension is built and installed
2. PHPUnit is available (via Composer)
3. Test documents are accessible
4. Sufficient memory is allocated

Example CI command:

```bash
cd e2e/php && phpunit --testdox
```

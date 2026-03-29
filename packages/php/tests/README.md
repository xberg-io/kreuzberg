# Kreuzberg PHP Tests

Comprehensive, behavior-driven test suite for the Kreuzberg PHP bindings.

## Test Philosophy

All tests follow **Behavior-Driven Development (BDD)** principles:

✅ **DO TEST:**

- User-facing behavior and functionality
- Observable outcomes users can see
- Real-world document processing scenarios
- Error handling and exceptions
- Configuration effects on results

❌ **DO NOT TEST:**

- Implementation details (getters, setters)
- Constants or type definitions
- Framework features (array_map, etc.)
- Trivial code paths

## Directory Structure

```text
tests/
├── bootstrap.php                      # PHPUnit bootstrap file
├── Unit/                              # Unit tests (48 tests)
│   ├── ConfigTest.php                # Configuration behavior
│   ├── DocumentExtractionTest.php    # Core extraction behavior
│   ├── BatchProcessingTest.php       # Batch operations (11 tests)
│   └── ErrorHandlingTest.php         # Exception behavior (21 tests)
└── Integration/                      # Integration tests (63 tests)
    ├── ExtensionTest.php             # PHP extension verification
    ├── DocumentFormatsTest.php       # Multi-format extraction (14 tests)
    ├── OcrExtractionTest.php         # OCR functionality (11 tests)
    ├── TableExtractionTest.php       # Table extraction (12 tests)
    └── ChunkingAndEmbeddingsTest.php # Chunking & embeddings (11 tests)
```

**E2E Tests:**

```text
e2e/php/tests/
├── CompleteWorkflowTest.php          # End-to-end workflows
└── Helpers.php                       # E2E test utilities
```

## Running Tests

### Prerequisites

1. **Install dependencies:**

   ```bash
   cd packages/php
   composer install
   ```

2. **Build the Kreuzberg extension:**

   ```bash
   composer build
   ```

3. **Verify extension is loaded:**

   ```bash
   php -m | grep kreuzberg
   ```

### All Tests

```bash
# Unit + Integration tests
composer test
# or
./vendor/bin/phpunit

# E2E tests
cd ../../e2e/php
phpunit
```

### By Test Group

```bash
# Unit tests only (48 tests)
./vendor/bin/phpunit --group unit

# Integration tests only (63 tests)
./vendor/bin/phpunit --group integration

# Batch processing tests (11 tests)
./vendor/bin/phpunit --group batch

# OCR tests (11 tests)
./vendor/bin/phpunit --group ocr

# Table extraction tests (12 tests)
./vendor/bin/phpunit --group tables

# Chunking & embeddings tests (11 tests)
./vendor/bin/phpunit --group chunking

# Error handling tests (21 tests)
./vendor/bin/phpunit --group errors

# Document formats tests (14 tests)
./vendor/bin/phpunit --group formats
```

### By Test File

```bash
# Document extraction
./vendor/bin/phpunit tests/Unit/DocumentExtractionTest.php

# Batch processing
./vendor/bin/phpunit tests/Unit/BatchProcessingTest.php

# Error handling
./vendor/bin/phpunit tests/Unit/ErrorHandlingTest.php

# OCR functionality
./vendor/bin/phpunit tests/Integration/OcrExtractionTest.php

# Table extraction
./vendor/bin/phpunit tests/Integration/TableExtractionTest.php

# Chunking and embeddings
./vendor/bin/phpunit tests/Integration/ChunkingAndEmbeddingsTest.php
```

### Specific Test

```bash
./vendor/bin/phpunit --filter test_name

# Example:
./vendor/bin/phpunit --filter it_extracts_text_from_simple_pdf
```

### Advanced Options

**Verbose output:**

```bash
./vendor/bin/phpunit --testdox
```

**Stop on first failure:**

```bash
./vendor/bin/phpunit --stop-on-failure
```

**List all available groups:**

```bash
./vendor/bin/phpunit --list-groups
```

**Coverage report (requires Xdebug or PCOV):**

```bash
./vendor/bin/phpunit --coverage-html coverage/
```

## Test Coverage

### Unit Tests (48 tests)

**DocumentExtractionTest.php** - Core extraction behavior

- Extracts text from PDFs and Office documents
- Handles file and byte extraction
- Auto-detects MIME types
- Applies configurations correctly
- Throws exceptions for invalid input
- Supports multiple document formats

**BatchProcessingTest.php** (11 tests)

- Processes multiple files in parallel
- Handles byte arrays
- Maintains result order
- Applies configs to all items
- Validates input arrays
- Handles empty batches

**ErrorHandlingTest.php** (21 tests)

- Nonexistent files
- Invalid paths and null bytes
- Corrupted data
- Mismatched MIME types
- Unreadable files
- Batch operation errors
- Exception factory methods
- Meaningful error messages

**ConfigTest.php** - Configuration objects

- Default and custom values
- Nested configurations
- Array serialization
- Readonly class validation
- Null value filtering

### Integration Tests (63 tests)

**DocumentFormatsTest.php** (14 tests)

- PDF, DOCX, ODT, EPUB, Markdown
- Metadata extraction
- MIME type detection
- Unicode content handling
- Complex documents with formulas/lists

**OcrExtractionTest.php** (11 tests)

- Tesseract OCR backend
- Multiple language support
- Page segmentation modes
- Table detection in images
- Batch OCR processing
- UTF-8 output validation

**TableExtractionTest.php** (12 tests)

- PDF and ODT table extraction
- Markdown representation
- Page number tracking
- Multiple tables per document
- Enable/disable configuration
- Table structure validation

**ChunkingAndEmbeddingsTest.php** (11 tests)

- Chunk size and overlap configuration
- Sentence boundary respect
- Embedding generation
- Normalization settings
- Chunk metadata
- Batch chunking
- UTF-8 validation

**ExtensionTest.php** - PHP extension verification

- Extension loading
- Function availability
- MIME type detection
- Version matching
- Config serialization

### E2E Tests

**CompleteWorkflowTest.php** - Complete workflows

- Full PDF extraction pipeline
- OCR workflow on images
- Batch processing mixed documents
- Embeddings workflow
- Table markdown conversion
- Multipage processing
- Bytes extraction workflow
- Office document processing
- API consistency across formats

**Helpers.php** - E2E utilities

- Document path resolution
- Config building
- MIME type assertions
- Content length assertions
- Table count assertions
- Language detection assertions
- Metadata assertions

## Extension Requirements

The integration tests require the Kreuzberg PHP extension to be loaded. Tests will gracefully skip if the extension is not available.

Check if the extension is loaded:

```bash
php -m | grep kreuzberg
```

Get extension version:

```bash
php -r "echo phpversion('kreuzberg');"
```

## PHPUnit 11 Features Used

- **Attributes** - Using PHP 8+ attributes for test metadata:
  - `#[Test]` - Mark test methods
  - `#[CoversClass]` - Specify covered classes
  - `#[Group]` - Organize tests by group
  - `#[RequiresPhpExtension]` - Skip tests if extension missing

- **Strict Mode** - Tests run with strict assertions and error checking
- **Type Safety** - Full type hints on all test methods
- **Readonly Classes** - Tests verify readonly class behavior

## Writing New Tests

### Unit Test Template

```php
<?php

declare(strict_types=1);

namespace Kreuzberg\Tests\Unit;

use PHPUnit\Framework\Attributes\CoversClass;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\TestCase;

#[CoversClass(YourClass::class)]
final class YourClassTest extends TestCase
{
    #[Test]
    public function it_does_something(): void
    {
        // Arrange
        $object = new YourClass();

        // Act
        $result = $object->doSomething();

        // Assert
        $this->assertTrue($result);
    }
}
```

### Integration Test Template

```php
<?php

declare(strict_types=1);

namespace Kreuzberg\Tests\Integration;

use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\Attributes\RequiresPhpExtension;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\TestCase;

#[Group('integration')]
#[RequiresPhpExtension('kreuzberg')]
final class YourIntegrationTest extends TestCase
{
    protected function setUp(): void
    {
        if (!extension_loaded('kreuzberg')) {
            $this->markTestSkipped('Kreuzberg extension not loaded');
        }
    }

    #[Test]
    public function it_integrates_with_extension(): void
    {
        // Your test code
    }
}
```

## Writing Tests

### Test Naming Convention

Use descriptive, behavior-focused names:

```php
✅ GOOD:
public function it_extracts_text_from_pdf_document(): void
public function it_throws_exception_when_file_not_found(): void
public function it_applies_config_to_batch_operations(): void

❌ BAD:
public function testExtractFile(): void
public function testConstructor(): void
public function testGetVersion(): void
```

### Test Structure

```php
#[Test]
public function it_describes_observable_behavior(): void
{
    // Given - Setup
    $config = new ExtractionConfig(extractTables: true);
    $kreuzberg = new Kreuzberg($config);

    // When - Action
    $result = $kreuzberg->extractFile($filePath);

    // Then - Assertion (behavior, not implementation)
    $this->assertNotEmpty($result->content);
    $this->assertIsArray($result->tables);
}
```

### Using Groups

Organize tests with PHPUnit groups:

```php
#[Group('unit')]
#[Group('batch')]
#[RequiresPhpExtension('kreuzberg')]
final class BatchProcessingTest extends TestCase
{
    // Test methods...
}
```

### Best Practices

1. **Test behavior, not implementation** - Focus on what users observe
2. **Use descriptive assertions** - Make test intent clear
3. **Skip gracefully** - Use `markTestSkipped()` when prerequisites are missing
4. **Test real scenarios** - Use actual documents and realistic configs
5. **Handle missing files** - Check file existence before testing
6. **Validate data types** - Ensure results have correct types
7. **Check edge cases** - Test boundaries and error conditions

## Troubleshooting

### Extension Not Loaded

**Error:** `Kreuzberg extension is not loaded`

**Solution:**

```bash
cd packages/php
composer build
php -m | grep kreuzberg
```

### Test Documents Missing

**Error:** `Test file not found`

**Solution:** Ensure `test_documents/` directory exists in workspace root.

### Memory Limit

**Error:** `Allowed memory size exhausted`

**Solution:**

```bash
php -d memory_limit=1G ./vendor/bin/phpunit
```

### Skipped Tests

Many tests skip if:

- Kreuzberg extension is not loaded
- Test files are missing
- Optional features unavailable

View skip reasons:

```bash
./vendor/bin/phpunit --verbose
```

## Continuous Integration

Tests run automatically on:

- Pull requests
- Commits to main branch
- Release builds

Example CI commands:

```bash
# Unit tests (fast, no extension required for some)
./vendor/bin/phpunit --group unit

# Integration tests (requires extension)
./vendor/bin/phpunit --group integration

# E2E tests
cd ../../e2e/php && phpunit
```

## Summary

**Total Tests:** 111+ comprehensive tests

**Test Breakdown:**

- Unit Tests: 48 tests
- Integration Tests: 63 tests
- E2E Tests: Multiple workflow tests

**Coverage Areas:**

- Document extraction (PDF, DOCX, ODT, Markdown, EPUB, etc.)
- Batch processing
- OCR with Tesseract
- Table extraction and markdown conversion
- Text chunking and embeddings
- Error handling and exceptions
- Complete end-to-end workflows

**Quick Start:**

```bash
# Install and build
composer install
composer build

# Run all tests
./vendor/bin/phpunit

# Run specific group
./vendor/bin/phpunit --group ocr
```

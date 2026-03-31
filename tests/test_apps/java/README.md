# Kreuzberg Java Comprehensive Test Suite

This directory contains a comprehensive JUnit 5 test suite for Kreuzberg Java FFM API bindings (version 4.3.6).

## Test Suite Overview

The test suite includes **45+ tests** organized into 9 test categories:

### 1. Type Verification Tests (10 tests)

Verifies all exported types and classes are accessible:

- ExtractionResult class
- ExtractionConfig class and builder
- All config sub-types (OcrConfig, ChunkingConfig, LanguageDetectionConfig, PdfConfig, ImageExtractionConfig)
- Table, Chunk, ExtractedImage classes
- ErrorCode enum
- KreuzbergException
- Main Kreuzberg API

### 2. Synchronous File Extraction Tests (8 tests)

Tests synchronous file extraction for multiple formats:

- PDF files (gmft/tiny.pdf)
- DOCX files (documents/lorem_ipsum.docx)
- XLSX files (spreadsheets/test_01.xlsx)
- PNG images (images/sample.png)
- JPG images (images/example.jpg)
- ODT files (documents/simple.odt)
- Markdown files (documents/markdown.md)
- String path extraction

### 3. Asynchronous File Extraction Tests (7 tests)

Tests async/concurrent file extraction:

- Async PDF extraction
- Async DOCX extraction
- Async XLSX extraction
- Async image extraction
- Async ODT extraction
- Async Markdown extraction
- Multiple concurrent async operations

### 4. Byte Extraction Tests (7 tests)

Tests in-memory byte array extraction:

- Synchronous byte extraction (PDF, DOCX with config)
- Asynchronous byte extraction
- Image byte extraction
- Null/empty data validation
- MIME type validation

### 5. Batch Extraction Tests (8 tests)

Tests batch extraction operations:

- Batch extract multiple files (sync and async)
- Batch extract with configuration
- Batch extract bytes (sync and async)
- Empty list handling
- List size validation

### 6. MIME Type Detection Tests (6 tests)

Tests MIME type detection from various sources:

- Detection from PDF bytes
- Detection from DOCX bytes
- Detection from XLSX bytes
- Detection from image bytes
- Detection from file path strings
- Multiple format detection

### 7. Configuration Handling Tests (9 tests)

Tests configuration creation and usage:

- Default extraction config
- Config with cache disabled
- Config with quality processing enabled
- Config with forced OCR
- Config with chunking settings
- Config with language detection
- Config with PDF options
- Config with image extraction settings
- Custom config in extractions
- Config toMap() method

### 8. Result Structure Validation Tests (9 tests)

Verifies extraction results have expected fields:

- Content field
- MIME type field
- Success flag
- Metadata map
- Tables list
- Chunks list
- Images list
- Detected languages list
- Optional fields (language, date, subject)
- String representation

### 9. Error Handling Tests (8 tests)

Tests error conditions:

- Non-existent file throws IOException
- Null path handling
- Invalid MIME types
- Null list validation
- KreuzbergException error information
- Exception with cause handling
- Async exception handling
- File type coverage validation

### 10. Concurrent Operation Tests (2 tests)

Tests concurrent extraction behavior:

- Multiple synchronous extractions
- Batch operations with multiple files

## Requirements

- **Java**: 21 or later (main Kreuzberg package requires Java 25, this test app uses Java 21)
- **Maven**: 3.9.0 or later
- **Test Documents**: Located at `../../../../test_documents/` (relative to test file location)
- **Dependencies**:
  - JUnit Jupiter 5.11.4
  - AssertJ 3.26.3
  - Kreuzberg 4.3.6

## Building and Running Tests

### Prerequisites

First, build the main Kreuzberg library (Java 25 required):

```bash
cd packages/java
mvn clean install -DskipTests
```

### Run All Tests

```bash
cd test_apps/java
mvn clean test
```

### Run Specific Test Class

```bash
mvn clean test -Dtest=ExtractionTests
```

### Run Specific Test Category

```bash
# Type verification tests
mvn clean test -Dtest=ExtractionTests#TypeVerificationTests

# Sync file extraction tests
mvn clean test -Dtest=ExtractionTests#SyncFileExtractionTests

# Async file extraction tests
mvn clean test -Dtest=ExtractionTests#AsyncFileExtractionTests

# Byte extraction tests
mvn clean test -Dtest=ExtractionTests#ByteExtractionTests

# Batch extraction tests
mvn clean test -Dtest=ExtractionTests#BatchExtractionTests

# MIME type detection tests
mvn clean test -Dtest=ExtractionTests#MimeTypeDetectionTests

# Configuration handling tests
mvn clean test -Dtest=ExtractionTests#ConfigurationTests

# Result structure validation tests
mvn clean test -Dtest=ExtractionTests#ResultValidationTests

# Error handling tests
mvn clean test -Dtest=ExtractionTests#ErrorHandlingTests

# File type coverage tests
mvn clean test -Dtest=ExtractionTests#FileTypeCoverageTests

# Concurrent operation tests
mvn clean test -Dtest=ExtractionTests#ConcurrentOperationTests
```

### Run Tests with Verbose Output

```bash
mvn clean test -X
```

## Test Structure

The test suite follows these conventions:

- **Package**: `com.kreuzberg.test_app`
- **Main Test Class**: `ExtractionTests.java`
- **Nested Test Classes**: One for each test category using `@Nested` annotation
- **Assertions**: AssertJ fluent assertions for readable test code
- **Test Methods**: Clear, descriptive names with `@DisplayName` annotations
- **Test Documents**: Located at `../../../../test_documents/` relative to the test class

## Supported File Types

The test suite verifies extraction for these document types:

1. **PDF** - gmft/tiny.pdf
2. **Word Documents** - documents/lorem_ipsum.docx
3. **Spreadsheets** - spreadsheets/test_01.xlsx
4. **Images** - images/sample.png, images/example.jpg
5. **OpenDocument** - documents/simple.odt
6. **Markdown** - documents/markdown.md

## Key Testing Features

- **Type Safety**: Uses Java 21 advanced features (sealed classes, records, pattern matching)
- **Comprehensive Coverage**: Tests sync, async, bytes, and batch operations
- **Configuration Testing**: Validates all config builder options
- **Error Validation**: Tests error conditions and exception handling
- **Concurrent Operations**: Tests async execution and concurrent batch processing
- **Result Validation**: Verifies all expected fields in extraction results
- **MIME Detection**: Tests MIME type detection from multiple sources

## Test Execution Guarantees

All tests:

- Verify their inputs exist (test documents)
- Assert on return values using fluent assertions
- Handle both success and failure scenarios
- Clean up resources properly
- Are independent and can run in any order

## Configuration Options Tested

The test suite validates configuration for:

- Cache usage (enabled/disabled)
- Quality processing (enabled/disabled)
- OCR forcing (enabled/disabled)
- Chunking settings (chunk size, overlap)
- Language detection
- PDF processing options
- Image extraction
- Image preprocessing
- Post-processing
- Token reduction
- HTML options
- Keyword extraction
- Page tracking

## Results

Test results are reported in standard JUnit format by Maven Surefire.

Success output example:

```text
[INFO] Tests run: 45, Failures: 0, Errors: 0, Skipped: 0
```

## Files

- `ExtractionTests.java` - Main test class with all 45+ test methods
- `pom.xml` - Maven project configuration
- `README.md` - This documentation

## Notes

- The main Kreuzberg library requires Java 25. This test app compiles with Java 21 but will execute with Java 25 at runtime if the Kreuzberg JAR was built with Java 25.
- All test documents are included in the repository at `test_documents/`
- Tests use FFM API (Foreign Function & Memory) for native library access
- Native libraries must be available in `../../target/release/` at test runtime

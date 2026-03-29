# PHP Examples for Kreuzberg

This directory contains comprehensive examples demonstrating the PHP bindings for Kreuzberg, a high-performance document intelligence library.

## Overview

These examples showcase both Object-Oriented Programming (OOP) and procedural API usage patterns, following PHP 8.2+ best practices with strict typing and comprehensive error handling.

## Examples

### 1. Basic Usage (`basic_usage.php`)

**Purpose**: Introduction to Kreuzberg's core functionality

**Topics Covered**:

- Simple file extraction (OOP and procedural APIs)
- Extraction with configuration
- Extracting from bytes
- MIME type detection
- Accessing metadata and content
- Working with tables
- Basic error handling

**Run**:

```bash
php basic_usage.php
```

---

### 2. Advanced Configuration (`advanced_config.php`)

**Purpose**: Demonstrate complex configurations with all available options

**Topics Covered**:

- PDF-specific configuration (page ranges, image extraction)
- Advanced OCR configuration with Tesseract options
- Image extraction with size filters and OCR
- Page extraction with custom markers
- Language detection configuration
- Keyword extraction
- Comprehensive configuration combining all options
- Dynamic configuration based on file type

**Run**:

```bash
php advanced_config.php
```

---

### 3. Batch Processing (`batch_processing.php`)

**Purpose**: Efficient processing of multiple documents in parallel

**Topics Covered**:

- Batch file extraction (OOP and procedural)
- Batch extraction from bytes
- Processing multiple file formats
- Processing directory contents
- Error handling in batch operations
- Performance optimization and benchmarking
- Result filtering and statistics
- Mixed file type processing

**Run**:

```bash
php batch_processing.php
```

---

### 4. OCR Example (`ocr_example.php`)

**Purpose**: Optical Character Recognition from scanned documents and images

**Topics Covered**:

- Basic OCR extraction with Tesseract
- Multi-language OCR (English, German, etc.)
- Page Segmentation Mode (PSM) configuration
- OCR Engine Mode (OEM) selection
- Table detection in scanned documents
- OCR fallback for hybrid documents
- Character whitelisting and blacklisting
- OCR from image files (PNG, JPG, TIFF)
- Image preprocessing for better accuracy
- Performance comparison of OCR configurations

**Run**:

```bash
php ocr_example.php
```

---

### 5. Chunking Example (`chunking_example.php`)

**Purpose**: Text chunking for RAG (Retrieval-Augmented Generation) applications

**Topics Covered**:

- Basic text chunking with overlap
- Small chunks for fine-grained retrieval
- Large chunks for context
- Sentence-aware chunking
- Paragraph-aware chunking
- Chunk overlap analysis
- Chunking with page information
- Iterating and processing chunks
- Chunk statistics and filtering
- Comparing chunking strategies

**Run**:

```bash
php chunking_example.php
```

---

### 6. Embeddings Example (`embeddings_example.php`)

**Purpose**: Generate embeddings for semantic search and RAG

**Topics Covered**:

- Basic embedding generation
- Different embedding models (MiniLM, MPNet)
- Normalized vs non-normalized embeddings
- Embedding batch size configuration
- Cosine similarity calculation
- Semantic search example
- Embedding statistics
- Building a vector database
- Complete RAG pipeline example
- Best practices for production use

**Run**:

```bash
php embeddings_example.php
```

---

### 7. Error Handling (`error_handling.php`)

**Purpose**: Comprehensive error handling strategies

**Topics Covered**:

- Basic error handling with try-catch
- File not found errors
- Invalid file format handling
- Corrupted file handling
- Invalid MIME type errors
- OCR configuration errors
- Batch processing error handling
- Graceful degradation strategies
- Error recovery with retries
- Logging errors with context
- Custom error messages
- Validating files before extraction
- Error handling best practices

**Run**:

```bash
php error_handling.php
```

---

### 8. Metadata Extraction (`metadata_extraction.php`)

**Purpose**: Extract and work with detailed document metadata

**Topics Covered**:

- Basic metadata extraction
- Detailed metadata fields (title, author, dates, etc.)
- Custom metadata fields
- Table metadata and analysis
- Image metadata (dimensions, format, colorspace)
- Page-level metadata
- Language detection metadata
- Keyword extraction metadata
- Comprehensive metadata reports
- Metadata comparison across file types
- Exporting metadata to JSON

**Run**:

```bash
php metadata_extraction.php
```

---

### 9. Async Extraction (`async_extraction.php`)

**Purpose**: Non-blocking document extraction using the async API

**Topics Covered**:

- Single file async extraction (OOP and procedural APIs)
- Non-blocking polling with `isReady()` and `tryGetResult()`
- Blocking wait with timeout via `wait()`
- Batch async extraction with `batchExtractFilesAsync()`
- Concurrent async extractions
- Procedural batch async API

**Run**:

```bash
php async_extraction.php
```

---

## Requirements

- PHP 8.2 or higher
- Kreuzberg PHP extension installed
- Composer dependencies installed

## Installation

1. Install the Kreuzberg PHP extension:

```bash
# Follow installation instructions in the main README
```

2. Install Composer dependencies:

```bash
cd packages/php
composer install
```

3. Run examples:

```bash
cd examples/php
php basic_usage.php
```

## API Patterns

All examples demonstrate both API styles:

### Object-Oriented API

```php
use Kreuzberg\Kreuzberg;
use Kreuzberg\Config\ExtractionConfig;

$kreuzberg = new Kreuzberg($config);
$result = $kreuzberg->extractFile('document.pdf');
```

### Procedural API

```php
use function Kreuzberg\extract_file;

$result = extract_file('document.pdf', config: $config);
```

## Configuration

Examples use various configuration objects:

- `ExtractionConfig` - Main extraction configuration
- `OcrConfig` - OCR settings (Tesseract backend)
- `TesseractConfig` - Tesseract-specific options
- `ChunkingConfig` - Text chunking settings
- `EmbeddingConfig` - Embedding generation settings
- `ImageExtractionConfig` - Image extraction settings
- `PageConfig` - Page extraction settings
- `LanguageDetectionConfig` - Language detection settings
- `KeywordConfig` - Keyword extraction settings
- `PdfConfig` - PDF-specific settings

## Error Handling

All examples include proper error handling:

```php
try {
    $kreuzberg = new Kreuzberg();
    $result = $kreuzberg->extractFile('document.pdf');
    // Process result...
} catch (KreuzbergException $e) {
    echo "Error: {$e->getMessage()}\n";
}
```

## Common Use Cases

### Document Processing Pipeline

1. Extract text: `basic_usage.php`
2. Handle errors: `error_handling.php`
3. Process in batch: `batch_processing.php`

### RAG Application

1. Chunk text: `chunking_example.php`
2. Generate embeddings: `embeddings_example.php`
3. Store in vector database
4. Implement semantic search

### OCR Workflow

1. Configure OCR: `ocr_example.php`
2. Process scanned documents
3. Extract tables and images: `metadata_extraction.php`

### Async Processing

1. Async extraction: `async_extraction.php`
2. Batch async: `async_extraction.php` (Example 5)
3. Concurrent extractions for maximum throughput

### Metadata Extraction

1. Extract metadata: `metadata_extraction.php`
2. Advanced configuration: `advanced_config.php`
3. Export to JSON or database

## Sample Documents

Examples reference sample documents from `../sample-documents/` directory. Create test files or modify paths to use your own documents.

## Best Practices

1. **Always use strict typing**: `declare(strict_types=1);`
2. **Error handling**: Wrap all extraction calls in try-catch blocks
3. **Configuration**: Create config objects for complex scenarios
4. **Validation**: Validate file paths and formats before processing
5. **Performance**: Use batch processing for multiple files
6. **Memory**: Process large files in chunks when possible
7. **Security**: Validate and sanitize file paths from user input

## Additional Resources

- [Main Documentation](../../README.md)
- [API Reference](../../packages/php/README.md)
- [Python Examples](../python/)
- [TypeScript Examples](../typescript/)

## Support

For issues or questions:

- GitHub Issues: [kreuzberg-dev/kreuzberg](https://github.com/kreuzberg-dev/kreuzberg)
- Documentation: [kreuzberg.dev](https://kreuzberg.dev)

## License

See the main repository LICENSE file.

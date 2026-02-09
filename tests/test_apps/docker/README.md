# Kreuzberg Docker Image Test Suite

A comprehensive test application for validating Kreuzberg Docker images (core and full variants). This test suite ensures both images are functioning correctly and contain the expected components.

## Overview

This directory contains:

- **docker-compose.yml**: Orchestrates both core and full image containers with selective profiles
- **tests/**: Individual test scripts for various functionality areas
- **fixtures/**: Real test documents (symlinked from kreuzberg/test_documents/)

### Test Coverage

The test suite validates:

- Container startup and health checks
- API server endpoints (/health, /extract, /detect)
- CLI command availability (extract, serve, mcp)
- MCP (Model Context Protocol) support over stdio
- OCR functionality (Tesseract)
- Embeddings generation (ONNX Runtime)
- Core-specific features (Tesseract OCR)
- Full-specific features (legacy Office format support via native OLE/CFB)
- Volume mounting and cache directories
- Multi-format document extraction (PDF, DOCX, XLSX, ODT, Markdown, images)
- Force OCR on text documents and image-only PDFs

## Prerequisites

- Docker Engine (v20.10+)
- Docker Compose v2 or later (or `docker compose` plugin)
- Bash shell
- `curl` command-line tool (for API testing)

## Quick Start

### 1. Start the Containers

Docker Compose profiles allow selective testing. Choose one:

```bash
cd <kreuzberg-root>/test_apps/docker/

# Start ONLY core image tests
docker-compose --profile core up -d

# Start ONLY full image tests
docker-compose --profile full up -d

# Start BOTH core and full images (default)
docker-compose --profile all up -d
```

Wait for containers to reach "healthy" status:

```bash
docker-compose ps
```

With profiles:
- **core**: Only `kreuzberg-core-test` should show `(healthy)`
- **full**: Only `kreuzberg-full-test` should show `(healthy)`
- **all**: Both should show `(healthy)`

### 2. Run the Test Suite

Run all tests:

```bash
bash tests/test-all.sh
```

Run specific test suites:

```bash
# Health checks only
bash tests/test-health.sh

# CLI commands only
bash tests/test-cli.sh

# API endpoints and multi-format extraction
bash tests/test-api.sh

# MCP protocol only
bash tests/test-mcp.sh

# OCR-specific tests (force OCR, images, scanned PDFs)
bash tests/test-ocr.sh

# Embeddings generation tests (ONNX Runtime)
bash tests/test-embeddings.sh

# Core image specific tests
bash tests/test-core.sh

# Full image specific tests (legacy Office formats)
bash tests/test-full.sh
```

### 3. Stop the Containers

```bash
docker-compose down

# To also remove volumes
docker-compose down -v
```

## Test Scripts

### `test-all.sh`

Master test runner that executes all test suites in sequence and provides a comprehensive summary.

**Usage:**
```bash
bash tests/test-all.sh
```

**Output:** Color-coded test results with pass/fail summary

### `test-health.sh`

Validates container lifecycle and basic health checks.

**Tests:**
- Container existence
- Container running status
- Health check passing
- Version command execution

### `test-cli.sh`

Validates command-line interface functionality.

**Tests:**
- Version command
- Help/usage commands
- Subcommand availability (extract, serve, mcp)
- Command argument parsing

### `test-api.sh`

Validates HTTP API server endpoints.

**Tests:**
- Health endpoint (GET /health)
- Extract endpoint (POST /extract)
- API response format
- Text extraction via API
- Status codes

**Core Container API:** http://localhost:8000
**Full Container API:** http://localhost:8001

### `test-mcp.sh`

Validates MCP (Model Context Protocol) support.

**Tests:**
- MCP command availability
- MCP stdio mode
- MCP help/usage
- Server mode availability

### `test-ocr.sh`

Validates OCR (Optical Character Recognition) capabilities.

**Tests:**
- OCR on JPEG images
- OCR on PNG images
- Force OCR on image-only PDFs
- Force OCR on regular PDFs
- Large PDF processing with OCR
- Normal PDF extraction without OCR
- OCR image extraction from various formats

**Features:**
- Tests default OCR behavior
- Tests force OCR with `force_ocr` flag
- Tests multiple image formats (JPG, PNG)
- Tests OCR on PDFs (both text and scanned documents)

### `test-embeddings.sh`

Validates ONNX Runtime and embeddings generation.

**Tests:**
- Embeddings generation for text files
- Embeddings for PDF documents
- Embeddings for DOCX documents
- Embeddings for XLSX spreadsheets
- Cache directory writability (for embeddings cache)
- Embeddings endpoint availability

**Features:**
- Tests `generate_embeddings` flag
- Tests `embedding_model` parameter
- Validates ONNX Runtime availability
- Checks embeddings cache directory

### `test-core.sh`

Validates core image specific features.

**Tests:**
- Tesseract OCR is available
- Text extraction works
- PDF processing capability
- Markdown extraction capability
- ODT document extraction
- DOCX extraction (native)
- Image OCR capability
- ONNX Runtime is available
- Embeddings generation
- Cache directory writability
- Core API endpoints (health, extract)
- Tesseract data files available
- Image size efficiency

**API Tests (via http://localhost:8000):**
- PDF extraction
- Text file extraction
- Markdown extraction
- ODT extraction
- Image OCR
- Embeddings generation
- DOCX extraction

### `test-full.sh`

Validates full image specific features.

**Tests:**
- Tesseract OCR is available
- Office document extraction capability
- DOCX/XLSX mime type detection
- Runtime dependencies installed
- Cache directory writability
- Legacy .doc file extraction (Word 97-2003, native OLE/CFB)
- Modern .docx file extraction
- DOCX with tables extraction
- XLSX spreadsheet extraction
- ONNX Runtime availability in full image
- Image file OCR
- ODT document processing
- Full API health checks
- Disk space usage

**API Tests (via http://localhost:8001):**
- Legacy .doc extraction
- DOCX extraction
- DOCX with tables extraction
- XLSX extraction
- Image OCR
- ODT extraction
- Embeddings generation
- Full document format support

## Fixtures

Real test documents (symlinked from `kreuzberg/test_documents/`):

### Text & Markup
- **sample.txt** - Plain text file for basic extraction testing
- **extraction_test.md** - Markdown document with formatting

### PDFs
- **tiny.pdf** - Small PDF with tables (text-based)
- **medium.pdf** - Medium-sized PDF with multiple tables
- **large.pdf** - Large PDF with complex content (text + images)
- **image_only_german_pdf.pdf** - Scanned PDF (image-only, requires OCR)

### Microsoft Office (Modern)
- **lorem_ipsum.docx** - Standard DOCX document
- **docx_tables.docx** - DOCX document with tables
- **word_sample.docx** - DOCX sample document
- **stanley_cups.xlsx** - XLSX spreadsheet with data

### Office Documents (Legacy)
- **unit_test_lists.doc** - Legacy Word 97-2003 document (.doc format)

### OpenDocument
- **simple.odt** - OpenDocument Text format

### Images
- **example.jpg** - JPEG image for OCR testing
- **sample.png** - PNG image for OCR testing
- **ocr_image.jpg** - Image with text for OCR testing

Fixtures are mounted as read-only volumes at `/fixtures` in containers. These are symlinks to ensure we use real, diverse test documents.

## API Examples

### Health Check

```bash
curl http://localhost:8000/health    # Core
curl http://localhost:8001/health    # Full
```

### Extract Text from File

```bash
curl -X POST http://localhost:8000/extract \
  -H "Content-Type: application/json" \
  -d '{"path": "/fixtures/sample.txt"}'
```

### Extract PDF with Table Detection

```bash
curl -X POST http://localhost:8000/extract \
  -H "Content-Type: application/json" \
  -d '{"path": "/fixtures/tiny.pdf"}'
```

### Force OCR on Scanned PDF

```bash
curl -X POST http://localhost:8000/extract \
  -H "Content-Type: application/json" \
  -d '{"path": "/fixtures/image_only_german_pdf.pdf", "force_ocr": true}'
```

### Extract Image with OCR

```bash
curl -X POST http://localhost:8000/extract \
  -H "Content-Type: application/json" \
  -d '{"path": "/fixtures/ocr_image.jpg"}'
```

### Generate Embeddings for Document

```bash
curl -X POST http://localhost:8000/extract \
  -H "Content-Type: application/json" \
  -d '{"path": "/fixtures/sample.txt", "generate_embeddings": true}'
```

### Extract from DOCX (Full Image Only)

```bash
curl -X POST http://localhost:8001/extract \
  -H "Content-Type: application/json" \
  -d '{"path": "/fixtures/lorem_ipsum.docx"}'
```

### Extract from Legacy .doc File (Full Image Only)

```bash
curl -X POST http://localhost:8001/extract \
  -H "Content-Type: application/json" \
  -d '{"path": "/fixtures/unit_test_lists.doc"}'
```

### Extract with Custom Configuration

```bash
curl -X POST http://localhost:8000/extract \
  -H "Content-Type: application/json" \
  -d '{
    "path": "/fixtures/tiny.pdf",
    "use_cache": true,
    "enable_quality_processing": true
  }'
```

## Troubleshooting

### Containers Won't Start

Check logs:
```bash
docker-compose logs kreuzberg-core-test
docker-compose logs kreuzberg-full-test
```

Ensure sufficient disk space and that port 8000/8001 are available.

### Health Checks Failing

The health check runs `kreuzberg --version`. If it fails:

```bash
# Check container logs
docker-compose logs kreuzberg-core-test

# Test manually
docker exec kreuzberg-core-test kreuzberg --version
```

### API Not Responding

Verify the server is running:

```bash
docker exec kreuzberg-core-test ps aux | grep kreuzberg
```

Check if the API is listening:

```bash
docker exec kreuzberg-core-test netstat -tlnp | grep 8000
```

### OCR Tests Failing

Tesseract data files should be available at:
```bash
docker exec kreuzberg-core-test ls /usr/share/tesseract-ocr/5/tessdata/
```

## Image Tags

The docker-compose configuration uses:
- `kreuzberg/kreuzberg:4.0.0-rc.14` for both core and full

To use different tags, edit `docker-compose.yml`:

```yaml
services:
  kreuzberg-core:
    image: kreuzberg/kreuzberg:4.0.0-rc.14  # Change here

  kreuzberg-full:
    image: kreuzberg/kreuzberg:4.0.0-rc.14  # Change here
```

## Test Output

Tests use color coding:

- **GREEN** `[PASS]` - Test passed
- **RED** `[FAIL]` - Test failed
- **YELLOW** `[SKIP]` - Test skipped (fixture not found, etc.)
- **BLUE** `[INFO]` - Informational message

Example output:
```
[INFO] Test 1: Core container exists
[PASS] Core container exists
[PASS] Core container is running
[PASS] Core container is healthy
[INFO] Test 3: Version command works
[PASS] Core version command returns kreuzberg

========================================
Test Summary
========================================
Total:   8
Passed:  8
Failed:  0
Skipped: 0
========================================
All tests passed!
```

## CI/CD Integration

To run tests in CI pipelines:

```bash
#!/bin/bash
set -e

cd /path/to/docker

# Start containers
docker-compose up -d

# Wait for health
sleep 10

# Run tests
bash tests/test-all.sh
EXIT_CODE=$?

# Cleanup
docker-compose down -v

exit $EXIT_CODE
```

## Performance Notes

- **First run**: ~30-60 seconds (pulling image if needed)
- **Startup**: ~5-10 seconds per container
- **Health check**: ~5 seconds
- **Full test suite**: ~2-3 minutes

## Related Documentation

- [Kreuzberg Main README](../../README.md)
- [Docker Images Overview](../../docker/README.md)
- [API Documentation](../../docs/api/)
- [CLI Documentation](../../docs/cli/)

## License

Same as Kreuzberg main project.

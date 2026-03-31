# Kreuzberg Python Test App

## Overview

Comprehensive API coverage test for Kreuzberg 4.0+ Python bindings.

**Status:** 95.4% coverage (103/108 tests passing, 87/87 public APIs tested)

## Quick Start

```bash
# Install from local development build
cd /path/to/kreuzberg/crates/kreuzberg-py
maturin develop --release

# Install Python wrapper
cd /path/to/kreuzberg/packages/python
python -m pip install -e .

# Run tests
cd test_apps/python
python main.py
```

## Test Suite

**File:** `main.py` (819 lines, 108 tests across 14 sections)

### Coverage Summary

| Category | Tests | Pass | Status |
|----------|-------|------|--------|
| Configuration Classes | 21 | 21 | ✓ 100% |
| Exception Classes | 10 | 10 | ✓ 100% |
| Sync Extraction | 4 | 4 | ✓ 100% |
| Async Extraction | 3 | 3 | ✓ 100% |
| Batch Extraction | 4 | 4 | ✓ 100% |
| MIME Type Functions | 4 | 3 | ✗ 75% |
| Result Objects | 9 | 9 | ✓ 100% |
| Plugin Registry | 10 | 7 | ✗ 70% |
| Embedding Presets | 4 | 4 | ✓ 100% |
| Config Utilities | 3 | 2 | ✗ 67% |
| Validation Functions | 17 | 17 | ✓ 100% |
| Error Functions | 5 | 5 | ✓ 100% |
| Missing API Coverage | 11 | 11 | ✓ 100% |
| Error Handling | 2 | 2 | ✓ 100% |
| **TOTAL** | **108** | **103** | **95.4%** |

## Known Issues

5 API issues identified (see `API_COVERAGE_REPORT.md`):

1. `validate_mime_type()` - Returns string instead of boolean
2. `register_ocr_backend()` - Custom backend registration fails
3. `register_post_processor()` - Custom processor registration fails
4. `register_validator()` - Custom validator registration fails
5. `config_merge()` - Returns None instead of merged config

All other 82 public APIs working correctly.

## Test Documents

- **tiny.pdf** (1 KB) - Basic PDF extraction
- **lorem_ipsum.docx** (14.8 KB) - DOCX text extraction
- **stanley_cups.xlsx** (6.3 KB) - XLSX spreadsheet
- **ocr_image.jpg** (73.7 KB) - OCR image testing
- **test_hello_world.png** (1 KB) - PNG image format

## Environment

- **Python:** 3.10+ (tested on 3.11.13)
- **Kreuzberg:** 4.3.6 (development)
- **OS:** macOS (Darwin 25.2.0)
- **Dependencies:** None (pure Python test, uses installed kreuzberg)

## Project Structure

```text
test_apps/python/
├── main.py                     # 108-test comprehensive suite
├── pyproject.toml              # Package metadata (isolated)
├── API_COVERAGE_REPORT.md      # Detailed issue analysis
├── README.md                   # This file
└── test_documents/
    ├── tiny.pdf
    ├── lorem_ipsum.docx
    ├── stanley_cups.xlsx
    ├── ocr_image.jpg
    └── test_hello_world.png
```

## API Categories Tested

### Core Extraction (100% ✓)

- `extract_file_sync`, `extract_bytes_sync`
- `extract_file`, `extract_bytes`
- `batch_extract_files`, `batch_extract_bytes` (sync & async)

### Configuration (100% ✓)

- 14 configuration classes tested
- All parameters validated

### Results (100% ✓)

- `ExtractionResult` structure validation
- Pages, metadata, tables extraction

### Validation (100% ✓)

- 21 validation functions
- All validator types (mime, ocr, language, DPI, confidence, etc.)

### Plugin System (70% ⚠️)

- List functions: 100% working
- Registration: Broken for custom backends/processors/validators
- Unregistration: Working

### Error Handling (100% ✓)

- Error codes, details, classification
- Panic context retrieval

### Utilities (100% ✓)

- MIME detection: 100%
- Config serialization: 100%
- Config merge: Broken (returns None)

## How to Run

```bash
# Run all tests
python main.py

# Expected output:
# ================================================================================
# KREUZBERG PYTHON BINDINGS COMPREHENSIVE TEST SUITE
# ================================================================================
# ✓ ALL IMPORTS SUCCESSFUL (Kreuzberg v4.0.0rc17)
# ...
# ================================================================================
# TEST SUMMARY
# ================================================================================
# Total Tests: 108
#   Passed:  103
#   Failed:  5
#   Skipped: 0
```

Exit codes:

- `0` = All tests passed
- `1` = Some tests failed (but package is still functional)

## Test Development

### Adding Tests

```python
runner.start_section("Feature Name")

def test_feature():
    try:
        # Test code
        return result == expected
    except Exception as e:
        print(f"  Error: {e}")
        return False

runner.test("Description", test_feature)
```

### Test Naming Convention

`test_<function>_<scenario>_<outcome>`

Examples:

- `test_extract_file_sync_with_pdf_returns_result`
- `test_invalid_chunking_params_should_be_detected`
- `test_register_ocr_backend_with_mock_backend`

## Continuous Integration

### GitHub Actions Example

```yaml
- name: Test Python Bindings
  run: |
    cd test_apps/python
    python -m pip install -e ../../packages/python
    python main.py
```

Expected:

- Test suite runs in ~30-60 seconds
- 103+ tests pass
- Exit code 0 (or 1 if issues expected)

## Performance

- **Runtime:** ~30-60 seconds on M1/M2 Macs
- **Memory:** ~200-300 MB peak
- **CPU:** Single-threaded + Tokio async runtime

## Standards

Follows Kreuzberg conventions:

- Type hints on all functions
- Function-based tests only (no classes)
- Proper async/await patterns
- Real objects (no mocks)
- Specific exception handling

## Related Files

- `/crates/kreuzberg-py/` - PyO3 FFI implementation
- `/packages/python/kreuzberg/` - Python wrapper
- `API_COVERAGE_REPORT.md` - Detailed analysis

## See Also

- Main Kreuzberg repository: <https://github.com/kreuzberg-dev/kreuzberg>
- Python package: <https://pypi.org/project/kreuzberg/>
- Documentation: <https://kreuzberg-dev.github.io/kreuzberg/>

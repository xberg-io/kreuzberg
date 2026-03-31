# Kreuzberg C# Smoke Test Suite

Comprehensive smoke test suite for the Kreuzberg document extraction library C# bindings.

## Overview

This test application validates the Kreuzberg C# library (v4.3.6) by testing document extraction across multiple file formats with and without OCR.

## Prerequisites

- .NET 10.0 SDK or later
- Kreuzberg native library (built from the main project)

## Project Structure

```text
.
├── KreuzbergSmokeTest.csproj    # .NET project file
├── Program.cs                    # Main smoke test runner
├── .gitignore                    # .NET gitignore
├── README.md                     # This file
└── test_documents/               # Test files (PDF, DOCX, XLSX, JPG, PNG)
    ├── tiny.pdf
    ├── lorem_ipsum.docx
    ├── stanley_cups.xlsx
    ├── ocr_image.jpg
    └── test_hello_world.png
```

## Features

- Tests extraction from 5 document types: PDF, DOCX, XLSX, JPG, PNG
- Tests standard extraction (no OCR) for all formats
- Tests forced OCR extraction for PDF and image files
- Validates extracted text length and content
- Detailed error reporting with exception types
- JSON output of all results
- Pass/fail summary with counters
- Uses .NET 10 features (records, pattern matching, file-scoped namespaces)

## Setup

### 1. Ensure .NET 10 SDK is Installed

```bash
dotnet --version  # Should show 10.x.x
```

### 2. Build Kreuzberg Native Library

Since this test app uses a locally-built version of Kreuzberg (not from NuGet), you must first build the Rust FFI library:

```bash
# From the kreuzberg repository root
cd ../../kreuzberg
cargo build --release --package kreuzberg-ffi
```

This creates `target/release/libkreuzberg_ffi.dylib` (macOS) or `target/release/libkreuzberg_ffi.so` (Linux).

### 3. Build Kreuzberg C# Bindings

```bash
cd packages/csharp/Kreuzberg
dotnet build
```

This builds the C# bindings that reference the native library.

### 4. Build Test App

```bash
cd ../../../../test_apps/csharp
dotnet build
```

This builds the test application and links to the Kreuzberg C# project.

## Running Tests

### Option 1: Run with dotnet run (Recommended)

```bash
dotnet run
```

This automatically builds if needed and runs the test suite.

### Option 2: Run Compiled Executable

```bash
dotnet build
./bin/Debug/net10.0/KreuzbergSmokeTest
```

### Option 3: Run Release Build

```bash
dotnet build -c Release
dotnet run -c Release
```

## Expected Output

### Standard Extraction Tests

```text
Starting kreuzberg 4.3.6 test suite
Test documents directory: /path/to/test_documents
--------------------------------------------------------------------------------
TEST  PDF             tiny.pdf                       OK   (text: 1234 chars)
TEST  DOCX            lorem_ipsum.docx               OK   (text: 3483 chars)
TEST  XLSX            stanley_cups.xlsx              OK   (text: 355 chars)
TEST  JPG Image       ocr_image.jpg                  OK   (text: 19 chars)
TEST  PNG Image       test_hello_world.png           OK   (text: 18 chars)
--------------------------------------------------------------------------------
OCR Tests (force_ocr=True)
--------------------------------------------------------------------------------
TEST  PDF with OCR              tiny.pdf                       OK   (text: 1234 chars)
TEST  JPG Image with OCR        ocr_image.jpg                  OK   (text: 19 chars)
--------------------------------------------------------------------------------
Summary
--------------------------------------------------------------------------------
Passed: 7/7
Failed: 0/7

✓ All tests passed!
```

### With Native Library Issues

If the native library isn't found or PDFium isn't bundled:

```text
TEST  PDF             tiny.pdf                       FAIL
      Error: KreuzbergException: Failed to extract file
```

## Test Coverage

The test suite covers:

1. **Standard Extraction** (no OCR):
   - PDF documents
   - Microsoft Word (DOCX)
   - Microsoft Excel (XLSX)
   - JPEG images
   - PNG images

2. **OCR Extraction** (force_ocr=true):
   - PDF documents with forced OCR
   - JPEG images with OCR

3. **Error Handling**:
   - Catches and reports exceptions
   - Displays error type and message
   - Non-zero exit code on failure

## Code Quality

- .NET 10 with latest C# language features
- Nullable reference types enabled
- Immutable records for data structures
- Strong typing with explicit types
- Clean separation of concerns
- File-scoped namespaces

## Troubleshooting

### Native Library Not Found

If you get "DllNotFoundException" or similar:

1. Ensure the Rust FFI library is built:

   ```bash
   cd ../../kreuzberg
   cargo build --release --package kreuzberg-ffi
   ls target/release/libkreuzberg_ffi.*  # Should show the native library
   ```

2. Verify the native library is in the runtime directory or PATH

3. On macOS, you may need to set:

   ```bash
   export DYLD_LIBRARY_PATH=../../kreuzberg/target/release
   ```

4. On Linux, you may need to set:

   ```bash
   export LD_LIBRARY_PATH=../../kreuzberg/target/release
   ```

### Test Documents Not Found

Ensure the `test_documents/` directory exists with all required files:

```bash
ls -l test_documents/
# Should show: lorem_ipsum.docx, ocr_image.jpg, stanley_cups.xlsx, test_hello_world.png, tiny.pdf
```

### .NET SDK Version Issues

Verify .NET 10 is installed:

```bash
dotnet --list-sdks
dotnet --version
```

If not installed, download from: <https://dotnet.microsoft.com/download>

### OCR Extraction Returns 0 Characters

This may indicate PDFium is not bundled correctly in the library. This is a warning, not a failure, as the extraction still succeeds.

### PDF Extraction Fails

This is expected for local builds. PDFium is not bundled in development builds. Options:

1. Wait for NuGet release with bundled native libraries
2. Build/download PDFium separately and set PDFIUM_DYLIB_PATH
3. Skip PDF tests when testing locally

This does not affect DOCX, XLSX, or image extraction, which work correctly.

## Dependencies

- **Kreuzberg** (4.3.6): Core document extraction library (local project reference)
- **System.Text.Json**: JSON serialization (part of .NET framework)

## Exit Code

- `0`: All tests passed
- `1`: Any test failed or test_documents directory not found

## License

This test application is for demonstration purposes only.

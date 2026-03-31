# Kreuzberg CLI Test App

Tests the published `kreuzberg-cli` crate from crates.io via `cargo install`.

## Purpose

Validates that users can install and use the Kreuzberg CLI tool from crates.io:

- Installation via `cargo install kreuzberg-cli`
- CLI extraction commands
- HTTP API server
- MCP server
- All command-line options and flags

## Requirements

- Rust toolchain with cargo
- Internet connection (to install from crates.io)

## Running Tests

### All Tests

```bash
cd tests/test_apps/cli
./tests/test-all.sh
```

### Individual Test Suites

```bash
# Install CLI from crates.io
./tests/install.sh

# Test extraction commands
./tests/test-extract.sh

# Test HTTP API server
./tests/test-serve.sh

# Test MCP server
./tests/test-mcp.sh
```

## Test Coverage

### Installation (`tests/install.sh`)

- Install via `cargo install kreuzberg-cli`
- Verify binary is in PATH
- Check version output

### Extraction (`tests/test-extract.sh`)

- Extract text from PDF
- Extract text from DOCX
- Extract text from XLSX
- Extract with OCR from images
- JSON output format
- Markdown output format
- Error handling (invalid files, missing files)

### HTTP API Server (`tests/test-serve.sh`)

- Start server on custom port
- POST /extract endpoint
- Health check endpoint
- Graceful shutdown

### MCP Server (`tests/test-mcp.sh`)

- Start MCP server
- Tool discovery
- Document extraction via MCP protocol

## Notes

- Tests use the PUBLISHED version from crates.io, not local builds
- Installation is done in a temporary directory to avoid conflicts
- Tests clean up after themselves

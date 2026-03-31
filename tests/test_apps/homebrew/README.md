# Kreuzberg Homebrew Test Suite

Comprehensive test suite for validating Kreuzberg installations via Homebrew on macOS.

## Overview

This test suite verifies that Kreuzberg v4.3.6 (or later) installs correctly via Homebrew and that all major components work as expected:

- **CLI functionality** - Command-line extraction and version checking
- **API server** - HTTP server for document extraction
- **MCP integration** - Model Context Protocol server functionality
- **End-to-end workflows** - Real document processing

## Prerequisites

- **macOS** (10.13 or later)
- **Homebrew** installed (<https://brew.sh>)
- **curl** - For downloading test documents and making HTTP requests
- **jq** (optional) - For parsing JSON responses in tests
- **bash** - Shell scripts use bash syntax

## Installation

1. Clone or download this test suite:

   ```bash
   cd test_apps/homebrew
   ```

2. Make scripts executable:

   ```bash
   chmod +x tests/*.sh
   ```

3. (Optional) Install dependencies via Brewfile:

   ```bash
   brew bundle
   ```

## Usage

### Run All Tests

```bash
./tests/test-all.sh
```

### Run Individual Tests

Install Kreuzberg from Homebrew:

```bash
./tests/install.sh
```

Test CLI functionality:

```bash
./tests/test-cli.sh
```

Test API server:

```bash
./tests/test-api.sh
```

Test MCP server:

```bash
./tests/test-mcp.sh
```

## Test Structure

```text
homebrew/
├── README.md                 # This file
├── Brewfile                  # Optional Homebrew dependencies
├── .gitignore               # Git ignore rules
├── tests/
│   ├── install.sh           # Install Kreuzberg from Homebrew
│   ├── test-cli.sh          # Test CLI commands
│   ├── test-api.sh          # Test HTTP API server
│   ├── test-mcp.sh          # Test MCP server
│   └── test-all.sh          # Run all tests in sequence
└── test_documents/
    └── table.pdf            # Sample test PDF (auto-downloaded)
```

## Test Details

### install.sh

- Checks if Homebrew is installed
- Installs Kreuzberg via `brew install kreuzberg`
- Verifies installation succeeded
- Displays installed version

### test-cli.sh

- Tests `kreuzberg --version`
- Tests `kreuzberg --help`
- Downloads a sample PDF if needed
- Runs extraction: `kreuzberg extract --input test.pdf --output result.json`
- Verifies output file contains valid JSON and extracted content

### test-api.sh

- Starts Kreuzberg API server on `127.0.0.1:8000`
- Waits for server readiness (health check)
- Tests health endpoint: `GET /health`
- Tests extraction endpoint: `POST /extract` with sample PDF
- Verifies JSON response contains extracted text
- Gracefully shuts down the server

### test-mcp.sh

- Starts Kreuzberg MCP server
- Verifies successful startup
- Tests MCP protocol initialization
- Sends test request to MCP
- Validates response format
- Gracefully shuts down the server

### test-all.sh

- Runs all tests in sequence (install → CLI → API → MCP)
- Generates a summary report (`test_report.txt`)
- Reports pass/fail status for each test
- Shows execution time and any failures

## Troubleshooting

### Homebrew Not Found

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### Kreuzberg Installation Fails

- Check Homebrew is up to date: `brew update`
- Check if formula exists: `brew search kreuzberg`
- Review Homebrew logs: `brew log kreuzberg`

### API Server Won't Start

- Check if port 8000 is already in use: `lsof -i :8000`
- Try a different port by editing `test-api.sh`
- Check server logs in `/tmp/kreuzberg_api.log`

### MCP Server Issues

- Check socket permissions: `ls -la /tmp/kreuzberg.sock` (if applicable)
- Review MCP logs in `/tmp/kreuzberg_mcp.log`
- Ensure no other MCP instances are running

### Test Document Download Fails

- Check internet connectivity: `curl -I https://www.w3.org/`
- Manual download: `curl -o test_documents/table.pdf https://www.w3.org/WAI/WCAG21/Techniques/pdf/img/table.pdf`

### Timeout Issues

- Increase timeout values in test scripts (search for `timeout` or `sleep`)
- Ensure system has sufficient CPU/memory available
- Check system load: `uptime`

## Environment Variables

You can customize test behavior:

```bash
# Set custom test document path
export TEST_DOC_PATH="/path/to/custom.pdf"

# Set custom API port
export API_PORT=8080

# Set custom API host
export API_HOST="127.0.0.1"

# Enable verbose output
export VERBOSE=1

# Run all tests
./tests/test-all.sh
```

## Notes

- Tests are designed to be idempotent (safe to run multiple times)
- Each test cleans up its artifacts (temp files, running processes)
- Test scripts use `set -e` to fail fast on errors
- All scripts include comprehensive error messages
- Suitable for CI/CD integration (returns appropriate exit codes)

## Support

For issues with Kreuzberg itself, see: <https://github.com/kreuzberg-dev/kreuzberg>

For Homebrew issues, see: <https://docs.brew.sh>

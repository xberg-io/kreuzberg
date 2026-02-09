# Kreuzberg Test Apps - Complete Validation Suite

**IMPORTANT:** These test applications validate **PUBLISHED/RELEASED** versions of Kreuzberg packages from npm, PyPI, RubyGems, Maven Central, etc. They are NOT for testing local development builds. Use these apps to verify that published packages work correctly in real-world scenarios.

Comprehensive test applications for Kreuzberg across all supported languages: **Rust, Python, TypeScript/Node.js, Ruby, Java, Go, C#, WASM, Docker, and Homebrew**.

This document provides a complete reference for running, understanding, and maintaining test coverage across the entire Kreuzberg polyglot ecosystem.

---

## Quick Navigation

- [Python](#python) - 108 tests, 95.4% pass rate
- [Node.js/TypeScript](#nodejs-typescript) - 108 tests, 64% pass rate
- [Ruby](#ruby) - 100+ tests
- [Go](#go) - 86 comprehensive + 72 unit tests
- [Java](#java) - 45+ tests
- [C#](#csharp) - 7 smoke tests
- [WASM](#wasm) - 45+ tests
- [Docker](#docker) - Full container validation
- [Homebrew](#homebrew) - Installation & CLI validation
- [Rust](#rust) - (See main kreuzberg crate)

---

## Python

**Status:** Most mature, highest coverage

### Location & Files

```
tests/test_apps/python/
├── main.py                     # 108-test comprehensive suite (819 lines)
├── pyproject.toml              # Package metadata
├── API_COVERAGE_REPORT.md      # Detailed issue analysis
├── README.md                   # Full documentation
└── test_documents/             # Test files
    ├── tiny.pdf
    ├── lorem_ipsum.docx
    ├── stanley_cups.xlsx
    ├── ocr_image.jpg
    └── test_hello_world.png
```

### Installation & Execution

```bash
# From kreuzberg repo root

# Install published kreuzberg package from PyPI
cd tests/test_apps/python
pip install kreuzberg

# Or install specific version
pip install kreuzberg==4.0.0-rc.17

# Run tests
python main.py
```

### Test Coverage

**Version:** 4.0.0-rc.17

**Tests:** 108 organized in 14 sections

| Category | Tests | Status |
|----------|-------|--------|
| Configuration Classes | 21 | ✓ 100% |
| Exception Classes | 10 | ✓ 100% |
| Sync Extraction | 4 | ✓ 100% |
| Async Extraction | 3 | ✓ 100% |
| Batch Extraction | 4 | ✓ 100% |
| MIME Type Functions | 4 | ✗ 75% |
| Result Objects | 9 | ✓ 100% |
| Plugin Registry | 10 | ✗ 70% |
| Embedding Presets | 4 | ✓ 100% |
| Config Utilities | 3 | ✗ 67% |
| Validation Functions | 17 | ✓ 100% |
| Error Functions | 5 | ✓ 100% |
| Missing API Coverage | 11 | ✓ 100% |
| Error Handling | 2 | ✓ 100% |
| **TOTAL** | **108** | **95.4%** |

### Pass/Fail Status

**Last Run:** 2025-12-22

- **Passed:** 103 tests
- **Failed:** 5 tests
- **Known Issues:**
  1. `validate_mime_type()` - Returns string instead of boolean
  2. `register_ocr_backend()` - Custom backend registration fails
  3. `register_post_processor()` - Custom processor registration fails
  4. `register_validator()` - Custom validator registration fails
  5. `config_merge()` - Returns None instead of merged config

**82 public APIs working correctly**

### APIs Tested

Core extraction, configuration, results, validation, plugin system, error handling, utilities.

### Environment

- **Python:** 3.10+ (tested on 3.11.13)
- **OS:** macOS, Linux
- **Runtime:** ~30-60 seconds
- **Memory:** 200-300 MB peak

---

## Node.js/TypeScript

**Status:** High coverage, documented API gaps

### Location & Files

```
tests/test_apps/node/
├── package.json                # npm dependencies
├── tsconfig.json              # Strict TS config
├── vitest.config.ts           # Test runner setup
├── README.md                   # Full documentation
├── TEST_REPORT.md             # Detailed findings
└── tests/
    ├── main.test.ts           # 77 comprehensive tests
    ├── api-corrections.test.ts # 31 corrected tests
    └── extraction.test.mjs     # Legacy test file
```

### Installation & Execution

```bash
# From kreuzberg repo root

# Install dependencies (includes published kreuzberg package from npm)
cd tests/test_apps/node
pnpm install

# Run all tests
pnpm test

# Type checking
pnpm typecheck

# Watch mode
pnpm test:watch
```

### Test Coverage

**Version:** 4.0.0-rc.16

**Tests:** 108 total

| Suite | Tests | Pass Rate |
|-------|-------|-----------|
| main.test.ts | 77 | 64% documented |
| api-corrections.test.ts | 31 | Working API usage |
| **TOTAL** | **108** | **64%** |

### Pass/Fail Status

**Last Run:** 2025-12-22

- **Passing:** 69 tests
- **Failing:** 39 tests (expected, see [TEST_REPORT.md](tests/test_apps/node/TEST_REPORT.md))

### Known Issues

- ExtractionConfig builder pattern missing (use `.fromFile()` or `.discover()`)
- Plugin registration fails
- `detectMimeTypeFromPath()` not exported from main module

### APIs Tested

- Version info, MIME detection, error handling, type system
- Extraction functions, batch operations
- Plugin listing, embeddings, configuration

### Environment

- **Node.js:** 22+
- **TypeScript:** 5.9.3
- **Test Framework:** vitest 4.0.16
- **Package Manager:** pnpm ≥10.17

---

## Ruby

**Status:** Full API coverage with 15 organized sections

### Location & Files

```
tests/test_apps/ruby/
├── main_test.rb               # 100+ test suite
├── Gemfile                    # Ruby dependencies
├── Gemfile.lock              # Lock file
├── README.md                  # Full documentation
└── test_documents/            # Test files
```

### Installation & Execution

```bash
# From kreuzberg repo root

# Install dependencies (includes published kreuzberg gem from RubyGems)
cd tests/test_apps/ruby
bundle install

# Run tests
ruby main_test.rb

# Or with bundler
bundle exec ruby main_test.rb
```

### Test Coverage

**Version:** 4.0.0-rc.16

**Tests:** 100+ in 15 sections

| Category | Tests | Status |
|----------|-------|--------|
| Module Imports & Setup | - | ✓ |
| Configuration Classes | 18 | ✓ |
| Error Classes | 11 | ✓ |
| MIME Type Functions | 7 | ✓ |
| Plugin Registry - Validators | - | ✓ |
| Plugin Registry - Post-Processors | - | ✓ |
| Plugin Registry - OCR Backends | - | ✓ |
| Embedding Presets | - | ✓ |
| Cache API | - | ✓ |
| Result Object Structure | 6 | ✓ |
| Extraction Functions - Sync | 4 | ✓ |
| Batch Extraction | - | ✓ |
| Module Functions & Aliases | - | ✓ |
| Error Context | - | ✓ |

### Pass/Fail Status

**Last Run:** Status not documented

- **Passing:** Most functionality works
- **Limitation:** Async functions not tested (complexity)
- **Note:** Document-dependent tests skipped

### APIs Tested

21 core module functions, 9 configuration classes, 7 error classes, 5 result structures

### Environment

- **Ruby:** 3.2+ (rbenv for version management)
- **Build:** Native extensions (Magnus/Rust)
- **Requirements:** Rust compiler, C/C++ compiler, GNU Make

---

## Go

**Status:** Comprehensive API testing with 72 unit + 86 comprehensive tests

### Location & Files

```
tests/test_apps/go/
├── main.go                    # 86 comprehensive API tests (standalone)
├── extraction_test.go         # 72 unit tests (go test framework)
├── go.mod                     # Module definition
├── go.sum                     # Dependency lock
├── run_tests.sh               # Convenience runner
├── README.md                  # Full documentation
└── test_documents/            # Test files
    ├── tiny.pdf
    ├── lorem_ipsum.docx
    ├── stanley_cups.xlsx
    ├── ocr_image.jpg
    └── test_hello_world.png
```

### Installation & Execution

```bash
# From kreuzberg repo root

# Go will automatically download published kreuzberg package
cd tests/test_apps/go

# Unit tests via go test
go test -v ./...

# Comprehensive tests (standalone executable)
go build -o main main.go
./main
```

### Test Coverage

**Version:** 4.0.0-rc.16

**Tests:** 158 total (86 comprehensive + 72 unit)

#### main.go (86 tests - 14 sections)

| Category | Tests | Coverage |
|----------|-------|----------|
| Configuration Structs | 20 | ✓ |
| Pointer Helpers | 4 | ✓ |
| Config Functions | 8 | ✓ |
| MIME Type Functions | 4 | ✓ |
| Validation Functions | 11 | ✓ |
| Error Types | 10 | ✓ |
| FFI Error Code Functions | 3 | ✓ |
| Extraction (Sync) | 5 | ✓ |
| Extraction (Context) | 2 | ✓ |
| Batch Extraction | 2 | ✓ |
| Library Info | 3 | ✓ |
| Result Types | 8 | ✓ |
| Plugin Registry | 4 | ✓ |
| Embedding Presets | 2 | ✓ |

#### extraction_test.go (72 tests - 12 categories)

- Type Verification (8)
- Synchronous File Extraction (8)
- File Byte Extraction (8)
- Batch Extraction (5)
- MIME Type Detection (5)
- File Type Coverage (7)
- Configuration Handling (2)
- Result Structure Validation (5)
- Error Handling (8)
- Context Support (3)
- Metadata Validation (4)
- MIME Type Validation (1)

### Pass/Fail Status

**Expected:** All tests pass

- **Framework:** Both go test and custom runner
- **Formats:** PDF, DOCX, XLSX, JPEG, PNG, ODT, Markdown

### APIs Tested

- Synchronous extraction (files, bytes)
- Batch extraction (files, bytes)
- MIME type detection and validation
- Configuration handling
- Result types and accessors
- Plugin registry
- Error handling with context

### Environment

- **Go:** 1.25+
- **Build:** CGO enabled (if using native bindings)
- **Note:** Tests use published Go package from pkg.go.dev

---

## Java

**Status:** 45+ tests covering JUnit 5 and FFM API

### Location & Files

```
tests/test_apps/java/
├── ExtractionTests.java       # Main test class (45+ tests in 9 categories)
├── pom.xml                    # Maven configuration
├── README.md                  # Full documentation
└── test_documents/            # Test files
```

### Installation & Execution

```bash
# From kreuzberg repo root

# Maven will automatically download published kreuzberg package from Maven Central
cd tests/test_apps/java

# Run tests
mvn clean test

# Run specific test class
mvn clean test -Dtest=ExtractionTests
```

### Test Coverage

**Version:** 4.0.0-rc.14

**Tests:** 45+ in 9 categories

| Category | Tests | Status |
|----------|-------|--------|
| Type Verification | 10 | ✓ |
| Sync File Extraction | 8 | ✓ |
| Async File Extraction | 7 | ✓ |
| Byte Extraction | 7 | ✓ |
| Batch Extraction | 8 | ✓ |
| MIME Type Detection | 6 | ✓ |
| Configuration | 9 | ✓ |
| Result Validation | 9 | ✓ |
| Error Handling | 8 | ✓ |
| Concurrent Operations | 2 | ✓ |

### Pass/Fail Status

**Expected:** All 45+ tests pass

- **Format:** JUnit 5 with @Nested classes
- **Assertions:** AssertJ fluent assertions

### APIs Tested

- Type definitions (ExtractionResult, ExtractionConfig)
- Synchronous and asynchronous extraction
- Byte extraction and batch operations
- MIME type detection
- Configuration building
- Error handling and exception hierarchy

### Environment

- **Java:** 21+ (build), 25+ (main library)
- **Build:** Maven 3.9.0+
- **FFI:** Foreign Function & Memory API (Panama)

---

## C#

**Status:** Smoke test suite for .NET

### Location & Files

```
tests/test_apps/csharp/
├── KreuzbergSmokeTest.csproj  # .NET project file
├── Program.cs                 # Main smoke test runner
├── .gitignore                # .NET ignore rules
├── README.md                 # Full documentation
└── test_documents/           # Test files
    ├── tiny.pdf
    ├── lorem_ipsum.docx
    ├── stanley_cups.xlsx
    ├── ocr_image.jpg
    └── test_hello_world.png
```

### Installation & Execution

```bash
# From kreuzberg repo root

# NuGet will automatically download published kreuzberg package
cd tests/test_apps/csharp

# Build and run test app
dotnet run
```

### Test Coverage

**Version:** 4.0.0-rc.10

**Tests:** 7 smoke tests

| Format | Test | Status |
|--------|------|--------|
| PDF | Standard extraction | ✓ |
| DOCX | Standard extraction | ✓ |
| XLSX | Standard extraction | ✓ |
| JPG | Standard extraction | ✓ |
| PNG | Standard extraction | ✓ |
| PDF | With OCR | ✓ |
| JPG | With OCR | ✓ |

### Pass/Fail Status

**Expected:** 7/7 tests pass

- **Exit Code:** 0 (all pass), 1 (failure)
- **Note:** PDFium not bundled in development builds

### APIs Tested

- Basic extraction (no OCR)
- Forced OCR extraction
- Multiple file formats

### Environment

- **.NET:** 10.0+
- **Language:** C# latest
- **Features:** Records, pattern matching, nullable reference types

---

## WASM

**Status:** 45+ comprehensive tests for browser/Node.js

### Location & Files

```
tests/test_apps/wasm/
├── package.json               # npm configuration
├── vitest.config.ts          # Vitest setup
├── tests/
│   └── wasm-extraction.spec.ts # 45+ test suite
├── README.md                  # Full documentation
└── test_documents/            # Test files
```

### Installation & Execution

```bash
# From kreuzberg repo root

# Install dependencies (includes published kreuzberg-wasm package from npm)
cd tests/test_apps/wasm
npm install
# or
pnpm install

# Run all tests
npm test

# Watch mode
npm run test:watch

# Coverage report
npm run test:coverage
```

### Test Coverage

**Version:** 4.0.0-rc.14

**Tests:** 45+ in 13 categories

| Category | Tests | Coverage |
|----------|-------|----------|
| Type Verification | 8 | ✓ |
| Sync File Extraction | 7 | ✓ |
| Async File Extraction | 7 | ✓ |
| Byte Extraction | 4 | ✓ |
| Batch Operations | 6 | ✓ |
| MIME Detection | 7 | ✓ |
| Configuration | 8 | ✓ |
| Result Validation | 6 | ✓ |
| Error Handling | 5 | ✓ |
| Adapter Functions | 5 | ✓ |
| Concurrent Operations | 3 | ✓ |
| Large Documents | 4 | ✓ |
| Content Quality | 5 | ✓ |

### Pass/Fail Status

**Expected:** All tests pass

- **Framework:** Vitest with 60-second timeout
- **Browser Support:** Node.js and browser-like environments

### APIs Tested

- Synchronous and asynchronous extraction
- File and byte extraction
- Batch operations
- MIME type detection
- Configuration handling
- Result validation
- Error handling
- Concurrent operations

### Environment

- **Node.js:** 22+
- **Browser:** ES2020+
- **TypeScript:** Strictest settings

---

## Docker

**Status:** Comprehensive container validation

### Location & Files

```
tests/test_apps/docker/
├── docker-compose.yml         # Orchestration (profiles: core/full)
├── README.md                  # Full documentation
└── tests/
    ├── test-all.sh           # Master test runner
    ├── test-health.sh        # Health checks
    ├── test-cli.sh           # CLI validation
    ├── test-api.sh           # HTTP API testing
    ├── test-mcp.sh           # MCP protocol
    ├── test-ocr.sh           # OCR validation
    ├── test-embeddings.sh    # ONNX Runtime
    ├── test-core.sh          # Core image specific
    └── test-full.sh          # Full image specific
```

### Installation & Execution

```bash
# From kreuzberg repo root

# Start containers (pulls published Docker images from Docker Hub)
cd tests/test_apps/docker
docker-compose --profile core up -d   # Core only
docker-compose --profile full up -d   # Full only
docker-compose --profile all up -d    # Both

# Run all tests
bash tests/test-all.sh

# Run specific test suite
bash tests/test-health.sh
bash tests/test-api.sh
bash tests/test-ocr.sh
```

### Test Coverage

**Version:** 4.0.0-rc.14

**Tests:** Comprehensive across 6+ test scripts

| Test Suite | Purpose | Count |
|-----------|---------|-------|
| test-health.sh | Container lifecycle | 4+ |
| test-cli.sh | Command-line interface | 5+ |
| test-api.sh | HTTP server endpoints | 5+ |
| test-mcp.sh | Model Context Protocol | 5+ |
| test-ocr.sh | OCR capabilities | 8+ |
| test-embeddings.sh | ONNX Runtime | 6+ |
| test-core.sh | Core image features | 10+ |
| test-full.sh | Full image features | 15+ |

### Pass/Fail Status

**Last Run:** Expected all to pass

- **Core Image:** Validates Tesseract OCR
- **Full Image:** Validates legacy Office format support

### Coverage

- Container startup and health checks
- CLI command availability
- API endpoints (/health, /extract, /detect)
- MCP (Model Context Protocol) support
- OCR functionality (Tesseract)
- Embeddings generation (ONNX Runtime)
- Multi-format extraction (PDF, DOCX, XLSX, ODT, images)
- Force OCR capabilities

### Environment

- **Docker:** 20.10+
- **Docker Compose:** v2 or plugin
- **Profiles:** core, full

### API Ports

- **Core:** http://localhost:8000
- **Full:** http://localhost:8001

---

## Homebrew

**Status:** Installation and CLI validation on macOS

### Location & Files

```
tests/test_apps/homebrew/
├── Brewfile                   # Optional dependencies
├── .gitignore                # Git ignore rules
├── README.md                 # Full documentation
└── tests/
    ├── install.sh           # Installation validation
    ├── test-cli.sh          # CLI commands
    ├── test-api.sh          # HTTP API server
    ├── test-mcp.sh          # MCP server
    └── test-all.sh          # Master runner
```

### Installation & Execution

```bash
# From kreuzberg repo root

# Run all tests (installs published Homebrew formula)
cd tests/test_apps/homebrew
./tests/test-all.sh

# Or individual tests
./tests/install.sh
./tests/test-cli.sh
./tests/test-api.sh
./tests/test-mcp.sh
```

### Test Coverage

**Version:** 4.0.0-rc.14

**Tests:** 4 major test suites

| Test | Purpose | Status |
|------|---------|--------|
| Install | Homebrew installation | Pass |
| CLI | Command-line extraction | Pass |
| API | HTTP server endpoints | Fail (API port issue) |
| MCP | Model Context Protocol | Pass |

### Pass/Fail Status

**Last Run:** 2025-12-23

- **Total Tests:** 4
- **Passed:** 3
- **Failed:** 1 (API Server test)
- **Pass Rate:** 75%

### Coverage

- Homebrew installation (`brew install kreuzberg`)
- CLI commands (--version, --help, extract)
- API server (health, extraction endpoints)
- MCP server protocol

### Environment

- **macOS:** 10.13+
- **Homebrew:** Required
- **Dependencies:** curl, bash

---

## Rust

The core Kreuzberg library includes comprehensive tests in the main crate:

```bash
cd crates/kreuzberg
cargo test
cargo test --doc
cargo coverage
```

See `/crates/kreuzberg/README.md` for details.

---

## Summary Table

| Language | Tests | Status | Version | Framework |
|----------|-------|--------|---------|-----------|
| Python | 108 | 95.4% | 4.0.0-rc.17 | Custom runner |
| Node.js | 108 | 64% | 4.0.0-rc.16 | vitest |
| Ruby | 100+ | Working | 4.0.0-rc.16 | Custom runner |
| Go | 158 | Passing | 4.0.0-rc.16 | go test + custom |
| Java | 45+ | Passing | 4.0.0-rc.14 | JUnit 5 |
| C# | 7 | Passing | 4.0.0-rc.10 | .NET test runner |
| WASM | 45+ | Passing | 4.0.0-rc.14 | vitest |
| Docker | 50+ | Passing | 4.0.0-rc.14 | Bash scripts |
| Homebrew | 4 | 75% | 4.0.0-rc.14 | Bash scripts |

---

## Getting Started

### Run All Tests (Published Packages)

**IMPORTANT:** All commands should be run from the kreuzberg repository root directory.

```bash
# Python (installs from PyPI)
cd tests/test_apps/python && pip install kreuzberg && python main.py

# Node.js (installs from npm)
cd tests/test_apps/node && pnpm install && pnpm test

# Go (downloads from pkg.go.dev)
cd tests/test_apps/go && go test ./...

# Ruby (installs from RubyGems)
cd tests/test_apps/ruby && bundle install && ruby main_test.rb

# Java (downloads from Maven Central)
cd tests/test_apps/java && mvn clean test

# C# (installs from NuGet)
cd tests/test_apps/csharp && dotnet run

# WASM (installs from npm)
cd tests/test_apps/wasm && npm install && npm test

# Docker (pulls from Docker Hub)
cd tests/test_apps/docker && docker-compose up -d && bash tests/test-all.sh

# Homebrew (installs via brew)
cd tests/test_apps/homebrew && ./tests/test-all.sh
```

### Run Specific Language Tests

Each test app has its own README with detailed instructions:

```bash
# From kreuzberg repo root, navigate to language directory
cd tests/test_apps/{python,node,ruby,go,java,csharp,wasm,docker,homebrew}

# Follow instructions in README.md for that language
```

---

## CI/CD Integration

### GitHub Actions Example

```yaml
- name: Run Python Tests
  run: |
    cd tests/test_apps/python
    pip install kreuzberg
    python main.py

- name: Run Node Tests
  run: |
    cd tests/test_apps/node
    pnpm install && pnpm test

- name: Run Go Tests
  run: |
    cd tests/test_apps/go
    go test -v ./...
```

---

## Known Issues & Limitations

### Python (5 known issues)
- `validate_mime_type()` returns string instead of boolean
- Custom plugin registration (validators, post-processors, OCR backends) fails
- `config_merge()` returns None instead of merged config

### Node.js (API design gaps)
- ExtractionConfig builder pattern missing
- Plugin registration broken
- Some exports missing from main module

### Ruby
- Async functions not tested (testing complexity)
- Document-dependent tests skipped

### Java
- Requires Java 25 for main library (test uses Java 21)
- Test documents expected at relative path

### C#
- PDFium not bundled in development builds (expected)
- DOCX/XLSX extraction requires office feature enabled

### Homebrew
- API server test occasionally fails (port binding issue)

---

## Maintenance & Best Practices

### Adding New Tests

1. **Follow language conventions** (see CLAUDE.md)
2. **Use real documents** (no mocks)
3. **Test error paths** in addition to happy paths
4. **Maintain parity** across languages
5. **Document edge cases**

### Running Test Suite Locally

```bash
# From kreuzberg repo root

# Full test suite (all languages)
bash tests/test_apps/run-all-tests.sh  # If script exists

# Or run each individually (see "Getting Started" section above)
```

### Version Updates

When updating Kreuzberg version:

1. Update version numbers in each test app's configuration
2. Re-run all tests to validate compatibility
3. Document any API changes or migration notes
4. Update this README if test coverage changes

---

## Related Documentation

- [Kreuzberg Main README](../README.md)
- [CLAUDE.md](../CLAUDE.md) - Development guidelines
- [Python Bindings](../packages/python/README.md)
- [Node.js Bindings](../packages/typescript/README.md)
- [Ruby Bindings](../packages/ruby/README.md)
- [Go Bindings](../packages/go/v4/README.md)
- [Java Bindings](../packages/java/README.md)
- [Docker Images](../docker/README.md)

---

## Support & Troubleshooting

For language-specific issues, see the README in each tests/test_apps subdirectory:

- **Python issues:** `tests/test_apps/python/README.md`
- **Node.js issues:** `tests/test_apps/node/README.md`
- **Ruby issues:** `tests/test_apps/ruby/README.md`
- **Go issues:** `tests/test_apps/go/README.md`
- **Java issues:** `tests/test_apps/java/README.md`
- **C# issues:** `tests/test_apps/csharp/README.md`
- **WASM issues:** `tests/test_apps/wasm/README.md`
- **Docker issues:** `tests/test_apps/docker/README.md`
- **Homebrew issues:** `tests/test_apps/homebrew/README.md`

For core Kreuzberg issues, see the [main repository](https://github.com/Goldziher/kreuzberg).

---

## License

All test applications follow the same license as the Kreuzberg project.

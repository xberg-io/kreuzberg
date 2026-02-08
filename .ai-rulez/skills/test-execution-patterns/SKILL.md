---
name: test-execution-patterns
priority: critical
---

# Test Execution Patterns

**Proper test execution using Taskfile with required environment setup**

## Critical Rule: ALWAYS Use Taskfile

**NEVER run tests directly.** The Taskfile ensures correct env vars, dependencies (PDFium, ONNX Runtime), library paths, FFI bindings, and platform-specific quirks.

```bash
# WRONG: go test ./... | pytest tests/ | npm test
# CORRECT: task <language>:test
```

### Top Task Commands

| Command | Purpose |
|---------|---------|
| `task go:test` | Go tests (standard) |
| `task go:test:ci` | Go tests (CI mode with enhanced debugging) |
| `task python:test` | Python tests |
| `task python:test:ci` | Python tests with coverage |
| `task typescript:test` | TypeScript tests |
| `task java:test` | Java tests |
| `task ruby:test` | Ruby tests |
| `task csharp:test` | C# tests |
| `task php:test` | PHP tests |
| `task elixir:test` | Elixir tests |

All language task definitions are in `.task/languages/<lang>.yml` with corresponding scripts in `scripts/<lang>/test.sh`.

## ONNX Runtime Setup

Embedding tests require ONNX Runtime. Set `ORT_LIB_LOCATION` to the library directory:

| Platform | Install | Path |
|----------|---------|------|
| macOS (Apple Silicon) | `brew install onnxruntime` | `/opt/homebrew/opt/onnxruntime/lib` |
| macOS (Intel) | `brew install onnxruntime` | `/usr/local/opt/onnxruntime/lib` |
| Linux | Download from GitHub releases | `/path/to/onnxruntime-linux-x64-<version>/lib` (also add to `LD_LIBRARY_PATH`) |
| Windows | Download binaries | Set `ORT_LIB_LOCATION` and add to `PATH` |
| CI | `.github/actions/setup-onnx-runtime` | Automatic (sets `ORT_LIB_LOCATION`, `ORT_DYLIB_PATH`, library search paths) |

If `ORT_LIB_LOCATION` is not set, tests fail with: `libonnxruntime.dylib: cannot open shared object file`

## Test Execution Workflow

```
task <lang>:test
  -> scripts/<lang>/test.sh
    -> source scripts/lib/common.sh
    -> source scripts/lib/library-paths.sh
      -> setup_go_paths()    # CGO flags, PKG_CONFIG_PATH, platform lib paths
      -> setup_pdfium_paths() # PDFium library location
      -> setup_onnx_paths()  # ONNX Runtime if ORT_LIB_LOCATION set
    -> run tests with fully configured environment
```

### Required Build Order
1. `cargo build --release --package kreuzberg-ffi` (Rust FFI library)
2. PDFium runtime (auto-downloaded by test scripts)
3. Set `ORT_LIB_LOCATION` if testing embeddings
4. `task <lang>:test`

## Key Debugging Patterns

### 1. Check Environment Setup
```bash
export CI=true VERBOSE_MODE=true && task go:test
```
Shows: Go version, working directory, library paths, CGO flags.

### 2. Verify FFI Library Exists
```bash
ls target/release/libkreuzberg_ffi.{dylib,so,dll}
```

### 3. Verify ONNX Runtime
```bash
echo $ORT_LIB_LOCATION && ls $ORT_LIB_LOCATION/libonnxruntime*
```

## Common Failure Patterns

| Error | Cause | Fix |
|-------|-------|-----|
| `Package 'kreuzberg-ffi' not found` | PKG_CONFIG_PATH not set | Use `task go:test` (auto-sets), or `export PKG_CONFIG_PATH=$PWD/crates/kreuzberg-ffi:$PKG_CONFIG_PATH` |
| `libonnxruntime.dylib: cannot open` | ORT_LIB_LOCATION not set | Set `ORT_LIB_LOCATION` per platform table above |
| `undefined reference to kreuzberg_extract_file_sync` | FFI library not built or not in path | `cargo build --release --package kreuzberg-ffi` then verify `DYLD_LIBRARY_PATH` |
| Segmentation fault | Version mismatch, threading issue | Set `RUST_BACKTRACE=full`, run single test with `-run TestName -v`, check `-race` flag |

## Test Isolation Notes

- **Go**: FFI calls serialized via `sync.Mutex` in `packages/go/v4/ffi.go` (PDFium is not thread-safe)
- **Python**: Uses `@pytest.mark.asyncio` and `pytest-asyncio` for async isolation
- **TypeScript**: Jest worker threads for parallel execution with FFI bindings

## CI vs Local

- CI: clean containers, pre-built artifacts, auto-downloads dependencies, env vars via GitHub Actions
- Local: may have stale artifacts, requires manual ONNX install, FFI must be built locally

**Best Practice**: Always test locally with `task` commands BEFORE pushing to CI.

## Summary Checklist

Before running tests:
- [ ] Built FFI library (`cargo build --release --package kreuzberg-ffi`)
- [ ] Set `ORT_LIB_LOCATION` if testing embeddings
- [ ] Using `task <language>:test` (NOT direct test commands)

When tests fail in CI:
- [ ] Reproduce locally with `task <language>:test`
- [ ] Check env vars match CI setup
- [ ] Verify dependencies (ONNX Runtime, PDFium)
- [ ] Run single failing test in isolation

## Related Skills

- **extraction-pipeline-patterns** - Understanding what tests validate
- **ocr-backend-management** - ONNX Runtime and Tesseract setup for OCR tests

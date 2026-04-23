---
name: alef
description: >-
  Generate fully-typed polyglot language bindings for Rust libraries using Alef.
  Use when configuring alef.toml, running alef CLI commands, writing e2e test
  fixtures, debugging binding generation, or setting up CI/CD for multi-language
  Rust libraries. Covers 11 language backends, DTO styles, adapter patterns,
  version sync, and pre-commit hooks.
license: MIT
metadata:
  author: kreuzberg-dev
  version: "1.0"
  repository: https://github.com/kreuzberg-dev/alef
---

# Alef Polyglot Binding Generator

Alef generates fully-typed, lint-clean language bindings for Rust libraries across 11 languages from a single TOML config file. It handles the entire pipeline: API extraction, code generation, type stubs, package scaffolding, build orchestration, version sync, and e2e test generation.

Use this skill when:

- Configuring `alef.toml` for a new or existing Rust library
- Running alef CLI commands (generate, build, test, verify, e2e)
- Writing or debugging e2e test fixtures (JSON fixtures -> multi-language test suites)
- Adding a new language backend to a project
- Setting up CI/CD pipelines for polyglot Rust libraries
- Debugging binding generation issues (stale bindings, type mismatches, missing types)
- Configuring DTO styles, adapter patterns, or custom FFI bridges

## Installation

```bash
# Pre-built binary (fastest)
cargo binstall alef-cli

# From crates.io
cargo install alef-cli

# Via Homebrew
brew install kreuzberg-dev/tap/alef

# From source
git clone https://github.com/kreuzberg-dev/alef.git
cd alef && cargo install --path crates/alef-cli
```

## Quick Start

### 1. Initialize

```bash
cd your-rust-crate
alef init --lang python,node,ruby,go
```

This creates `alef.toml` with your crate's configuration.

### 2. Generate Bindings

```bash
alef generate              # Generate all configured languages
alef generate --lang node  # Generate for specific language
alef generate --clean      # Regenerate everything (ignore cache)
```

### 3. Build

```bash
alef build                 # Build all languages
alef build --lang python   # Build specific (runs maturin)
alef build --release       # Release profile
```

### 4. Test

```bash
alef test                  # Run all language tests
alef test --e2e            # Include e2e tests
alef test --lang python,go # Specific languages
```

### 5. Verify (CI)

```bash
alef verify --exit-code    # Fails if any binding is stale
alef diff                  # Show what would change
```

## Minimal Configuration

```toml
[crate]
name = "my-library"
sources = ["src/lib.rs", "src/types.rs"]

languages = ["python", "node", "go", "java"]

[output]
python = "crates/my-library-py/src/"
node = "crates/my-library-node/src/"
ffi = "crates/my-library-ffi/src/"

[python]
module_name = "_my_library"

[node]
package_name = "@myorg/my-library"

[dto]
python = "dataclass"
node = "interface"
```

## Supported Languages

| Language | Framework | DTO Styles |
|----------|-----------|------------|
| Python | PyO3 | `dataclass`, `typed-dict`, `pydantic`, `msgspec` |
| TypeScript/Node.js | NAPI-RS | `interface`, `zod` |
| WebAssembly | wasm-bindgen | -- |
| Ruby | Magnus | `struct`, `dry-struct`, `data` |
| PHP | ext-php-rs | `readonly-class`, `array` |
| Go | cgo + C FFI | `struct` |
| Java | Panama FFM | `record` |
| C# | P/Invoke | `record` |
| Elixir | Rustler | `struct`, `typed-struct` |
| R | extendr | `list`, `r6` |
| C | cbindgen | -- |

## Common Workflows

### Add a New Language

1. Add the language to `languages` array in `alef.toml`
2. Add output directory in `[output]`
3. Add language-specific config section (e.g., `[python]`)
4. Run `alef generate && alef scaffold`

### Update After Changing Rust API

```bash
alef all                   # Full pipeline: generate + stubs + scaffold + readme
alef verify --exit-code    # Or just check what changed
```

### Run E2E Tests

```bash
alef e2e generate          # Generate test suites from fixtures
alef test --e2e            # Run all tests including e2e
```

### Version Bump

```bash
alef sync-versions --bump patch   # Bump patch and sync everywhere
alef sync-versions                # Just sync current version
```

## Pre-commit Hooks

Alef provides pre-commit hooks for consumer repos:

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/kreuzberg-dev/alef
    rev: v0.3.2
    hooks:
      - id: alef-verify    # Check-only: fails if stale
      # OR
      - id: alef-generate  # Auto-regenerate on change
```

## Caching

Alef uses blake3-based content hashing to skip regeneration when inputs haven't changed. The cache lives in `.alef/` (gitignored).

```bash
alef cache status   # Show cache state
alef cache clear    # Force full regeneration next run
```

## Common Pitfalls

1. **Missing `ffi` language**: Go, Java, and C# require the C FFI layer. Add `ffi` to `languages` or it's implicitly included.
2. **Stale bindings after Rust changes**: Run `alef generate` or `alef all` after modifying your Rust source files.
3. **Wrong DTO style**: Check `[dto]` section. Python `typed-dict` is read-only, `dataclass` is mutable. Choose based on usage.
4. **Types not appearing**: Check `[exclude]`/`[include]` filters. Use `alef extract -o /dev/stdout | jq` to inspect the IR.
5. **Version mismatch**: Always use `alef sync-versions` instead of manually editing package manifests.
6. **Opaque vs transparent types**: Types with private fields or complex generics need `[opaque_types]` config.

## Additional References

- [Configuration Reference](references/configuration.md) -- Complete `alef.toml` documentation
- [CLI Reference](references/cli-reference.md) -- All commands with flags and examples
- [E2E Testing](references/e2e-testing.md) -- Fixture schema, assertion types, generation
- [Language Backends](references/backends.md) -- Per-language details, DTO styles, limitations
- [Adapter Patterns](references/adapters.md) -- Custom FFI bridging patterns
- [Troubleshooting](references/troubleshooting.md) -- Common errors and fixes

# E2E Test Generation System

## Overview

All E2E test files under `e2e/` are **auto-generated** from shared fixture definitions. **Never edit generated files directly** — always modify the generators or fixtures.

## Architecture

```text
fixtures/*.json          → Single source of truth (test definitions)
tools/e2e-generator/     → Rust-based code generator
  src/main.rs            → CLI: `generate --lang <lang>` | `list`
  src/fixtures.rs        → Shared fixture schema & loader
  src/python.rs          → Python (pytest) generator + helper template
  src/typescript.rs      → TypeScript (vitest) generator + helper template
  src/ruby.rs            → Ruby (rspec) generator + helper template
  src/java.rs            → Java (junit) generator + helper template
  src/go.rs              → Go (testing) generator + helper template
  src/csharp.rs          → C# (xunit) generator + helper template
  src/php.rs             → PHP (phpunit) generator + helper template
  src/elixir.rs          → Elixir (exunit) generator + helper template
  src/rust.rs            → Rust (cargo test) generator + helper template
  src/wasm_deno.rs       → Deno WASM generator + helper template
  src/wasm_workers.rs    → Cloudflare Workers WASM generator + helper template
e2e/                     → Generated output (DO NOT EDIT)
```

## Key Principle

Each language generator file (e.g., `python.rs`) contains an **embedded helper template** as a const string. This template is written directly into the generated `helpers.py` / `helpers.ts` / `helpers.rb` etc. files. The test spec files are generated from fixture JSON definitions.

## Workflow

### Adding/Modifying Test Assertions

1. Edit the generator template in `tools/e2e-generator/src/<language>.rs`
2. Regenerate: `task e2e:generate:all` or `bash scripts/task/e2e-generate.sh <lang>`
3. The generated files in `e2e/` will be updated

### Adding New Test Fixtures

1. Create a JSON fixture in `fixtures/<category>/`
2. Regenerate tests for all languages

### Fixture Schema

```json
{
  "id": "fixture_id",
  "category": "category_name",
  "description": "Human description",
  "document": { "path": "relative/path", "media_type": "..." },
  "extraction": { "config": {...}, "method": "sync|async", "input_type": "file|bytes" },
  "assertions": { "expected_mime": [...], "min_content_length": N, ... }
}
```

## Common Pitfalls

### Dict vs Attribute Access

Some bindings (Python PyO3, Node NAPI-RS) return document structures as dicts/plain objects from JSON serialization. Generator helpers must handle both dict-key and attribute access patterns.

### Snake Case vs Camel Case

- Rust serde serializes fields as `snake_case` (e.g., `node_type`)
- NAPI-RS `#[napi(object)]` converts to `camelCase` for struct fields but NOT for serialized JSON values
- Generator helpers should check both `node_type` and `nodeType` patterns

### Config Field Names

Each binding has its own config parsing that maps JSON field names to language-idiomatic names:

- Python: snake_case (`include_document_structure`)
- TypeScript/Node: camelCase mapping via `buildConfig` helper (`includeDocumentStructure`)
- Ruby: symbol keys via `symbolize_keys` (`include_document_structure:`)

When adding new config fields to the Rust core, ensure ALL binding config parsers are updated.

## Generated Files (Never Edit Directly)

- `e2e/python/tests/helpers.py`
- `e2e/python/tests/test_*.py`
- `e2e/typescript/tests/helpers.ts`
- `e2e/typescript/tests/*.spec.ts`
- `e2e/ruby/spec/helpers.rb`
- `e2e/ruby/spec/*_spec.rb`
- `e2e/java/src/test/java/com/kreuzberg/e2e/*.java`
- `e2e/go/*_test.go`
- `e2e/php/tests/*.php`
- `e2e/csharp/*.cs`
- `e2e/rust/tests/*.rs`
- `e2e/wasm-deno/*.ts`
- `e2e/wasm-workers/tests/*.ts`

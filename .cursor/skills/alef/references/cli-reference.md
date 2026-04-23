# Alef CLI Reference

All commands accept a global `--config <path>` flag (default: `alef.toml`) to specify the configuration file.

```text
alef [--config <path>] <command> [options]
```

---

## `alef extract`

Extract API surface from Rust source into an intermediate representation (IR) JSON file.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `-o`, `--output` | path | `.alef/ir.json` | Output IR JSON file path |

```bash
alef extract
alef extract --output api-surface.json
```

The IR contains all extracted types, functions, enums, and errors from the configured Rust source files.

---

## `alef generate`

Generate language bindings from the extracted IR. Also generates type stubs and public API wrappers when enabled in config.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--lang` | string (comma-separated) | all from config | Languages to generate bindings for |
| `--clean` | bool | `false` | Ignore cache and regenerate everything |

```bash
alef generate
alef generate --lang python,node
alef generate --clean
alef generate --lang ruby --clean
```

Generated Rust files are auto-formatted with `cargo fmt`. Caching is based on blake3 content hashing of source files and config -- use `--clean` to force regeneration.

---

## `alef stubs`

Generate type stub files for editor support and static analysis: `.pyi` (Python), `.rbs` (Ruby), `.d.ts` (TypeScript).

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--lang` | string (comma-separated) | all from config | Languages to generate stubs for |

```bash
alef stubs
alef stubs --lang python
```

Caching: skips generation when the IR and config have not changed since the last run.

---

## `alef scaffold`

Generate complete package manifests for each language (`pyproject.toml`, `package.json`, `.gemspec`, `composer.json`, `mix.exs`, `go.mod`, `pom.xml`, `.csproj`, `DESCRIPTION`, `Cargo.toml`).

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--lang` | string (comma-separated) | all from config | Languages to scaffold for |

```bash
alef scaffold
alef scaffold --lang go,java
```

Caching: skips generation when the IR and config have not changed since the last run.

---

## `alef readme`

Generate per-language README files from templates.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--lang` | string (comma-separated) | all from config | Languages to generate READMEs for |

```bash
alef readme
alef readme --lang python,node
```

Caching: skips generation when the IR and config have not changed since the last run.

---

## `alef docs`

Generate API reference documentation in Markdown format (suitable for mkdocs).

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--lang` | string (comma-separated) | all from config | Languages to generate docs for |
| `--output` | string | `docs/reference` | Output directory for generated documentation |

```bash
alef docs
alef docs --lang python --output docs/api
```

Caching: skips generation when the IR and config have not changed since the last run.

---

## `alef sync-versions`

Sync the version from `Cargo.toml` (or the file specified by `[crate].version_from`) to all package manifests and any configured `[sync]` targets.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--bump` | string | -- | Bump version before syncing: `major`, `minor`, or `patch` |

```bash
alef sync-versions
alef sync-versions --bump patch
alef sync-versions --bump minor
```

Updates all auto-detected package manifests plus any files listed in `[sync].extra_paths` and `[[sync.text_replacements]]`.

---

## `alef build`

Build language bindings using native tools (`maturin`, `napi build`, `wasm-pack`, `cargo build` + `cbindgen`).

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--lang` | string (comma-separated) | all from config | Languages to build |
| `-r`, `--release` | bool | `false` | Build with release optimizations |

```bash
alef build
alef build --lang node
alef build --release
alef build --lang python,wasm --release
```

The build profile is `dev` by default and `release` when `--release` is passed. Post-processing steps (such as patching `.d.ts` files for `verbatimModuleSyntax` compatibility) run automatically.

---

## `alef lint`

Run configured lint and format commands on generated output. Commands are defined in `[lint.<lang>]` sections of `alef.toml`.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--lang` | string (comma-separated) | all from config | Languages to lint |

```bash
alef lint
alef lint --lang python
```

---

## `alef test`

Run configured test suites for each language. Commands are defined in `[test.<lang>]` sections of `alef.toml`.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--lang` | string (comma-separated) | all from config | Languages to test |
| `--e2e` | bool | `false` | Also run e2e tests (uses `[test.<lang>].e2e` commands) |

```bash
alef test
alef test --lang python,go
alef test --e2e
alef test --lang node --e2e
```

---

## `alef verify`

Verify that generated bindings are up to date. Regenerates bindings in memory and compares against files on disk. Designed for CI pipelines.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--exit-code` | bool | `false` | Exit with code 1 if any binding is stale (CI mode) |
| `--compile` | bool | `false` | Also run a compilation check |
| `--lint` | bool | `false` | Also run lint checks |
| `--lang` | string (comma-separated) | all from config | Languages to verify |

```bash
alef verify
alef verify --exit-code
alef verify --exit-code --compile --lint
alef verify --lang python,node --exit-code
```

Lists all stale files when differences are detected. Combine with `--exit-code` to fail CI when bindings are out of date.

---

## `alef diff`

Show what files would change without writing anything. Useful for previewing the effect of config or source changes.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--exit-code` | bool | `false` | Exit with code 1 if any changes exist (CI mode) |

```bash
alef diff
alef diff --exit-code
```

Always operates on all configured languages (no `--lang` filter).

---

## `alef all`

Run the full pipeline: generate + stubs + public API + scaffold + readme + sync. Equivalent to running `generate`, `stubs`, `scaffold`, and `readme` in sequence.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--clean` | bool | `false` | Ignore cache and regenerate everything |

```bash
alef all
alef all --clean
```

Always operates on all configured languages.

---

## `alef init`

Initialize a new `alef.toml` configuration file in the current directory.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--lang` | string (comma-separated) | -- | Languages to include in the generated config |

```bash
alef init
alef init --lang python,node,ruby,go
```

---

## `alef e2e` -- E2E Test Subcommands

Generate and manage fixture-driven e2e test suites. Requires an `[e2e]` section in `alef.toml`.

### `alef e2e generate`

Generate e2e test projects from JSON fixture files.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--lang` | string (comma-separated) | all from `[e2e].languages` | Languages to generate tests for |

```bash
alef e2e generate
alef e2e generate --lang python,rust
```

Caching: skips generation when fixtures, IR, and config have not changed. The cache hash includes the full contents of the fixtures directory.

Per-language formatters are run automatically on generated test files.

### `alef e2e init`

Initialize the fixture directory with a JSON schema file and an example fixture.

```bash
alef e2e init
```

No flags. Creates the directory specified by `[e2e].fixtures` if it does not exist.

### `alef e2e scaffold`

Scaffold a new fixture JSON file with the correct structure.

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--id` | string | *required* | Fixture ID (snake_case, used as test function name) |
| `--category` | string | *required* | Category name (e.g., `smoke`, `basic`, `edge-case`) |
| `--description` | string | *required* | Human-readable description of the test |

```bash
alef e2e scaffold --id parse_empty_input --category edge-case --description "Parsing empty input returns error"
```

### `alef e2e list`

List all fixtures with counts per category.

```bash
alef e2e list
```

No flags. Reads from the directory specified by `[e2e].fixtures`.

### `alef e2e validate`

Validate all fixture files against the JSON schema. Exits with code 1 if validation errors are found.

```bash
alef e2e validate
```

No flags. Reports all validation errors with details.

---

## `alef cache` -- Cache Management

Manage the `.alef/` build cache directory. Alef uses blake3-based content hashing to skip regeneration when source files and config have not changed.

### `alef cache clear`

Clear the entire `.alef/` cache directory.

```bash
alef cache clear
```

### `alef cache status`

Show current cache status, including which stages have cached hashes.

```bash
alef cache status
```

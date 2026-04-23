# alef.toml Configuration Reference

Alef is configured via `alef.toml` in your project root. Run `alef init` to generate a starter config.

## Minimal Example

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

---

## `[crate]` -- Source Configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | string | *required* | Rust crate name |
| `sources` | string[] | *required* | Rust source files to extract |
| `version_from` | string | `"Cargo.toml"` | File to read version from (supports workspace Cargo.toml) |
| `core_import` | string | `{name}` with `-` replaced by `_` | Import path for the core crate in generated bindings |
| `workspace_root` | string | -- | Workspace root for resolving `pub use` re-exports from sibling crates |
| `skip_core_import` | bool | `false` | Skip adding `use {core_import};` to generated bindings |
| `features` | string[] | `[]` | Cargo features treated as always-present (`#[cfg(feature)]` fields are included) |
| `path_mappings` | map | `{}` | Rewrite extracted Rust path prefixes (e.g., `{ "spikard" = "spikard_http" }`) |

---

## `languages` -- Target Languages

Top-level array of languages to generate bindings for:

```toml
languages = ["python", "node", "ruby", "php", "elixir", "wasm", "ffi", "go", "java", "csharp", "r"]
```

The `ffi` language generates the C FFI layer required by `go`, `java`, and `csharp`. If you enable any of those three, `ffi` is implicitly included.

---

## `[exclude]` / `[include]` -- Filtering

Control which types and functions are included in or excluded from generated output.

```toml
[exclude]
types = ["InternalHelper"]
functions = ["deprecated_fn"]
methods = ["MyType.internal_method"]   # dot-notation: "TypeName.method_name"

[include]
types = ["PublicApi", "Config"]        # whitelist only these types
functions = ["extract", "parse"]       # whitelist only these functions
```

When `[include]` is specified, only the listed items are included. `[exclude]` takes precedence when both are present for the same item.

---

## `[output]` -- Output Directories

Per-language output directories for generated Rust binding code. The `{name}` placeholder is replaced with the crate name.

```toml
[output]
python = "crates/{name}-py/src/"
node = "crates/{name}-node/src/"
ruby = "crates/{name}-rb/src/"
php = "crates/{name}-php/src/"
elixir = "crates/{name}-rustler/src/"
wasm = "crates/{name}-wasm/src/"
ffi = "crates/{name}-ffi/src/"
go = "packages/go/"
java = "packages/java/src/main/java/"
csharp = "packages/csharp/"
r = "crates/{name}-extendr/src/"
```

---

## Language-Specific Sections

### `[python]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `module_name` | string | `_{name}` | Python module name (the native extension name) |
| `async_runtime` | string | -- | Async runtime spec for `pyo3_async_runtimes` |
| `stubs.output` | string | -- | Output directory for `.pyi` stub files |
| `features` | string[] | inherits `[crate] features` | Per-language Cargo feature override |
| `serde_rename_all` | string | `"snake_case"` | Override JSON field naming strategy for this language |

### `[node]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `package_name` | string | `{name}` | npm package name |
| `features` | string[] | inherits `[crate] features` | Per-language Cargo feature override |
| `serde_rename_all` | string | `"camelCase"` | Override JSON field naming strategy for this language |

### `[ruby]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `gem_name` | string | `{name}` with `_` | Ruby gem name |
| `stubs.output` | string | -- | Output directory for `.rbs` type stubs |
| `features` | string[] | inherits `[crate] features` | Per-language Cargo feature override |
| `serde_rename_all` | string | `"snake_case"` | Override JSON field naming strategy for this language |

### `[php]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `extension_name` | string | `{name}` with `_` | PHP extension name |
| `feature_gate` | string | `"extension-module"` | Feature gate wrapping all generated code |
| `stubs.output` | string | -- | Output directory for PHP facades/stubs |
| `features` | string[] | inherits `[crate] features` | Per-language Cargo feature override |
| `serde_rename_all` | string | `"snake_case"` | Override JSON field naming strategy for this language |

### `[elixir]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `app_name` | string | `{name}` with `_` | Elixir application name |
| `features` | string[] | inherits `[crate] features` | Per-language Cargo feature override |
| `serde_rename_all` | string | `"snake_case"` | Override JSON field naming strategy for this language |

### `[wasm]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `exclude_functions` | string[] | `[]` | Functions to exclude from WASM bindings |
| `exclude_types` | string[] | `[]` | Types to exclude from WASM bindings |
| `type_overrides` | map | `{}` | Override types (e.g., `{ "DOMNode" = "JsValue" }`) |
| `features` | string[] | inherits `[crate] features` | Per-language Cargo feature override |
| `serde_rename_all` | string | `"camelCase"` | Override JSON field naming strategy for this language |

### `[ffi]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prefix` | string | `{name}` with `_` | C symbol prefix for all exported functions |
| `error_style` | string | `"last_error"` | Error reporting convention |
| `header_name` | string | `{prefix}.h` | Generated C header filename |
| `lib_name` | string | `{prefix}_ffi` | Native library name (for Go/Java/C# linking) |
| `visitor_callbacks` | bool | `false` | Generate visitor/callback FFI support |
| `features` | string[] | inherits `[crate] features` | Per-language Cargo feature override |
| `serde_rename_all` | string | `"snake_case"` | Override JSON field naming strategy for this language |

### `[go]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `module` | string | `github.com/kreuzberg-dev/{name}` | Go module path |
| `package_name` | string | derived from module path | Go package name |
| `features` | string[] | inherits `[crate] features` | Per-language Cargo feature override |
| `serde_rename_all` | string | `"snake_case"` | Override JSON field naming strategy for this language |

### `[java]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `package` | string | `dev.kreuzberg` | Java package name |
| `ffi_style` | string | `"panama"` | FFI binding style (Panama Foreign Function & Memory API) |
| `features` | string[] | inherits `[crate] features` | Per-language Cargo feature override |
| `serde_rename_all` | string | `"camelCase"` | Override JSON field naming strategy for this language |

### `[csharp]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `namespace` | string | PascalCase of `{name}` | C# namespace |
| `target_framework` | string | -- | Target framework version |
| `features` | string[] | inherits `[crate] features` | Per-language Cargo feature override |
| `serde_rename_all` | string | `"camelCase"` | Override JSON field naming strategy for this language |

### `[r]`

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `package_name` | string | `{name}` | R package name |
| `features` | string[] | inherits `[crate] features` | Per-language Cargo feature override |
| `serde_rename_all` | string | `"snake_case"` | Override JSON field naming strategy for this language |

---

## `[dto]` -- Type Generation Styles

Controls how Rust structs are represented in each language's public API. An optional `_output` variant allows using a different style for return types.

```toml
[dto]
python = "dataclass"         # dataclass | typed-dict | pydantic | msgspec
python_output = "typed-dict" # separate style for return types (optional)
node = "interface"           # interface | zod
ruby = "struct"              # struct | dry-struct | data
php = "readonly-class"       # readonly-class | array
elixir = "struct"            # struct | typed-struct
go = "struct"                # struct
java = "record"              # record
csharp = "record"            # record
r = "list"                   # list | r6
```

| Language | Available Styles |
|----------|-----------------|
| Python | `dataclass`, `typed-dict`, `pydantic`, `msgspec` |
| Node/TypeScript | `interface`, `zod` |
| Ruby | `struct`, `dry-struct`, `data` |
| PHP | `readonly-class`, `array` |
| Elixir | `struct`, `typed-struct` |
| Go | `struct` |
| Java | `record` |
| C# | `record` |
| R | `list`, `r6` |

---

## `[scaffold]` -- Package Metadata

Metadata used when generating package manifests (`pyproject.toml`, `package.json`, `.gemspec`, `composer.json`, `mix.exs`, `go.mod`, `pom.xml`, `.csproj`, `DESCRIPTION`):

```toml
[scaffold]
description = "My library for doing things"
license = "MIT"
repository = "https://github.com/org/repo"
homepage = "https://docs.example.com"
authors = ["Your Name"]
keywords = ["parsing", "extraction"]
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `description` | string | -- | Package description used in all manifests |
| `license` | string | -- | SPDX license identifier |
| `repository` | string | -- | Source code repository URL |
| `homepage` | string | -- | Project homepage or documentation URL |
| `authors` | string[] | -- | List of package authors |
| `keywords` | string[] | -- | Keywords/tags for package registries |

---

## `[[adapters]]` -- Custom FFI Adapters

Define custom binding patterns that alef cannot extract automatically. Each `[[adapters]]` entry defines a single adapter.

```toml
[[adapters]]
name = "convert"
pattern = "sync_function"
core_path = "my_crate::convert"
params = [
  { name = "input", type = "String" },
  { name = "options", type = "Options", optional = true },
]
returns = "Result"
error_type = "ConvertError"
gil_release = true    # Python: release GIL during call
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | string | *required* | Adapter function/method name |
| `pattern` | string | *required* | Adapter pattern type (see below) |
| `core_path` | string | *required* | Fully-qualified Rust path to the core function |
| `params` | array | `[]` | Parameter definitions with `name`, `type`, and optional `optional` flag |
| `returns` | string | -- | Return type (`Result`, `Option`, concrete type, or omitted for void) |
| `error_type` | string | -- | Error type name for `Result` returns |
| `gil_release` | bool | `false` | Python: release the GIL during this call |

Supported patterns:

- `sync_function` -- synchronous standalone function
- `async_method` -- async method on a type
- `callback_bridge` -- callback-based FFI bridge
- `streaming` -- streaming/iterator pattern
- `server_lifecycle` -- server start/stop lifecycle management

---

## `[generate]` / `[generate_overrides.<lang>]` -- Generation Control

Toggle individual generation passes. All default to `true`.

```toml
[generate]
bindings = true          # struct wrappers, From impls, module init
errors = true            # error type hierarchies from thiserror enums
configs = true           # config builder constructors from Default types
async_wrappers = true    # async/sync function pairs with runtime management
type_conversions = true  # recursive type marshaling helpers
package_metadata = true  # package manifests (pyproject.toml, package.json, etc.)
public_api = true        # idiomatic public API wrappers
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `bindings` | bool | `true` | Generate struct wrappers, `From` impls, and module init code |
| `errors` | bool | `true` | Generate error type hierarchies from `thiserror` enums |
| `configs` | bool | `true` | Generate config builder constructors from `Default` types |
| `async_wrappers` | bool | `true` | Generate async/sync function pairs with runtime management |
| `type_conversions` | bool | `true` | Generate recursive type marshaling helpers |
| `package_metadata` | bool | `true` | Generate package manifests |
| `public_api` | bool | `true` | Generate idiomatic public API wrappers |

Override per language with `[generate_overrides.<lang>]`:

```toml
[generate_overrides.wasm]
async_wrappers = false
```

---

## `[sync]` -- Version Synchronization

Configure the `alef sync-versions` command. Version is read from the file specified by `[crate].version_from` (default: `Cargo.toml`).

```toml
[sync]
extra_paths = ["packages/go/go.mod"]

[[sync.text_replacements]]
path = "crates/*/cbindgen.toml"
search = 'header = ".*"'
replace = 'header = "/* v{version} */"'
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `extra_paths` | string[] | `[]` | Additional files to update version in (beyond auto-detected manifests) |
| `text_replacements` | array | `[]` | Regex-based text replacements with `path`, `search`, and `replace` fields |

The `{version}` placeholder in `replace` is substituted with the current version.

---

## `[test.<lang>]` / `[lint.<lang>]` -- Test and Lint Commands

Define per-language shell commands for testing and linting generated output.

### Test commands

```toml
[test.python]
command = "pytest packages/python/tests/"
e2e = "cd e2e/python && pytest"

[test.node]
command = "npx vitest run"
```

| Field | Type | Description |
|-------|------|-------------|
| `command` | string | Shell command to run unit tests for this language |
| `e2e` | string | Shell command to run e2e tests (used when `alef test --e2e` is passed) |

### Lint commands

```toml
[lint.python]
format = "ruff format packages/python/"
check = "ruff check packages/python/"
typecheck = "mypy packages/python/"
```

| Field | Type | Description |
|-------|------|-------------|
| `format` | string | Shell command to format generated code |
| `check` | string | Shell command to run lint checks |
| `typecheck` | string | Shell command to run type checking |

---

## `[e2e]` -- E2E Test Generation

Configure fixture-driven test generation. Requires an `[e2e]` section to use `alef e2e` subcommands.

```toml
[e2e]
fixtures = "fixtures"
output = "e2e"
languages = ["python", "node", "rust", "go"]

[e2e.call]
function = "extract"
module = "my_library"
async = true
args = [
  { name = "path", field = "input.path", type = "string" },
]
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `fixtures` | string | *required* | Directory containing JSON fixture files |
| `output` | string | *required* | Output directory for generated e2e test projects |
| `languages` | string[] | *required* | Languages to generate e2e tests for |
| `call.function` | string | -- | Function name to invoke in generated tests |
| `call.module` | string | -- | Module/package to import in generated tests |
| `call.async` | bool | `false` | Whether the function is async |
| `call.args` | array | `[]` | Argument mappings from fixture fields to function parameters |

---

## `[opaque_types]` -- External Type Declarations

Declare types from external crates that alef cannot extract from source. These get opaque wrapper structs in all backends with handle-based FFI access.

```toml
[opaque_types]
Tree = "tree_sitter_language_pack::Tree"
```

Keys are the type name used in bindings. Values are the fully-qualified Rust path.

---

## `[custom_modules]` / `[custom_registrations]` -- Hand-Written Code

Declare hand-written modules that alef should include in `mod` declarations and module init registration.

### `[custom_modules]`

Per-language lists of module names to include in the generated `mod` declarations:

```toml
[custom_modules]
python = ["custom_handler"]
```

### `[custom_registrations.<lang>]`

Per-language registration of hand-written classes, functions, and init calls:

```toml
[custom_registrations.python]
classes = ["CustomHandler"]
functions = ["custom_extract"]
init_calls = ["register_custom_types(m)?;"]
```

| Field | Type | Description |
|-------|------|-------------|
| `classes` | string[] | Class/type names to register in the module init |
| `functions` | string[] | Function names to register in the module init |
| `init_calls` | string[] | Raw Rust expressions to include in the module init function |

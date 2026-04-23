# E2E Test Generation

Alef generates complete, runnable end-to-end test suites for 12 languages from a single set of JSON fixture files. Each generated project is self-contained with build files, test files, and local package references.

## Supported Languages

Rust, Python, TypeScript, Ruby, Go, Java, C#, Elixir, PHP, WebAssembly (JS), R, C.

Each language has a dedicated generator under `crates/alef-e2e/src/codegen/`.

## How It Works

1. Fixtures are JSON files placed in the configured fixtures directory (default: `fixtures/`).
2. Alef loads all `.json` files recursively, validates for duplicate IDs, and sorts by (category, id).
3. For each target language, a generator emits a complete test project under `e2e/{language}/`.
4. Generated test files include language-native assertions translated from the fixture format.
5. Per-language formatters run after generation (configured via `[e2e.format]`).

Files starting with `_` and `schema.json` are skipped during loading.

## Fixture JSON Schema

Each JSON file contains either a single fixture object or an array of fixtures.

### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Unique snake_case identifier. Used as the test function name. Pattern: `^[a-z][a-z0-9_]*$` |
| `description` | string | Yes | Human-readable test description |
| `category` | string | No | Test category. Defaults to the parent directory name if omitted |
| `tags` | string[] | No | Optional labels for filtering during test runs |
| `skip` | object | No | Conditions for skipping the test (see below) |
| `call` | string | No | Named call config to use. References `[e2e.calls.<name>]`. Falls back to `[e2e.call]` when omitted |
| `input` | object | No | Input data passed to the function under test. Fields map to function args via `[e2e.call.args]` |
| `assertions` | array | No | List of assertions to check on the result |

### Example Fixture

```json
{
  "id": "basic_html_conversion",
  "description": "Converts simple HTML to markdown",
  "input": {
    "html": "<h1>Hello</h1><p>World</p>"
  },
  "assertions": [
    { "type": "not_error" },
    { "type": "contains", "field": "content", "value": "# Hello" },
    { "type": "not_empty", "field": "content" }
  ]
}
```

### Skip Directive

```json
{
  "skip": {
    "languages": ["c", "wasm"],
    "reason": "No async support in C FFI layer"
  }
}
```

- `languages` (string[]): Which languages to skip. An empty array means skip for all languages.
- `reason` (string): Human-readable explanation. Emitted as a comment in generated test code.

## Assertion Types

Every assertion object has a required `type` field and optional `field`, `value`, and `values` fields.

### Error assertions

| Type | Description | Fields Used |
|------|-------------|-------------|
| `not_error` | The function call must succeed (no exception/error) | None |
| `error` | The function call must fail | None |

### Equality assertions

| Type | Description | Fields Used |
|------|-------------|-------------|
| `equals` | Field value must equal expected value | `field`, `value` |
| `not_equals` | Field value must not equal expected value | `field`, `value` |

### String/collection containment

| Type | Description | Fields Used |
|------|-------------|-------------|
| `contains` | Field value must contain the expected substring or element | `field`, `value` |
| `contains_all` | Field value must contain all expected values | `field`, `values` |
| `contains_any` | Field value must contain at least one expected value | `field`, `values` |
| `not_contains` | Field value must not contain any of the expected values | `field`, `values` |

### Emptiness checks

| Type | Description | Fields Used |
|------|-------------|-------------|
| `not_empty` | Field value must not be empty (string, list, etc.) | `field` |
| `is_empty` | Field value must be empty | `field` |

### String pattern assertions

| Type | Description | Fields Used |
|------|-------------|-------------|
| `starts_with` | Field value must start with expected string | `field`, `value` |
| `ends_with` | Field value must end with expected string | `field`, `value` |
| `matches_regex` | Field value must match the regular expression | `field`, `value` |

### Length/count assertions

| Type | Description | Fields Used |
|------|-------------|-------------|
| `min_length` | Field length must be >= expected value | `field`, `value` |
| `max_length` | Field length must be <= expected value | `field`, `value` |
| `count_min` | Collection size must be >= expected value | `field`, `value` |
| `count_equals` | Collection size must equal expected value | `field`, `value` |

### Boolean assertions

| Type | Description | Fields Used |
|------|-------------|-------------|
| `is_true` | Field value must be truthy/true | `field` |
| `is_false` | Field value must be falsy/false | `field` |

### Numeric comparison

| Type | Description | Fields Used |
|------|-------------|-------------|
| `greater_than` | Field value must be > expected | `field`, `value` |
| `less_than` | Field value must be < expected | `field`, `value` |
| `greater_than_or_equal` | Field value must be >= expected | `field`, `value` |
| `less_than_or_equal` | Field value must be <= expected | `field`, `value` |

## Field Path Resolution

The `field` in assertions uses dot notation to access nested struct fields on the result object.

### Basic access

- `content` -- top-level field on the result
- `metadata.title` -- nested struct field
- `metadata.document.title` -- deeply nested

### Array indexing

- `links[0].url` -- first element of an array field
- `links[]` -- the array field itself (used with count assertions)

### Map access

- `headers[content-type]` -- map/dict key access

### Field aliases

The `[e2e.fields]` config maps fixture field paths to actual API struct paths. This lets fixtures use short, stable names while the underlying API structure can differ:

```toml
[e2e.fields]
"metadata.title" = "metadata.document.title"
"links" = "result_links"
```

### Optional fields

Fields listed in `[e2e.fields_optional]` get null-safe accessors in generated code. For Rust, this generates `.as_deref().unwrap_or("")` for strings and `.is_some()` checks for structs.

### Array fields

Fields listed in `[e2e.fields_array]` cause the generator to add `[0]` indexing when a fixture path traverses through them.

### Result fields validation

When `[e2e.result_fields]` is non-empty, assertions targeting fields not in this set are emitted as comments (skipped) rather than executable code. This prevents broken assertions when fixtures reference fields from a different call config.

### Enum fields

Fields listed in `[e2e.fields_enum]` receive special handling in languages that cannot directly compare enum values as strings (e.g., Java calls `.getValue()` on the enum).

### C FFI type chains

`[e2e.fields_c_types]` maps `"{parent_type}.{field}"` to the PascalCase return type for chained FFI accessor calls:

```toml
[e2e.fields_c_types]
"conversion_result.metadata" = "HtmlMetadata"
"html_metadata.document" = "DocumentMetadata"
```

## Category Organization

Fixtures are organized into categories by directory structure. Conventional categories:

| Category | Purpose |
|----------|---------|
| `smoke` | Basic sanity checks -- function runs and returns without error |
| `basic` | Core functionality with simple inputs |
| `parsing` | Input parsing correctness |
| `edge-case` | Boundary conditions, unusual inputs, large files |
| `error-handling` | Invalid inputs, expected failures |

Category is automatically derived from the parent directory name unless explicitly set in the fixture's `category` field.

## alef.toml `[e2e]` Configuration

```toml
[e2e]
fixtures = "fixtures"       # Path to fixture JSON files
output = "e2e"              # Output directory for generated projects
languages = ["python", "rust", "node"]  # Override top-level languages list

[e2e.call]
function = "convert"        # Function to test
module = "my_library"       # Module/package where function lives
result_var = "result"       # Variable name for return value
async = true                # Whether the function is async

[[e2e.call.args]]
name = "input"              # Argument name in function signature
field = "html"              # JSON field path in fixture's input object
type = "string"             # Type hint: string, int, float, bool, json_object, bytes
optional = false            # Whether this argument is optional

# Named call configs for multi-function testing
[e2e.calls.embed]
function = "embed"
module = "my_library"
async = true

[[e2e.calls.embed.args]]
name = "text"
field = "text"
type = "string"

# Per-language overrides
[e2e.call.overrides.python]
module = "my_library._my_library"
options_type = "ConversionOptions"
options_via = "kwargs"          # "kwargs", "dict", or "json"

[e2e.call.overrides.go]
alias = "mylib"                 # Import alias
module = "github.com/org/mylib"

[e2e.call.overrides.java]
class = "MyLibrary"             # Java class name

[e2e.call.overrides.elixir]
handle_struct_type = "Config"   # Elixir struct name for config args

# Per-language package references
[e2e.packages.rust]
name = "my-library"
path = "../../crates/my-library"

[e2e.packages.python]
name = "my-library"
path = "../../packages/python"

[e2e.packages.go]
module = "github.com/org/mylib"
path = "../../packages/go"

# Per-language formatters
[e2e.format]
rust = "cargo fmt --manifest-path e2e/rust/Cargo.toml"
python = "ruff format e2e/python/"

# Field path configuration
[e2e.fields]
"metadata.title" = "metadata.document.title"

fields_optional = ["metadata.description", "metadata.author"]
fields_array = ["links", "assets"]
result_fields = ["content", "metadata", "links"]
fields_enum = ["links[].link_type"]
```

## CLI Commands

### `alef e2e generate`

Generate e2e test projects from fixtures. Results are cached -- re-runs are skipped unless fixtures, config, or IR change.

```bash
alef e2e generate              # All configured languages
alef e2e generate --lang rust,python  # Specific languages
```

### `alef e2e init`

Initialize the fixture directory with the JSON schema file and an example smoke test fixture.

```bash
alef e2e init
```

Creates:

- `fixtures/schema.json` -- the fixture JSON schema
- `fixtures/smoke/` -- the smoke test category directory
- `fixtures/smoke/basic.json` -- an example fixture derived from `[e2e.call.args]`

### `alef e2e scaffold`

Scaffold a new empty fixture file with the correct structure.

```bash
alef e2e scaffold --id my_new_test --category basic --description "Tests new feature"
```

Creates `fixtures/basic/my_new_test.json` with a pre-filled template using the configured `[e2e.call.args]`.

### `alef e2e list`

List all fixtures grouped by category with counts.

```bash
alef e2e list
# Fixtures: 42 total
#   basic: 15 fixture(s)
#   edge-case: 8 fixture(s)
#   error-handling: 5 fixture(s)
#   smoke: 14 fixture(s)
```

### `alef e2e validate`

Validate all fixture files against the JSON schema. Exits with code 1 if any errors are found.

```bash
alef e2e validate
# All fixtures are valid.
```

### Running generated tests

```bash
alef test --e2e                  # Run all tests including e2e
alef test --e2e --lang python    # E2E tests for specific language
```

## Workflow

### Adding a new test

1. Scaffold the fixture:

   ```bash
   alef e2e scaffold --id my_test --category basic --description "What this tests"
   ```

2. Edit the fixture JSON to add input data and assertions.
3. Regenerate:

   ```bash
   alef e2e generate
   ```

4. Run:

   ```bash
   alef test --e2e
   ```

### Multi-function testing

Use named call configs when your library exposes multiple functions:

```json
{
  "id": "embed_basic",
  "description": "Basic embedding test",
  "call": "embed",
  "input": { "text": "Hello world" },
  "assertions": [
    { "type": "not_error" },
    { "type": "not_empty", "field": "embedding" }
  ]
}
```

The `"call": "embed"` references `[e2e.calls.embed]` in alef.toml.

### CI integration

```yaml
- name: Verify e2e tests are up to date
  run: |
    alef e2e generate
    git diff --exit-code e2e/
```

Generated files include a `DO NOT EDIT` header. Never hand-edit files under `e2e/` -- modify fixtures or the generator instead.

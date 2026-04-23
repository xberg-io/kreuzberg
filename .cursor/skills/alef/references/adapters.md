# Adapter Patterns Reference

Adapters bridge language-specific calling conventions to Rust core functions. They are configured in `alef.toml` via `[[adapters]]` tables and generate method/function bodies that get injected into the binding code produced by each backend.

Source: `crates/alef-adapters/src/lib.rs`

## How Adapters Work

The `build_adapter_bodies()` function reads all `[[adapters]]` entries from config and produces an `AdapterBodies` map (keyed by `"OwnerType.method_name"` for methods or `"function_name"` for free functions). Each backend backend then looks up these bodies during code generation and inserts them into the appropriate function/method.

---

## sync_function

Synchronous function bridging. Calls a Rust core function directly with type conversion on arguments and results.

### When to use

- Free functions or static methods that execute synchronously
- No `self` receiver -- the function is standalone
- Short-lived operations that complete immediately

### alef.toml config

```toml
[[adapters]]
name = "convert"
pattern = "sync_function"
core_path = "html_to_markdown_rs::convert"
returns = "ConvertResult"
gil_release = true

[[adapters.params]]
name = "input"
type = "String"

[[adapters.params]]
name = "options"
type = "ConvertOptions"
optional = true
```

### Key fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Function name in the generated binding |
| `pattern` | Yes | Must be `"sync_function"` |
| `core_path` | Yes | Full Rust path to the core function (e.g., `my_crate::convert`) |
| `params` | No | List of parameters with `name`, `type`, and optional `optional` flag |
| `returns` | No | Return type name. Used in `.map(ReturnType::from)` conversion. Defaults to `"()"` |
| `error_type` | No | Not used by sync_function (errors are converted per-language) |
| `owner_type` | No | If set, the adapter body is keyed as `"OwnerType.name"` instead of `"name"` |
| `gil_release` | No | Python only: wraps the call in `py.allow_threads()` to release the GIL. Default: `false` |

### Backend support

| Backend | Supported | Notes |
|---------|-----------|-------|
| Python (PyO3) | Yes | Supports `gil_release`. Error via `PyErr::new::<PyRuntimeError, _>` |
| Node (NAPI-RS) | Yes | Error via `napi::Error::from_reason` |
| Ruby (Magnus) | Yes | Error via `magnus::Error::new(runtime_error(), ...)` |
| PHP (ext-php-rs) | Yes | Error via `PhpException::default` |
| Elixir (Rustler) | Yes | Error mapped to string |
| WASM (wasm-bindgen) | Yes | Error via `JsValue::from_str`. Default return: `"JsValue"` |
| C FFI | Yes | String params converted from `CStr`. Result serialized to JSON. Null + `update_last_error` on error |
| Go | Yes | Calls `C.{prefix}_{name}()`. Result JSON-unmarshaled. `defer C.free` for cleanup |
| Java (Panama) | Yes | Uses `Arena.ofConfined()`. Params allocated via `arena.allocateFrom()` |
| C# (P/Invoke) | Yes | Calls `{prefix}_{name}_native()`. `Marshal.PtrToStringUTF8` for result |
| R (extendr) | Yes | Error via `extendr_api::Error::Other` |

---

## async_method

Async method with runtime management. Calls an async method on an owned inner core type, managing the async runtime for each target language.

### When to use

- Methods on a client/service object that perform async I/O
- The owning type holds an `inner` field with the core Rust type
- Operations that return futures/promises to the host language

### alef.toml config

```toml
[[adapters]]
name = "extract"
pattern = "async_method"
core_path = "extract"
owner_type = "Client"
returns = "ExtractionResult"
error_type = "ExtractionError"

[[adapters.params]]
name = "request"
type = "ExtractionRequest"
```

### Key fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Method name in the generated binding |
| `pattern` | Yes | Must be `"async_method"` |
| `core_path` | Yes | Method name on the inner core type (e.g., `"extract"` calls `self.inner.extract(...)`) |
| `params` | No | List of parameters. Converted to core types via `let core_{name}: {core_import}::{type} = {name}.into()` |
| `returns` | No | Return type name for `.map(ReturnType::from)` conversion. Defaults to `"()"` |
| `error_type` | No | Error type name (used in error conversion) |
| `owner_type` | No | The owning struct name. Used to key the body as `"OwnerType.name"`. FFI backends derive snake_case function names from this. Defaults to `"Client"` for FFI backends |
| `gil_release` | No | Not directly used (Python async already releases the GIL via `future_into_py`) |

### Backend support

| Backend | Supported | Async mechanism |
|---------|-----------|----------------|
| Python (PyO3) | Yes | `pyo3_async_runtimes::tokio::future_into_py`. Clones `self.inner` and converts params to core types before entering the async block |
| Node (NAPI-RS) | Yes | Native `async fn` on `#[napi]` methods. Awaits `self.inner.{core_path}()` |
| Ruby (Magnus) | Yes | `tokio::runtime::Runtime::new()` + `rt.block_on()` (blocks the calling thread) |
| PHP (ext-php-rs) | Yes | `WORKER_RUNTIME.block_on()` (blocks on a shared runtime) |
| Elixir (Rustler) | Yes | `tokio::runtime::Runtime::new()` + `rt.block_on()` |
| WASM (wasm-bindgen) | Yes | Native `async` -- awaits directly (single-threaded) |
| C FFI | Yes | Creates a tokio runtime, calls `rt.block_on()`. Dereferences client handle pointer. JSON serialization for complex params |
| Go | Yes | Calls `C.{prefix}_{owner_snake}_{name}()`. JSON marshal/unmarshal for params and results |
| Java (Panama) | Yes | `{prefix}_{owner_snake}_{name}.invokeExact(this.handle, ...)` via Panama FFM |
| C# (P/Invoke) | Yes | `{prefix}_{owner_snake}_{name}_native(this.handle, ...)` via P/Invoke |
| R (extendr) | Yes | `tokio::runtime::Runtime::new()` + `rt.block_on()` |

---

## callback_bridge

Host language callback to Rust trait. Generates a bridge struct that wraps a host-language callable and implements a Rust trait, allowing Rust code to invoke host-language functions.

### When to use

- A Rust trait needs to be implemented by host-language code
- Plugin/handler patterns where users provide custom logic
- The Rust code calls back into the host language

### alef.toml config

```toml
[[adapters]]
name = "handler"
pattern = "callback_bridge"
core_path = "my_crate::handler_trait"
trait_name = "SpikardHandler"
trait_method = "handle"
returns = "Response"
error_type = "HandlerError"
detect_async = true

[[adapters.params]]
name = "input"
type = "Request"
```

### Key fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Bridge name. Struct is named `{Lang}{PascalCase(name)}Bridge` (e.g., `PyHandlerBridge`) |
| `pattern` | Yes | Must be `"callback_bridge"` |
| `core_path` | Yes | Rust module path containing the trait. Used to derive import paths |
| `trait_name` | No | Rust trait to implement. Default: `"Handler"` |
| `trait_method` | No | Trait method name to implement. Default: `"handle"` |
| `params` | No | Parameters passed to the trait method |
| `returns` | No | Trait method return type. Default: `"()"` |
| `error_type` | No | Error type for the trait method. Default: `"anyhow::Error"` |
| `detect_async` | No | Python only: detect if the callback is a coroutine function at construction time. Default: `false` |

### Generated output

Unlike other patterns, `callback_bridge` generates **two** code fragments:

1. **Struct definition** (`{name}.__bridge_struct__`) -- the bridge struct holding the host callback
2. **Trait impl** (`{name}.__bridge_impl__`) -- the `impl Trait for Bridge` block

### Backend support

| Backend | Supported | Bridge struct name | Callback storage |
|---------|-----------|-------------------|-----------------|
| Python (PyO3) | Yes (most complete) | `Py{Name}Bridge` | `Py<PyAny>` + `is_async` flag. Detects coroutine functions. Uses `spawn_blocking` + `Python::attach` for thread safety |
| Node (NAPI-RS) | Partial | `Js{Name}Bridge` | `ThreadsafeFunction`. JSON serialization for params. Result conversion has TODO |
| Ruby (Magnus) | Partial | `Rb{Name}Bridge` | `Opaque<Value>`. Uses `spawn_blocking` under GVL. Result conversion has TODO |
| PHP (ext-php-rs) | Partial | `Php{Name}Bridge` | `ZendCallable`. Uses `try_call`. Result conversion has TODO |
| Elixir (Rustler) | Partial | `Ex{Name}Bridge` | `Term<'static>`. Scheduler invocation has TODO |
| WASM (wasm-bindgen) | Partial | `Wasm{Name}Bridge` | `js_sys::Function`. Awaits Promise results. Result conversion has TODO |
| C FFI | Partial | `Ffi{Name}Bridge` | `extern "C" fn` pointer. JSON serialization. Requires manual `Send + Sync` |
| Go | Partial | `Go{Name}Bridge` | `extern "C" fn` pointer. Same as FFI |
| Java (Panama) | Partial | `Java{Name}Bridge` | `extern "C" fn` pointer. Same as FFI |
| C# (P/Invoke) | Partial | `Cs{Name}Bridge` | `extern "C" fn` pointer. Same as FFI |
| R (extendr) | Partial | `R{Name}Bridge` | `Robj`. Uses `pairlist!()` for calls. Result conversion has TODO |

---

## streaming

Iterator/stream patterns. Wraps a Rust `Stream` to expose it as a language-native iterator or collected array.

### When to use

- Core functions that return `BoxStream<'static, Result<Item, Error>>`
- Chunked/streaming APIs where results arrive incrementally
- Large result sets that benefit from lazy evaluation

### alef.toml config

```toml
[[adapters]]
name = "extract_stream"
pattern = "streaming"
core_path = "extract_stream"
owner_type = "Client"
item_type = "Chunk"
error_type = "ExtractionError"

[[adapters.params]]
name = "request"
type = "ExtractionRequest"
```

### Key fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Method name in the generated binding |
| `pattern` | Yes | Must be `"streaming"` |
| `core_path` | Yes | Method on the inner core type that returns a stream |
| `params` | No | Parameters passed to the stream-producing method |
| `item_type` | No | The type of each stream item. Used in `.map(\|r\| r.map(ItemType::from))`. Default varies by language |
| `error_type` | No | Error type for stream items |
| `owner_type` | No | The owning struct name (used for body key) |

### Generated output

The streaming adapter returns `(method_body, Option<struct_definition>)`:

- **Method body**: the code inside the stream-producing method
- **Struct definition**: for languages that need a separate iterator type (currently only Python). Stored under `"{item_type}.__stream_struct__"` key

### Backend support

| Backend | Supported | Strategy |
|---------|-----------|----------|
| Python (PyO3) | Yes | Generates a `#[pyclass]` async iterator with `__aiter__`/`__anext__`. Stream wrapped in `Arc<Mutex<BoxStream>>`. True async iteration |
| Node (NAPI-RS) | Yes | Collects entire stream into `Vec` via `StreamExt::collect`. Returns array |
| Ruby (Magnus) | Yes | `Runtime::new()` + `block_on`, collects into `Vec` |
| PHP (ext-php-rs) | Yes | `WORKER_RUNTIME.block_on`, collects into `Vec` |
| Elixir (Rustler) | Yes | `Runtime::new()` + `block_on`, collects into `Vec` |
| WASM (wasm-bindgen) | Yes | Collects entire stream into `Vec`. Returns array |
| R (extendr) | Yes | `Runtime::new()` + `block_on`, collects into `Vec` |
| C FFI | No | `compile_error!("streaming not supported via FFI: {name}")` |
| Go | No | `compile_error!("streaming not supported via FFI: {name}")` |
| Java (Panama) | No | `compile_error!("streaming not supported via FFI: {name}")` |
| C# (P/Invoke) | No | `compile_error!("streaming not supported via FFI: {name}")` |

---

## server_lifecycle

Server start/stop lifecycle management. Not yet implemented.

### When to use

- Long-running server processes that need start/stop/health-check methods
- Services that bind to ports and run until signaled

### Current status

All backends emit a `compile_error!` for this pattern:

```rust
compile_error!("adapter pattern not yet implemented: {name}")
```

### alef.toml config (planned)

```toml
[[adapters]]
name = "server"
pattern = "server_lifecycle"
core_path = "my_crate::Server"
```

### Backend support

No backends currently support this pattern.

---

## AdapterConfig Schema Reference

Full fields available on `[[adapters]]`:

```toml
[[adapters]]
name = "function_name"           # Required: function/method name
pattern = "sync_function"        # Required: one of sync_function, async_method, callback_bridge, streaming, server_lifecycle
core_path = "crate::path::fn"    # Required: Rust path to core function/method
returns = "ReturnType"           # Optional: return type for .map(T::from) conversion
error_type = "ErrorType"         # Optional: error type name
owner_type = "OwnerStruct"       # Optional: owning type (makes it a method, keys as "Owner.name")
item_type = "ItemType"           # Optional: streaming item type
gil_release = false              # Optional: Python GIL release (sync_function only)
trait_name = "TraitName"         # Optional: callback_bridge trait to implement
trait_method = "method"          # Optional: callback_bridge trait method name
detect_async = false             # Optional: callback_bridge async detection at construction

[[adapters.params]]
name = "param_name"              # Required: parameter name
type = "ParamType"               # Required: Rust type name
optional = false                 # Optional: wrap in Option, use .map(Into::into) for conversion
```

## Pattern Support Matrix

| Pattern | Python | Node | Ruby | PHP | Elixir | WASM | FFI | Go | Java | C# | R |
|---------|--------|------|------|-----|--------|------|-----|-----|------|----|---|
| sync_function | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| async_method | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |
| callback_bridge | Full | Partial | Partial | Partial | Partial | Partial | Partial | Partial | Partial | Partial | Partial |
| streaming | Yes | Yes | Yes | Yes | Yes | Yes | No | No | No | No | Yes |
| server_lifecycle | No | No | No | No | No | No | No | No | No | No | No |

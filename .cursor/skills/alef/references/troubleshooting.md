# Troubleshooting

Common issues and fixes when working with alef.

## Stale Bindings

**Symptom:** Generated binding code is out of date with the Rust source. CI fails on `alef verify --exit-code` or `git diff --exit-code`.

**Fix:**

```bash
alef generate           # Regenerate bindings
alef all                # Full pipeline (generate + stubs + scaffold + readme)
alef generate --clean   # Force regeneration, ignore cache
```

**Verify:**

```bash
alef verify --exit-code          # Exit 1 if any binding is stale
alef verify --exit-code --lint   # Also check lint
alef verify --exit-code --compile # Also check compilation
alef diff --exit-code            # Show files that would change
```

## Cache Issues

**Symptom:** Changes to Rust source or config are not reflected in output. Alef reports "up to date (cached)" but output is wrong.

**Fix:**

```bash
alef cache clear    # Delete .alef/ cache directory
alef cache status   # Inspect current cache state
```

The cache uses blake3 content hashing. It lives in `.alef/` (should be gitignored). Clearing it forces full regeneration on the next run.

## Missing Types in Generated Output

**Symptom:** A Rust type does not appear in generated bindings. The struct/enum exists in Rust source but is absent from the IR or output.

**Diagnosis:**

```bash
alef extract -o /dev/stdout | jq '.types | keys'   # Check what was extracted
```

**Common causes:**

1. **Type is excluded.** Check `[exclude]` in alef.toml:

   ```toml
   [exclude]
   types = ["InternalType", "HelperStruct"]
   functions = ["internal_helper"]
   methods = ["MyType.private_method"]
   ```

2. **Type is not in the include list.** If `[include]` is set, only listed items are extracted:

   ```toml
   [include]
   types = ["PublicType", "Config"]
   functions = ["convert", "extract"]
   ```

3. **Type is from an external crate.** Use `opaque_types` to declare types alef cannot extract from source:

   ```toml
   [opaque_types]
   Tree = "tree_sitter_language_pack::Tree"
   ```

4. **Source file not listed.** Ensure the file containing the type is in `[crate] sources`:

   ```toml
   [crate]
   sources = ["src/lib.rs", "src/types.rs", "src/config.rs"]
   ```

## Version Mismatch Across Packages

**Symptom:** Python package version differs from npm package version, or Cargo.toml version doesn't match package manifests.

**Fix:**

```bash
alef sync-versions              # Sync current Cargo.toml version to all manifests
alef sync-versions --bump patch # Bump patch version first, then sync
alef sync-versions --bump minor # Bump minor
alef sync-versions --bump major # Bump major
```

Never manually edit version strings in `pyproject.toml`, `package.json`, `Gemfile`, `pom.xml`, etc. The version in `Cargo.toml` (specified by `[crate] version_from`) is the single source of truth.

For custom version locations, use the `[sync]` config:

```toml
[sync]
extra_paths = ["docs/conf.py"]

[[sync.text_replacements]]
path = "README.md"
search = 'version = "{version}"'
replace = 'version = "{version}"'
```

## Build Failures by Language

### Python (maturin)

**Symptom:** `alef build --lang python` fails.

- Ensure `maturin` is installed: `pip install maturin`
- Check that `pyproject.toml` exists in the Python package directory
- Verify the Rust toolchain matches what maturin expects: `rustup show`
- For editable installs: `maturin develop` in the package directory

### Node.js (NAPI-RS)

**Symptom:** `alef build --lang node` fails.

- Ensure `@napi-rs/cli` is installed: `pnpm add -D @napi-rs/cli`
- Check `package.json` has the correct `napi` section
- Run `napi build --release` directly for more detailed errors
- Node.js version mismatch: ensure the target Node.js version matches your installed version

### WebAssembly (wasm-pack)

**Symptom:** `alef build --lang wasm` fails.

- Install wasm-pack: `cargo install wasm-pack`
- Install the wasm target: `rustup target add wasm32-unknown-unknown`
- Check for `std::thread` or synchronous I/O usage -- these are not available in WASM
- For `wasm-opt` errors, install binaryen: `brew install binaryen`

### C FFI (cbindgen)

**Symptom:** C header generation fails or FFI build errors.

- Install cbindgen: `cargo install cbindgen`
- Check for missing `#[no_mangle]` or `extern "C"` on exported functions
- Verify `cbindgen.toml` exists in the FFI crate directory
- Run `cbindgen --crate my-ffi --output header.h` directly for detailed errors

### Ruby (Magnus / rb_sys)

**Symptom:** `alef build --lang ruby` fails.

- Ensure `rake-compiler` is installed: `gem install rake-compiler`
- Check Ruby version (3.2+ required)
- rb_sys needs the Ruby development headers: `ruby -e 'puts RbConfig::CONFIG["rubyhdrdir"]'`
- On macOS, ensure Xcode CLT is installed: `xcode-select --install`

### Elixir (Rustler)

**Symptom:** `alef build --lang elixir` fails.

- Ensure Erlang/OTP 25+ and Elixir 1.14+ are installed
- Check `mix.exs` has the `rustler` dependency
- Run `mix compile` directly for detailed errors
- For NIF loading failures, check that the `.so`/`.dylib` path in `priv/native/` is correct

### PHP (ext-php-rs)

**Symptom:** `alef build --lang php` fails.

- Ensure PHP 8.2+ development headers are installed
- Check `php-config --includes` returns valid paths
- On macOS with Homebrew: `brew install php`
- Verify the generated `.so` loads: `php -d extension=./target/release/libmy_ext.so -m`

### Go (cgo)

**Symptom:** `alef build --lang go` or `go test` fails.

- The C FFI library must be built first: `alef build --lang ffi`
- Set `CGO_ENABLED=1`
- Ensure the C header and `.a`/`.so` library are in the correct path
- For linking errors, check `CGO_LDFLAGS` and `CGO_CFLAGS` in the Go package

### Java (Panama FFM)

**Symptom:** `alef build --lang java` fails.

- Requires Java 21+ (Panama Foreign Function & Memory API)
- Ensure the C FFI library is built first
- Check `java.library.path` includes the directory with the native library
- For `UnsatisfiedLinkError`: verify library name matches what P/Invoke expects

### C# (P/Invoke)

**Symptom:** `alef build --lang csharp` fails.

- Requires .NET 8+
- The C FFI library must be built first
- Check `[DllImport]` library name matches the built artifact
- For runtime errors: ensure the native library is in the output directory or system path

## Cross-Compilation Issues

### aws-lc-sys

**Symptom:** Build fails with `aws-lc-sys` errors when cross-compiling (common in CI for ARM targets).

- aws-lc-sys requires CMake and a C compiler for the target platform
- Set `AWS_LC_SYS_CMAKE_BUILDER=1` to force CMake builds
- For musl targets, install `musl-tools`: `apt-get install musl-tools`
- Consider using `ring` instead of `aws-lc-rs` if cross-compilation is a priority

### General cross-compilation

- Install the Rust target: `rustup target add aarch64-unknown-linux-gnu`
- Set the appropriate linker in `.cargo/config.toml`:

  ```toml
  [target.aarch64-unknown-linux-gnu]
  linker = "aarch64-linux-gnu-gcc"
  ```

- C FFI libraries need cross-compilation toolchains for each target

## Pre-commit Hook Failures

**Symptom:** Pre-commit hook rejects the commit with "stale bindings" message.

**Fix:**

```bash
alef generate          # Regenerate
alef all               # Or full pipeline
git add -A && git commit  # Re-stage and commit
```

The `alef-verify` hook runs `alef verify --exit-code` and fails if any generated file is out of date. The `alef-generate` hook auto-regenerates but you must stage the updated files.

**Configuration:**

```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/kreuzberg-dev/alef
    rev: v0.3.2
    hooks:
      - id: alef-verify    # Check only (CI-friendly)
      # OR
      - id: alef-generate  # Auto-regenerate
```

## DTO Style Mismatch Errors

**Symptom:** Generated code uses the wrong type style (e.g., Python `dataclass` instead of `TypedDict`, or Rust generates a struct but Python expects a dict).

**Fix:** Check `[dto]` in alef.toml:

```toml
[dto]
python = "dataclass"    # or "typed-dict", "pydantic", "msgspec"
node = "interface"      # or "zod"
ruby = "struct"         # or "dry-struct", "data"
php = "readonly-class"  # or "array"
go = "struct"
java = "record"
csharp = "record"
elixir = "struct"       # or "typed-struct"
r = "list"              # or "r6"
```

After changing DTO style, regenerate with `--clean` to avoid mixing styles:

```bash
alef generate --clean
```

## FFI Pointer Issues (Go/Java/C#)

**Symptom:** Segfaults, null pointer dereferences, or use-after-free in Go, Java, or C# bindings.

**Common causes:**

1. **Missing null check.** All FFI functions must check pointers before use. The generated C API returns null on error.

2. **Use-after-free.** Every `_new()` function has a matching `_free()`. Once freed, the handle is invalid:

   ```go
   handle := C.my_type_new()
   defer C.my_type_free(handle)  // Always pair with defer
   // Use handle here
   ```

3. **Borrowed vs owned pointers.** `*const` returns are owned by Rust and valid until the handle is freed. `*mut` returns transfer ownership to the caller.

4. **Missing FFI language.** Go, Java, and C# require the C FFI layer. Ensure `ffi` is in the languages list (it is implicitly included when `go`, `java`, or `csharp` is present).

5. **FFI prefix mismatch.** Check that `[ffi] prefix` matches what the Go/Java/C# bindings expect:

   ```toml
   [ffi]
   prefix = "my_library"
   lib_name = "my_library_ffi"
   ```

## CI Integration Tips

### Verify bindings are up to date

```yaml
- name: Verify bindings
  run: alef verify --exit-code --compile --lint
```

### Verify e2e tests are up to date

```yaml
- name: Verify e2e tests
  run: |
    alef e2e generate
    git diff --exit-code e2e/
```

### Cache the .alef directory

```yaml
- uses: actions/cache@v4
  with:
    path: .alef/
    key: alef-${{ hashFiles('alef.toml', 'src/**/*.rs') }}
```

### Build profile

Always set `BUILD_PROFILE=ci` in GitHub Actions for consistent builds:

```yaml
env:
  BUILD_PROFILE: ci
```

### Full CI pipeline

```yaml
- run: alef all --clean
- run: alef verify --exit-code --compile --lint
- run: alef test --e2e
- run: git diff --exit-code  # No uncommitted changes
```

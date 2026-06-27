# xberg-ffi Scope

This document defines the **canonical public C FFI surface** for `xberg-ffi`.
The crate is consumed by Go (cgo), Java (Panama FFM), and C# (P/Invoke) bindings,
which use **JSON marshaling** for typed values rather than per-field accessors.

## Target surface (~60 exports)

The FFI must mirror the canonical public Xberg API plus the
minimum machinery to make it usable across language boundaries.

### 1. High-level functions

#### Extraction (2)

- `xberg_extract`
- `xberg_extract_batch`

#### Embeddings (4)

- `xberg_embed_texts`
- `xberg_embed_texts_async` _(FFI is synchronous from C ABI perspective; bindings provide async wrappers. May resolve to a single `embed_texts` export.)_
- `xberg_get_embedding_preset`
- `xberg_list_embedding_presets`

#### MIME / format (3)

- `xberg_detect_mime_type`
- `xberg_detect_mime_type_from_bytes`
- `xberg_get_extensions_for_mime`

#### PDF render (1)

- `xberg_render_pdf_page_to_png` _(currently missing — must be added)_

#### Plugin lifecycle (11)

For each of the four plugin axes — `ocr_backend`, `post_processor`, `validator`,
`document_extractor`:

- `xberg_register_<axis>`
- `xberg_unregister_<axis>`
- `xberg_clear_<axis>s`
- `xberg_list_<axis>s`

The `document_extractor` axis is currently missing register/unregister/clear (only
`xberg_list_extractors` exists) — must be added.

### 2. JSON marshaling (~10–15)

Each typed value crossing the FFI boundary that bindings need to round-trip gets
exactly **one** `_from_json` / `_to_json` pair. No per-field getters.

Required pairs:

- `xberg_extraction_config_from_json` / `_to_json` _(present)_
- `xberg_extraction_result_to_json` _(present)_
- `xberg_embedding_config_from_json` / `_to_json`
- `xberg_batch_bytes_item_from_json` / `_to_json`
- `xberg_batch_files_item_from_json` / `_to_json`
- one pair each for any other type bindings serialize across the boundary
  (e.g., `EmbeddingResult`, `PdfRenderConfig`)

### 3. Handle lifecycle (~5–10)

One `_free` per opaque handle type the caller can own:

- `xberg_extraction_config_free`
- `xberg_extraction_result_free`
- `xberg_embedding_config_free`
- `xberg_batch_bytes_item_free`
- `xberg_batch_files_item_free`
- (any other heap-allocated handles produced by `_from_json` constructors)

### 4. String + error handling (5)

- `xberg_free_string` (renames over time → `xberg_string_free` to match
  the rest of the `_free` convention; keep both as alias during deprecation)
- `xberg_last_error_message` _(currently `xberg_last_error_context` — rename)_
- `xberg_last_error_code`
- `xberg_last_error_clear` _(currently missing — must be added)_
- `xberg_version`

### 5. Plugin trampoline plumbing

Vtable + userdata register/unregister functions used by language-implemented
plugins (OCR backends, post-processors, validators, extractors, embedding
backends written in Go/Java/C# and registered into the Rust core). These are
already documented in the typed-bridge design and should remain.

## Out of scope

Per-field getter/setter/clone exports for:

- `AccelerationConfig`, `ContentFilterConfig`, `EmailConfig`, `ExtractionConfig`
- `FileExtractionConfig`, `BatchBytesItem`, `BatchFilesItem`
- `TesseractConfig`, `ServerConfig`, `ArchiveMetadata`, `CacheClearResponse`
- `VersionResponse`, and ~30 other config/response types

These ~1300 exports exist in the current `lib.rs` (1464 total exports) and are
**not** part of the supported FFI surface. Bindings already use JSON
marshaling, so these accessors are dead weight on consumers. They will be
removed in a follow-up cycle (see issue link below).

## Current state

- **Total exports:** 1464
- **Target exports:** ~60–80
- **Reduction needed:** ~1300+ accessors to delete
- **Header file:** `include/xberg.h` (12.3k lines) — will shrink dramatically
  once accessors are removed.

## Migration plan

This document captures the target. The accessor deletion is deferred until
binding audits confirm no silent dependency on per-field accessors:

1. Audit Java/C#/Go bindings for any per-field getter usage; replace with JSON
   round-trip via `*_to_json`.
2. Delete the ~1300 accessor exports from `lib.rs`.
3. Regenerate `include/xberg.h` via `cbindgen`.
4. Bump FFI major for the unified extraction ABI cleanup.

## Why JSON marshaling

Go (cgo), Java (Panama FFM), C# (P/Invoke) all pay similar marshaling overhead
for typed struct walks vs. a single `char*` JSON round-trip. JSON keeps the FFI
ABI surface tiny, removes per-field versioning constraints, and lets bindings
deserialize into language-native records (`record` in Java, `record` in C#,
struct in Go) using existing JSON libraries already on every binding's
critical path. The cost — one allocation + one parse per call — is dwarfed by
extraction work itself.

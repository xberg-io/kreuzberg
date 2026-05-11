# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Removed

- **Gleam binding** (`kreuzberg_gleam` Hex package): dropped entirely. Gleam targets the BEAM and Gleam consumers can keep using the Elixir binding via Erlang interop, so the dedicated package added negligible audience reach at a real maintenance cost (regen on every alef bump, dedicated CI workflow, Hex publish job, e2e fixtures, 92 doc snippets). Existing published versions of `kreuzberg_gleam` on Hex remain available for anyone still pinning them — no further releases will be made.

### Security

- **[GHSA-gg9g-p963-p7x4]**: `HwpxExtractor` now validates the ZIP container against `SecurityLimits` before passing bytes to `unhwp::parse_bytes`. Previously the `ExtractionConfig` was silently discarded (`_config`), allowing a crafted HWPX file with a >100:1 DEFLATE ratio to exhaust process memory (CWE-409). The fix adds an upfront byte-count check (`max_archive_size`) and a `ZipBombValidator` pass over the central directory before any decompression occurs. Affects all builds with the `hwpx`, `formats`, or `full` feature enabled since `5.0.0-rc.1`.

### Added

- **Heuristic PDF table extraction on the default path (#897)**: the default
  text-layer PDF extraction now falls back from pdf_oxide's native grid
  detector to a heuristic reconstruction when the native detector returns
  empty. The fallback clusters words into vertically-contiguous regions by
  abnormally-large row gaps, runs the existing
  `reconstruct_table → post_process_table → is_well_formed_table` chain
  (the same one used by the OCR pipeline and the layout-detection path),
  and emits the surviving grids as `Table` entries with bounding boxes.
  This recovers tables on text-layer PDFs (invoices, financial statements,
  scientific tables) that lack the explicit ruling lines pdf_oxide's grid
  detector requires — without needing the 12 GB ONNX layout-detection
  models. New config: `PdfConfig.extract_tables: bool` (default `true`)
  and CLI flag `--pdf-extract-tables`. Set to `false` to skip table
  extraction entirely.
- **InternalDocument is serde-bridgeable**: `InternalDocument` and its four
  previously-non-serde sub-types (`InternalElement`, `ElementKind`,
  `Relationship`, `RelationshipTarget`) gained `Serialize` + `Deserialize`
  derives. Combined with the `Cow<'static, str>` → `String` migration on
  `source_format` and `mime_type`, foreign-language plugin authors can now
  construct `InternalDocument` values that round-trip through JSON at the
  FFI boundary — unblocking `DocumentExtractor` and `Renderer` trait bridges
  in alef 0.15.25+.
- **DocumentExtractor + Renderer cross-language plugins**: both trait_bridges
  are now active in `alef.toml`. All 16 language bindings expose
  `register_document_extractor` / `unregister_document_extractor` /
  `clear_document_extractors` / `list_document_extractors` and the matching
  Renderer lifecycle. Foreign-language plugin authors can now implement
  arbitrary document extractors and renderers in their host language.
- **#619 follow-up**: `POST /extract-async` now returns HTTP 429 when more than 100 jobs are active simultaneously, preventing unbounded memory growth under load.
- **WASM OCR backend**: `TesseractWasmBackend` registered for the
  `ocr-wasm` feature set, exposing OCR on the WASM target via
  `tesseract-wasm` while the native path continues to use leptess.
- **Renderer plugin**: `Renderer` now extends `Plugin`, picking up the
  shared `name/version/initialize/shutdown` lifecycle (with no-op defaults
  on `Plugin` so stateless renderers stay boilerplate-free). Public
  helpers `register_renderer`, `unregister_renderer`, `list_renderers`,
  `clear_renderers` match the convention of the other plugin registries
  and now all return `Result` for symmetric cross-language codegen.

### Changed

- **BREAKING (wire format)**: `LayoutClass` now serializes as snake_case
  in JSON output (e.g. `"list_item"` instead of `"ListItem"`).
  `LayoutDetection.class_name` returned by HTTP/MCP/Python APIs flips
  PascalCase → snake_case, matching the documentation that already used
  snake_case. The internal `LayoutClass::name()` accessor is renamed to
  `as_str()` and remains available for callers that need a `&'static str`.
- `TableModel` serialization is now symmetric snake_case: previously
  `serde_json::to_string(&TableModel::SlanetWired)` returned
  `"SlanetWired"` (PascalCase) but `from_str` only accepted
  `"slanet_wired"` (snake_case), so JSON config round-trips were
  impossible. Both directions now use snake_case via
  `#[serde(rename_all = "snake_case")]`. The hand-rolled `Deserialize`
  impl is removed.
- **Plugin lifecycle defaults**: `Plugin::version()`, `initialize()`, and
  `shutdown()` now have no-op default implementations (`name()` stays
  abstract). Existing `impl Plugin` blocks that returned `Ok(())` /
  `"1.0.0"` continue to work; new stateless plugins can omit the
  boilerplate.

### Fixed

- **PDF layout-classify regression on text-heavy structure-tree PDFs**:
  three coupled fixes to the layout-for-markdown pipeline that recover
  quality lost when RT-DETR hints were applied too aggressively.
  - `apply_spatial_overrides` now content-gates promotion-class hints
    (Title / SectionHeader / Caption / Footnote / ListItem). A hint is
    rejected when paragraph text doesn't match the hint type — short
    text (≤200 chars) for heading/caption/footnote classes; list-marker
    prefix for ListItem. Prevents promoting long body paragraphs whose
    bbox happens to overlap a heading hint.
  - `apply_proportional_overrides` is now bypassed for structure-tree
    pages (paragraphs without positional data). The fractional-position
    matching misaligned when paragraphs reordered between font-clustering
    and RT-DETR. The structure tree's font-size classification is the
    more reliable signal here; hints are skipped silently.
  - Heading-promotion thresholds tightened:
    `MAX_BOLD_HEADING_WORD_COUNT` 15 → 12, body→heading font-size
    delta `+0.5pt → +1.0pt` in three call sites of `classify.rs`.
- **PDF Markdown / Djot / Plain quality gates now run by default**:
  `test_pdf_quality_gate`, `test_pdf_djot_quality_gate`, and
  `test_pdf_plain_quality_gate` in `crates/kreuzberg/tests/pdf_markdown_regression.rs`
  no longer carry `#[ignore = "TODO: pdf_oxide upstream"]`. The
  upstream issue (#484) hasn't been demonstrated to currently trigger
  these failures, and the calibrated `PDFIUM_GROUND_TRUTH` thresholds
  are the binding contract for layout-pipeline changes.
- **#911**: `extraction_timeout_secs` now explicitly returns a `KreuzbergError::Validation` error when configured in non-tokio or WASM builds. Previously, timeouts in these environments were silently ignored, leading to unexpected hangs.
- **swift e2e command**: `[crates.test.swift].e2e` now runs from
  `packages/swift` (was `e2e/swift`). The generated XCTest cases live inside
  `packages/swift/Tests/<Module>Tests/` because SwiftPM 6.0 forbids
  inter-package `.package(path:)` references in a monorepo, so
  `e2e/swift/Package.swift` is a documentation-only stub with no buildable
  target. The previous setting failed with `error: The package does not
  contain a buildable target.` on every `task swift:e2e` invocation.
- **Java FFI compile error**: `readJsonList` now wraps the null-check and
  `checkLastError()` call inside try-catch, resolving an unreported `Throwable`
  exception that blocked Java e2e test compilation.
- **alef.toml**: `TesseractWasmBackend` added to `[crates.exclude].types`
  so non-WASM bindings (kreuzberg-py, kreuzberg-nif, etc.) no longer
  reference the WASM-only OCR backend (which is
  `#[cfg(feature = "ocr-wasm")]`-gated) and break the build under
  default features.
- **dart e2e**: `extract_file` overridden to `extract_bytes` in `alef.toml`
  (dart cannot pass file paths through the FRB bridge); e2e generator
  regenerated to forward actual `[BatchBytesItem]` / `Uint8List`
  arguments rather than empty parameter lists.
- **e2e/gleam**: regenerated against alef gleam codegen with
  `contains_any` OR logic + `gleam/list` import; full FFI shim
  (`packages/gleam/src/kreuzberg_gleam_ffi.erl`) wraps the
  `Elixir.Kreuzberg.Native` `*_sync` NIFs and converts the Erlang map
  results into Gleam-typed tagged tuples (`extraction_result`,
  `metadata`, `document_structure`, `format_metadata`, `excel_metadata`).
- **e2e/zig**: regenerated for Zig 0.16 API (allocator + IO surface) and
  `FormatMetadata` internally-tagged enum path lookups now skip the
  variant-name segment.
- **Gleam dependency manifest**: restored canonical hex version ranges in
  `packages/gleam/gleam.toml` (`gleam_stdlib = ">= 0.34.0 and < 2.0.0"`,
  `gleeunit = ">= 1.0.0 and < 2.0.0"`). An earlier `alef sync-versions` had
  routed `gleam.toml` through the catch-all SEMVER replace path and
  overwrote both ranges with `">= 5.0.0-rc.1 and < 5.0.0-rc.1"` (an empty
  range gleam refuses to resolve), wedging `gleam test`. The
  `restore_gleam_dep_ranges` helper in alef now keeps these stable on
  future syncs.
- **#853**: HWP structured extraction now returns an error instead of silently returning an empty document when no BodyText sections are found. Fixes a regression introduced in the structured extraction refactor.
- **#619 follow-up**: `POST /extract-async` handler no longer panics on mutex poison — returns HTTP 500 and marks the job as Failed instead.
- Fixed dead conditional-import warning on `KreuzbergError` in `plugins/registry/ocr.rs` under non-OCR feature sets.
- **Zig e2e tests**: Added `default_extraction_config` constant and `extract_file_sync_default`, `extract_bytes_sync_default` overloads for e2e test generation. Configured alef codegen to emit these default variants.

## [5.0.0-rc.1] - 2026-05-05

### Breaking Changes

- **`Metadata.format` is now a nested object, not flattened**: The `format`
  field (and its `format_type` discriminator) moved from the root of the
  `Metadata` JSON object into a dedicated `format` key:
  `{"format": {"format_type": "pdf", ...}}` instead of
  `{"format_type": "pdf", ...}` at root. The `additional` postprocessor map
  is likewise nested under an `"additional"` key. The top-level
  `sheet_count` / `sheet_names` mirror fields are gone; access them via
  `metadata.format.excel.sheet_count` / `.sheet_names`. Affects REST,
  MCP, CLI (`--output-format json`), and every binding.
- **Go module path changed from `v4` to `v5`**: Import path is now `github.com/kreuzberg-dev/kreuzberg/v5`. Update your `go.mod` and all import statements.
- **PHP binding parameter names are now lowerCamelCase**: Function parameters such as `$mime_type` are now `$mimeType`, `$page_index` → `$pageIndex`, etc., matching PHP naming conventions.
- **Python `_to_rust_extraction_config` dict-coercion refactored**: The `isinstance(value, dict)` branch now delegates to `_coerce_dict_extraction_config()`. No public API change; internal helper is not part of the public surface.

### Changed

- Version bump from `4.x` to `5.0.0-rc.1` reflecting accumulated breaking changes across bindings since the `4.10.0` series.
- All binding manifests (Node, Ruby, PHP, Java, C#, Go, Python, Elixir, Gleam, R, Dart, Swift, Zig) updated to `5.0.0-rc.1`.

### Changed

- **Inlined `text-splitter` into `crates/kreuzberg/src/chunking/text_splitter/`.** The upstream crate pins `tokenizers = "0.22"`, which conflicted with kreuzberg's direct `tokenizers 0.23` dependency and produced a duplicate copy of `tokenizers` in the build graph plus a `Tokenizer: ChunkSizer` trait-bound failure in `chunking::core`. The inlined fork drops the unused `code` (tree-sitter) and `tiktoken-rs` features and rebuilds against `tokenizers 0.23`. Kreuzberg's own tree-sitter–based code splitter is unaffected. See `ATTRIBUTIONS.md` for full provenance and license terms.

### Added

- **#619**: Async extraction API — `POST /extract-async` accepts the same multipart form as `POST /extract` and immediately returns `AsyncJobResponse` (`job_id`) with HTTP 202. Background task runs the extraction pipeline (with a configurable timeout) and stores the result in an in-memory `JobStore` (5-minute TTL, evicted on restart). `GET /jobs/{job_id}` returns `JobStatus` (`job_id`, `state`, `created_at`, `updated_at`, `result`, `error`) allowing clients to poll for `Pending` → `Running` → `Completed` / `Failed` transitions. Both endpoints are gated behind the `api` feature flag.
- **HWPX Extraction Support (#875)**: Integrated the `unhwp` crate to natively extract text, structure, and comprehensive document metadata from modern Hangul Word Processor XML (`application/haansofthwpx`) documents. Features dedicated MIME routing distinct from legacy HWP5.
- **#761**: `ExtractionResult.extraction_method` — new field exposing how text was extracted (`native`, `ocr`, `mixed`). Populated by PDF (native vs OCR vs `force_ocr_pages` mixed) and image (always `ocr`) extractors. Surfaced across every binding (Python, Node, PHP, Ruby, Java, C#, Go, R, Dart, Swift, Elixir, Gleam, Zig, WASM, C FFI).
- **#788 follow-up**: Image classification + tile clustering on `ExtractedImage`. New optional fields `image_kind` (a public `ImageKind` enum: `Photograph`, `Diagram`, `Chart`, `Drawing`, `TextBlock`, `Decoration`, `Logo`, `Icon`, `TileFragment`, `Mask`, `Unknown`), `kind_confidence` (`f32` ∈ [0.0, 1.0]), and `cluster_id` (`u32`). The classifier is offline, deterministic, and uses already-captured signals: dimensions, aspect ratio, colorspace, bits-per-component, format, plus Shannon entropy of a 64×64 thumbnail. The clusterer groups same-page images with similar dimensions whose bounding boxes sit within half a tile-side of each other and reclassifies the members to `TileFragment`, so a technical drawing composed of N raster fragments surfaces as one `cluster_id`. Wired through every extractor that produces `ExtractedImage`: PDF (lopdf and pdfium fallback), DOCX, PPTX, HTML, ODT, EPUB, FictionBook, Jupyter, RTF, and the standalone image extractor. Toggle via `ImageExtractionConfig.classify` (default `true`).
- **#784**: `PageInfo.has_vector_graphics: bool` — page-level flag indicating that a PDF page contains non-trivial vector drawing content (paths, shapes, curves) that is otherwise invisible to `ExtractionResult.images` because it isn't embedded as a raster XObject. Populated by counting `PdfPageObject::Path` instances on the pdfium-rendered page; the flag flips when more than 8 paths are present. Lets downstream consumers (RAG / VLM pipelines) decide per-page whether to rasterise the page themselves to capture vector charts and diagrams produced by Adobe InDesign, LaTeX TikZ, etc. Bounding-box detection of vector regions and auto-rasterisation are deferred to a later minor.

### Fixed

- **#799**: Extract images nested inside PDF Form XObjects across XObject references — recursive Form XObject descent (depth-limited to 8) now follows indirect references through the resource chain, with cycle detection via a visited-set so self-referential XObject DAGs no longer hang. Both the lopdf and pdfium image-decoding paths benefit.
- **#824**: Robust PDF image extraction across XObject references — fixes silent image drops when documents reference XObjects through indirect chains. Combined with the depth limit and cycle guard from #799 to harden the recursive walker against malformed structures.
- **#826**: WASM loading on Next.js / Turbopack — `@kreuzberg/wasm` now bundles cleanly under webpack 5, Turbopack, and Next.js's app router. Dynamic imports of Node built-ins and the pdfium-js subsystem carry `/* webpackIgnore: true */` markers so bundlers stop trying to inline platform-specific binaries.
- **#834**: DOCX `inject_placeholders` flag honored end-to-end — image placeholders now appear in markdown/plain/djot output when `ImageExtractionConfig.inject_placeholders = true`, and the DOCX OCR pipeline runs before rendering so OCR text reaches the final document. Adds extractor-level security regression tests for LaTeX, EPUB, ODT, Jupyter, RST, and RTF inputs (deeply nested envs, unclosed math, oversized control words, entity bombs, depth bombs, large item lists).
- **#836**: Prevent base64 image data leaking into structured PDF output when image extraction is disabled. The structure pipeline now suppresses `populate_images_from_pdfium` and `inject_placeholders` whenever neither `ImageExtractionConfig.extract_images` nor `pdf_options.extract_images` is enabled, so disabled-by-default users no longer see embedded raster blobs in their results.
- **#838**: OCR `elements` are now propagated through the extraction pipeline — image and PDF OCR backends populate `ExtractionResult.ocr_elements` consistently, fixing downstream consumers that relied on per-token bounding boxes.
- **#839**: `extraction_timeout_secs` now applies to the single-file `extract_file` / `extract_bytes` paths. Previously the timeout only fired in batch and async wrappers, so a hostile single document could hang past the configured limit. The timeout is gated on `tokio-runtime`; non-tokio builds remain timeout-less by design.
- **#797**: Chunking presets no longer auto-inject an `EmbeddingConfig`. Setting `ChunkingConfig.preset` (e.g. `"multilingual"`) without an explicit `embedding` field previously caused `resolve_preset()` to silently inject `Some(EmbeddingConfig { model: Preset { name }, ..default() })`, which made the extraction pipeline run `generate_embeddings_for_chunks()` and populate every `chunk.embedding` with vectors the caller never asked for. Presets now configure chunking parameters only; opt into embedding generation by providing an `EmbeddingConfig` explicitly.
- **#782**: `result_format = "element_based"` now classifies headings and image placeholders correctly. `process_hierarchy` maps `h1` → `ElementType::Title` and `h2..h6` → `ElementType::Heading`, with the numeric level stored as `metadata.additional["heading_level"]`. `process_content` detects single-line markdown ATX headings (`# Title`, `## Section`, …) and emits them as `Title`/`Heading` instead of `NarrativeText`. `[Image: …]` placeholder lines are now emitted as `ElementType::Image` carrying the description in `metadata.additional["image_description"]`. `process_images` writes `metadata.additional["image_index"]` so consumers can join elements back to the `ExtractionResult.images` array by index.
- **#844**: Python wrapper `extract_*` functions no longer crash on every call. Picked up upstream alef 0.11.24+ codegen fixes (kreuzberg-dev/alef#44): `#[serde(skip)]` is propagated to wrapper structs (no more `unknown field 'cancel_token'`), `api.py` wrappers forward arguments by keyword (no more `extract_file` mime*type/config arg-reorder `TypeError`), async pyo3 functions emit `async def` + `await`, and trait-bridge `register*\*`helpers are re-exported through`api.py`/`**init**.py` `**all**`.
- **`force_ocr_pages` now reliably yields `ExtractionMethod::Mixed`** even when `pages` config is not explicitly set. The PDF text path now synthesizes a default `PageConfig` when force-ocr-pages is non-empty so byte boundaries are always available for splicing OCR text into the right ranges.
- **PDF page extraction strategy enum renamed `ExtractionMethod` → `PageExtractionMethod`** in `kreuzberg-pdfium-render` to disambiguate from `kreuzberg::ExtractionMethod` (the new native/ocr/mixed strategy enum from #761). The pdfium variant remains exported via `pdfium_render::prelude` under the new name.

### Changed

- **#787 (log hygiene)**: Default tracing subscriber now layers `ureq=warn`, `ureq_proto=warn`, `rustls=warn`, `hyper_util=warn`, `hf_hub=info`, `tower_http=info` on top of any user-supplied `RUST_LOG`, and the API router's `TraceLayer` demotes per-request/response events to DEBUG (failures stay at WARN). Default `info` log level no longer produces a wall of HTTP/TLS/transport DEBUG lines around HuggingFace model downloads. Per-target `RUST_LOG` rules continue to win, so `RUST_LOG=ureq=debug` still surfaces full transport detail when needed. The HuggingFace fetch chatter itself was never a re-fetch bug — models are cached on disk under `HF_HOME` and in-process via `LazyLock` engine caches; the noise was purely third-party transport DEBUG bleeding through.
- **Dependabot bumps**: `swift-actions/setup-swift` 2 → 3 (#841), `gradle/actions` 4 → 6 (#840), `docker/setup-qemu-action` 3 → 4 (#833), `mlugg/setup-zig` 1 → 2 (#832).

## [4.10.0-rc.4] - 2026-04-28

Cycle 4 of the alef-backed publish-pipeline iteration. Cycle 3 surfaced fourteen build-stage publish failures across Elixir, Ruby, Python, PHP, Node, and C#; this RC bundles the targeted fixes for each.

### Fixed

- **Elixir native libs build now uses the actual NIF directory name (`kreuzberg_nif`).** The publish workflow had references to `packages/elixir/native/kreuzberg_rustler` that never existed in this repo (the crate is named `kreuzberg_nif`). All five Elixir matrix targets failed in cycle 3 with `start process … working directory … invalid`. Replaced 16 `kreuzberg_rustler` references in `.github/workflows/publish.yaml`.
- **Ruby gem build now declares its `async-trait` dependency.** `packages/ruby/ext/kreuzberg_rb/src/lib.rs` (alef-generated) imports `async_trait::async_trait` for trait bridges, but the matching `Cargo.toml` was missing the dep. Both Ruby gem matrix targets failed in cycle 3 with `unresolved import async_trait`.
- **Ruby `batch_reduce_tokens`, `chunk_text`, `chunk_texts_batch`, `chunk_semantic` are now excluded from the alef-generated Magnus binding.** alef's Magnus codegen referenced undeclared local bindings (`texts_refs`, `page_boundaries_core`) for these functions; the codegen fix is tracked upstream.
- **`task php:build` now exists** (`cargo build --release -p kreuzberg-php`). The publish workflow invoked this task name; without it both PHP PIE matrix targets failed in cycle 3 with `task: Task "php:build" does not exist`.
- **Python wheel build on `linux-aarch64` symlinks `aarch64-linux-gnu-gcc` to `gcc`** when running natively in the manylinux container. The repo's `.cargo/config.toml` pins `aarch64-unknown-linux-gnu`'s linker to the cross-compiler binary name, but that binary doesn't exist in the `manylinux_2_28` container on a native ARM runner. Added a symlink in the cibuildwheel `before-script-linux` step.
- **C# native assets `linux-musl-arm64` build no longer trips on `packages/dart/rust`.** `docker/Dockerfile.musl-ffi`'s sed pattern was missing the dart/swift workspace-member exclusions (added in earlier cycles to other Dockerfiles); cargo failed with `failed to load manifest for workspace member /build/packages/dart/rust`.
- **Node bindings build uses a path-based pnpm filter** (`pnpm --filter ./crates/kreuzberg-node`) instead of the never-resolved `pnpm --filter @kreuzberg/node`. `crates/kreuzberg-node/package.json`'s name is `kreuzberg`, so the scoped filter never matched. Updated three sites (workflow + two scripts).
- **alef bumped to v0.11.7.** Carries two codegen fixes the regenerated bindings depend on: (1) optional `string`/`bytes` arguments in Rust e2e tests now bind to a typed `Option<String>`/`Option<Vec<u8>>` and pass via `.as_deref()` so signatures expecting `Option<&str>` no longer receive `&Option<_>`; (2) PHP backend correctly threads `data_enum_names` through the type mapper for tagged data enums.

## [4.10.0-rc.2] - 2026-04-28

Cycle 2 of the alef-backed publish-pipeline iteration. RC1 surfaced two failures: the `actions/check-registry@v1` and `actions/prepare-release-metadata@v1` shims now require alef ≥ 0.11.0 for the `check-registry` and `release-metadata` subcommands, but `alef.toml`'s top-level `version` field still pinned 0.10.4 (which `install-alef@v1` resolves "latest" against). Bump the alef pin to 0.11.0 so all kreuzberg jobs install an alef binary that has the new subcommands. Also fix the `release-metadata.json` artifact upload that was being wiped by the `prepare` job's re-checkout step (stash to /tmp before re-checkout, restore after).

## [4.10.0-rc.1] - 2026-04-28

First release candidate of v4.10.0. The release pipeline itself is the headline feature: this RC kicks off the iteration loop that proves out the alef-backed publish workflow against real registry endpoints. Substantive functional changes from v4.9.5 are listed below.

### Changed (release pipeline)

- **The publish workflow now runs end-to-end on prerelease tags.** Previously, `if: !github.event.release.prerelease` on the `prepare` job blocked every RC tag from triggering CI. The gate is removed; RCs publish for real with prerelease dist-tags (npm `next`, gemspec `.pre.rc.N`, PyPI `rc{N}`). Homebrew formula updates remain gated on stable releases via a new `is_prerelease` metadata flag.
- **`task version:set -- <version>`** is the canonical way to set a release version (wraps `alef sync-versions --set`). Use it for both stable releases and RCs.
- **Cross-manifest version validation in `scripts/publish/validate-version-consistency.sh` now glob-discovers the Ruby `version.rb` file** across all three rb-sys-style layouts and silently skips Go/PHP whose version-bearing manifest is absent (those ecosystems version via git tags, not in source).

### Fixed

- **C FFI, PHP, Ruby, R bindings shipped 40+ stub functions that returned `Not implemented` at runtime**. Every batch API (`batch_extract_file_sync`, `batch_extract_bytes_sync`, plus async variants), `extract_file`, `extract_file_sync`, and most of the Ruby gem's surface (21 functions) silently failed with error code 99. Root cause was in the alef binding generator: bare `Path` was misresolved to `Named("Path")` and sanitized to `String`; sanitized batch-tuple params (`Vec<(PathBuf, Option<FileExtractionConfig>)>`) were never handled by the PHP/Magnus/FFI codegen even though the IR carried the original type for JSON-roundtrip; Magnus rejected every extraction function via an over-strict `is_named_ref_param` check; and the R backend panicked on every async function. Fixed in alef and regenerated all bindings — Python, Node, FFI all build clean. Only `kreuzberg_get_preset` remains stubbed (return-type sanitization edge case; tracked separately).
- **#788**: Extract images nested inside PDF Form XObjects — `lopdf::get_page_images` only scanned the page-level `Resources` dictionary and silently skipped images stored inside `Subtype=Form` XObjects, which is the structure used by technical drawings composed of tiled raster tiles. PDF image extraction now recursively descends into Form XObjects (up to 8 levels deep) in both the lopdf and pdfium code paths, so all constituent images are collected.
- **Security limits actually enforce now**. `SecurityLimits` config fields (`max_nesting_depth`, `max_entity_length`, `max_content_size`, `max_iterations`, `max_xml_depth`, `max_table_cells`) and the matching `SecurityError` variants previously advertised protection that no extractor invoked — the validator helpers were `#[cfg(test)]`-gated and removed in commit `c58069201` as dead code, leaving only the config knobs. Five internal validators (`StringGrowthValidator`, `IterationValidator`, `DepthValidator`, `EntityValidator`, `TableValidator`) are restored and now run on every extraction path that ingests user-controlled bytes — XML-class formats (DOCX/PPTX/XLSX/ODT/EPUB/SVG/JATS/DocBook/FictionBook/OPML), HTML, JSON/YAML/TOML, tabular extraction (CSV/Excel/HTML tables/DOCX cells), and final text accumulation for plain-text formats (Markdown/Org/RST/LaTeX/Jupyter/RTF). Hostile inputs (billion-laughs entity expansion, depth bombs, cell bombs, quadratic string growth, iteration bombs) now fail with a structured `KreuzbergError::Security` instead of OOMing or hanging. The validators are internal core-only types; bindings observe the protection through the new unified `Security` error variant returned from every `extract_*` entry point. Defaults relaxed where the previous values false-positived on legitimate documents: `max_nesting_depth` 100 → 1024, `max_xml_depth` 100 → 1024, `max_entity_length` 32 → 1 048 576 (per-token cap; cumulative size remains bounded by `max_content_size`).
- **#789**: PDF image extraction would hang indefinitely on documents with thousands of image objects on a single page (observed: 2487 images). The `max_images_per_page` cap was added to `ImageExtractionConfig` in #766 but only wired to the structure pipeline's position counting, never to the byte-decoding path; pages exceeding the cap are now skipped with a `WARN` log before the FlateDecode loop runs. Both `extract_images_from_pdf` and the pdfium fallback now run inside `tokio::task::spawn_blocking`, so `extraction_timeout_secs` can interrupt them. (#800)
- **#794**: Fix Helm chart default install broken by two conflicts: (1) the cache init container ran as `root` while `podSecurityContext.runAsNonRoot: true` is the default, causing kubelet to reject the pod; (2) Kubernetes service discovery injects `KREUZBERG_PORT=tcp://...` when the release is named `kreuzberg`, which the binary parses as a `u16` and panics. Fixed by adding `runAsNonRoot: false` to the init container's `securityContext`, a new `cache.initChown` toggle (default `true`, set to `false` on fsGroup-aware storage to skip the init container entirely), and defaulting `enableServiceLinks: false` in the pod spec. (#822)
- **#825**: `kreuzberg cache manifest` no longer fails with `E0282` when `kreuzberg-cli` is built without `paddle-ocr` or `layout-detection` (e.g. `--no-default-features --features bundled-pdfium`). The command now bails with a clear actionable error if invoked at runtime in such a build.
- **`@kreuzberg/node` prebuilt bindings fail to load on RHEL 8 / AlmaLinux 8 / Rocky 8 / RHEL 9**: the Linux x64/arm64 GNU prebuilds are now built via `cargo-zigbuild`, which caps the glibc floor at link time. Fixes the `GLIBC_2.38 not found` / `GLIBCXX_3.4.31 not found` / `undefined symbol: __isoc23_strtoll` load errors on RHEL 8, AlmaLinux 8, Rocky 8 (glibc 2.28) and RHEL 9 (glibc 2.34). Verified locally: the prebuilt `.node` drops from `GLIBC_2.38` / `GLIBCXX_3.4.31` down to `GLIBC_2.28` / no `GLIBCXX` dependency. `kreuzberg-tesseract/build.rs` auto-detects the zigbuild toolchain and (1) disables tesseract's AVX512 codepath (zig/clang requires an explicit `evex512` feature that tesseract's CMake doesn't pass) and (2) skips linking `stdc++fs` (zig's libstdc++ has `std::filesystem` inline). The publish pipeline now (a) runs `objdump -T` against each linux-gnu prebuild and rejects any artifact requiring `GLIBC_*` > 2.28, any `GLIBCXX_*` symbol, or any `__isoc23_*` symbol, and (b) loads the prebuilt `.node` inside `redhat/ubi8` (glibc 2.28) and exercises the napi surface before publishing to npm. Refs #352.
- **#781**: Fix DOCX OCR pipeline integration — reordered the extraction pipeline to ensure OCR processing runs before document rendering. Markdown, Plain, and Djot renderers now correctly receive and inject OCR text output instead of dropping it or omitting images.
- **#823**: Fix WASM loading in Next.js / Turbopack by adding `/* webpackIgnore: true */` to dynamic imports of Node.js built-ins and resolving bundling issues in the pdfium-js subsystem.

### Added

- **#788**: Extract images nested inside PDF Form XObjects — PDF image extraction now recursively descends into Form XObjects (up to 8 levels deep) in both the lopdf and pdfium code paths.

### Changed

- Fix `use use` duplicate-import syntax error in alef-generated elixir NIF binding (`kreuzberg_nif/src/lib.rs`).
- Apply `cargo fmt` uniformly across all workspace crates (formatting only, no logic changes).
- Fix typo `entrys` → `entries` in auto-generated API reference docs.

### Added

- **#788**: Extract images nested inside PDF Form XObjects — PDF image extraction now recursively descends into Form XObjects (up to 8 levels deep) in both the lopdf and pdfium code paths.

### Changed

- Fix `use use` duplicate-import syntax error in alef-generated elixir NIF binding (`kreuzberg_nif/src/lib.rs`).
- Apply `cargo fmt` uniformly across all workspace crates (formatting only, no logic changes).
- Fix typo `entrys` → `entries` in auto-generated API reference docs.

---

## [4.9.5] - 2026-04-23

### Fixed

- **#790**: Fix GPU acceleration — kreuzberg now bundles CPU-only ONNX Runtime by default (zero-config). When a GPU execution provider (`cuda`, `tensorrt`, `coreml`) is explicitly requested via `AccelerationConfig` but unavailable, kreuzberg returns an error with setup instructions instead of silently falling back to CPU. `Auto` mode gracefully falls back to CPU with an info log. For GPU support, set `ORT_DYLIB_PATH` to a GPU-enabled ONNX Runtime.
- **#791**: Fix DOCX OCR extraction — OCR now runs on embedded images before document rendering, and OCR text is injected into the rendered output. Previously, OCR results were discarded and replaced with placeholder text.
- **#783**: PaddleOCR backend not utilizing GPU (CUDA) despite `AccelerationConfig` — `AccelerationConfig` from `ExtractionConfig` was never reaching PaddleOCR ONNX sessions, silently falling back to CPU. Acceleration is now propagated through `OcrConfig` to all OCR call sites (image extractor, PDF OCR).
- **#779**: Expose `PaddleOcrConfig` in Python bindings and update `OcrConfig` for backward compatibility.
- **#792**: Fix Ruby gem packaging — exclude staged `libpdfium.dylib` from gem artifacts by narrowing the native extension glob to only include the compiled `kreuzberg_rb.*` extension.

### Added

- GPU CI workflow (`ci-gpu.yaml`) targeting self-hosted GPU runners with NVIDIA GPUs.
- Comprehensive GPU integration tests covering all ORT-accelerated paths: PaddleOCR (det/cls/rec), layout detection (RT-DETR), embeddings, document orientation detection, and end-to-end extraction. Tests use tracing log capture to verify CUDA EP is actually invoked.

---

## [4.9.4] - 2026-04-22

### Fixed

- **Ruby gem build failure** — add missing `max_images_per_page` field to `ImageExtractionConfig` initializer in Ruby binding (`kreuzberg-rb`), fixing compilation error E0063 on all platforms.
- **Node binding build failure on Linux** — stop removing `/usr/local/lib/node_modules` in CI disk cleanup script; npm was being deleted before `pnpm/action-setup` could use it, causing `spawn npm ENOENT`.
- **Homebrew formula publish failure** — grant `contents: write` permission to the `publish-homebrew` job so `gh release upload` can attach bottle artifacts (was `contents: read`).
- **#783**: PaddleOCR now correctly utilises the GPU (CUDA) when `AccelerationConfig(provider="cuda")` is set. Previously `self.acceleration` on `PaddleOcrBackend` was always `None` (hardcoded at construction time), so the ONNX session builder never received the requested execution provider and silently fell back to CPU. `AccelerationConfig` is now threaded from `ExtractionConfig` into the ephemeral `OcrConfig` at each `process_image` call site (image extractor and both PDF OCR paths), and `PaddleOcrBackend::process_image` sets the module-level thread-local before the engine-pool slow path — so ONNX sessions are created with the correct provider on first use.

---

## [4.9.3] - 2026-04-22

### Added

- **Layout detection regions on PageContent** — new `layout_regions` field exposes detected layout regions (class, confidence, bounding box, area fraction) from the RT-DETR model when layout detection is enabled. Enables programmatic detection of diagrams, figures, tables, and other content types per page. Available across all 10 bindings. (#579)
- **LayoutRegion type files** for Java, PHP, and Elixir bindings (were referenced but missing).
- **E2E assertions for layout regions** — `has_layout_regions` and `layout_classes_include` assertion types in all 12 language generators.

### Fixed

- **#779**: Fix `PaddleOcrConfig` not bound in Python API — exposed `PaddleOcrConfig` as a first-class class in the Python bindings. Updated `OcrConfig` to accept both `PaddleOcrConfig` objects and raw dictionaries for backward compatibility. Added `paddle_ocr_config` property (getter/setter) to `OcrConfig`.
- **#770**: DOCX page extraction (`extract_pages=True`) now works correctly — `result.pages` and `result.get_page_count()` are no longer always `None`/`0`. Two bugs fixed: (1) computed `PageContent` blocks were never stored on `InternalDocument.prebuilt_pages`, so the derivation pipeline always fell back to `None`; (2) page-break markers inside table cells were incorrectly added to the top-level element list, creating phantom page boundaries before tables and corrupting `table_page_numbers`. Page breaks (`w:br[w:type="page"]` and `w:lastRenderedPageBreak`) in body text are stored as `DocumentElement::PageBreak` and mapped to precise character offsets; breaks inside table cells are intentionally ignored at document level (tables spanning multiple pages remain a known limitation).
- **#773**: `serve` and `mcp` CLI subcommands now correctly apply `KREUZBERG_*` environment variable overrides. Previously, variables such as `KREUZBERG_OCR_LANGUAGE`, `KREUZBERG_LLM_MODEL`, and `KREUZBERG_LLM_API_KEY` were silently ignored when starting the API or MCP server — only the `extract` command honoured them. Also fixes the provider env-var fallback in the LLM client: `MISTRAL_API_KEY` is now picked up for bare `mistral-*` model names (e.g. `mistral-large-latest`), not only for the `mistral/` prefix form.
- **#774**: Tagged-PDF structure tree dropped paragraph body text when a block had both own text and children, and wrapped numbered section headings in an invalid `List → Heading` AST (panics comrak in debug, emits malformed markdown in release). `flatten_blocks` now emits parent text alongside children; text-pattern list detection in `element_to_paragraph` is gated on `heading_level.is_none()`.
- **Semantic chunker fallback path now respects `max_characters`** — previously the non-embedding fallback hardcoded a 4000-char ceiling and silently ignored the caller's `max_characters`. A warning is also emitted when `chunker_type='semantic'` is used without an `EmbeddingConfig` so the fallback mode is discoverable. The `ChunkerType::Semantic` docstring has been corrected to describe both paths accurately.
- **OCR backend dispatch**: `OcrConfig(backend=...)` with a non-default backend no longer silently falls back to paddleocr when the chosen backend errors — auto-fallback is limited to the default tesseract backend; users who want multi-backend fallback configure it via `OcrConfig.pipeline` (unchanged).
- **EasyOCR on PDFs**: `EasyOCRBackend.supports_document_processing()` returns `False` so Rust's `PdfRenderer` handles page rendering, removing the implicit `pdf2image`/`pymupdf` requirement that was never declared in the `[easyocr]` extra.
- **Cross-format parity test failure** — HTML extractor now normalizes setext headings to ATX and strips trailing whitespace from html-to-markdown-rs output.
- **Broken wasm-deno/wasm-workers e2e tasks** — removed non-functional deno and workers e2e generate/lint/test tasks that referenced invalid generator lang values.
- **oxlint path in node e2e lint** — `oxlint --fix typescript` changed to `oxlint --fix .` (was looking for nonexistent `typescript/` directory).
- **Clippy warnings in benchmark-harness** — `sort_by` replaced with `sort_by_key` + `Reverse`.
- **Clippy warnings and compilation errors across workspace** — added missing `max_images_per_page` field to `ImageExtractionConfig` in node and Python bindings; added missing `vlm_prompt` argument to VLM OCR test calls; collapsed nested `if-let` in WASM embeddings; added `embeddings` and `tree-sitter` passthrough features to `kreuzberg-ffi` to silence `unexpected_cfgs` warnings.
- **Cancellation token not wired in oxide segment structure pipeline** — `cancel_token` was passed into `SegmentStructureConfig` but never checked, meaning cancellation/timeout had no effect during pdf-oxide table extraction or paragraph building. Added cancellation checks at table page prep, heuristic table extraction loops, and a pre-flight guard before parallel paragraph extraction.
- **#771**: `OcrConfig.vlm_prompt` is now correctly honored in VLM OCR requests. Previously, it was documented but never forwarded to the underlying VLM calls, causing the default template to be used regardless of configuration.
- **#762**: PDF image links are no longer silently dropped from markdown output. Image extraction now correctly preserves correspondence between pdfium objects and lopdf data, and respects the `inject_placeholders` configuration.
- **#769**: Downgraded `pre-commit-shfmt` to `v3.13.1-1` (fixes broken CI due to non-existent version in `main`).
- **#766**: PDF extraction with large numbers of image fragments no longer hangs indefinitely — added `ImageExtractionConfig.max_images_per_page` (default `None`) to cap images processed per page. Batch-level `extraction_timeout_secs` now interrupts blocking pdfium threads at the next inter-page checkpoint via a `CancellationToken`, preventing the timeout from being silently bypassed.
- **#764**: PST extractor now populates email attachments — `attachments` was hardcoded to an empty list and never read from the message; now reads attachment name, filename, MIME type, size, and binary data via the attachment table. PST entry IDs are now formatted as proper 48-char MAPI hex strings instead of Rust Debug output.

### Added

- `ImageExtractionConfig.max_images_per_page` — optional cap on images decoded per page; prevents hangs on PDFs with thousands of inline image fragments.

### Changed

- Removed redundant `.task/workflows/e2e.yml` — e2e tasks consolidated in top-level `Taskfile.yml`.

---

## [4.9.2] - 2026-04-19

### Fixed

- Fix cancellation token not checked in WASM (non-tokio) path for Excel, DOC, PPT, Pages, Keynote, and Numbers extractors — cancellation was silently ignored in WASM builds
- Propagate `Cancelled` error code (9) to all bindings — Go, C FFI, Python, TypeScript, and C API docs now include the new code
- Fix PHP e2e embed tests calling instance methods statically — use procedural `\Kreuzberg\embed()` functions
- Fix TypeScript e2e embed tests using wrong field names (`type`/`name` → `modelType`/`value`) for embedding model config
- Fix Elixir e2e embed tests calling non-existent `embed_async/2` — use sync `embed/2`
- Fix TypeScript e2e generator missing `html_output` config mapping for styled HTML tests
- Fix `ORT_DYLIB_PATH` on Windows CI pointing to `lib/` instead of the actual DLL location
- Fix C# CI build conditional to require successful FFI build
- Add `libuv1-dev` to Linux CI system dependencies for R package builds

---

## [4.9.1] - 2026-04-19

### Fixed

- **#754**: Preserve `_internal_bindings.pyi` type stub during wheel artifact cleanup — published wheels now include inline type information for the core binding module
- Add missing `Default` impl for `PyCancellationToken` to satisfy clippy `new_without_default` lint
- Improve download resilience for `eng.traineddata` in build script — increase retries from 3 to 5, add fallback URL via `raw.githubusercontent.com`, and increase timeout to 300s
- Increase Task installer retry resilience in CI — 5 attempts with `--retry-all-errors` curl flag

---

## [4.9.0] - 2026-04-18

### Fixed

- **#588**: Suppress C23 glibc symbols (`__isoc23_strtoll` etc.) in manylinux wheels — added CMake flag propagation and CI verification step to prevent incompatible symbols on glibc < 2.38 (Debian 12, Ubuntu 22.04)
- **#748**: Remove `kreuzberg-cli` from Python wheel to fix `libonnxruntime.so.1` loading failure — CLI is available as standalone release
- **#749**: Add cancellation token support — cancelled extractions no longer block subsequent calls via `PDFIUM_OPERATION_LOCK`; wired across Python, Node.js, Ruby, WASM, and C FFI bindings
- **#750**: Fix `kreuzberg[easyocr]` extra silently installing nothing on Python 3.14+; clean up stale `[paddleocr]` references in docs
- **#752**: Fix ~1000x slowdown on Ghostscript-produced PDFs with structured output — replace O(N²) `Vec::contains` with O(1) `AHashSet` lookup, add minimum dimension filter for tiny inline images
- **#753**: Fix `llm_usage` returning `None` when using VLM-based OCR — propagate usage through PDF OCR, image OCR, and `force_ocr_pages` paths

### Added

- Cancellation token API available in all language bindings (`CancellationToken` in Python/Node/Ruby/WASM/FFI)

### Changed

- **Breaking**: `kreuzberg-cli` binary is no longer bundled in the Python wheel — install the standalone CLI from GitHub releases

---

## [4.8.6] - 2026-04-17

### Added

- **PST message EntryID in extracted metadata** — the `entry_id` field from Outlook PST message entries is now included in the `metadata` HashMap of `EmailExtractionResult`, enabling callers to unambiguously link extracted data back to its source message. (#739)
- **AccelerationConfig wired through all ORT model loading** — `AccelerationConfig` (CUDA, CoreML, TensorRT, Auto) is now propagated to all ONNX Runtime sessions: layout detection (RT-DETR, YOLO, SLANeT, TATR, TableClassifier), embeddings, document orientation, and PaddleOCR. Previously, GPU acceleration was silently ignored and all models used CPU. The `acceleration` field is also added to `LayoutDetectionConfig` and `EmbeddingConfig` across all 11 bindings (Python, TypeScript, Ruby, Go, Java, C#, PHP, R, Elixir, FFI, WASM). (#740)

### Added

- Semantic chunker (`ChunkerType::Semantic`) for topic-aware document splitting
- `topic_threshold` configuration field for embedding-based topic detection
- `utils/markdown_utils` shared utility for ATX heading detection
- `preset_chunk_size()` helper in embeddings module
- E2e contract fixtures for semantic chunking

### Fixed

- **Batch extraction panics with "Lazy instance has previously been poisoned" on ARM64 Linux** — OCR backend registry initialization used `panic!()` on Tesseract/PaddleOCR init failures, poisoning the `Lazy` static and cascading to all concurrent batch tasks. Replaced with `tracing::warn!()` + graceful skip. Also converted `GLOBAL_RUNTIME`, `EXTRACTORS_INITIALIZED`, and 3 `PROCESSOR_INITIALIZED` statics from `once_cell::sync::Lazy` to `once_cell::sync::OnceCell` (retry on failure instead of permanent poisoning). Migrated ~15 collection/cache `Lazy` statics to `std::sync::LazyLock`. (#741)
- **PaddleOCR `model_tier` from TOML config ignored by API server** — the singleton PaddleOcrBackend always used `self.config.model_tier` (default "mobile") to resolve models, ignoring the per-request `paddle_ocr_config.model_tier` from the user's TOML/API config. Engine initialization now uses the effective per-request config. (#725)
- **VLM OCR backend ignored when paddle-ocr feature enabled** — the auto-constructed OCR pipeline hardcoded `vlm_config: None` on pipeline stages, silently discarding the user's VLM configuration. Users who configured `OcrConfig(backend="vlm", vlm_config=LlmConfig(...))` got tesseract/paddleocr output instead of VLM. The pipeline now propagates `vlm_config` from the parent `OcrConfig`. (#738)
- **Doubled OCR content and corrupted page text in image extraction** — OCR elements were injected into the rendering pipeline as `OcrText` internal elements, causing `render_plain` to append every raw word token after the coherent HOCR string. `ExtractionResult.content` was effectively duplicated and `pages[*].content` contained a word-by-word dump instead of the readable text. OCR elements are now stored directly via `prebuilt_ocr_elements`, bypassing the rendering pipeline. (#706)
- **Image OCR pages[] empty** — `include_elements` was not forced true for image extraction, so backends that gate element output (e.g. paddle-ocr) returned `None`, leaving `pages[]` empty. (#723)
- **`LlmConfig` missing `Default` trait** — the documented `..Default::default()` struct-update pattern failed to compile with "trait not satisfied". Added `Default` to the derive macro; all optional fields default to `None`, `model` to `""`. (#716)
- **Incorrect `llm` Cargo feature name in docs** — `llm-integration.md`, `api-rust.md`, and `configuration.md` referenced a `llm` feature that does not exist; the correct name is `liter-llm`. (#717)
- **LLM embedding provider panics in server mode** — `embed_texts` called `block_on` inside a new runtime, which panics when already inside tokio (HTTP server, MCP). Uses `block_in_place` with the current runtime handle when available, falls back to a new runtime for standalone sync callers. (#713, #714)
- **Duplicate `output_format` key in OCR metadata** — stale `additional` HashMap insert caused a duplicate JSON key violating RFC 8259. The value is already on the typed `Metadata::output_format` field. (#712)
- **OCR table metadata serialized as strings instead of numbers** — `table_count`, `tables_detected`, `table_rows`, and `table_cols` were `"0"` instead of `0`, breaking numeric comparisons in all bindings. (#712)
- **Ruby `structured_output` not exposed on Result** — the field was missing from the Ruby binding's `Result` class and not serialized from the native extension. (#736)
- **Stale hf-hub lock files block embedding model downloads** — cleaned up orphaned lock files before downloading. (#721)
- **WASM live demo `enableOcr()` not called** — OCR was silently unavailable in the demo; also throws on missing Rust registry export. (#719, #720)
- **DOCX tables assigned wrong page numbers** — tables were numbered by index instead of by their actual document position based on page breaks. (#718)
- **`ocr.enabled=false` config ignored** — OCR ran even when explicitly disabled; also dropped trailing newline in `--format text` output. (#715)
- **Go module tag push fallback** — added `git push` fallback when tag push fails.
- **Go E2E `LlmUsage` type mismatch** — generated Go test helper used `[]interface{}{}` instead of `[]kreuzberg.LlmUsage{}`.
- **Rust E2E `extractMetadata` field name** — html_options fixture used camelCase `extractMetadata` instead of snake_case `extract_metadata` expected by html-to-markdown-rs v3.2.
- **R package documentation stale** — 14 exported functions lacked `.Rd` man pages and `extraction_config.Rd` was missing 13 parameters added in v4.8.0–4.8.5. Regenerated all roxygen2 documentation.

### Changed

- Updated all dependencies including html-to-markdown-rs 3.1→3.2, pdf_oxide 0.3.30→0.3.32, tokio 1.51→1.52.

---

## [4.8.5] - 2026-04-14

### Added

- **LLM usage tracking** — new `llm_usage` field on `ExtractionResult` captures token counts, estimated cost (USD), model identifier, and finish reason for every LLM call (VLM OCR, structured extraction, LLM embeddings). Multiple entries are produced when multiple LLM calls occur in a single extraction. Exposed across all bindings: Python, TypeScript, Ruby, PHP, Go, Java, C#, Elixir, R, C FFI, and WASM.

### Fixed

- **Markdown chunker duplicates heading when `prepend_heading_context` is enabled** — the heading was prepended twice when a chunk boundary aligned with a heading node, producing repeated heading text in the output. (#701)
- **Helm chart icon 404 on Artifact Hub** — `Chart.yaml` referenced `logo.png` but the file is `logo.svg`.
- **Python wheel manylinux compliance failure** — bumped manylinux from `2_38` to `2_39` to allow `GLIBCXX_3.4.31` symbols from the build toolchain, matching the v4.6.x baseline that worked.
- **Python wheel requires glibc ≥ 2.38 (breaks Debian 12, Ubuntu 22.04)** — GCC 14 in the `manylinux_2_39` build container emitted C23-versioned glibc symbols (`__isoc23_strtoll`, `__isoc23_sscanf`, etc.), making the wheel uninstallable on systems with glibc < 2.38. Downgraded to `manylinux_2_28` and added `-std=gnu11`/`-std=gnu++17` CFLAGS to suppress C23 symbol emission. (#588)
- **FFI memory leak** — `kreuzberg_free_result` was not freeing `djot_content_json`, `structured_output_json`, and `llm_usage_json` pointers.
- **R e2e embed tests fail** — generated R embedding config was missing the `type` discriminator field required by Rust's tagged enum deserialization.
- **Elixir parity test fails** — `ExtractionConfig` struct was missing the `:html_output` field.
- **Go LLM e2e tests fail** — `EmbeddingModelType` struct was missing `Llm` nested config, `ExtractionConfig` was missing `StructuredExtraction` field.
- **WASM tree-sitter build fails** — `tree-sitter-language-pack` 1.6.0 removed the `wasm` feature; removed stale feature gate from wasm32 target dependency.

---

## [4.8.4] - 2026-04-13

### Added

- **Helm chart for Kubernetes deployment** — minimal, security-hardened Helm chart with Deployment, Service, Ingress, PVC, HPA, PDB, and ServiceAccount templates. Publishes to GHCR as an OCI artifact. (#695)
- **Helm lint and kubeconform pre-commit hooks** — added `helm lint --strict` and `kubeconform` (k8s 1.28.0 schema validation) to pre-commit and CI pipeline.
- **Helm chart publish workflow** — new `publish-helm.yaml` GitHub Actions workflow pushes versioned chart to `oci://ghcr.io/kreuzberg-dev/charts`.

### Fixed

- **Helm chart: init container cannot chown as non-root** — the `init-cache` container needs root to `chown` the PVC mount. Added `securityContext.runAsUser: 0` to the init container.
- **Helm chart: unpinned busybox image tags** — pinned `busybox:latest` to `busybox:1.37-glibc` in init container and test pod for reproducibility.
- **Comrak bridge panics on multi-byte UTF-8 boundaries** — annotation byte offsets landing inside multi-byte characters (e.g. Cyrillic, `\u00ab\u00bb`) caused panics in `build_inlines()`. Snaps offsets to valid char boundaries using `ceil_char_boundary()`/`floor_char_boundary()`. (#696)

---

## [4.8.3] - 2026-04-12

### Fixed

- **ONNX session creation fails on Linux x86-64 with "graph_optimization_level is not valid"** — `GraphOptimizationLevel::Level3` maps to `ORT_ENABLE_LAYOUT` (value 3), only valid in ORT >= 1.21. The Linux wheel bundled ORT 1.20.1 due to a hardcoded version override in the publish workflow. Fixed by switching to `GraphOptimizationLevel::All` (ORT_ENABLE_ALL = 99, valid across all ORT 1.x) and aligning all ORT versions to 1.24.2 (matching ort-sys 2.0.0-rc.12). Also upgraded manylinux target from `manylinux_2_28` to `manylinux_2_35` to support the newer ORT binaries. (#683)

### Documentation

- **Documented AVX/AVX2 CPU requirement for ONNX Runtime features** — CPUs without AVX support (e.g. Intel Atom, Celeron N5105/Jasper Lake) cannot use PaddleOCR, layout detection, or embeddings. Added warning and system requirements entry to installation docs. (#691)

---

## [4.8.2] - 2026-04-10

### Added

- **`HtmlOutputConfig` typed in all bindings** — `html_output` config field (themes, CSS classes, embed CSS, custom CSS, class prefix) now fully typed in Python, TypeScript/Node, Go, Ruby, Elixir, PHP, Java, C#, R, and FFI. Previously only available in Rust core.

### Fixed

- **PDF: legitimate repeated content stripped during page merging regardless of `strip_repeating_text` flag** — `deduplicate_paragraphs()` in the PDF merge pipeline runs unconditionally after per-page extraction, removing consecutive identical paragraphs (≥5 chars) and non-consecutive body-text duplicates (≥15 chars) via HashSet dedup. This strips brand names and other legitimately repeated content even when `ContentFilterConfig.strip_repeating_text` is set to `false`. Gated both deduplication passes behind the `strip_repeating_text` flag so they are skipped when content filtering is disabled (#670, #681)
- **R package build failure** — R binding Cargo.toml version was stuck at 4.6.3 while core was at 4.8.1, causing tokio version resolution failure. Version sync script now includes the R native extension Cargo.toml.
- **CI: PyPI publish action failure** — pinned `pypa/gh-action-pypi-publish` to v1.13.0 (v1.14.0 has broken Docker image on GHCR)
- **E2E: Elixir generator emitted undefined `is_nan/1` function** — added helper function definition to the generated Elixir test helpers

---

## [4.8.1] - 2026-04-09

### Added

- **Styled HTML output** — New `HtmlOutputConfig` on `ExtractionConfig` with 5 built-in themes (`default`, `github`, `dark`, `light`, `unstyled`), semantic `kb-*` CSS class hooks on every structural element, CSS custom properties (`--kb-*`), custom CSS injection (inline or file), and configurable class prefix. The existing `Html` output format is upgraded in-place when `html_output` is set (#633, #665)
- 5 new CLI flags: `--html-theme`, `--html-css`, `--html-css-file`, `--html-class-prefix`, `--html-no-embed-css` — any flag implicitly sets `--content-format html`
- `HtmlOutputConfig` and `HtmlTheme` types exposed in Rust public API

### Changed

- **Vendored yake-rust 1.0.3** into kreuzberg core, removing external dependency
  - Fixes #676: `BacktrackLimitExceeded` panic on large files (10+ MB) by replacing regex-based sentence splitting with memchr-based approach
  - Expanded YAKE stopwords from 34 to 64 languages using kreuzberg's unified stopwords module
  - Removed 6 transitive dependencies (yake-rust, segtok, fancy-regex, streaming-stats, hashbrown, levenshtein)
- Styled HTML renderer included in the `html` feature (no separate `html-styled` feature gate)

### Fixed

- **PPTX: panic on non-char-boundary during page boundary recomputation** — byte offsets could land inside multi-byte UTF-8 characters (e.g. `…` U+2026), causing a panic when slicing content (#674)
- **PDF: `include_headers` / `include_footers` flags ignored by layout-model furniture stripping** — when a layout-detection model classified paragraphs as `PageHeader` or `PageFooter`, they were unconditionally stripped as furniture regardless of `ContentFilterConfig` flag values. Setting `strip_repeating_text=false` with `include_headers=true` now correctly preserves those regions (#670)
- **PDF: heuristic table detector misclassifies body text as tables on slide-like PDFs** — PowerPoint-exported PDFs with column-like text gaps produced false-positive 2–3 row "tables" whose bounding boxes covered the entire page, suppressing all body text from the structured extraction pipeline. Tables with ≤3 rows spanning >50% of the page height are now rejected as false positives
- **PPTX: `ImageExtractionConfig.inject_placeholders` silently ignored** — setting `inject_placeholders=false` now correctly suppresses `![alt](target)` image references in PPTX markdown output (#671, #677)
- **DOCX/HTML/DocBook/LaTeX/RST: `inject_placeholders` config ignored** — all extractors now honour `ImageExtractionConfig.inject_placeholders` to suppress image reference injection when set to `false`
- **PPTX public API cleanup** — `extract_pptx_from_path` and `extract_pptx_from_bytes` now accept `&PptxExtractionOptions` instead of 6 positional parameters

---

## [4.8.0] - 2026-04-08

### Added

- **Cross-extractor content filtering configuration** — New `ContentFilterConfig` on `ExtractionConfig` with `include_headers`, `include_footers`, `strip_repeating_text`, and `include_watermarks` flags. Controls header/footer/furniture inclusion across PDF, DOCX, RTF, ODT, HTML, EPUB, and PPT extractors. Typed in all bindings (Python, TypeScript, Ruby, Go, Elixir, PHP, Java, C#, WASM).
- **Local LLM support** via liter-llm 1.2 — use Ollama, LM Studio, vLLM, llama.cpp, LocalAI, or llamafile as VLM OCR, embedding, or structured extraction backends with zero API key configuration
- **LLM-powered document intelligence via liter-llm** — Integrates with 146 LLM providers (including local inference engines) for three new capabilities:
  - **VLM OCR**: Vision language models as OCR backend (OpenAI GPT-4o, Anthropic Claude, Google Gemini, etc.). Superior accuracy for low-quality scans, handwriting, Arabic/Farsi, and complex layouts. Configure via `ocr.backend = "vlm"` with `ocr.vlm_config`.
  - **Structured Extraction**: Extract structured JSON data from documents using a JSON schema constraint. Users provide a schema and optional Jinja2 prompt template; the LLM returns conforming data. Supports strict mode (OpenAI) with automatic schema sanitization for cross-provider compatibility.
  - **VLM Embeddings**: Provider-hosted embedding models (e.g., `openai/text-embedding-3-small`, `mistral/mistral-embed`) as alternative to local ONNX models. Works through existing `/embed` API, `embed_text` MCP tool, and `embed` CLI command.
- **New CLI command**: `kreuzberg extract-structured` for schema-guided LLM extraction
- **New API endpoint**: `POST /extract-structured` with multipart file upload
- **New MCP tool**: `extract_structured` for AI assistant integration
- **Minijinja template engine** for customizable LLM prompts — structured extraction supports `{{ content }}`, `{{ schema }}`, `{{ schema_name }}`, `{{ schema_description }}`; VLM OCR supports `{{ language }}`
- **5 new environment variables**: `KREUZBERG_LLM_MODEL`, `KREUZBERG_LLM_API_KEY`, `KREUZBERG_LLM_BASE_URL`, `KREUZBERG_VLM_OCR_MODEL`, `KREUZBERG_VLM_EMBEDDING_MODEL`
- `LlmConfig` and `StructuredExtractionConfig` types exposed in Python, Node.js, and PHP bindings
- `structured_output` field on `ExtractionResult` across all languages
- `structured_output_json` field in C FFI `CExtractionResult` struct
- `EmbeddingModelType::Llm` variant for provider-hosted embeddings
- VLM OCR registered as plugin backend in OCR registry
- Standalone text embedding API (#599, #614) with `/embed` endpoint, `embed_text` MCP tool, and `embed` CLI command

### Changed

- **License changed from MIT to Elastic License 2.0 (ELv2)** — copyright holder changed to Kreuzberg, Inc. Forked upstream crates (kreuzberg-paddle-ocr, kreuzberg-tesseract, kreuzberg-pdfium-render) retain their original MIT licenses.
- All `ExtractionResult` constructors refactored to use `..Default::default()` for forward compatibility
- Embed CLI command extended with `--provider llm` and `--model` flags
- Embed MCP tool extended with `model` and `api_key` parameters
- Extract CLI overrides extended with `--vlm-model`, `--vlm-api-key`, `--vlm-prompt`
- API returns 501 Not Implemented (instead of 500) when liter-llm feature is disabled
- JSON schema `additionalProperties` automatically stripped for non-OpenAI providers

### Fixed

- FFI error code tests updated for Embedding variant
- Flaky FFI string_intern tests serialized with `serial_test`
- TypeScript `NativeBinding` interface updated with `embedSync`/`embed` declarations
- E2E generator emits minimal `cfg` (no `any()` wrapper for single conditions)
- **PDF: brand names stripped by repeating text detection** — `ContentFilterConfig.strip_repeating_text = false` disables cross-page repeating text removal that incorrectly strips brand names from PowerPoint-exported decks (#667)
- **PPTX: slide order scrambled for decks with 10+ slides** — Fixed lexicographic sort of slide paths (`slide10.xml` before `slide2.xml`) to use numeric ordering (#669)
- **UTF-8 panic in arXiv watermark stripping** — `strip_arxiv_watermark_noise` panics when a multi-byte character spans the 6000-byte search limit. Fixed with `floor_char_boundary` (#663)
- **DOC: garbled text from old Word files** — CP1252 text misread as UTF-16LE when the fCompressed bit is unreliable. Added heuristic to detect and re-decode garbled output (#666)
- **WASM: table extraction returns empty array** — TypeScript validation silently drops tables when `pageNumber` is null. Fixed to default to page 0 (#655)

---

## [4.7.4] - 2026-04-06

### Added

- Re-added `--layout` boolean CLI flag for easy layout detection enablement (use `--layout` to enable with model defaults, `--layout false` to explicitly disable)
- arXiv watermark/sidebar noise filtering for academic PDFs — strips LaTeX sidebar identifiers from extracted text
- Second-tier cross-page repeating text detection — catches conference headers and journal running titles that repeat on >70% of pages but appear outside the margin zone
- Figure/picture text suppression — text inside layout-detected Picture regions is now marked as page furniture and excluded from body output

### Fixed

- **Figure-internal text leaking into body output** — Text from inside figures and diagrams (e.g., diagram labels, axis text) was incorrectly included in the extracted body content, sometimes promoted to headings. The layout detection pipeline now suppresses text paragraphs classified as Picture regions.
- CLI tests now correctly reference `--content-format` instead of deprecated `--output-format`
- **Empty image references in PDF markdown/HTML output** — PDFs with embedded images produced empty `![]()` references in markdown and `<img src="" alt="">` in HTML output. The PDF structure pipeline now extracts actual image pixel data via pdfium and populates document images, producing proper `![](image_N.png)` references.
- **Invalid `extractFromFile` config in documentation** — Demo code in the TypeScript API reference included invalid configuration parameters that caused runtime errors.
- **WASM build failure with `extern "C-unwind"`** — The LLVM WASM backend does not support `cleanupret` instructions generated by `extern "C-unwind"` FFI blocks. Added `ffi_extern!` macro that uses `extern "C-unwind"` on native targets (for C++ exception safety) and `extern "C"` on WASM.
- **Go module tag format** — Go module tags now use the correct `packages/go/v4/vX.Y.Z` format matching the module path in `go.mod`, plus the legacy `packages/go/vX.Y.Z` format for backwards compatibility. Backfilled tags for all stable releases.

### Changed

- CLI documentation updated with all missing extraction override flags (`--layout-table-model`, `--disable-ocr`, `--cache-namespace`, `--cache-ttl-secs`)

---

## [4.7.3] - 2026-04-05

### Fixed

- **Archive extraction SIGBUS crash on macOS ARM64** — ZIP, 7Z, TAR, and GZIP archive extraction crashed with SIGBUS (signal 10) in release builds due to miscompilation of unsafe code in `sevenz-rust2` and `zip` crates under `opt-level=3`. Reduced optimization level to 2 for these crates. This also fixes Elixir, R, Go, and C benchmark crashes when processing archive files.
- **Native-text PDF extraction fails when OCR backend unavailable** (#646) — PDFs with extractable native text hard-failed with `ParsingError: All OCR pipeline backends failed` when no OCR backend (PaddleOCR/Tesseract) was installed, even though pdfium already extracted text successfully. The automatic OCR quality-enhancement pass now gracefully falls back to the native extraction result when OCR backends are unavailable, emitting a warning instead of failing.
- **Elixir Logger pollutes stdout** — Elixir benchmark scripts produced `[debug] Initialized Kreuzberg.Plugin.Registry` on stdout, corrupting JSON output. Logger default handler now configured to write to stderr via `config :logger, :default_handler`.
- **WASM benchmark module resolution** — WASM benchmark script failed to load `@kreuzberg/wasm` through pnpm virtual store due to `import.meta.url` resolution issues in tsx. Changed to direct import from local build path.
- **CI: FFI-dependent tests fail when FFI build skipped** — Go, Elixir, R, C FFI, and CLI test jobs ran and failed when `build-ffi` was skipped by paths-filter. Added `needs.build-ffi.result == 'success'` guard.
- **Rust cannot catch foreign exceptions crash** (#606) — C++ exceptions from Tesseract or Leptonica (e.g. on corrupted images or edge-case inputs) propagated across the FFI boundary unhandled, causing `fatal runtime error: Rust cannot catch foreign exceptions, aborting`. All Tesseract/Leptonica FFI declarations now use `extern "C-unwind"` to allow foreign exceptions to unwind safely, and OCR processing is wrapped with `catch_unwind` to convert them to recoverable errors.

---

## [4.7.2] - 2026-04-04

### Added

- **E2E generator published mode** — `cargo run -p kreuzberg-e2e-generator -- generate --mode published --version <V>` generates standalone test apps against published registry versions (PyPI, npm, Maven, NuGet, crates.io, Hex, RubyGems). All 12 language generators now also produce their project/dependency files (pyproject.toml, package.json, composer.json, etc.).

### Changed

- **Global model cache** (#641) — Models now download to platform-appropriate global cache (`~/.cache/kreuzberg/` on Linux, `~/Library/Caches/kreuzberg/` on macOS, `%LOCALAPPDATA%/kreuzberg/` on Windows) instead of per-directory `.kreuzberg/` folders. Override with `KREUZBERG_CACHE_DIR` env var. Consolidates 7 duplicate cache-dir resolution implementations into a single `cache_dir::resolve_cache_dir()` function.

### Fixed

- **Embedded HTML in PDF text layers** — PDFs with raw HTML in their text layer (`<p>`, `<br />`, `<a href>`) produced escaped garbage (`\<p\>`) in output. Now detected and converted to clean markdown using `html-to-markdown-rs`, the same crate and config used by the HTML extractor. Comrak-generated `<!-- end list -->` comments also stripped from output.
- **Code classification false positives** — Layout model sometimes classified regular prose as Code blocks. Added a prose guard that rejects Code classification for text with sentence punctuation, low syntax density, and many words.
- **PageBreak rendering as `-----` separators** — PageBreak elements in InternalDocument were rendered as ThematicBreak (`-----`) in markdown and `<hr>` in HTML output. This polluted extraction output with separators that don't exist in the source document. PageBreak is now treated as structural metadata — paragraph breaks between elements provide sufficient page separation, matching the pdfium baseline behavior.
- **Leptonica DPI crash** (#606) — Images with resolution 0 DPI caused Leptonica preprocessing (background normalization, unsharp mask, grayscale conversion) to trigger a C++ exception that Rust cannot catch, aborting the process. Now validates and fixes DPI to 72 before preprocessing. Also disabled C++ exception handling on Windows MSVC builds (`/EHsc` removed).
- **Node.js `ExtractionResult.children` missing at runtime** — The `children` field was declared in TypeScript definitions but missing from the runtime NAPI object in the published v4.7.1 binary, causing parity test failures.
- **Layout detection fixture stale `preset` field** — E2E fixture `layout_detection.json` included removed `preset` field, causing Python test failures. Removed from fixture.
- **Node.js `disable_ocr` config not respected** — Setting `disableOcr: true` in the Node.js binding still produced OCR content for images instead of returning empty content.
- **C# `Serialization` class inaccessible** — Generated e2e tests referenced `Serialization` class with insufficient access level in the published NuGet package.
- **Java `PdfAnnotation` missing getters** — `getContent()` and `getPageNumber()` methods were missing from the Java record, causing parity test failures. Added JavaBean-style getters to match `getAnnotationType()` and `getBoundingBox()`.
- **Java `Table` missing getters** — `getCells()`, `getMarkdown()`, and `getPageNumber()` methods were missing from the Java record. Added JavaBean-style getters to match existing `getBoundingBox()`.
- **Go test_app module conflict** — Generated Go test_apps used the same module name as e2e/go, causing workspace conflicts. Published mode now uses a distinct module path.
- **PaddleOCR angle classification crash** (#643) — V2 angle classifier model (`PP-LCNet_x1_0_textline_ori`) expects `[N, 3, 80, 160]` input but preprocessing resized to `[N, 3, 48, 192]` (old mobile cls dimensions). Fixed input dimensions to match the v2 model.
- **Centralized concurrency controls** — Fixed 5 places bypassing `resolve_thread_budget()`: embeddings ONNX session (no thread config at all), image OCR (hardcoded 8 tasks), batch extraction fallback (`num_cpus * 1.5`), doc orientation (`.min(4)` cap), PaddleOCR BaseNet (`inter_threads` set to `num_thread` instead of `1`).
- **Chunk page numbers missing** (#636) — Chunks produced with `first_page: null, last_page: null` when chunking was configured without explicit `pages` config. Three fixes: (1) auto-enable page tracking when chunking is configured, so the PDF extractor always produces per-page boundaries; (2) improved page boundary recomputation with first-line fallback when exact content match fails due to rendering transformations; (3) allow zero-length boundaries for blank pages instead of failing validation.

---

## [4.7.1] - 2026-04-03

### Added

- **Tree-sitter grammar management CLI** — New `kreuzberg tree-sitter` subcommand with `download`, `list`, `cache-dir`, and `clean` sub-commands for managing tree-sitter grammar parsers. Supports downloading by language name, group (`--groups web,systems,scripting`), or all (`--all`). Reads `[tree_sitter]` config from `kreuzberg.toml` with `--from-config`.
- **Tree-sitter grammar management API** — New REST endpoints: `POST /grammars/download`, `GET /grammars/list`, `GET /grammars/cache`, `DELETE /grammars/cache` for programmatic grammar management.
- **Tree-sitter grammar management MCP tools** — New MCP tools: `download_grammars`, `list_grammars`, `grammar_cache_info`, `clean_grammar_cache` for AI assistant-driven grammar management.
- **Tree-sitter config startup initialization** — API and MCP servers auto-download tree-sitter grammars on startup when `[tree_sitter]` config specifies `languages` or `groups`.

### Changed

- **Normalized OCR+layout pipeline** — Tesseract+layout path now follows the same architecture as pdfium+layout: hOCR → PdfParagraph → `apply_layout_overrides` → `assemble_internal_document` → comrak. Replaces the broken custom `apply_layout_to_ocr_document` path that destroyed paragraph structure and reading order.
- **Elixir NIF crash protection** — All extraction and batch NIFs now wrapped with `catch_unwind` to prevent panics in native C libraries (pdfium, tesseract) from crashing the BEAM VM. Panics are caught and returned as `{:error, reason}` tuples with error-level tracing including backtraces.

### Fixed

- **hOCR parser depth tracking** — Fixed paragraph boundary detection in the hOCR parser that used a generic depth counter for `<p>`, `<span>`, and `<div>` tags. Closing tags from inner word spans could prematurely terminate a paragraph, causing content after that point to be silently dropped. Now uses tag-name-specific depth tracking.
- **hOCR multi-page content loss** — Per-page hOCR documents from tesseract always report `ppageno=0` (page=1), but the paragraph conversion filtered by the actual page index, silently dropping all content on pages 2+. Removed the per-page filter since each hOCR document is independently extracted per page.
- **OCR batch parallelization** — OCR page processing was hardcoded to 4 concurrent pages regardless of available CPUs. Now uses `resolve_thread_budget()` (auto-detects CPUs, capped at 8) for significantly faster multi-page document processing.
- **Benchmark workflow** — Removed reference to deleted `kreuzberg-extract` binary target.
- **Ruby OCR backend** — Added missing `ocr_internal_document` field to `ExtractionResult` construction.
- **Keyword extraction tests** — Updated test assertions to use new `extracted_keywords` field instead of deprecated `metadata.additional["keywords"]`.
- **PaddleOCR cache dir test** — Fixed test failure when `KREUZBERG_CACHE_DIR` environment variable is set by CI setup actions.
- **API `pdf_password` handler** — Added `#[cfg(feature = "pdf")]` gate to prevent compile error when `api` feature is enabled without `pdf`.
- **Chunking page boundary regression** (#636): Page boundaries were computed against raw extractor text but `result.content` uses rendered text with different byte lengths. Chunks now recompute boundaries from per-page content, fixing `first_page`/`last_page` being null and the "Page boundary byte_end exceeds text length" validation warning.
- **HF Hub environment variables** (#634): Use `ApiBuilder::from_env()` instead of `ApiBuilder::new()` for Hugging Face model downloads, respecting `HF_HOME` and `HF_ENDPOINT` environment variables. Fixes permission errors on Kubernetes when running as non-root.
- **PDF bridge tracing panic on multibyte characters** (#635): Use `.chars().take()` instead of byte indexing for `text_preview` in PDF structure bridge tracing, preventing panics on multibyte UTF-8 characters (e.g., `•`).
- **Go FFI struct layout** — vendored C header was missing `children_json` field, causing 8-byte offset shift. All FFI fields after `chunks_json` read wrong memory (e.g., `ocr_elements_json` read `mime_type` instead).
- **Java FFI struct layout** — `CExtractionResult` layout was missing `code_intelligence_json` field, causing `success` flag to read from wrong offset. All Java extractions returned `success=false`.
- **PHP `__get` magic method bypass** — six JSON fields (`elements`, `djotContent`, `document`, `ocrElements`, `children`, `uris`) returned raw JSON strings instead of deserialized arrays because `#[php(prop)]` intercepted property access before `__get`.
- **Ruby `disable_ocr` config** — `disable_ocr` keyword was not parsed in Ruby config handler, causing OCR to run even when explicitly disabled.
- **Node.js `ExtractionResult` parity** — `document`, `djotContent`, and `ocrElements` fields were `Option<Value>` which NAPI-RS omitted from JS objects when `None`. Changed to `Value` defaulting to `null`.
- **Node.js `convertChunk` missing `chunkType`** — TypeScript type converter did not forward the `chunk_type` field from NAPI bindings.
- **ODT caption text extraction** — text inside `draw:frame > draw:text-box > text:p` (e.g., image captions) was not extracted. The ODT extractor now recurses into text-box content.
- **OCR InternalDocument propagation** — `run_ocr_pipeline` discarded the structured InternalDocument built by `extract_with_ocr`, causing OCR results to fall back to naive `\n\n` paragraph splitting. Now propagated through the full pipeline.
- **OCR table cells** — OCR-detected tables (via TATR) had empty `cells` vectors, causing comrak to render them as paragraphs instead of proper tables. Now populated from the cell grid, matching the native text path fix.
- **OCR non-layout InternalDocument** — When layout detection is not active, the OCR path now builds an InternalDocument from results instead of returning None. Ensures structured output regardless of layout detection availability.
- **Italian/European PDF ligature corruption** — Extended contextual ligature repair to handle `tt`, `ti`, `tti` ligatures common in Italian fonts. Fixes garbled text like `Dire*ore` → `Direttore`, `ges:one` → `gestione`, `progeM` → `progetti`.
- **OCR layout false heading classification** — Tesseract+layout pipeline was worse than pure tesseract (33% vs 41% SF1) because layout confidence threshold was too low (0.5). Raised to 0.7 for OCR path where font-size validation is unavailable.
- **OCR table rendering** — OCR-detected tables were not linked to InternalDocument elements, causing comrak to skip them entirely. Tables now properly registered via `push_table()` with corresponding `ElementKind::Table` elements.
- **Spurious table detection** — Multi-column prose with short cells (like nougat_008) bypassed the prose row check due to a 30-char minimum row length. Lowered to 15 chars so short-cell prose tables are correctly rejected.
- **PHP enum registration** — PHP enums (ContentLayer, ElementType, etc.) were registered with `.class()` instead of `.enumeration()`, causing empty case lists. Virtual properties on ExtractionResult and ArchiveEntry now declared via builder modifiers for reflection visibility.
- **Go macOS FFI linking** — monorepo dev build (`ffi_dev.go`) was missing `-framework Foundation` in CGO LDFLAGS, causing linker failures on macOS with CoreML-enabled ONNX Runtime.
- **Unified WASM e2e tests** — replaced broken separate Deno/Workers e2e generators with a single vitest-based WASM generator. ORT-dependent features (embeddings, layout, paddle-ocr) gracefully skip.
- **WASM Rayon thread pool panic** — Rayon's `par_iter()` / `into_par_iter()` and `ThreadPoolBuilder::build_global()` panicked in WASM (`RuntimeError: unreachable`) because WASM has no threading support. All Rayon usages now fall back to sequential iteration on `wasm32` target.
- **PHP virtual property reflection** — `ClassBuilder::property()` declarations for `__get`-backed fields (metadata, chunks, document, etc.) shadowed the magic method, returning null. Replaced with getter methods that don't interfere with `__get`. Parity test updated to check both `hasProperty()` and getter methods.

---

## [4.7.0] - 2026-03-30

### Added

- **Semantic chunk labeling** (#600): Chunks now include a `chunk_type` field identifying the semantic nature of the content (e.g., `paragraph`, `heading`, `list_item`, `table_cell`, `code_block`). Supported across all 11 language bindings with updated E2E test parity.
- **Unified InternalDocument architecture**: All extractors now return a canonical `InternalDocument` with typed elements, relationships, images, and tables. Replaces format-specific intermediate representations.
- **Unified rendering layer**: New `new_markdown.rs` renderer produces CommonMark from `InternalDocument`, supporting headings, lists, tables, code blocks, formulas, footnotes, images, and inline annotations (bold, italic, links).
- **PDF structure pipeline**: Full rewrite of PDF extraction using `page.text().all()` for clean text, char-indexed font metadata for heading/bold detection, segment-based paragraph gap detection, and pdfium segment bounding boxes for precise paragraph regions.
- **Image extraction across 8 formats**: Embedded images now extracted as `ExtractedImage` with binary data, format, dimensions, and alt text. Supported for DOCX, PPTX, PDF, EPUB, ODT, HTML (data URIs), RTF (hex-decoded), and Markdown/MDX/Jupyter. Markdown output renders as `![alt](image_N.ext)` with binary data in `ExtractionResult.images`.
- **Recursive OCR on embedded images**: When OCR is configured, extracted images from EPUB, ODT, HTML, and RTF are processed through `process_images_with_ocr()`, producing nested `ExtractionResult` in `ExtractedImage.ocr_result`.
- **PDF watermark artifact filtering**: Uses pdfium's `/Artifact` content marks (PDF tagged content spec) to identify and filter watermark text from output.
- **Vertical table header reconstruction**: Detects and fixes rotated column headers in PDF tables where pdfium extracts characters as spaced single characters in reverse order (e.g., "y t i r o h t u A o N" → "NoAuthority").
- **Position-based page furniture detection**: Cross-page repeating text detection now uses actual page margins (top/bottom 10%) and page heights instead of word-count heuristics.
- **html-to-markdown v3 migration**: Switched to html-to-markdown v3 with unified `convert()` API returning `ConversionResult` (content, metadata, tables, images, document structure in a single call). Uses visitor-based table collection. hOCR module vendored as `table_core`.
- **Markdown ground truth for 336 documents**: Pandoc-generated GT across 10 formats (DOCX, HTML, RTF, PPTX, EPUB, ODT, XLSX, XLS, CSV, DOC) for structural quality benchmarking. All 371 markdown GT files cleaned of HTML remnants (415 tables converted to GFM pipe tables, 28 inline tags fixed).
- **Benchmark quality scoring improvements**: Content normalization for HTML blocks in markdown scoring, Image↔Paragraph and Table↔ListItem type compatibility, `correct` field in `QualityMetrics`, HTML detection in ground truth validation.
- **Benchmark harness overhaul**: Per-format SF1/TF1 aggregation, noise detection (10 heuristics for HTML remnants, garbled text, broken tables, page artifacts), diagnostic diff mode (`--diagnose`), JSON output (`--json-output`), ground truth validation subcommand (`validate-gt`). Comprehensive tracing across all extractors and the rendering layer.
- **Markdown ground truth for 23 formats**: 350+ benchmark fixtures across CSV, DOCX, HTML, EPUB, LaTeX, RST, RTF, PPTX, ODT, XLSX, XLS, OPML, ORG, JATS, IPYNB, FictionBook, DocBook, Typst, DOC, PPT, and more. GT generated via pandoc and verified against source documents.
- **OpenWebUI integration**: Kreuzberg serves as a document extraction backend for Open WebUI chat interfaces.
- **URI extraction**: New `Uri` type with `UriKind` classification (Hyperlink, Image, Anchor, Citation, Reference, Email) extracted from 20+ document formats. URIs are always-on, deduplicated by (url, kind) pair, and capped at 100k per document. Available in `ExtractionResult.uris`.
- **Recursive email attachment extraction**: EML/MSG/PST attachments are now recursively extracted as `ArchiveEntry` children using the same pattern as archive extractors. Nested `message/rfc822` parts also extracted as children. Respects `max_archive_depth`.
- **PDF embedded file extraction**: PDF file attachments (portfolios) are now recursively extracted as `ArchiveEntry` children via lopdf. Includes filename sanitization, decompression size limits, and name tree depth guards.
- **PDF bookmark/outline extraction**: Document outlines (bookmarks) extracted as URIs — page destinations as `UriKind::Anchor`, external links as `UriKind::Hyperlink`.
- **DOCX/PPTX embedded object extraction**: OLE objects and embedded files from `word/embeddings/` and `ppt/embeddings/` directories are now recursively extracted as children.
- **PPTX hyperlink extraction**: Hyperlinks from slide XML (`<a:hlinkClick>` in run properties) now resolved via relationship files and extracted as URIs.
- **Image path resolution for markup formats**: When using `extract_file()`, relative image paths in Markdown, MDX, LaTeX, RST, OrgMode, Typst, Djot, and DocBook are resolved from the filesystem and extracted as `ExtractedImage` data. OS-agnostic with path traversal prevention.
- **Unified image OCR pipeline stage**: Image OCR moved from per-extractor calls to a single pipeline stage after derivation. All extracted images (including path-resolved markup images) are now OCR'd uniformly when OCR is configured. Concurrency limited to 8 concurrent tasks.
- **FictionBook image and link extraction**: Base64-encoded `<binary>` images and `<a>` hyperlinks now extracted from FB2 documents.
- **Apple iWork extractor improvements**: Numbers outputs tables instead of paragraphs, Keynote has improved slide structure, Pages has heading detection. All three extract metadata from ZIP plist.
- **`code_intelligence` field on ExtractionResult**: Top-level access to tree-sitter `ProcessResult` with full structure, imports, exports, chunks, symbols, diagnostics, and docstrings. Previously only available inside `FormatMetadata::Code` metadata.
- **`CodeContentMode` config**: Control code extraction content mode -- `chunks` (semantic TSLP chunks, default), `raw` (source as-is), `structure` (headings + docstrings only). Configured via `TreeSitterProcessConfig.content_mode`.
- **TSLP semantic chunking for code**: Code files bypass the text-splitter entirely. TSLP's `CodeChunks` (function/class-aware) map directly to kreuzberg `Chunk`s with semantic types and heading context.
- **Cross-format output parity tests**: 36 tests verifying Markdown, HTML, Djot, and Plain produce equivalent text content. GFM lint validation, bracket escaping checks, structural block comparison.
- **HTML input markdown passthrough**: HTML files extracted as Markdown now use html-to-markdown output directly via `pre_rendered_content`, bypassing the lossy InternalDocument to comrak round-trip.

### Code Intelligence

- **Tree-sitter integration** for 248 programming languages via [tree-sitter-language-pack](https://github.com/kreuzberg-dev/tree-sitter-language-pack)
  - Extract functions, classes, imports, exports, symbols, docstrings, diagnostics
  - Syntax-aware code chunking
  - Language detection from file extension and shebang
  - Dynamic grammar download (native) / 30-language static subset (WASM)
  - New `tree-sitter` and `tree-sitter-wasm` feature flags (included in `full` and `wasm-target`)
  - `TreeSitterConfig` and `TreeSitterProcessConfig` in `ExtractionConfig`
  - Re-exported TSLP types (`ProcessResult`, `StructureItem`, `FileMetrics`, etc.)
  - [TSLP documentation](https://docs.tree-sitter-language-pack.kreuzberg.dev)

### Typed Metadata

- New `FormatMetadata` variants: `Code`, `Csv`, `Bibtex`, `Citation`, `FictionBook`, `Dbf`, `Jats`, `Epub`, `Pst`
- Extended `PptxMetadata` with `image_count` and `table_count`
- Migrated deprecated `metadata.additional` writes to typed fields across all extractors
- Strong types for all new metadata variants across all 11 language bindings

### Breaking Changes

- **Layout detection preset removed**: The `preset` field on `LayoutDetectionConfig` has been removed across all bindings. Layout detection now uses the RT-DETR v2 model unconditionally — no "fast" vs "accurate" distinction. The `--layout-preset` CLI flag is removed. Old configs with `"preset": "..."` are silently ignored for backward compatibility.
- **Table model config typed**: `table_model` on `LayoutDetectionConfig` changed from `Option<String>` to a `TableModel` enum (`tatr`, `slanet_wired`, `slanet_wireless`, `slanet_plus`, `slanet_auto`, `disabled`). Defaults to `tatr`. String values still accepted in JSON/TOML configs.

### Fixed

- **PDF table rendering**: Populate `Table.cells` from TATR/SLANeXT grid so comrak renders proper Table nodes instead of wrapping markdown in a Paragraph. Table SF1 improved from 15.5% to 53.7%.
- **Markdown GFM quality**: Enable `prefer_fenced` for code blocks, un-escape brackets/parens (`\[` to `[`), fix code block language spacing in djot.
- **Semantic HTML output**: Enable `github_pre_lang` and `full_info_string` for code blocks with `class="language-X"`.
- **Djot text normalization**: Shared `normalize_inline_text()` for consistent whitespace handling. MD-to-Djot TF1 now 1.0000.
- **PDF structural extraction quality**: Improved heading detection (font-size-ratio H2/H3 differentiation, section numbering patterns, ALL-CAPS detection, paragraph-to-heading rescue pass), table discrimination (reject multi-column prose misclassified as tables via flow-through detection, row-count/column-count ratio, and table quality validation), list detection (multi-token prefix patterns), image scoring (normalize image block matching), and formula detection (math character density heuristic). Layout SF1 improved from 40.7% to 43.7% across 157 verified PDF fixtures.
- **PDF ground truth verified**: All 157 PDF benchmark fixtures verified using vision (rendered page images vs GT markdown). 7 broken Mistral OCR GTs with hallucinated content replaced with vision-verified markdown.
- **LaTeX extraction**: Convert `\href`, `\emph`, `\textbf`, `\textgreater`, `\verb`, `\sout`, blockquotes, lists, special characters, and typographic ligatures to markdown.
- **XLSX/XLS sheet name headings**: Emit `## SheetName` heading before each sheet's table, matching pandoc convention.
- **OPML outline headings**: All outline nodes now emit headings at appropriate depth, not just parent outlines. Inline HTML in text attributes converted to markdown.
- **IPYNB heading detection**: Markdown cells now detect ATX headings and emit proper heading elements. Code cell outputs (stdout, execute_result) included in extraction.
- **JATS abstract and references**: Abstract section with sub-headings now included. References rendered as numbered list with structured citation formatting.
- **ODT formula extraction**: Embedded MathML formula objects extracted as formula content instead of empty image placeholders. Image alt text and captions now extracted from `draw:frame` elements.
- **PPTX slide titles**: Title placeholders detected via OOXML placeholder type and emitted as H2 headings. Bulleted/numbered lists in slides extracted with proper ListStart/ListEnd wrapping.
- **ORG source blocks**: `#+BEGIN_SRC` blocks converted to fenced code blocks with language annotation. `#+BEGIN_EXAMPLE` blocks converted to unfenced code blocks. Inline code `~text~` converted to backtick spans. Paragraph line wrapping joined.
- **RST heading levels**: Overline+underline document titles assigned H1. Code block language hints preserved from `.. highlight::` and `.. code::` directives. `::` literal block shorthand handled.
- **RTF formatting**: Bold/italic/strikethrough formatting now uses exact byte offsets from a unified text+formatting extraction pass, eliminating bold bleeding across paragraphs. Hidden text (`\v`) suppressed. Hyperlink field parsing fixed. Strikethrough support added. Table row rendering fixed for multi-row tables. Ordered list detection from `\listtext` markers.
- **HTML preprocessing**: Navigation elements, forms, and sidebars now stripped by default. Previously disabled, causing page chrome to appear in extraction output.
- **PDF table detection**: Reject false table detections where >70% of cells contain single-word fragments (justified prose incorrectly classified as multi-column table).
- **DocBook root element handling**: XML fragments without a root element now wrapped automatically, fixing extraction of multi-element DocBook files.
- **FictionBook poem support**: Verse lines (`<v>`), subtitles, text-author, and date elements within poem blocks now extracted. Heading levels aligned with pandoc conventions.
- **PDF image FlateDecode fallback**: When `decode_flate_to_png()` fails for FlateDecode, CCITT, or JBIG2 streams, images are now re-extracted via pdfium's bitmap rendering pipeline, producing valid PNG output instead of unusable raw bytes (#615).
- **Metadata standardization**: Metadata from PPTX, Excel, ODT, RST, OrgMode, Typst, RTF, JATS, DOC, PPT, HTML, Email, BibTeX, and Citation extractors now mapped to standard `Metadata` struct fields (title, authors, dates, keywords, language) instead of only `additional` map.
- **MDX link parity with Markdown**: Links and annotations in headings and list items now extracted (was silently dropped).
- **RST hyperlink extraction**: Inline hyperlinks (`` `text <url>`_ ``) and reference targets now extracted.
- **LaTeX `\url{}` extraction**: `\url{...}` commands now extracted as URIs alongside `\href`.
- **OrgMode image detection**: Added .webp, .bmp, .tiff, .avif to recognized image extensions.
- **BibTeX URI classification**: URL fields now correctly classified as Hyperlink (was Citation). Entry title used as label instead of BibTeX key.
- **JATS title field**: Article title now stored in `metadata.title` (was only in `subject`).
- **PDF bookmark stack safety**: Sibling traversal converted from recursion to iterative loop preventing stack overflow on wide outlines.
- **PDF embedded file security**: Filename sanitization (strip directory components), decompressed size limit (50MB), name tree depth limit (50 levels).

- **Tesseract C++ exception crash** (#606): Fixed fatal runtime error where C++ exceptions from Tesseract unwound through Rust FFI frames, triggering `std::terminate()`. Now compiles Tesseract with `-fno-exceptions` on macOS, Linux, and MinGW. The Tesseract CLI executable target (which uses `try`/`catch`) is patched out of CMakeLists.txt at build time since only the library is needed.

- **ExtractionConfig rejects unknown fields**: `#[serde(deny_unknown_fields)]` added to `ExtractionConfig`. Previously, typos or invalid fields (e.g., `layout_analysis` instead of `layout`) were silently ignored.
- **RTF delimiter space consumption**: Fixed space-in-word bug where font encoding directives (`\loch`, `\hich`, `\dbch`) caused spaces mid-word ("H eading" → "Heading"). Root cause: RTF spec requires consuming trailing delimiter space after control words.
- **PPTX markdown mode**: Derive plain/markdown mode from `output_format` config instead of hardcoding `plain=true`. Tables now render as markdown tables, lists get bullet markers, text elements get newline separation.
- **EPUB test compilation**: Added `InternalDocument::content()` method and fixed `epub_spine_semantics_tests` to use it instead of removed `.content` field.
- **HTML extraction rewrite**: Replaced ~400-line manual HTML tag parser with html-to-markdown v3's `DocumentStructure` mapping. Single-pass conversion eliminates CSS/script content leakage and `[image: X]` placeholder artifacts.
- **Chunking heading context with plain output**: Fixed `heading_context` always returning `None` when using plain text output format. The markdown chunker now receives the original markdown for heading map building even when content is rendered as plain text.
- **WASM build compatibility**: Inlined workspace-inherited fields (`version`, `edition`, `authors`) in kreuzberg-wasm Cargo.toml because wasm-pack 0.14.0 cannot resolve `field.workspace = true` references.
- **Pre-commit hooks**: Fixed rumdl hook config (use `rumdl-fmt` from official repo), wasm build (feature-gate layout config access), kreuzberg-node build (missing `formatted_content` field), broken relative links in READMEs and CHANGELOG.
- **Binding compilation**: Added missing `formatted_content` field to kreuzberg-py and kreuzberg-php binding crates.
- **PDF heading body_size_guard**: Narrowed guard range from `≤ body+0.5` to `body±1.5pt` so headings well below body font size (e.g., 8pt in 12pt body) pass through.
- **RTF table extraction**: Fixed critical bug where table cell content was written to both result string and TableState, causing cells to appear as individual lines instead of proper markdown tables.
- **DOCX merged cells**: Repeat content across gridSpan (horizontal) and vMerge (vertical) spans. Added `source_path` field to `ExtractedImage` for DOCX image relationship paths.
- **DOCX formatting**: Merge adjacent runs with identical formatting to prevent spurious `****` sequences. Strip `<u>` underline HTML tags.
- **Python wheel `__isoc23_strtoll` error on older Linux distributions** (#588): Downgraded the Linux build environment `manylinux` target from `manylinux_2_39` to `manylinux_2_28` for pre-compiled Python wheels to ensure compatibility with systems using glibc versions prior to 2.39 (e.g., Ubuntu 20.04/22.04, Debian 11/12).
- **`clear_ocr_backends` now fully clears the registry**: Calls `shutdown_all()` instead of `reset_to_defaults()`, so the backend list is empty after clearing as expected by the API contract.
- **Go macOS link failure**: Added missing `-framework Foundation` to CGO LDFLAGS. ORT's CoreML provider uses Foundation for NSLog/NSFileManager, causing undefined symbol errors on macOS.
- **Tesseract Windows MinGW build (Elixir/Go/C FFI publish)**: CMake resolved bare `g++` to MSVC `cl.exe` on CI runners with both toolchains. Added `resolve_mingw_compiler()` to find absolute paths from MSYS2 subsystem dirs. Bumped Tesseract cache key to invalidate stale MSVC-compiled artifacts.
- **Windows GNU ORT linking**: `bundled` strategy on Windows GNU now uses dynamic linking with pre-downloaded Microsoft ORT (pyke.io has no static binaries for `x86_64-pc-windows-gnu`). Documented ONNX Runtime DLL requirement for Go, Elixir, and C/C++ on Windows.

### Changed

# Building the `xberg` R package on r-universe

The org universe lives at **https://xberg-io.r-universe.dev**, driven by the
registry repo **`xberg-io/xberg-io.r-universe.dev`** (`packages.json` lists the
`xberg` and `htmltomarkdown` R packages, each at `subdir: packages/r`, branch
`main`).

`htmltomarkdown` builds and publishes there today. **`xberg` does not yet** — it
needs the three changes below. r-universe builds each package as an **isolated
source tarball** (`R CMD build` over `packages/r/` only), with **no network and
no access to the rest of the monorepo**, on stock runners without OCR/ML system
libraries.

## Required changes (do these together; they are interdependent)

1. **Feature set — align with the WASM binding.**
   `packages/r/src/rust/Cargo.toml` now depends on the core as
   `xberg = { …, default-features = false, features = ["wasm-target"] }`
   (matching `crates/xberg-wasm`). `wasm-target = ["no-ort-target",
   "excel-wasm", "tree-sitter-wasm", "ocr-wasm"]` — i.e. **no** ONNX/ORT,
   Paddle, candle, layout-detection, embeddings, reranker, or transcription,
   none of which compile on r-universe runners.

2. **Regenerate the binding with `wasm-target`'s exclusions.**
   `src/rust/src/lib.rs` is alef-generated and currently references ORT-only
   types (`EmbeddingBackend`, `LayoutDetection`, `NerConfig`, `RerankerBackend`,
   …) that `wasm-target` removes, so it will **not compile** against the slim
   feature set until regenerated. Apply the same type/function exclusions the
   WASM backend uses (see `alef.toml` `wasm-target` exclusions) to the R binding
   and run the R generator. **Do this as part of the active xberg regeneration**
   — a standalone feature swap breaks the build.

3. **Vendor the Rust dependencies.**
   r-universe builds offline, so the core crate + all transitive crates must be
   bundled into the tarball. Mirror `html-to-markdown/packages/r/src/rust/`,
   which ships a vendored core copy + a `vendor/` tree + a `.cargo/config.toml`
   redirect. Generate with `rextendr::vendor_pkgs()` (or the repo's vendor step)
   and commit the result. The current R package has **none** of these and
   path-deps `../../../../crates/xberg`, which does not exist in the isolated
   tarball (`failed to read crates/xberg/Cargo.toml`).

## Verifying

After the above, confirm a clean isolated build locally:

```sh
R CMD build packages/r          # produces xberg_<ver>.tar.gz with no monorepo paths
R CMD INSTALL xberg_<ver>.tar.gz
```

Then push; r-universe rebuilds automatically and the package appears at
`https://xberg-io.r-universe.dev/xberg`. Watch the build at
`https://github.com/r-universe/xberg-io` ("Build package/xberg").

## Note on `Makevars`

`alef.toml [crates.r]` adds `HEIF_LIBS`/`extra_pkg_libs` for libheif. With
`wasm-target` (no `heic`), that linkage is unnecessary and the prelude can be
dropped once the slim build is confirmed.

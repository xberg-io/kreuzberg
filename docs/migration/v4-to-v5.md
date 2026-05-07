# Migrating from v4 to v5

v5 is a major release that removes pdfium as a PDF backend and promotes `pdf_oxide` — a pure-Rust PDF library — as the sole backend. The migration is mostly mechanical.

## Breaking Changes

### PDF backend removed

**v4**: kreuzberg used `pdfium-render` (a C library wrapper) for PDF extraction, page rendering, and image extraction. The crate `kreuzberg-pdfium-render` was a workspace member.

**v5**: kreuzberg uses `pdf_oxide` exclusively — a pure-Rust implementation. No native library is required.

### Removed: `PdfBackend` enum

```rust
// v4 — no longer exists
use kreuzberg::PdfConfig;
let config = PdfConfig {
    backend: PdfBackend::Pdfium,
    ..Default::default()
};

// v5 — backend field removed, pdf_oxide is always used
let config = PdfConfig::default();
```

### Removed: pdfium feature flags

| v4 feature | v5 replacement |
|------------|---------------|
| `bundled-pdfium` | removed — no binary to bundle |
| `static-pdfium` | removed |
| `system-pdfium` | removed |
| `pdf-oxide` | removed — merged into `pdf` |
| `pdf` | `pdf` (unchanged, now always uses pdf_oxide) |

**Update your `Cargo.toml`:**

```toml
# v4
kreuzberg = { version = "4", features = ["bundled-pdfium", "pdf"] }

# v5
kreuzberg = { version = "5", features = ["pdf"] }
```

### Removed: environment variables

- `KREUZBERG_PDFIUM_PATH` — no longer needed
- `DYLD_LIBRARY_PATH` / `LD_LIBRARY_PATH` pdfium staging — no longer needed

### Removed: CLI flags

The `--pdf-extract-images` and `--pdf-extract-metadata` flags that were previously only available with `bundled-pdfium` / `static-pdfium` features are now always available when `pdf` feature is active.

## Installation Changes

### v4

PDF support required bundling or installing a native pdfium binary:

```toml
kreuzberg = { version = "4", features = ["bundled-pdfium"] }
# OR
kreuzberg = { version = "4", features = ["system-pdfium"] }
```

### v5

PDF support is fully pure Rust — add the `pdf` feature and you're done:

```toml
kreuzberg = { version = "5", features = ["pdf"] }
```

No system library installation. No binary download. Works on any Rust target that Cargo supports.

## Docker Changes

v4 Docker images bundled the pdfium native library. v5 Docker images no longer need it — pdf_oxide is compiled into the binary.

If your Dockerfile copies or mounts a pdfium library, remove those steps.

## API Changes Summary

| Area | v4 | v5 |
|------|----|----|
| `PdfConfig.backend` | `PdfBackend::Pdfium` or `PdfBackend::PdfOxide` | removed (always pdf_oxide) |
| `PdfBackend` enum | exists | removed |
| `render_pdf_page_to_png` | uses pdfium renderer | uses pdf_oxide renderer |
| Image extraction | requires bundled-pdfium | always available with `pdf` feature |
| Password-protected PDFs | pdfium `open_with_password` | `authenticate(&[u8])` internally |

## Behavior Changes

- **Text extraction**: Generally equivalent. pdf_oxide may produce slightly different whitespace handling in edge cases — test your documents.
- **Image extraction**: Binary image data (JPEG, pixel data) is now extracted via pdf_oxide's native API.
- **Page rendering**: `render_pdf_page_to_png` now uses pdf_oxide's tiny-skia renderer. Output is visually equivalent but pixel-perfect identical output is not guaranteed.
- **Password-protected PDFs**: Pass passwords via `PdfConfig.passwords` as before — the API is unchanged.

## Quick Migration Checklist

- [ ] Remove `bundled-pdfium`, `static-pdfium`, `system-pdfium`, `pdf-oxide` from `features` in `Cargo.toml`
- [ ] Remove `PdfBackend` references from your code
- [ ] Remove `KREUZBERG_PDFIUM_PATH` from CI/environment
- [ ] Remove pdfium binary download/staging from CI
- [ ] Remove `libpdfium.dylib` / `libpdfium.so` / `pdfium.dll` handling from deployment scripts
- [ ] Update Docker images — remove pdfium layer
- [ ] Test PDF extraction on your documents

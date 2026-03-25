---
description: "security limits dos protection"
priority: critical
---

## priority: critical

# Security Limits & DoS Protection

## Overview

Defense-in-depth DoS protection via `SecurityLimits` and validator helpers in `crates/kreuzberg/src/extractors/security.rs`. All archive and complex format extractors MUST use these.

## SecurityLimits Struct

| Field | Default | Purpose |
|-------|---------|---------|
| `max_archive_size` | 500 MB | Uncompressed archive size limit |
| `max_compression_ratio` | 100:1 | Zip bomb detection threshold |
| `max_files_in_archive` | 10,000 | Archive file count limit |
| `max_nesting_depth` | 100 | Structure nesting limit |
| `max_entity_length` | 32 | XML entity length limit |
| `max_content_size` | 100 MB | String growth per document |
| `max_iterations` | 10M | Loop iteration limit |
| `max_xml_depth` | 100 | XML nesting depth |
| `max_table_cells` | 100K | Table cell count limit |

Access via `config.security_limits.clone().unwrap_or_default()`.

## Validators

### ZipBombValidator (archives)

```rust
let limits = config.security_limits.clone().unwrap_or_default();
let validator = ZipBombValidator::new(limits);
validator.validate(&mut archive)?;  // Checks ratio, size, file count
```

### StringGrowthValidator (content accumulation)

```rust
let mut validator = StringGrowthValidator::new(limits.max_content_size);
validator.check_append(text.len())?;  // Call before each append
content.push_str(&text);
```

### DepthValidator (nesting)

```rust
let mut depth = DepthValidator::new(limits.max_nesting_depth);
depth.push()?;  // Entering nested structure
// ... process ...
depth.pop();     // Exiting
```

### IterationValidator (loops)

```rust
let mut iter = IterationValidator::new(limits.max_iterations);
for item in collection {
    iter.check_iteration()?;
}
```

### TableValidator (spreadsheets/tables)

```rust
let mut validator = TableValidator::new(limits.max_table_cells);
validator.add_cells(rows * cols)?;
```

## When to Apply

| Format Family | Required Validators |
|--------------|-------------------|
| Archives (ZIP/TAR/7z/GZIP) | `ZipBombValidator` before extraction |
| Office XML (DOCX/PPTX/ODT) | `DepthValidator` + `StringGrowthValidator` |
| XML/HTML | `DepthValidator` + `StringGrowthValidator` |
| Spreadsheets (XLSX/ODS) | `TableValidator` + `StringGrowthValidator` |
| Any loop-heavy processing | `IterationValidator` |

## Critical Rules

1. **NEVER skip** security validation for user-provided content
2. **Always default** if `config.security_limits` is `None`
3. **Validate BEFORE extraction** (fail fast)
4. Errors return `KreuzbergError::validation(msg)`
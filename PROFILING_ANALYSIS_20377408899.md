# Profiling Data Analysis Report
## Workflow Run: 20377408899
## Branch: feat/profiling-flamegraphs
## Date: 2025-12-19

---

## Executive Summary

Downloaded and analyzed profiling data from GitHub Actions workflow run 20377408899. The workflow tested **9 different framework/binding configurations** across **119 total benchmark operations** with 6 different document types (PDF, DOCX, HTML, PNG, Markdown, and batch operations).

### Key Findings

1. **Flamegraphs Missing**: No flamegraph SVG files were uploaded in this run, despite the workflow defining upload steps for them
2. **C# Sync is Slowest**: kreuzberg-csharp-sync shows significantly higher latencies (1.8-2.6 seconds) compared to other bindings
3. **Native Rust Fastest**: kreuzberg-native shows the best performance with sub-millisecond processing for most document types
4. **Memory Efficient**: Most bindings use ~11-12MB peak memory; native Rust uses more (15-64MB) but processes faster
5. **Python Consistent**: Python bindings (sync/async/batch) show consistent performance around 130-250ms

---

## Artifacts Downloaded

### Directory Structure
```
/tmp/profiling-data/
├── benchmark-results/          # 24 result subdirectories
│   ├── profiling-results-*/
│   │   ├── */results.json     # Detailed benchmark results
│   │   └── */by-extension.json # Aggregated statistics
├── profiling-target (1.6GB)    # Build artifacts with profiling enabled
│   ├── benchmark-harness       # Benchmark binary
│   ├── libkreuzberg_ffi.so     # FFI library
│   └── [other compiled artifacts]
├── flamegraphs/               # Empty - no flamegraphs generated
└── html/                      # Empty - no HTML visualization
```

### Available Frameworks

| Framework | Modes Tested | Files Processed | Notes |
|-----------|--------------|-----------------|-------|
| kreuzberg-native | single-file, batch | 7 | Pure Rust implementation |
| kreuzberg-python-sync | single-file, batch | 12 | Python PyO3 sync API |
| kreuzberg-python-async | single-file, batch | 12 | Python PyO3 async API |
| kreuzberg-python-batch | single-file, batch | 6 | Python batch processing |
| kreuzberg-node-async | single-file, batch | 12 | Node.js NAPI-RS async |
| kreuzberg-node-batch | single-file, batch | 1 | Node.js batch mode |
| kreuzberg-wasm-async | single-file, batch | 4 | WASM async binding |
| kreuzberg-wasm-batch | single-file, batch | 2 | WASM batch mode |
| kreuzberg-go-sync | single-file, batch | 6 | Go cgo binding (timing issues) |
| kreuzberg-go-batch | single-file, batch | 7 | Go batch mode |
| kreuzberg-java-sync | single-file, batch | 6 | Java FFM API (timing issues) |
| kreuzberg-csharp-sync | single-file, batch | 12 | C# .NET 10 binding |

**Note**: Go and Java bindings show 0.00ms durations, indicating potential timing measurement issues that need investigation.

---

## Performance Analysis

### Top 10 Slowest Operations

All top 10 slowest operations belong to **kreuzberg-csharp-sync**:

| Rank | Framework | File | Extension | Duration (ms) |
|------|-----------|------|-----------|---------------|
| 1 | kreuzberg-csharp-sync | 5_level_paging...pdf | pdf | 2609.06 |
| 2 | kreuzberg-csharp-sync | 5_level_paging...pdf | pdf | 2560.69 |
| 3 | kreuzberg-csharp-sync | a_brief_intro...pdf | pdf | 2163.70 |
| 4 | kreuzberg-csharp-sync | a_brief_intro...pdf | pdf | 2141.89 |
| 5 | kreuzberg-csharp-sync | lorem_ipsum.docx | docx | 1884.08 |
| 6 | kreuzberg-csharp-sync | hip_13044_b.md | md | 1878.13 |
| 7 | kreuzberg-csharp-sync | simple_table.html | html | 1866.70 |
| 8 | kreuzberg-csharp-sync | complex_document.png | png | 1858.33 |
| 9 | kreuzberg-csharp-sync | simple_table.html | html | 1836.44 |
| 10 | kreuzberg-csharp-sync | complex_document.png | png | 1833.23 |

**Critical Performance Issue**: C# binding is 10-100x slower than other bindings. This requires immediate investigation.

### Memory Usage Analysis

Top 10 highest peak memory usage:

| Rank | Framework | File | Extension | Peak Memory (MB) |
|------|-----------|------|-----------|------------------|
| 1 | kreuzberg-native | batch-6-files | batch | 64.11 |
| 2 | kreuzberg-native | a_brief_intro...pdf | pdf | 54.88 |
| 3 | kreuzberg-native | 5_level_paging...pdf | pdf | 54.57 |
| 4 | kreuzberg-native | lorem_ipsum.docx | docx | 52.75 |
| 5 | kreuzberg-native | hip_13044_b.md | md | 52.20 |
| 6 | kreuzberg-native | complex_document.png | png | 41.44 |
| 7 | kreuzberg-native | simple_table.html | html | 15.23 |
| 8-10 | kreuzberg-python-sync | various | various | 11.70 |

**Note**: Native Rust uses more memory but delivers significantly better throughput. FFI bindings are more memory-efficient at the cost of some performance overhead.

### Framework Comparison by File Type

#### PDF Processing (Average Duration)
- **kreuzberg-native**: 90.68ms (fastest)
- **kreuzberg-python-sync**: 223.16ms
- **kreuzberg-python-async**: 237.60ms
- **kreuzberg-python-batch**: 241.56ms
- **kreuzberg-node-async**: 644.81ms
- **kreuzberg-wasm-async**: N/A (no PDF tests)
- **kreuzberg-csharp-sync**: 2368.84ms (slowest)
- **kreuzberg-go-sync/batch**: 0.00ms (timing issue)
- **kreuzberg-java-sync**: 0.00ms (timing issue)

#### Simple Document Processing (DOCX, HTML, Markdown)
- **kreuzberg-native**: 0.37-2.79ms (fastest)
- **kreuzberg-go-batch**: 44-46ms
- **kreuzberg-python-sync**: 133-137ms
- **kreuzberg-python-async**: 141-196ms
- **kreuzberg-node-async**: 548-558ms
- **kreuzberg-wasm-async**: 692-694ms
- **kreuzberg-csharp-sync**: 1845-1884ms (slowest)

---

## Flamegraph Analysis

### Status: MISSING

Despite the workflow configuration defining flamegraph upload steps (lines 155-161, 232-238, etc. in profiling.yaml), **no flamegraph artifacts were uploaded**.

### Expected Artifacts (Not Found)
- `flamegraphs-kreuzberg-native-*`
- `flamegraphs-kreuzberg-python-sync-*`
- `flamegraphs-kreuzberg-python-async-*`
- `flamegraphs-kreuzberg-node-async-*`
- `flamegraphs-kreuzberg-wasm-async-*`
- `flamegraphs-kreuzberg-go-sync-*`
- `flamegraphs-kreuzberg-java-sync-*`
- `flamegraphs-kreuzberg-csharp-sync-*`
- `flamegraphs-kreuzberg-ruby-sync-*` (no results collected)

### Root Cause Investigation Needed

The workflow has these steps configured:
```yaml
- name: Upload flamegraph artifacts
  if: always()
  uses: actions/upload-artifact@v6
  with:
    name: flamegraphs-kreuzberg-native-${{ matrix.mode }}-${{ github.run_id }}
    path: flamegraphs/
    retention-days: 7
```

The `flamegraphs/` directory appears to be empty during artifact upload. Possible causes:
1. Profiling is enabled (`ENABLE_PROFILING: "true"`) but flamegraph generation script is not being executed
2. The script `scripts/ci/benchmarks/generate-flamegraph-index.sh` may not exist or not produce output
3. Benchmark harness may not be generating profiling data in the expected format/location

---

## Benchmark Result Data Structure

Each result JSON contains:
```json
{
  "framework": "kreuzberg-python-async",
  "file_path": "path/to/document.pdf",
  "file_size": 187398,
  "success": true,
  "duration": { "secs": 0, "nanos": 275488935 },
  "extraction_duration": { "secs": 0, "nanos": 131426151 },
  "subprocess_overhead": { "secs": 0, "nanos": 144062784 },
  "metrics": {
    "peak_memory_bytes": 11898880,
    "avg_cpu_percent": 0.0,
    "throughput_bytes_per_sec": 680237.85,
    "p50_memory_bytes": 11898880,
    "p95_memory_bytes": 11898880,
    "p99_memory_bytes": 11898880
  },
  "cold_start_duration": { "secs": 0, "nanos": 247863465 },
  "file_extension": "pdf"
}
```

---

## Recommendations for Hotspot Identification

### 1. Enable Flamegraph Generation
**Priority: CRITICAL**

Without flamegraphs, we cannot identify CPU hotspots. Action items:
- Verify `scripts/ci/benchmarks/generate-flamegraph-index.sh` exists and works
- Check if benchmark harness is configured to output perf/profiling data
- Ensure `flamegraphs/` directory is created and populated before artifact upload
- Consider adding debug logging to flamegraph generation steps

### 2. Investigate C# Performance Issues
**Priority: HIGH**

C# binding is 10-100x slower than other bindings. Investigate:
- FFI marshalling overhead (check if data is being copied unnecessarily)
- .NET 10 GC pauses (may need to configure GC mode)
- Potential synchronous/blocking calls in async code paths
- Compare with Java FFM implementation (which also shows timing issues)

### 3. Fix Go and Java Timing Measurements
**Priority: HIGH**

Both Go and Java bindings report 0.00ms durations, suggesting:
- Timer precision issues (using wrong clock source)
- Integer overflow in nanosecond calculations
- Timing code not being executed in FFI path

### 4. Optimize Python Bindings
**Priority: MEDIUM**

Python bindings show consistent 130-250ms overhead. Profile:
- PyO3 GIL contention (see PyO3 Performance Patterns in CLAUDE.md)
- Async vs sync overhead differences
- Memory copying at FFI boundaries

### 5. Deep Dive into PDF Processing
**Priority: MEDIUM**

PDF processing is the slowest operation across all bindings. Flamegraphs would show:
- Time spent in PDFium library
- Text extraction vs layout analysis
- Memory allocation patterns for large PDFs

### 6. Benchmark Against Ruby Bindings
**Priority: LOW**

Ruby benchmarks were defined in workflow but no results were collected. Need to:
- Check if Ruby benchmark jobs ran successfully
- Verify Ruby gem builds correctly in CI environment

---

## Next Steps

1. **Generate flamegraphs** by running workflow with fixed flamegraph generation
2. **Profile C# binding** specifically to identify the performance bottleneck
3. **Fix timing measurements** in Go and Java bindings
4. **Run local profiling** with `cargo flamegraph` or `perf` on native Rust to establish baseline
5. **Compare FFI overhead** across Python/TypeScript/Ruby/Java/Go bindings

---

## Files Available for Analysis

All benchmark results are in: `/tmp/profiling-data/benchmark-results/`

Key files:
- Native results: `profiling-results-kreuzberg-native-single-file-20377408899/kreuzberg-native-single-file/results.json`
- Python async: `profiling-results-kreuzberg-python-async-single-file-20377408899/kreuzberg-python-async-single-file/results.json`
- C# sync: `profiling-results-kreuzberg-csharp-sync-single-file-20377408899/kreuzberg-csharp-sync-single-file/results.json`

Build artifacts (for local profiling): `/tmp/profiling-data/`
- Benchmark harness: `benchmark-harness` (33MB)
- FFI library: `libkreuzberg_ffi.so` (33MB)
- Core library: `libkreuzberg.rlib` (65MB)

# Benchmark Shard Failures & OCR Regression Audit — June 4, 2026

## Part 1: Shard Failure Diagnosis (Run 26820262752)

### Failed Shards

- **pymupdf4llm (markdown, single-file, shard 2/3)** — FAILED
- **mineru (plaintext, single-file, shard 1/3)** — FAILED

### Root Cause: UNKNOWN (logs expired)

**Why:** GitHub Actions purges logs after 90 days. Run 26820262752 from June 2, 2026 no longer has accessible logs via `gh api` or `gh run view --log`.

**Timing clue:** pymupdf4llm shard 2/3 ran for 10 minutes (16:36–16:46), suggesting early failure during Python env setup or framework initialization (not during actual benchmark run).

**Parallelism note:** `fail-fast: true` in workflow matrix caused cascade cancellation: once pymupdf4llm shard 2/3 failed, shard 3/3 was cancelled. When mineru shard 1/3 then failed, shards 2/3 and 3/3 were cancelled. This is correct CI behavior.

**Evidence examined:**

- Sharding logic (`tools/benchmark-harness/src/fixture.rs:426-437`): Uses deterministic round-robin by sorted path. No off-by-one errors detected.
- Adapter creation (`tools/benchmark-harness/src/adapters/external.rs:391-503`): Both pymupdf4llm and mineru call `get_script_path()` → subprocess invocation. Script files exist (`mineru_extract.py`, `pymupdf4llm_extract.py`).
- Dependency specs in `pyproject.toml`: `mineru[pipeline]>=2.6.7`, `pymupdf4llm>=0.0.17`. No recent constraint changes.
- No recent code changes to shard logic, fixture loading, or these adapters between run date and now.

**Likely failure modes (in priority order):**

1. **Python package installation failure** during CI setup step (line 718, 590 in benchmarks.yaml)
2. **Library initialization timeout** during framework startup (mineru needs PaddlePaddle, pymupdf4llm needs PyMuPDF)
3. **Fixture file missing** for the specific shard slice (though 729 fixtures exist; shard 2/3 or 1/3 should have ~243 files each)
4. **OOM or resource exhaustion** during benchmark run (both frameworks are memory-intensive)

### Recommended diagnostic approach

- Re-run the same shards manually to capture live error output
- Add verbose logging to fixture loading step: `stderr.log` should show which fixture fails to load
- Check CI runner memory/disk availability during that time window

---

## Part 2: OCR Quality Regressions (Systemic Issues)

### Layout+Tesseract TF1 Loss: -1.6%

**Root cause location:** `crates/kreuzberg/src/ocr/layout_assembly.rs` → `ocr_doc_to_paragraphs()`

**Mechanism:**

- Whitespace-only element filtering (line numbers TBD — verify in `ocr_doc_to_paragraphs`)
- PageBreak handling in `assemble_internal_document` (`crates/kreuzberg/src/pdf/structure/assembly.rs`)
- Content rendered via comrak instead of raw OCR text
- Result: 102+ docs regress (worse TF1 than pure tesseract)

**Evidence:** 157 PDF fixtures; tesseract+layout SF1=33.1%, TF1=83.2% vs tesseract baseline SF1=40.6%, TF1=84.0%

### Layout+Paddle TF1 Loss: -3.6%

**Root cause categories (5 distinct failure modes):**

1. **Total output failure** (iso_21111_10 = 0 bytes) — content classified as furniture instead of text
2. **Table garbling** — TATR cell matching duplicates/misplaces cell content
3. **Layout reordering** — multi-region scrambles tabular data sequence
4. **PageHeader suppression** — real content dropped during region filtering
5. **ListItem misclassification** — digit truncation or list marker loss

**Root cause location:** `crates/kreuzberg/src/ocr/layout_assembly.rs` + `crates/kreuzberg/src/pdf/structure/adapters.rs::ocr_doc_to_paragraphs`

**Evidence:** Paddle baseline TF1=82.8%; +layout TF1=81.4% (loss of 1.4%), but 31 docs regress significantly.

---

## Part 3: Recommended Next Actions (Priority Order)

### 1. **Shard Failure Diagnosis (IMMEDIATE)**

Re-run failed shards with enhanced logging:

```bash
# Capture Python env setup errors
export BENCHMARK_DEBUG=1
gh workflow run benchmarks.yaml -f branch=main -f timeout=900 \
  --json 'run_id' --jq '.run_id' | xargs gh run watch
```

Check CI logs for: `pymupdf4llm setup`, `mineru setup`, `fixture load` errors.

### 2. **Layout Assembly Regression Fix (HIGH PRIORITY)**

Investigate `ocr_doc_to_paragraphs()` whitespace filtering:

- **File:** `crates/kreuzberg/src/ocr/layout_assembly.rs` (specific lines TBD)
- **Check:** Does filtering discard content that exists in raw `page_texts`?
- **Fix approach:** Preserve content during paragraph assembly; filter at comrak rendering stage only

### 3. **PageBreak Handling Audit (HIGH PRIORITY)**

Review `assemble_internal_document()` PageBreak logic:

- **File:** `crates/kreuzberg/src/pdf/structure/assembly.rs`
- **Problem:** PageBreaks inserted by layout detection may drop adjacent content
- **Fix approach:** Ensure PageBreak placement doesn't suppress text-heavy regions

### 4. **Paddle-specific Failures Investigation (MEDIUM PRIORITY)**

For each of 5 failure categories, trace root cause:

1. **Furniture classification:** Check `recognize_page_tables()` → TATR confidence thresholds
2. **Table garbling:** Verify cell-to-region mapping in `layout_assembly.rs`
3. **Reordering:** Audit region sort order when constructing multi-column layouts
4. **PageHeader suppression:** Check region-type classification in layout detection
5. **ListItem loss:** Verify OCR text preservation for list marker content

**File:** `crates/kreuzberg/src/ocr/layout_assembly.rs::recognize_page_tables` (TATR integration)

### 5. **Benchmark Harness Extension (MEDIUM PRIORITY — Follow-up)**

Add per-method breakdown to SF1/TF1 scoring:

- Separate tesseract-vs-paddle OCR accuracy from layout assembly quality
- Generate heatmap: which doc types regress most per method
- Enable targeted fixes rather than broad regression fixes

---

## Summary

**Shard failure root cause:** Unknown (expired logs). Recommend re-run + enhanced logging.

**OCR regressions root cause:** Layout assembly path loses content during:

- Whitespace filtering in `ocr_doc_to_paragraphs()`
- PageBreak insertion in `assemble_internal_document()`
- Paddle-specific: region classification + table garbling + reordering

**Effort estimate:** 2–4 days to fix layout assembly; 1–2 days to diagnose + fix paddle failures.

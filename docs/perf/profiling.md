# Performance Profiling Workflow

Reproducible flamegraph-driven workflow for Xberg PDF performance work. The infrastructure (pprof, the `--profile-dir` harness flag, the `task benchmark:profile` command) already exists; this page codifies how to use it as the entry gate for any optimization.

## When to use

**Mandatory** before any code change you intend to call a "performance optimization." A flamegraph proves the function you're about to touch actually consumes meaningful CPU time. Speculative optimization without a flamegraph is forbidden — two iterations have been rejected on this repo for that exact reason.

## Generate a flamegraph

```bash
# 1. Build with debug symbols. The default release profile strips them,
#    so the flamegraph fills with raw addresses and `__mh_execute_header`.
cargo build --profile profiling -p xberg-cli --features full
cargo build --profile profiling -p benchmark-harness --features profiling

# 2. Run the harness with --profile-dir (note: pipeline-benchmark, not compare).
SHA=$(git rev-parse --short HEAD)
mkdir -p flamegraphs/$SHA
target/profiling/benchmark-harness pipeline-benchmark \
  --fixtures tools/benchmark-harness/fixtures \
  --paths baseline \
  --doc pdf \
  --profile-dir flamegraphs/$SHA \
  --json-output bench/profile-baseline-$SHA.json
```

The Taskfile wrapper:

```bash
task benchmark:profile FRAMEWORK=xberg PIPELINE=baseline OUTPUT_FORMAT=plaintext MODE=batch
```

…**works for the SF1 portion** but currently builds `--release` not `--profile profiling`, so the resulting SVGs only have system-symbol resolution. Until the task definition is fixed, drive flamegraph generation manually with the explicit commands above. (Tracked as a follow-up; don't optimize against a release-stripped flamegraph.)

What this does:

- Builds `xberg-cli` in release mode with `--features all`.
- Runs the pipeline against the corpus at `tools/benchmark-harness/fixtures/`.
- Captures CPU samples at 1000 Hz via the pprof wrapper in `tools/benchmark-harness/src/profiling.rs`.
- Writes `flamegraphs/<short-sha>/<pipeline>-<format>-<mode>.svg`.

Open the SVG in any browser; the flamegraph is interactive (click to zoom, search by symbol).

### Pipeline + format choices

| Pipeline     | Use it for                                               |
| ------------ | -------------------------------------------------------- |
| `baseline`   | Pure PDF text path — no layout, no OCR. Cleanest signal. |
| `layout`     | RT-DETR + layout-for-markdown overhead.                  |
| `paddle-ocr` | Full OCR path including PaddleOCR.                       |

`OUTPUT_FORMAT=plaintext` skips the markdown classify/assembly pass (`use_layout_for_markdown=false`, no font-clustering hierarchy). Use it as the most stripped-down benchmark — what's hot here is the floor of "raw extraction" cost.

`OUTPUT_FORMAT=markdown` exercises the full structure pipeline. Use it when investigating heading/table classification or the rendering pass.

`MODE=batch` runs multiple PDFs concurrently — better for steady-state CPU measurement. `MODE=single-file` measures latency one document at a time; useful when investigating tail latency.

## Reading flamegraphs

- **Width = total time** (self + children). Wide functions at the bottom of a stack are not necessarily hotspots — they're often just the entry point. Look at _self time_ (the visible non-child width).
- **Tall stacks** mean deep call chains; they're not problems unless the leaf is hot.
- **Repeated narrow towers** in different stacks are good candidates — the same function called from many places, each contributing a thin slice.
- Filter out `pdf_oxide`, `image`, `tokio`, and system libraries (`libc`, `libpthread`) — those are dependencies. Focus on `xberg::*` symbols.

## Memory profiling

Build with `--features memory-profiling` to enable jemalloc heap dumps. The harness's `dump_heap_profile()` writes a `.heap` file alongside the flamegraph. Use sparingly — CPU is almost always the bottleneck.

## Per-iteration commit policy

For each accepted optimization, commit:

- `flamegraphs/<commit>/before.svg` — flamegraph showing the hotspot **before** the change.
- `flamegraphs/<commit>/after.svg` — flamegraph showing the hotspot reduced or moved **after** the change.

This lets reviewers verify the change actually touched the function it claimed to touch.

---

## Iteration protocol — accept/reject gate

The protocol below is enforced for every perf candidate. Skipping a step has cost the project two reverted iterations.

### Before any code change

1. Generate flamegraph on current HEAD (`task benchmark:profile`).
2. Identify the top-15 self-time `xberg::*` functions. Filter out `pdf_oxide` / `image` / `tokio` / system libs.
3. Pick **one** candidate. Document:
   - File:line of the function.
   - Approximate self-time percentage.
   - The shape of the optimization (e.g., "replace `Vec::clone` with `Cow::Borrowed`").

### Implement (subagent — `performance-engineer`)

4. Make the smallest change that addresses the hotspot. No surrounding cleanup, no helper extractions, no scope creep (`minimal-changes` rule).

### Verify (main agent — never trust subagent ACCEPT verdicts without independent measurement)

5. `cargo build -p xberg --features full` — zero warnings.
6. `cargo clippy -p xberg --tests -- -D warnings` — clean.
7. `cargo test -p xberg` — green.
8. Re-run the harness:

   ```bash
   target/release/benchmark-harness compare \
     --fixtures tools/benchmark-harness/fixtures \
     --pipelines baseline \
     --json-output bench/iter-N.json
   ```

9. Compare `bench/iter-N.json` vs the most recent committed baseline JSON in `bench/`:
   - **SF1 must NOT regress** (any drop > 0.1pt → revert).
   - Lift on at least one of `{p50 ms/MB, p95 ms/MB, peak RSS}` ≥ 3% → **accept**.
   - Lift < 3%, no regression → mark commit body `marginal`, accept.
   - Regression on perf metrics → **revert**.

10. Generate post-fix flamegraph; commit `before.svg` + `after.svg` + the iter-N JSON.

### Stopping condition (curve flattens)

- **Three consecutive REJECT or "marginal" accepts** → stop. The optimization curve has flattened; further work is not worth the engineering cost.
- Aggregate p95 plaintext ms/MB within 20% of `pandoc` (the best-tuned competitor on plaintext) → stop.

## Failure-mode reminders

- **Don't trust subagent ACCEPT verdicts without measuring.** A `performance-engineer` agent has previously declared an optimization accepted with a correctness regression baked in. Always rerun the test suite locally after the agent reports done.
- **Behavior probes catch what F1 doesn't.** F1 metrics aggregate; a small regression on a corner case can wash out. When the optimization touches text-shape code (whitespace, escape, punctuation), write a 4-input mini-test asserting exact byte equality vs the unoptimized version.
- **Cache invalidation.** When you change the Xberg crate, rebuild _both_ `xberg-cli` and `benchmark-harness` (the harness links the crate in-process for `compare`). A build that "finished in 1.10s" without recompiling the Xberg crate is a sign the change wasn't picked up.

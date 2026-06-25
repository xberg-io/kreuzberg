# generate-test-fixtures

Deterministic fixture-generation toolkit for xberg integration tests.

Produces real on-disk DOCX / ODT / XLSX / PPTX / PDF documents that exercise
track-changes / revisions / comments / incremental-update / diff / security
code paths in `xberg::extract` and `xberg::diff::compare`. Every
binary fixture is paired with a `<stem>.gt.json` ground-truth sidecar that
integration tests load to assert structured expectations.

The generated fixtures fill the gap left by `test_documents/`, whose existing
~200 real-world corpus does not contain track-changes, comments, incremental
xref chains, or paired diff inputs.

## Layout

```text
tools/generate_test_fixtures/
  pyproject.toml
  src/generate_test_fixtures/
    __init__.py
    __main__.py            argparse entry point
    gt_schema.py           GroundTruth dataclass + JSON writer
    docx_revisions.py      DOCX w:ins / w:del / w:rPrChange fixtures
    odt_revisions.py       ODT text:tracked-changes fixtures
    xlsx_revisions.py      XLSX xl/revisions/revisionHeaders.xml fixtures
    pptx_comments.py       PPTX ppt/comments/comment{N}.xml fixtures
    pdf_incremental.py     PDF base + incremental xref chain fixtures
    diff_pairs.py          paired v1/v2 inputs for xberg::diff::compare
    security_fixtures.py   DDE / oversized embed / zip-bomb fixtures
  tests/
    test_generation.py     smoke test: each generator runs + GT JSON parses
```

## Usage

From the xberg repo root:

```bash
uv run --directory tools/generate_test_fixtures \
    python -m generate_test_fixtures all
```

Or per format:

```bash
uv run --directory tools/generate_test_fixtures \
    python -m generate_test_fixtures docx odt xlsx pptx pdf diff-pairs security
```

Default output: `test_documents/generated/<format>/<stem>.{ext,gt.json}`.
Override with `--output-dir <PATH>` (resolved relative to the cwd).

## Ground-truth schema

See `src/generate_test_fixtures/gt_schema.py`. Every sidecar is a JSON object
of the shape:

```json
{
  "fixture_path": "test_documents/generated/docx/docx_track_changes_basic.docx",
  "format": "docx",
  "feature": "revisions",
  "expectations": { ... feature-specific shape ... },
  "generated_by": "generate-test-fixtures 0.1.0"
}
```

## Determinism

Every generator pins timestamps to fixed ISO-8601 strings (no `now()`), uses
hardcoded author names, and seeds any randomness with `random.Random(42)`.
Re-running the generator on the same source code produces byte-identical
outputs except for the ZIP archive container's mtime — which the generators
override to `2024-01-01T00:00:00Z` via `zipfile.ZipInfo`.

## Why not check binaries in?

The user owns the call on whether these belong in the `test_documents/` git
submodule. The generator scripts are committed; the binary outputs are not.
The integration test scaffold (`crates/xberg/tests/`) is marked
`#[ignore]` until the binaries land.

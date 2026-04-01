#!/usr/bin/env bash
# Generate markdown ground truth for formats requiring LibreOffice conversion.
# Workflow: soffice → intermediate format → pandoc -t gfm → sanitize
#
# Prerequisites:
#   - soffice (LibreOffice) on PATH
#   - pandoc on PATH
#   - python3 on PATH
#
# Usage: bash tools/benchmark-harness/scripts/generate_libreoffice_gt.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
SANITIZE="$SCRIPT_DIR/sanitize_pandoc_gt.py"
TMP_DIR="/tmp/gt_convert"

mkdir -p "$TMP_DIR"

# --- DOC → DOCX → GFM ---
echo "=== DOC ground truth generation ==="
mkdir -p "$REPO_ROOT/test_documents/ground_truth/doc"

doc_files=(
  "$REPO_ROOT/test_documents/vendored/unstructured/doc/simple.doc"
  "$REPO_ROOT/test_documents/vendored/unstructured/doc/fake.doc"
  "$REPO_ROOT/test_documents/vendored/unstructured/doc/duplicate-paragraphs.doc"
  "$REPO_ROOT/test_documents/vendored/unstructured/doc/fake-doc-emphasized-text.doc"
  "$REPO_ROOT/test_documents/doc/unit_test_lists.doc"
)

for f in "${doc_files[@]}"; do
  if [ ! -f "$f" ]; then
    echo "  SKIP (not found): $f"
    continue
  fi
  name=$(basename "$f" .doc)
  gt_md="$REPO_ROOT/test_documents/ground_truth/doc/${name}.md"

  # Convert to docx via LibreOffice
  soffice --headless --convert-to docx --outdir "$TMP_DIR" "$f" 2>/dev/null
  converted="$TMP_DIR/${name}.docx"

  if [ -f "$converted" ]; then
    pandoc -f docx -t gfm --wrap=none "$converted" 2>/dev/null |
      python3 "$SANITIZE" >"$gt_md"
    size=$(wc -c <"$gt_md")
    echo "  doc: $name → $size bytes  ($gt_md)"
  else
    echo "  doc: $name FAILED conversion"
  fi
done

# --- PPT → PPTX → GFM ---
echo ""
echo "=== PPT ground truth generation ==="
mkdir -p "$REPO_ROOT/test_documents/ground_truth/ppt"

ppt_files=(
  "$REPO_ROOT/test_documents/ppt/simple.ppt"
)

for f in "${ppt_files[@]}"; do
  if [ ! -f "$f" ]; then
    echo "  SKIP (not found): $f"
    continue
  fi
  name=$(basename "$f" .ppt)
  gt_md="$REPO_ROOT/test_documents/ground_truth/ppt/${name}.md"

  soffice --headless --convert-to pptx --outdir "$TMP_DIR" "$f" 2>/dev/null
  converted="$TMP_DIR/${name}.pptx"

  if [ -f "$converted" ]; then
    pandoc -f pptx -t gfm --wrap=none "$converted" 2>/dev/null |
      python3 "$SANITIZE" >"$gt_md"
    size=$(wc -c <"$gt_md")
    echo "  ppt: $name → $size bytes  ($gt_md)"
  else
    echo "  ppt: $name FAILED conversion"
  fi
done

# --- ODS: no pandoc support for spreadsheet input ---
echo ""
echo "=== ODS: skipped (pandoc cannot read spreadsheet formats) ==="
echo "  Existing text GT in test_documents/ground_truth/ods/ is sufficient."

echo ""
echo "Done. Validate with:"
echo "  cargo run --release -p benchmark-harness -- validate-gt --fixtures tools/benchmark-harness/fixtures/doc/"
echo "  cargo run --release -p benchmark-harness -- validate-gt --fixtures tools/benchmark-harness/fixtures/"

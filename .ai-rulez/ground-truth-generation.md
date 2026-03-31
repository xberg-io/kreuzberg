# Ground Truth Generation Process

## Overview

Ground truth (GT) markdown files are used to benchmark the structural quality (SF1) of our document extraction. GT must be **independent** of our extractor — we use pandoc as the primary generation tool, then clean up artifacts.

## Pipeline

### Step 1: Generate from pandoc

```bash
pandoc <source_file> -t gfm --wrap=none -o <gt_file.md>
pandoc <source_file> -t plain --wrap=none -o <gt_file.txt>
```

- **GFM** (GitHub Flavored Markdown) produces pipe tables instead of grid tables
- `--wrap=none` prevents line wrapping

### Step 2: Automated artifact removal (sed)

```bash
sed -i '' 's/ {#[^}]*}//g' "$file"   # Remove {#id} attributes
sed -i '' 's/ {[^}]*}//g' "$file"     # Remove {.class} attributes
sed -i '' '/^:::/d' "$file"            # Remove fenced div markers
sed -i '' 's/\\\$/$/g' "$file"         # Unescape dollar signs
sed -i '' "s/\\\\'/'/g" "$file"        # Unescape quotes
```

### Step 3: Manual cleanup (subagent or human review)

Per-format issues that sed can't fix:

| Format | Common Issues |
|--------|--------------|
| DOCX | HTML tables (colspan/rowspan) → convert to pipe tables |
| EPUB | Heavy HTML divs/spans → strip, keep text content |
| RTF | Empty header rows in tables → promote first data row |
| ODT | `<span id="anchor">` in headings → strip |
| HTML | Raw HTML passthrough → convert to markdown |
| FB2 | `<div class="section">` wrappers → strip |

**Rules for cleanup:**

1. Convert ALL HTML to markdown equivalents where possible
2. For features markdown can't represent (colspan, rowspan), put content in first cell, leave others empty
3. Remove `<!-- -->` comments
4. Strip `<u>`, `<sup>`, `<sub>` tags (keep text content)
5. Convert `<img>` to `![alt](src)`
6. Collapse 3+ consecutive blank lines to 2
7. Never use our own extractor output as GT

### Step 4: Create fixture JSON

Each GT file needs a fixture at `tools/benchmark-harness/fixtures/<format>/<name>.json`:

```json
{
  "document": "relative/path/to/source.ext",
  "file_type": "docx",
  "file_size": 12345,
  "expected_frameworks": ["kreuzberg"],
  "metadata": { "description": "...", "source": "pandoc-generated" },
  "ground_truth": {
    "text_file": "relative/path/to/gt.txt",
    "markdown_file": "relative/path/to/gt.md",
    "source": "pandoc"
  }
}
```

## Supported Formats

Pandoc supports these input formats that we have test documents for:

**High priority:** docx, html, rtf, xlsx, csv, pptx, epub, odt
**Medium:** fb2, ipynb, org, opml, rst, tsv, docbook
**Low:** typst, latex, djot, ris, jats, asciidoc, commonmark

**Not supported by pandoc:** doc, xls, eml, msg, pdf, images, archives, yaml, toml, ods

For unsupported formats:

- **doc/xls**: Convert via libreoffice first, then pandoc
- **CSV with non-UTF8**: Our extractor handles encoding detection; verify GT matches
- **PDF**: Separate GT process (vision-based or manual)

## Running the Benchmark

```bash
./target/release/benchmark-harness pipeline-benchmark \
  -f tools/benchmark-harness/fixtures --paths baseline
```

The benchmark computes SF1 (structural F1) by comparing markdown blocks (headings, paragraphs, lists, tables, code, images) between our output and the GT.

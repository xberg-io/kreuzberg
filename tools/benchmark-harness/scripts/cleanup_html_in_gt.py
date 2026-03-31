#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.10"
# dependencies = ["beautifulsoup4"]
# ///
"""Convert HTML tags in ground-truth markdown files to GFM markdown.

Processes files under test_documents/ground_truth/ and converts all HTML
to GitHub-Flavored Markdown:
  - <table>/<tr>/<td>/<th>  → GFM pipe tables
  - <b>/<strong>             → **text**
  - <i>/<em>                 → *text*
  - <a href="url">text</a>  → [text](url)
  - <br>/<br/>               → newline
  - <hr>/<hr/>               → ---
  - <sup>/<sub>              → text (no GFM equivalent)
  - <div>/<span>/<p>         → stripped (content kept)
  - <code>                   → `text`
  - <pre>                    → fenced code block
  - <img src alt>            → ![alt](src)
  - any other tags           → stripped, content kept

Fenced code blocks (``` ... ```) are left untouched.
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

from bs4 import BeautifulSoup, Tag


# ---------------------------------------------------------------------------
# Table conversion using BeautifulSoup
# ---------------------------------------------------------------------------

def _cell_text(cell: Tag) -> str:
    """Extract plain text from a <td>/<th> cell, collapsing whitespace."""
    # Recursively get text, preserving inner newlines as spaces
    text = cell.get_text(separator=" ")
    # Collapse runs of whitespace (spaces, tabs) but keep intentional newlines
    # that come from multiline cell content — just normalise them to spaces
    text = re.sub(r"[ \t]+", " ", text)
    text = re.sub(r"\n+", " ", text)
    return text.strip()


def _table_to_gfm(table_tag: Tag) -> str:
    """Convert a <table> tag (bs4 Tag) into a GFM pipe table string."""
    rows: list[list[str]] = []

    # Collect all <tr> rows, handling nested tables by only taking direct
    # children at each level (nested tables are treated as cell text).
    for tr in table_tag.find_all("tr", recursive=True):
        # Skip rows that belong to a nested table
        parent = tr.parent
        while parent and parent != table_tag:
            if parent.name == "table":
                break
            parent = parent.parent
        if parent != table_tag:
            continue

        cells: list[str] = []
        for cell in tr.find_all(["td", "th"], recursive=False):
            cells.append(_cell_text(cell))
        if cells:
            rows.append(cells)

    if not rows:
        return ""

    # Determine column count (max across all rows)
    col_count = max(len(r) for r in rows)

    # Pad all rows to the same width
    padded = [r + [""] * (col_count - len(r)) for r in rows]

    # Build GFM table
    lines: list[str] = []

    # Header row (first row)
    header = padded[0]
    lines.append("| " + " | ".join(header) + " |")
    lines.append("| " + " | ".join(["---"] * col_count) + " |")

    # Data rows
    for row in padded[1:]:
        # Escape pipe characters inside cells
        escaped = [c.replace("|", "\\|") for c in row]
        lines.append("| " + " | ".join(escaped) + " |")

    return "\n".join(lines)


def convert_tables(text: str) -> tuple[str, int]:
    """Find all HTML table blocks and replace them with GFM pipe tables.

    Returns (converted_text, number_of_tables_converted).
    HTML tables may span multiple lines (cell content contains newlines).
    We use a simple stack-based scanner to extract complete <table>…</table>
    regions, then parse each with BeautifulSoup.
    """
    count = 0
    result_parts: list[str] = []
    pos = 0
    text_len = len(text)

    # Regex to find the start of a <table ...> tag (case-insensitive)
    table_open_re = re.compile(r"<table(?:\s[^>]*)?>", re.IGNORECASE)
    table_close_re = re.compile(r"</table>", re.IGNORECASE)

    while pos < text_len:
        m_open = table_open_re.search(text, pos)
        if not m_open:
            # No more tables — keep remaining text as-is
            result_parts.append(text[pos:])
            break

        # Append text before this table
        result_parts.append(text[pos : m_open.start()])

        # Find the matching </table> by counting nesting
        depth = 1
        scan_pos = m_open.end()
        while depth > 0 and scan_pos < text_len:
            m_close = table_close_re.search(text, scan_pos)
            m_next_open = table_open_re.search(text, scan_pos)

            if m_close is None:
                # Malformed: no closing tag — leave rest as-is
                result_parts.append(text[m_open.start() :])
                pos = text_len
                break

            if m_next_open and m_next_open.start() < m_close.start():
                depth += 1
                scan_pos = m_next_open.end()
            else:
                depth -= 1
                scan_pos = m_close.end()
                if depth == 0:
                    table_html = text[m_open.start() : m_close.end()]
                    soup = BeautifulSoup(table_html, "html.parser")
                    table_tag = soup.find("table")
                    if table_tag:
                        gfm = _table_to_gfm(table_tag)
                        if gfm:
                            result_parts.append(gfm)
                            count += 1
                        else:
                            result_parts.append(table_html)
                    else:
                        result_parts.append(table_html)
                    pos = m_close.end()
        else:
            if depth > 0:
                # Unmatched open tag
                result_parts.append(text[m_open.start() :])
                pos = text_len

    return "".join(result_parts), count


# ---------------------------------------------------------------------------
# Inline tag conversion using regex (simple tags)
# ---------------------------------------------------------------------------

# Tags where we only keep the inner content (strip the tag)
_STRIP_TAGS = {
    "div", "span", "section", "article", "header", "footer",
    "nav", "main", "aside", "figure", "figcaption",
    "ul", "ol", "li",  # basic list containers — keep content
    "sup", "sub",
    "p",
}


def _convert_inline_tags(text: str) -> tuple[str, int]:
    """Apply regex-based conversions for inline HTML tags.

    Returns (converted_text, total_replacement_count).
    """
    count = 0

    def sub(pattern: str, repl, s: str, flags: int = re.IGNORECASE | re.DOTALL) -> tuple[str, int]:
        new_s, n = re.subn(pattern, repl, s, flags=flags)
        return new_s, n

    # <b>text</b> or <strong>text</strong> → **text**
    text, n = sub(r"<(?:b|strong)>(.*?)</(?:b|strong)>", r"**\1**", text)
    count += n

    # <i>text</i> or <em>text</em> → *text*
    text, n = sub(r"<(?:i|em)>(.*?)</(?:i|em)>", r"*\1*", text)
    count += n

    # <code>text</code> → `text`
    text, n = sub(r"<code>(.*?)</code>", r"`\1`", text)
    count += n

    # <pre>text</pre> → fenced code block
    def pre_repl(m: re.Match) -> str:
        inner = m.group(1).strip()
        return f"```\n{inner}\n```"

    text, n = sub(r"<pre>(.*?)</pre>", pre_repl, text)
    count += n

    # <a href="url">text</a> → [text](url)
    text, n = sub(r'<a\s+(?:[^>]*\s+)?href=["\']([^"\']*)["\'][^>]*>(.*?)</a>', r"[\2](\1)", text)
    count += n

    # <img src="url" alt="text"> → ![text](url)
    def img_repl(m: re.Match) -> str:
        attrs = m.group(1)
        src_m = re.search(r'src=["\']([^"\']*)["\']', attrs, re.IGNORECASE)
        alt_m = re.search(r'alt=["\']([^"\']*)["\']', attrs, re.IGNORECASE)
        src = src_m.group(1) if src_m else ""
        alt = alt_m.group(1) if alt_m else ""
        return f"![{alt}]({src})"

    text, n = sub(r"<img\s+([^>]*)/?>" , img_repl, text)
    count += n

    # <br>, <br/>, <br /> → newline
    text, n = sub(r"<br\s*/?>", "\n", text)
    count += n

    # <hr>, <hr/>, <hr /> → ---
    text, n = sub(r"<hr\s*/?>", "---", text)
    count += n

    # <p> opening tags → paragraph break (two newlines before)
    text, n = sub(r"<p(?:\s[^>]*)?>", "\n\n", text)
    count += n

    # </p> closing → paragraph break
    text, n = sub(r"</p>", "\n\n", text)
    count += n

    # Strip tags whose content we keep (div, span, sup, sub, etc.)
    for tag in _STRIP_TAGS:
        text, n = sub(rf"</?{tag}(?:\s[^>]*)?>", "", text)
        count += n

    # Strip any remaining unknown tags (open or close, with or without attrs)
    # but keep their content — just remove the tag itself.
    text, n = sub(r"</?[a-zA-Z][a-zA-Z0-9]*(?:\s[^>]*)?>", "", text)
    count += n

    return text, count


# ---------------------------------------------------------------------------
# Fenced code block protection
# ---------------------------------------------------------------------------

# Placeholder that is extremely unlikely to appear in real content
_FENCE_PLACEHOLDER = "\x00FENCEBLOCK{index}\x00"


def _extract_fenced_blocks(text: str) -> tuple[str, list[str]]:
    """Replace fenced code blocks with placeholders.

    Returns (text_with_placeholders, list_of_original_blocks).
    Supports both ``` and ~~~ fences, with optional language specifier.
    """
    blocks: list[str] = []

    def replacer(m: re.Match) -> str:
        idx = len(blocks)
        blocks.append(m.group(0))
        return f"\x00FENCEBLOCK{idx}\x00"

    # Match fenced blocks: opening fence (``` or ~~~) optionally followed by
    # a language specifier, then content until the matching closing fence.
    pattern = re.compile(
        r"(?m)^(`{3,}|~{3,})([^\n]*)\n(.*?)^\1\s*$",
        re.DOTALL | re.MULTILINE,
    )
    protected = pattern.sub(replacer, text)
    return protected, blocks


def _restore_fenced_blocks(text: str, blocks: list[str]) -> str:
    """Restore fenced code block placeholders to their original content."""
    for idx, block in enumerate(blocks):
        text = text.replace(f"\x00FENCEBLOCK{idx}\x00", block)
    return text


# ---------------------------------------------------------------------------
# Inline HTML comment protection
# ---------------------------------------------------------------------------

def _extract_html_comments(text: str) -> tuple[str, list[str]]:
    """Replace HTML comments with placeholders so they are not mangled."""
    comments: list[str] = []

    def replacer(m: re.Match) -> str:
        idx = len(comments)
        comments.append(m.group(0))
        return f"\x00HTMLCOMMENT{idx}\x00"

    protected = re.sub(r"<!--.*?-->", replacer, text, flags=re.DOTALL)
    return protected, comments


def _restore_html_comments(text: str, comments: list[str]) -> str:
    for idx, comment in enumerate(comments):
        text = text.replace(f"\x00HTMLCOMMENT{idx}\x00", comment)
    return text


# ---------------------------------------------------------------------------
# HTML entity decoding (minimal — just what we need post-conversion)
# ---------------------------------------------------------------------------

_HTML_ENTITIES = {
    "&amp;": "&",
    "&lt;": "<",
    "&gt;": ">",
    "&quot;": '"',
    "&#x27;": "'",
    "&#39;": "'",
    "&apos;": "'",
    "&nbsp;": " ",
    "&#160;": " ",
}

_ENTITY_PATTERN = re.compile("|".join(re.escape(k) for k in _HTML_ENTITIES))


def _decode_entities(text: str) -> str:
    """Decode common HTML entities to their Unicode equivalents."""
    return _ENTITY_PATTERN.sub(lambda m: _HTML_ENTITIES[m.group(0)], text)


# ---------------------------------------------------------------------------
# Main conversion pipeline
# ---------------------------------------------------------------------------

def convert_file(path: Path) -> dict[str, int]:
    """Convert a single markdown file in place.

    Returns a dict with tag-type → count of conversions performed.
    """
    original = path.read_text(encoding="utf-8")

    # Step 1: protect fenced code blocks
    text, fenced_blocks = _extract_fenced_blocks(original)

    # Step 2: protect HTML comments (<!-- ... -->)
    text, html_comments = _extract_html_comments(text)

    # Step 3: convert <table> blocks (most complex — use BeautifulSoup)
    text, table_count = convert_tables(text)

    # Step 4: convert simple inline tags via regex
    text, inline_count = _convert_inline_tags(text)

    # Step 5: decode residual HTML entities introduced during conversion
    text = _decode_entities(text)

    # Step 6: restore protected sections
    text = _restore_html_comments(text, html_comments)
    text = _restore_fenced_blocks(text, fenced_blocks)

    # Step 7: clean up excessive blank lines (max 2 consecutive)
    text = re.sub(r"\n{3,}", "\n\n", text)

    stats = {
        "tables": table_count,
        "inline_tags": inline_count,
    }

    if text != original:
        path.write_text(text, encoding="utf-8")

    return stats


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------

def main() -> None:
    parser = argparse.ArgumentParser(
        description="Convert HTML tags in ground-truth markdown files to GFM markdown.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  %(prog)s path/to/file.md
  %(prog)s --all
  %(prog)s --all --gt-dir /custom/ground_truth/
""",
    )
    parser.add_argument(
        "file",
        nargs="?",
        type=Path,
        help="Path to a single markdown file to process.",
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="Process all .md files under test_documents/ground_truth/.",
    )
    parser.add_argument(
        "--gt-dir",
        type=Path,
        default=None,
        help=(
            "Override the ground_truth directory. "
            "Defaults to test_documents/ground_truth/ relative to the repo root."
        ),
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Report changes without writing files.",
    )
    args = parser.parse_args()

    if not args.file and not args.all:
        parser.error("Provide a file path or pass --all to process all ground-truth files.")

    # Resolve the list of files to process
    files: list[Path] = []
    if args.all:
        if args.gt_dir:
            gt_dir = args.gt_dir
        else:
            # Walk up from this script to find the repo root (contains test_documents/)
            script_dir = Path(__file__).resolve().parent
            repo_root = script_dir
            for _ in range(10):
                if (repo_root / "test_documents").exists():
                    break
                repo_root = repo_root.parent
            gt_dir = repo_root / "test_documents" / "ground_truth"

        if not gt_dir.is_dir():
            print(f"Error: ground_truth directory not found: {gt_dir}", file=sys.stderr)
            sys.exit(1)

        files = sorted(gt_dir.rglob("*.md"))
        if not files:
            print(f"No .md files found under {gt_dir}", file=sys.stderr)
            sys.exit(0)
    else:
        if not args.file.is_file():
            print(f"Error: file not found: {args.file}", file=sys.stderr)
            sys.exit(1)
        files = [args.file]

    total_tables = 0
    total_inline = 0
    changed_files = 0

    for fpath in files:
        if args.dry_run:
            # Run conversion but compare without writing
            original = fpath.read_text(encoding="utf-8")
            # Re-use convert_file logic but intercept the write
            text, fenced_blocks = _extract_fenced_blocks(original)
            text, html_comments = _extract_html_comments(text)
            text, table_count = convert_tables(text)
            text, inline_count = _convert_inline_tags(text)
            text = _decode_entities(text)
            text = _restore_html_comments(text, html_comments)
            text = _restore_fenced_blocks(text, fenced_blocks)
            text = re.sub(r"\n{3,}", "\n\n", text)
            stats = {"tables": table_count, "inline_tags": inline_count}
            modified = text != original
        else:
            original_content = fpath.read_text(encoding="utf-8")
            stats = convert_file(fpath)
            new_content = fpath.read_text(encoding="utf-8")
            modified = new_content != original_content

        total_tables += stats["tables"]
        total_inline += stats["inline_tags"]

        if modified:
            changed_files += 1
            prefix = "[dry-run] would update" if args.dry_run else "Updated"
            tag_summary = []
            if stats["tables"]:
                tag_summary.append(f"{stats['tables']} table(s)")
            if stats["inline_tags"]:
                tag_summary.append(f"{stats['inline_tags']} inline tag(s)")
            summary = ", ".join(tag_summary) if tag_summary else "whitespace only"
            print(f"  {prefix}: {fpath}  [{summary}]")
        else:
            print(f"  No changes: {fpath}")

    print()
    print(f"Summary: {changed_files}/{len(files)} file(s) changed")
    print(f"  Tables converted : {total_tables}")
    print(f"  Inline tags removed/converted: {total_inline}")


if __name__ == "__main__":
    main()

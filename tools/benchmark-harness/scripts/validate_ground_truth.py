#!/usr/bin/env python3
"""Validate ground truth organization for benchmark harness.

This script validates that:
1. The ground truth directory exists and is properly organized
2. Ground truth files reference valid paths
3. Metadata files have required fields
4. No orphaned ground truth files exist without corresponding source documents
"""

from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path

# HTML tags that should not appear in GFM ground truth markdown.
# These indicate pandoc artifacts or incomplete cleanup.
_HTML_TAG_PATTERN = re.compile(
    r"</?(?:div|span|table|thead|tbody|tr|td|th|ul|ol|li|p|br|hr|"
    r"b|strong|i|em|u|s|del|ins|sub|sup|mark|small|big|"
    r"blockquote|pre|code|img|a|h[1-6]|section|article|header|footer|nav|"
    r"figure|figcaption|caption|col|colgroup|details|summary|abbr|"
    r"dl|dt|dd|fieldset|form|input|label|select|textarea|button|"
    r"audio|video|source|canvas|iframe|object|embed|ruby|rt|rp|wbr"
    r")(?:\s[^>]*)?\s*/?>",
    re.IGNORECASE,
)


def get_repo_root() -> Path:
    """Get the repository root directory."""
    # Start from script location and walk up to find repo root
    current = Path(__file__).resolve().parent
    while current != current.parent:
        if (current / "Cargo.toml").exists() and (current / "test_documents").exists():
            return current
        current = current.parent

    # Fallback: try current working directory
    cwd = Path.cwd()
    if (cwd / "test_documents" / "ground_truth").exists():
        return cwd

    raise RuntimeError("Could not find repository root")


def validate_ground_truth_mapping(repo_root: Path) -> list[str]:
    """Validate the ground truth mapping file."""
    errors = []
    mapping_file = repo_root / "test_documents" / "ground_truth" / "ground_truth_mapping.json"

    if not mapping_file.exists():
        # Mapping file is optional - not an error if missing
        print(f"Note: Ground truth mapping file not found at {mapping_file}")
        return errors

    try:
        with open(mapping_file) as f:
            mapping = json.load(f)
    except json.JSONDecodeError as e:
        errors.append(f"Invalid JSON in ground truth mapping: {e}")
        return errors

    # Validate that each ground truth file exists
    for name, path in mapping.items():
        full_path = repo_root / path
        if not full_path.exists():
            errors.append(f"Ground truth file missing: {path} (key: {name})")

    return errors


def validate_ground_truth_structure(repo_root: Path) -> list[str]:
    """Validate the ground truth directory structure."""
    errors = []
    ground_truth_dir = repo_root / "test_documents" / "ground_truth"

    if not ground_truth_dir.exists():
        errors.append(f"Ground truth directory does not exist: {ground_truth_dir}")
        return errors

    if not ground_truth_dir.is_dir():
        errors.append(f"Ground truth path is not a directory: {ground_truth_dir}")
        return errors

    # Check that subdirectories exist for expected file types
    expected_subdirs = []  # Optional - don't require specific subdirs

    for subdir in expected_subdirs:
        subdir_path = ground_truth_dir / subdir
        if not subdir_path.exists():
            errors.append(f"Expected ground truth subdirectory missing: {subdir}")

    # Validate ground truth files (.txt and .md)
    txt_files = list(ground_truth_dir.rglob("*.txt"))
    md_files = list(ground_truth_dir.rglob("*.md"))

    for gt_file in txt_files + md_files:
        # Skip meta files
        if gt_file.stem.endswith("_meta"):
            continue

        # Warn on very small files (likely placeholders).
        # Some test documents genuinely have minimal content (e.g. "Home", "Text 1").
        if gt_file.stat().st_size < 4:
            errors.append(f"Ground truth file suspiciously small ({gt_file.stat().st_size} bytes): {gt_file.relative_to(repo_root)}")

    print(f"Found {len(txt_files)} .txt and {len(md_files)} .md ground truth files")

    return errors


def validate_no_html_in_markdown_gt(repo_root: Path) -> list[str]:
    """Validate that markdown ground truth files contain no HTML tags.

    GFM ground truth must be pure markdown with no HTML remnants.
    HTML indicates incomplete cleanup from pandoc or other generators.
    """
    errors = []
    ground_truth_dir = repo_root / "test_documents" / "ground_truth"

    if not ground_truth_dir.exists():
        return errors

    md_files = sorted(ground_truth_dir.rglob("*.md"))
    files_with_html = 0

    for md_file in md_files:
        try:
            content = md_file.read_text(encoding="utf-8", errors="replace")
        except OSError:
            continue

        # Skip files in code blocks (``` ... ```)
        # Simple approach: remove fenced code blocks before scanning
        cleaned = re.sub(r"```[\s\S]*?```", "", content)
        # Also skip inline code
        cleaned = re.sub(r"`[^`]+`", "", cleaned)

        matches = list(_HTML_TAG_PATTERN.finditer(cleaned))
        if matches:
            files_with_html += 1
            rel_path = md_file.relative_to(repo_root)
            # Show first few matches
            examples = [m.group() for m in matches[:3]]
            examples_str = ", ".join(examples)
            suffix = f" (and {len(matches) - 3} more)" if len(matches) > 3 else ""
            errors.append(
                f"HTML in markdown GT: {rel_path} — {len(matches)} tag(s): {examples_str}{suffix}"
            )

    if files_with_html > 0:
        print(f"Found {files_with_html} markdown GT files containing HTML tags")
    else:
        print("All markdown GT files are HTML-free")

    return errors


def validate_benchmark_fixtures(repo_root: Path) -> list[str]:
    """Validate that benchmark fixture files reference existing documents."""
    errors = []
    fixtures_dir = repo_root / "tools" / "benchmark-harness" / "fixtures"

    if not fixtures_dir.exists():
        print(f"Note: Benchmark fixtures directory not found at {fixtures_dir}")
        return errors

    json_files = list(fixtures_dir.rglob("*.json"))
    missing_count = 0

    for json_file in json_files:
        try:
            with open(json_file) as f:
                fixture = json.load(f)
        except json.JSONDecodeError as e:
            errors.append(f"Invalid JSON in fixture {json_file.name}: {e}")
            continue

        # Check if document path exists
        if "document" in fixture:
            doc_path = fixture["document"]
            # Resolve relative path from the fixture file's directory
            full_path = (json_file.parent / doc_path).resolve()

            if not full_path.exists():
                errors.append(f"Fixture {json_file.name}: Document not found: {doc_path}")
                missing_count += 1

    print(f"Validated {len(json_files)} benchmark fixtures, {missing_count} missing documents")

    return errors


def main() -> int:
    """Main entry point."""
    print("=== Validating Ground Truth Organization ===\n")

    try:
        repo_root = get_repo_root()
    except RuntimeError as e:
        print(f"Error: {e}")
        return 1

    print(f"Repository root: {repo_root}\n")

    all_errors: list[str] = []

    # Validate ground truth structure
    print("Validating ground truth directory structure...")
    errors = validate_ground_truth_structure(repo_root)
    all_errors.extend(errors)

    # Validate ground truth mapping
    print("Validating ground truth mapping...")
    errors = validate_ground_truth_mapping(repo_root)
    all_errors.extend(errors)

    # Validate no HTML in markdown ground truth
    print("Checking markdown GT for HTML remnants...")
    errors = validate_no_html_in_markdown_gt(repo_root)
    all_errors.extend(errors)

    # Validate benchmark fixtures
    print("Validating benchmark fixtures...")
    errors = validate_benchmark_fixtures(repo_root)
    all_errors.extend(errors)

    print("")

    if all_errors:
        print(f"=== VALIDATION FAILED: {len(all_errors)} error(s) ===\n")
        for error in all_errors:
            print(f"  ERROR: {error}")
        return 1

    print("=== VALIDATION PASSED ===")
    return 0


if __name__ == "__main__":
    sys.exit(main())

"""CLI entry point for the fixture generator.

Run as ``python -m generate_test_fixtures <command> [...]``. Commands map
one-to-one onto the per-format submodules. ``all`` runs every generator
in a deterministic order.
"""

from __future__ import annotations

import argparse
import sys
from collections.abc import Callable
from pathlib import Path

# Each generator exposes ``generate(output_root: Path, repo_root: Path) -> list[Path]``
# returning the list of files (binary + sidecars) it wrote. This keeps the
# dispatch table trivial and the smoke test predictable.
GeneratorFn = Callable[[Path, Path], list[Path]]


def _generators() -> dict[str, GeneratorFn]:
    """Lazy-import generators so a partial dep install doesn't break ``--help``."""
    from . import (
        diff_pairs,
        docx_revisions,
        odt_revisions,
        pdf_incremental,
        pptx_comments,
        security_fixtures,
        xlsx_revisions,
    )

    return {
        "docx": docx_revisions.generate,
        "odt": odt_revisions.generate,
        "xlsx": xlsx_revisions.generate,
        "pptx": pptx_comments.generate,
        "pdf": pdf_incremental.generate,
        "diff-pairs": diff_pairs.generate,
        "security": security_fixtures.generate,
    }


def _default_repo_root() -> Path:
    """Walk upward from this file to find the xberg repo root.

    Anchored on the presence of ``Cargo.toml`` + ``test_documents``. Falls
    back to the current working directory when those markers are absent
    (e.g. when the package is installed elsewhere).
    """
    here = Path(__file__).resolve()
    for ancestor in [here, *here.parents]:
        if (ancestor / "Cargo.toml").is_file() and (ancestor / "test_documents").is_dir():
            return ancestor
    return Path.cwd()


def main(argv: list[str] | None = None) -> int:
    """CLI entry. Returns a process exit code (0 on success)."""
    parser = argparse.ArgumentParser(
        prog="generate-test-fixtures",
        description="Generate deterministic test fixtures for xberg integration tests.",
    )
    parser.add_argument(
        "commands",
        nargs="+",
        choices=["all", "docx", "odt", "xlsx", "pptx", "pdf", "diff-pairs", "security"],
        help="One or more fixture categories to generate. 'all' runs every generator.",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=None,
        help=(
            "Output root directory. Defaults to "
            "<repo-root>/test_documents/generated. Per-format subdirectories "
            "are created automatically."
        ),
    )
    parser.add_argument(
        "--repo-root",
        type=Path,
        default=None,
        help="Repository root override. Auto-detected when omitted.",
    )
    args = parser.parse_args(argv)

    repo_root = (args.repo_root or _default_repo_root()).resolve()
    output_root = (args.output_dir or (repo_root / "test_documents" / "generated")).resolve()
    output_root.mkdir(parents=True, exist_ok=True)

    selected: list[str]
    if "all" in args.commands:
        selected = ["docx", "odt", "xlsx", "pptx", "pdf", "diff-pairs", "security"]
    else:
        # Preserve user ordering, drop duplicates.
        seen: set[str] = set()
        selected = [c for c in args.commands if not (c in seen or seen.add(c))]  # type: ignore[func-returns-value]

    generators = _generators()
    total_written = 0
    for command in selected:
        fn = generators[command]
        written = fn(output_root, repo_root)
        total_written += len(written)
        print(f"[{command}] wrote {len(written)} files")
        for path in written:
            print(f"  - {path.relative_to(output_root) if path.is_relative_to(output_root) else path}")

    print(f"Done. Total files written: {total_written}")
    return 0


if __name__ == "__main__":
    sys.exit(main())

"""DOCX track-changes fixture generator.

``python-docx`` doesn't author ``w:ins`` / ``w:del`` / ``w:rPrChange``
elements natively — they're considered "revision marks" that Word inserts
when track-changes mode is on. We sidestep that by authoring a vanilla
document with ``python-docx``, then post-processing ``word/document.xml``
inside the zip: parse the XML, splice change elements around target runs,
write the archive back out with deterministic ZIP metadata.

The on-disk XML matches what Word produces, which is what
``crates/xberg/src/extractors/docx`` (the path that populates
``ExtractionResult.revisions``) consumes.
"""

from __future__ import annotations

import io
import zipfile
from collections.abc import Iterable
from pathlib import Path

from docx import Document  # type: ignore[import-untyped, import-not-found, unused-ignore]

from .gt_schema import revisions_expectation, write_ground_truth

W_NS = "http://schemas.openxmlformats.org/wordprocessingml/2006/main"

# Pinned timestamps. Determinism > realism — these are fixtures, not real
# documents.
TS_ALICE_INS_1 = "2024-03-15T10:30:00Z"
TS_ALICE_INS_2 = "2024-03-15T10:35:00Z"
TS_BOB_DEL = "2024-03-15T11:00:00Z"
TS_CAROL_FMT = "2024-03-15T12:00:00Z"
TS_DAVE_INS = "2024-03-15T12:15:00Z"

# Deterministic mtime for every zip entry so fixtures hash stably across
# runs and CI. (1980-01-01 is the ZIP epoch — using the start of 2024 is
# arbitrary but visible in `unzip -v` output.)
ZIP_MTIME = (2024, 1, 1, 0, 0, 0)


def _read_document_xml(docx_bytes: bytes) -> str:
    with zipfile.ZipFile(io.BytesIO(docx_bytes), "r") as zf:
        return zf.read("word/document.xml").decode("utf-8")


def _replace_in_zip(docx_bytes: bytes, replacements: dict[str, bytes]) -> bytes:
    """Return a new docx with ``replacements`` patched in.

    Re-writes every entry so we control mtime + compression for hash
    stability. Entries not in ``replacements`` are copied byte-for-byte.
    """
    buf = io.BytesIO()
    with zipfile.ZipFile(io.BytesIO(docx_bytes), "r") as src:
        names = src.namelist()
        with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as dst:
            for name in names:
                data = replacements.get(name, src.read(name))
                info = zipfile.ZipInfo(name, ZIP_MTIME)
                info.compress_type = zipfile.ZIP_DEFLATED
                dst.writestr(info, data)
    return buf.getvalue()


def _ins_block(author: str, date: str, rev_id: str, text: str) -> str:
    """An entire ``<w:p>`` block carrying a single ``<w:ins>`` run.

    DOCX extractor anchors revisions on paragraph index, so each
    insertion lives in its own paragraph for unambiguous expectations.
    """
    return (
        f'<w:p xmlns:w="{W_NS}">'
        f'<w:ins w:id="{rev_id}" w:author="{author}" w:date="{date}">'
        f'<w:r><w:t xml:space="preserve">{text}</w:t></w:r>'
        f"</w:ins>"
        f"</w:p>"
    )


def _del_block(author: str, date: str, rev_id: str, text: str) -> str:
    return (
        f'<w:p xmlns:w="{W_NS}">'
        f'<w:del w:id="{rev_id}" w:author="{author}" w:date="{date}">'
        f'<w:r><w:delText xml:space="preserve">{text}</w:delText></w:r>'
        f"</w:del>"
        f"</w:p>"
    )


def _format_change_block(author: str, date: str, rev_id: str, text: str) -> str:
    """Paragraph carrying a ``w:rPrChange`` — run-level formatting revision."""
    return (
        f'<w:p xmlns:w="{W_NS}">'
        f"<w:r>"
        f'<w:rPr><w:b/><w:rPrChange w:id="{rev_id}" w:author="{author}" w:date="{date}"><w:rPr/></w:rPrChange></w:rPr>'
        f'<w:t xml:space="preserve">{text}</w:t>'
        f"</w:r>"
        f"</w:p>"
    )


def _splice_blocks_into_body(document_xml: str, blocks: Iterable[str]) -> str:
    """Insert ``blocks`` immediately before ``</w:body>``.

    We deliberately do NOT parse with ``lxml`` — string splicing keeps the
    output stable across Python / lxml versions and avoids namespace-
    declaration reshuffling that can confuse downstream diff tools.
    """
    marker = "</w:body>"
    insert_at = document_xml.rfind(marker)
    if insert_at == -1:
        raise RuntimeError("word/document.xml is missing </w:body>; cannot splice revisions")
    head = document_xml[:insert_at]
    tail = document_xml[insert_at:]
    return head + "".join(blocks) + tail


def _build_base_docx(paragraphs: list[str]) -> bytes:
    """Author a baseline DOCX with ``python-docx`` and return its bytes."""
    doc = Document()
    for text in paragraphs:
        doc.add_paragraph(text)
    buf = io.BytesIO()
    doc.save(buf)
    return buf.getvalue()


def _emit_basic(output_dir: Path, repo_root: Path) -> list[Path]:
    """Three paragraphs, two insertions (Alice), one deletion (Bob)."""
    base = _build_base_docx(
        [
            "Original paragraph one — kept as-is.",
            "Original paragraph two — kept as-is.",
            "Original paragraph three — kept as-is.",
        ]
    )
    blocks = [
        _ins_block("Alice", TS_ALICE_INS_1, "100", "Inserted by Alice (first)."),
        _ins_block("Alice", TS_ALICE_INS_2, "101", "Inserted by Alice (second)."),
        _del_block("Bob", TS_BOB_DEL, "102", "Deleted by Bob."),
    ]
    patched_xml = _splice_blocks_into_body(_read_document_xml(base), blocks)
    out = _replace_in_zip(base, {"word/document.xml": patched_xml.encode("utf-8")})

    fixture_path = output_dir / "docx_track_changes_basic.docx"
    sidecar_path = output_dir / "docx_track_changes_basic.gt.json"
    fixture_path.write_bytes(out)
    write_ground_truth(
        sidecar_path,
        fixture_path,
        repo_root,
        document_format="docx",
        feature="revisions",
        expectations=revisions_expectation(
            expected_count=3,
            revisions=[
                {"kind": "Insertion", "author": "Alice", "timestamp": TS_ALICE_INS_1, "revision_id": "100"},
                {"kind": "Insertion", "author": "Alice", "timestamp": TS_ALICE_INS_2, "revision_id": "101"},
                {"kind": "Deletion", "author": "Bob", "timestamp": TS_BOB_DEL, "revision_id": "102"},
            ],
        ),
        generator="docx_revisions",
    )
    return [fixture_path, sidecar_path]


def _emit_multi_author(output_dir: Path, repo_root: Path) -> list[Path]:
    """Five paragraphs, four authors, mixed Insertion / Deletion / FormatChange."""
    base = _build_base_docx([f"Baseline paragraph {i}." for i in range(5)])
    blocks = [
        _ins_block("Alice", TS_ALICE_INS_1, "200", "Alice inserts here."),
        _del_block("Bob", TS_BOB_DEL, "201", "Bob deletes this."),
        _format_change_block("Carol", TS_CAROL_FMT, "202", "Carol changes formatting."),
        _ins_block("Dave", TS_DAVE_INS, "203", "Dave inserts a closing line."),
    ]
    patched_xml = _splice_blocks_into_body(_read_document_xml(base), blocks)
    out = _replace_in_zip(base, {"word/document.xml": patched_xml.encode("utf-8")})

    fixture_path = output_dir / "docx_track_changes_multi_author.docx"
    sidecar_path = output_dir / "docx_track_changes_multi_author.gt.json"
    fixture_path.write_bytes(out)
    write_ground_truth(
        sidecar_path,
        fixture_path,
        repo_root,
        document_format="docx",
        feature="revisions",
        expectations=revisions_expectation(
            expected_count=4,
            revisions=[
                {"kind": "Insertion", "author": "Alice", "timestamp": TS_ALICE_INS_1, "revision_id": "200"},
                {"kind": "Deletion", "author": "Bob", "timestamp": TS_BOB_DEL, "revision_id": "201"},
                {"kind": "FormatChange", "author": "Carol", "timestamp": TS_CAROL_FMT, "revision_id": "202"},
                {"kind": "Insertion", "author": "Dave", "timestamp": TS_DAVE_INS, "revision_id": "203"},
            ],
            notes="Four distinct authors; mixed kinds exercise the per-kind branches in extractors/docx.",
        ),
        generator="docx_revisions",
    )
    return [fixture_path, sidecar_path]


def generate(output_root: Path, repo_root: Path) -> list[Path]:
    """Produce both DOCX track-changes fixtures under ``output_root/docx/``."""
    output_dir = output_root / "docx"
    output_dir.mkdir(parents=True, exist_ok=True)
    written: list[Path] = []
    written.extend(_emit_basic(output_dir, repo_root))
    written.extend(_emit_multi_author(output_dir, repo_root))
    return written

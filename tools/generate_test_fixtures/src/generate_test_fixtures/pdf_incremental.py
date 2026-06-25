"""PDF incremental-update fixture generator.

Produces a PDF with multiple historical ``xref`` sections, each carrying a
``trailer << /Prev <previous-xref-offset> >>``. The xberg PDF revisions
walker (``crates/xberg/src/pdf/xref_revisions.rs``) discovers them by
scanning backwards for ``%%EOF`` markers and following ``/Prev`` from the
latest xref.

We use ``reportlab`` for the base document (a single page that ``lopdf``
will happily load) and then append two incremental-update sections by hand.
The append technique matches the ``build_incremental_pdf`` helper used in
the Rust extractor's own unit tests:

    <new object>
    xref
    <subsection-header>
    <new-object-offset> 00000 n
    trailer << /Size N /Root <root> /Prev <previous-xref> /Info <info> >>
    startxref
    <new-xref-offset>
    %%EOF

The trailer keeps ``/Root`` and ``/Info`` references from the base so the
PDF is still a valid single-revision document for any tool that ignores
the ``/Prev`` chain.
"""

from __future__ import annotations

import io
import re
from pathlib import Path

from reportlab.lib.pagesizes import LETTER  # type: ignore[import-untyped, import-not-found, unused-ignore]
from reportlab.pdfgen import canvas  # type: ignore[import-untyped, import-not-found, unused-ignore]

from .gt_schema import revisions_expectation, write_ground_truth


def _build_baseline_pdf() -> bytes:
    """Author a single-page PDF with reportlab and return its bytes.

    reportlab is intentionally configured with a fixed creation date and
    deterministic ``invariant=True`` settings so the byte output is stable.
    """
    buf = io.BytesIO()
    pdf = canvas.Canvas(buf, pagesize=LETTER, invariant=True)
    pdf.setAuthor("Alice")
    pdf.setTitle("PDF incremental-updates fixture")
    pdf.setSubject("Three-revision xref chain")
    pdf.setCreator("generate-test-fixtures")
    # reportlab's invariant=True replaces the document creation date with a
    # fixed value internally, so the produced bytes hash stably.
    pdf.drawString(72, 720, "Original revision (base save).")
    pdf.showPage()
    pdf.save()
    return buf.getvalue()


def _parse_last_startxref(pdf_bytes: bytes) -> int:
    """Return the byte offset stored in the trailing ``startxref\\n<N>``.

    Used to populate the ``/Prev`` value of the first incremental update.
    """
    # Search the last 1024 bytes — every PDF should have its startxref well
    # within the trailer window.
    window = pdf_bytes[-1024:]
    match = re.search(rb"startxref\s+(\d+)", window)
    if not match:
        raise RuntimeError("baseline PDF missing trailing startxref")
    return int(match.group(1))


def _find_root_ref(pdf_bytes: bytes) -> str:
    """Locate the ``/Root <obj-num> <gen-num> R`` reference in the trailer.

    We need it to keep ``/Root`` populated in the new trailer of each
    incremental update.
    """
    match = re.search(rb"/Root\s+(\d+\s+\d+\s+R)", pdf_bytes)
    if not match:
        raise RuntimeError("baseline PDF missing /Root in trailer")
    return match.group(1).decode("ascii")


def _find_size(pdf_bytes: bytes) -> int:
    """Read ``/Size N`` from the baseline trailer (highest object number + 1)."""
    match = re.search(rb"/Size\s+(\d+)", pdf_bytes)
    if not match:
        raise RuntimeError("baseline PDF missing /Size in trailer")
    return int(match.group(1))


def _append_incremental_update(
    pdf_bytes: bytes,
    *,
    new_object_number: int,
    new_object_body: bytes,
    previous_xref_offset: int,
    new_size: int,
    root_ref: str,
) -> tuple[bytes, int]:
    """Append a single incremental-update section.

    Returns the new PDF bytes plus the byte offset of the new xref (useful
    as the ``/Prev`` value when chaining a second update).
    """
    # Ensure baseline ends with a newline so our appended section starts on
    # a fresh line — some validators reject `%%EOF<obj>`.
    if not pdf_bytes.endswith(b"\n"):
        pdf_bytes += b"\n"

    # New object definition.
    obj_offset = len(pdf_bytes)
    obj_block = f"{new_object_number} 0 obj\n".encode("ascii") + new_object_body + b"\nendobj\n"
    pdf_bytes += obj_block

    # xref subsection for the new object.
    xref_offset = len(pdf_bytes)
    xref_block = (
        b"xref\n" + f"{new_object_number} 1\n".encode("ascii") + f"{obj_offset:010d} 00000 n \n".encode("ascii")
    )
    pdf_bytes += xref_block

    # New trailer with /Prev pointing to the previous xref offset.
    trailer = (
        f"trailer\n<</Size {new_size} /Root {root_ref} /Prev {previous_xref_offset}>>\n"
        f"startxref\n{xref_offset}\n%%EOF\n"
    )
    pdf_bytes += trailer.encode("ascii")

    return pdf_bytes, xref_offset


def generate(output_root: Path, repo_root: Path) -> list[Path]:
    """Emit pdf_incremental_basic.pdf + sidecar under ``output_root/pdf/``."""
    output_dir = output_root / "pdf"
    output_dir.mkdir(parents=True, exist_ok=True)

    base = _build_baseline_pdf()
    base_xref_offset = _parse_last_startxref(base)
    root_ref = _find_root_ref(base)
    next_obj_number = _find_size(base)  # /Size = highest+1, so next obj reuses Size

    # First incremental update.
    after_first, first_xref_offset = _append_incremental_update(
        base,
        new_object_number=next_obj_number,
        new_object_body=b"<</Update 1 /Note (first incremental save)>>",
        previous_xref_offset=base_xref_offset,
        new_size=next_obj_number + 1,
        root_ref=root_ref,
    )

    # Second incremental update.
    final_bytes, _final_xref_offset = _append_incremental_update(
        after_first,
        new_object_number=next_obj_number + 1,
        new_object_body=b"<</Update 2 /Note (second incremental save)>>",
        previous_xref_offset=first_xref_offset,
        new_size=next_obj_number + 2,
        root_ref=root_ref,
    )

    fixture_path = output_dir / "pdf_incremental_basic.pdf"
    sidecar_path = output_dir / "pdf_incremental_basic.gt.json"
    fixture_path.write_bytes(final_bytes)

    write_ground_truth(
        sidecar_path,
        fixture_path,
        repo_root,
        document_format="pdf",
        feature="revisions",
        expectations=revisions_expectation(
            expected_count=2,
            revisions=[
                # The PDF extractor emits historical xref offsets oldest-first,
                # with revision_id = "xref-offset-{N}". The exact offsets vary
                # with reportlab's output size, so integration tests assert
                # count + revision_id PREFIX rather than exact offsets.
                {"kind": "Insertion", "revision_id_prefix": "xref-offset-", "author": "Alice"},
                {"kind": "Insertion", "revision_id_prefix": "xref-offset-", "author": "Alice"},
            ],
            notes=(
                "Three xref sections (base + two incremental updates). The PDF revisions "
                "walker emits 2 historical revisions (the latest xref represents the live "
                "state and is excluded). RevisionKind is always Insertion for PDFs — there "
                "is no DOCX-style typed change classification at the xref level. The two "
                "/Prev offsets vary with reportlab's output size; assert by prefix."
            ),
        ),
        generator="pdf_incremental",
    )
    return [fixture_path, sidecar_path]

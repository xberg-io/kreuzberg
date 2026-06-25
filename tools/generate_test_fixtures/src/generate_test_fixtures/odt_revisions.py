"""ODT tracked-changes fixture generator.

``odfpy`` authors valid ODT containers but does not expose helpers for
``<text:tracked-changes>`` — those are the OpenDocument equivalent of
Word's ``w:ins`` / ``w:del``. We author a baseline body with ``odfpy``,
then post-process ``content.xml`` to splice a ``<text:tracked-changes>``
block (with ``<text:changed-region>`` children for each revision) into
``<office:text>`` plus matching ``<text:change-start>`` / ``<text:change-end>``
markers around the live insertion text.

The shape mirrors what
``crates/xberg/src/extractors/odt.rs::parse_tracked_changes`` consumes:
``office:change-info`` -> ``dc:creator`` + ``dc:date``, child element
``insertion`` / ``deletion`` / ``format-change`` drives ``RevisionKind``.
"""

from __future__ import annotations

import io
import zipfile
from pathlib import Path

from odf.opendocument import OpenDocumentText  # type: ignore[import-untyped, import-not-found, unused-ignore]
from odf.text import H, P  # type: ignore[import-untyped, import-not-found, unused-ignore]

from .gt_schema import revisions_expectation, write_ground_truth

ZIP_MTIME = (2024, 1, 1, 0, 0, 0)

TS_ALICE = "2024-04-01T09:00:00Z"
TS_BOB = "2024-04-01T09:15:00Z"

# Pre-built tracked-changes block. Two changed-regions: ct1 = insertion by
# Alice, ct2 = deletion by Bob. The matching <text:change-start text:change-id="ct1"/>
# / <text:change-end text:change-id="ct1"/> markers are spliced into body
# paragraphs below.
TRACKED_CHANGES_XML = (
    '<text:tracked-changes xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0" '
    'xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" '
    'xmlns:dc="http://purl.org/dc/elements/1.1/">'
    '<text:changed-region text:id="ct1">'
    "<text:insertion>"
    "<office:change-info>"
    f"<dc:creator>Alice</dc:creator><dc:date>{TS_ALICE}</dc:date>"
    "</office:change-info>"
    "<text:p>Alice inserted this paragraph.</text:p>"
    "</text:insertion>"
    "</text:changed-region>"
    '<text:changed-region text:id="ct2">'
    "<text:deletion>"
    "<office:change-info>"
    f"<dc:creator>Bob</dc:creator><dc:date>{TS_BOB}</dc:date>"
    "</office:change-info>"
    "<text:p>Bob deleted this paragraph.</text:p>"
    "</text:deletion>"
    "</text:changed-region>"
    "</text:tracked-changes>"
)

# Body fragment that references the change-regions. The extractor walks
# body paragraphs and translates change-start/change-end markers into the
# matching revisions, so we include both insertion live text and a point-
# deletion marker.
BODY_REVISION_MARKERS = (
    '<text:p xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0">'
    '<text:change-start text:change-id="ct1"/>'
    "Alice inserted this paragraph."
    '<text:change-end text:change-id="ct1"/>'
    "</text:p>"
    '<text:p xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0">'
    '<text:change text:change-id="ct2"/>'
    "</text:p>"
)


def _build_baseline_odt() -> bytes:
    """Author a vanilla ODT with a heading + three paragraphs, return bytes."""
    doc = OpenDocumentText()
    doc.text.addElement(H(outlinelevel=1, text="ODT tracked-changes fixture"))
    doc.text.addElement(P(text="Baseline paragraph one — kept as-is."))
    doc.text.addElement(P(text="Baseline paragraph two — kept as-is."))
    doc.text.addElement(P(text="Baseline paragraph three — kept as-is."))
    buf = io.BytesIO()
    doc.write(buf)
    return buf.getvalue()


def _splice_tracked_changes(content_xml: str) -> str:
    """Insert the tracked-changes block + body markers into content.xml.

    Inserts ``<text:tracked-changes>`` immediately after the opening
    ``<office:text>`` tag, then inserts the body markers just before the
    closing ``</office:text>`` tag.
    """
    open_marker = "<office:text>"
    open_idx = content_xml.find(open_marker)
    # Some odfpy versions emit ``<office:text ...>`` with attributes; fall
    # back to locating the first ``>`` after ``<office:text``.
    if open_idx == -1:
        tag_idx = content_xml.find("<office:text")
        if tag_idx == -1:
            raise RuntimeError("content.xml missing <office:text> element")
        open_idx = content_xml.find(">", tag_idx) + 1
    else:
        open_idx = open_idx + len(open_marker)

    close_marker = "</office:text>"
    close_idx = content_xml.rfind(close_marker)
    if close_idx == -1:
        raise RuntimeError("content.xml missing </office:text> close tag")

    head = content_xml[:open_idx]
    middle = content_xml[open_idx:close_idx]
    tail = content_xml[close_idx:]
    return head + TRACKED_CHANGES_XML + middle + BODY_REVISION_MARKERS + tail


def _replace_in_zip(src_bytes: bytes, replacements: dict[str, bytes]) -> bytes:
    """Rewrite ``src_bytes`` (an ODT zip) with deterministic mtimes."""
    buf = io.BytesIO()
    with zipfile.ZipFile(io.BytesIO(src_bytes), "r") as src:
        with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as dst:
            for name in src.namelist():
                data = replacements.get(name, src.read(name))
                # ODT requires ``mimetype`` to be the first entry and stored
                # without compression. Preserve that invariant.
                info = zipfile.ZipInfo(name, ZIP_MTIME)
                if name == "mimetype":
                    info.compress_type = zipfile.ZIP_STORED
                else:
                    info.compress_type = zipfile.ZIP_DEFLATED
                dst.writestr(info, data)
    return buf.getvalue()


def generate(output_root: Path, repo_root: Path) -> list[Path]:
    """Emit odt_tracked_changes_basic.odt + sidecar under ``output_root/odt/``."""
    output_dir = output_root / "odt"
    output_dir.mkdir(parents=True, exist_ok=True)

    base = _build_baseline_odt()
    with zipfile.ZipFile(io.BytesIO(base), "r") as zf:
        content_xml = zf.read("content.xml").decode("utf-8")
    patched = _splice_tracked_changes(content_xml)
    out = _replace_in_zip(base, {"content.xml": patched.encode("utf-8")})

    fixture_path = output_dir / "odt_tracked_changes_basic.odt"
    sidecar_path = output_dir / "odt_tracked_changes_basic.gt.json"
    fixture_path.write_bytes(out)
    write_ground_truth(
        sidecar_path,
        fixture_path,
        repo_root,
        document_format="odt",
        feature="revisions",
        expectations=revisions_expectation(
            expected_count=2,
            revisions=[
                {"kind": "Insertion", "author": "Alice", "timestamp": TS_ALICE, "revision_id": "ct1"},
                {"kind": "Deletion", "author": "Bob", "timestamp": TS_BOB, "revision_id": "ct2"},
            ],
            notes=(
                "<text:changed-region> ids are 'ct1' (insertion) and 'ct2' (deletion). "
                "Body markers exercise both <text:change-start>/<text:change-end> pair "
                "and the point-marker <text:change> form."
            ),
        ),
        generator="odt_revisions",
    )
    return [fixture_path, sidecar_path]

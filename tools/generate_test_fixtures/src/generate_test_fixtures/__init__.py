"""Deterministic fixture generator for xberg integration tests.

Each submodule produces a single category of on-disk fixture (DOCX
track-changes, ODT tracked changes, XLSX revision headers, PPTX comments,
PDF incremental updates, paired diff inputs, security edge cases) together
with a ``<stem>.gt.json`` ground-truth sidecar that integration tests load
to assert structured expectations.
"""

__version__ = "0.1.0"

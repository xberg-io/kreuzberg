"""Regression tests for the bead bb-t58i main bug.

Before this change ``alef.toml`` set ``dto.python_output = "typed-dict"``,
which made the public ``ExtractionConfig`` a ``TypedDict``. Calling
``ExtractionConfig(...)`` produced a plain ``dict``, while the alef-generated
``_to_rust_extraction_config`` converter accessed fields via dot syntax and
raised ``AttributeError: 'dict' object has no attribute 'use_cache'`` on every
public ``extract_*`` call.

After flipping ``dto.python_output`` back to ``"dataclass"``, the public
``ExtractionConfig`` is the pyo3-defined class (with real attributes); the
converter's dot access now works. These tests pin the runtime shape so the
original regression cannot silently come back.
"""

from __future__ import annotations

import kreuzberg


def test_extraction_config_supports_attribute_access() -> None:
    """The public ExtractionConfig must expose fields as attributes, not dict keys.

    The bead's reproducer crashed because instances were dicts; the converter
    can't `value.use_cache` on a dict. Whichever class backs ``ExtractionConfig``
    after regen (dataclass or pyo3 class), it must expose attributes.
    """
    ec = kreuzberg.ExtractionConfig(disable_ocr=True)
    assert hasattr(ec, "use_cache")
    assert hasattr(ec, "disable_ocr")
    assert ec.disable_ocr is True


def test_extraction_config_default_construction() -> None:
    """Constructing with no args should succeed and yield an instance with attributes."""
    ec = kreuzberg.ExtractionConfig()
    assert hasattr(ec, "use_cache")
    assert hasattr(ec, "force_ocr")


def test_extraction_config_is_not_a_typeddict() -> None:
    """A TypedDict instance is a plain dict; that's what shipped the bug.

    This guard catches a future regen accidentally flipping ``python_output``
    back to ``typed-dict`` again.
    """
    ec = kreuzberg.ExtractionConfig(disable_ocr=True)
    assert not isinstance(ec, dict), (
        "ExtractionConfig must not be a TypedDict — that's the bb-t58i regression."
    )


def test_to_rust_extraction_config_does_not_attribute_error() -> None:
    """The converter must accept an ExtractionConfig without AttributeError.

    This is the *exact* code path that crashed in bead bb-t58i:

        File '.../kreuzberg/api.py', line 337, in _to_rust_extraction_config
            use_cache=value.use_cache,
        AttributeError: 'dict' object has no attribute 'use_cache'
    """
    from kreuzberg.api import _to_rust_extraction_config

    ec = kreuzberg.ExtractionConfig(disable_ocr=True)
    rust_config = _to_rust_extraction_config(ec)
    assert rust_config is not None

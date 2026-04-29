"""Tests for the embedding-backend registry round-trip.

Verifies that ``unregister_embedding_backend``, ``list_embedding_backends`` and
``clear_embedding_backends`` are exposed on the top-level ``kreuzberg`` module
and behave consistently with the OCR-backend equivalents. These three landed
in this PR via removal from the global IR-level exclude list.

``register_embedding_backend`` itself is wired through pyo3 (via the alef
trait_bridge codegen) but is not yet re-exported on the top-level package —
the trait_bridge generator does not currently surface it through ``api.py``
or ``__init__.py``. Until that is fixed in alef, callers reach it via
``kreuzberg._kreuzberg.register_embedding_backend``.
"""

from __future__ import annotations

import pytest

import kreuzberg


class _StubBackend:
    """Minimal Python-side embedding backend.

    Implements the duck-typed interface that the ``PyEmbeddingBackendBridge``
    invokes at registration and shutdown time: ``dimensions``, ``embed``,
    ``initialize``, ``shutdown``, and the introspection accessors used by the
    Plugin super-trait (``name``, ``version``).
    """

    cached_name: str = "stub-test-backend"

    def name(self) -> str:
        return self.cached_name

    def version(self) -> str:
        return "0.1.0-test"

    def initialize(self) -> None:
        return None

    def shutdown(self) -> None:
        return None

    def dimensions(self) -> int:
        return 4

    def embed(self, texts: list[str]) -> list[list[float]]:
        return [[0.1, 0.2, 0.3, 0.4] for _ in texts]


@pytest.fixture(autouse=True)
def _reset_embedding_registry() -> None:
    """Clear any registrations between tests so they don't leak state."""
    kreuzberg.clear_embedding_backends()
    yield
    kreuzberg.clear_embedding_backends()


def test_top_level_exports_present() -> None:
    """Drift item #4 from bb-t58i: unregister/list/clear must be re-exported."""
    assert hasattr(kreuzberg, "unregister_embedding_backend")
    assert hasattr(kreuzberg, "list_embedding_backends")
    assert hasattr(kreuzberg, "clear_embedding_backends")


def test_list_starts_empty() -> None:
    assert kreuzberg.list_embedding_backends() == []


def test_register_then_list_roundtrip() -> None:
    from kreuzberg._kreuzberg import register_embedding_backend

    register_embedding_backend(_StubBackend())
    names = kreuzberg.list_embedding_backends()
    assert _StubBackend.cached_name in names


def test_unregister_removes_backend() -> None:
    from kreuzberg._kreuzberg import register_embedding_backend

    register_embedding_backend(_StubBackend())
    assert _StubBackend.cached_name in kreuzberg.list_embedding_backends()

    kreuzberg.unregister_embedding_backend(_StubBackend.cached_name)
    assert _StubBackend.cached_name not in kreuzberg.list_embedding_backends()


def test_unregister_unknown_name_is_noop() -> None:
    """Unregistering a name that was never registered returns without raising.

    Mirrors the Rust core's ``unregister_embedding_backend`` semantics — it
    returns ``Ok(())`` for unknown names so callers can treat the call as
    idempotent. See ``crates/kreuzberg/src/plugins/embedding.rs``.
    """
    kreuzberg.unregister_embedding_backend("never-registered")


def test_clear_empties_registry() -> None:
    from kreuzberg._kreuzberg import register_embedding_backend

    register_embedding_backend(_StubBackend())
    assert kreuzberg.list_embedding_backends()

    kreuzberg.clear_embedding_backends()
    assert kreuzberg.list_embedding_backends() == []

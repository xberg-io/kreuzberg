"""Shared pytest configuration for Python E2E tests."""

import sys
from pathlib import Path

# Add repo root to path for imports
REPO_ROOT = Path(__file__).parent.parent.parent.parent
sys.path.insert(0, str(REPO_ROOT))


def pytest_configure(config):
    """Configure pytest with custom markers."""
    config.addinivalue_line(
        "markers",
        "e2e: marks tests as end-to-end (deselect with '-m \"not e2e\"')",
    )
    config.addinivalue_line(
        "markers",
        "slow: marks tests as slow (deselect with '-m \"not slow\"')",
    )


def pytest_collection_modifyitems(config, items):
    """Mark all tests as e2e by default."""
    for item in items:
        item.add_marker("e2e")

#!/usr/bin/env python3
"""Print the installed xberg Python package version."""

from __future__ import annotations

import sys


def main() -> int:
    try:
        import xberg  # type: ignore
    except Exception as exc:  # pragma: no cover - runtime helper
        print(f"Failed to import xberg: {exc}", file=sys.stderr)
        return 1

    print(f"Xberg version: {getattr(xberg, '__version__', 'unknown')}")
    return 0


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""Compare benchmark results for performance regression detection."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


def load_benchmark_results(file_path: Path) -> dict[str, Any]:
    """Load benchmark results from JSON file."""
    try:
        with file_path.open() as f:
            return json.load(f)
    except (FileNotFoundError, json.JSONDecodeError):
        sys.exit(1)


def compare_benchmarks(baseline: dict[str, Any], current: dict[str, Any], threshold: float = 0.2) -> bool:
    """Compare benchmark results and detect regressions.

    Args:
        baseline: Baseline benchmark results
        current: Current benchmark results
        threshold: Performance regression threshold (e.g., 0.2 = 20% slower)

    Returns:
        True if no significant regressions detected, False otherwise
    """
    baseline_benchmarks = {b["name"]: b for b in baseline["benchmarks"]}
    current_benchmarks = {b["name"]: b for b in current["benchmarks"]}

    regressions = []
    improvements = []

    for name, current_bench in current_benchmarks.items():
        if name not in baseline_benchmarks:
            continue

        baseline_bench = baseline_benchmarks[name]

        # Skip failed benchmarks
        if not current_bench.get("success", True) or not baseline_bench.get("success", True):
            continue

        # Compare duration
        baseline_duration = baseline_bench["duration"]
        current_duration = current_bench["duration"]

        if baseline_duration > 0:
            change_ratio = (current_duration - baseline_duration) / baseline_duration
            change_percent = change_ratio * 100

            if change_ratio > threshold:
                regressions.append((name, change_percent, baseline_duration, current_duration))
            elif change_ratio < -0.05:  # 5% improvement threshold
                # Note: using magic number for improvement threshold
                improvements.append((name, abs(change_percent), baseline_duration, current_duration))
            else:
                pass

    # Print summary

    if improvements:
        for _name, _improvement, _baseline_dur, _current_dur in improvements:
            pass

    if regressions:
        for _name, _regression, _baseline_dur, _current_dur in regressions:
            pass
        return False

    return True


def main() -> None:
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Compare benchmark results for performance regression detection")
    parser.add_argument("baseline", type=Path, help="Path to baseline benchmark results JSON")
    parser.add_argument("current", type=Path, help="Path to current benchmark results JSON")
    parser.add_argument(
        "--threshold", type=float, default=0.2, help="Performance regression threshold as decimal (default: 0.2 = 20%%)"
    )
    parser.add_argument(
        "--fail-on-regression", action="store_true", help="Exit with non-zero code if regressions detected"
    )

    args = parser.parse_args()

    baseline = load_benchmark_results(args.baseline)
    current = load_benchmark_results(args.current)

    no_regressions = compare_benchmarks(baseline, current, args.threshold)

    if args.fail_on_regression and not no_regressions:
        sys.exit(1)


if __name__ == "__main__":
    main()

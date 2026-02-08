"""Kreuzberg Python extraction wrapper for benchmark harness.

Supports four modes:
- sync: extract_file_sync() - synchronous extraction
- async: extract_file() - asynchronous extraction
- batch: batch_extract_files_sync() - synchronous batch extraction
- server: persistent mode reading paths from stdin
"""

from __future__ import annotations

import asyncio
import json
import sys
import time
from typing import Any

from kreuzberg import (
    ExtractionConfig,
    OcrConfig,
    batch_extract_files_sync,
    extract_file,
    extract_file_sync,
)


def extract_sync(file_path: str, ocr_enabled: bool) -> dict[str, Any]:
    """Extract using synchronous API."""
    # Use minimal config with cache disabled for benchmarking
    config = ExtractionConfig(use_cache=False)
    if ocr_enabled:
        config.ocr = OcrConfig(backend="tesseract")

    start = time.perf_counter()
    result = extract_file_sync(file_path, config=config)
    duration_ms = (time.perf_counter() - start) * 1000.0

    return {
        "content": result.content,
        "metadata": result.metadata or {},
        "_extraction_time_ms": duration_ms,
    }


async def extract_async(file_path: str, ocr_enabled: bool) -> dict[str, Any]:
    """Extract using asynchronous API."""
    # Use minimal config with cache disabled for benchmarking
    config = ExtractionConfig(use_cache=False)
    if ocr_enabled:
        config.ocr = OcrConfig(backend="tesseract")

    start = time.perf_counter()
    result = await extract_file(file_path, config=config)
    duration_ms = (time.perf_counter() - start) * 1000.0

    return {
        "content": result.content,
        "metadata": result.metadata or {},
        "_extraction_time_ms": duration_ms,
    }


def extract_batch_sync(file_paths: list[str], ocr_enabled: bool) -> list[dict[str, Any]]:
    """Extract multiple files using batch API."""
    # Use minimal config with cache disabled for benchmarking
    config = ExtractionConfig(use_cache=False)
    if ocr_enabled:
        config.ocr = OcrConfig(backend="tesseract")

    start = time.perf_counter()
    results = batch_extract_files_sync(file_paths, config=config)  # type: ignore[arg-type]
    total_duration_ms = (time.perf_counter() - start) * 1000.0

    per_file_duration_ms = total_duration_ms / len(file_paths) if file_paths else 0

    return [
        {
            "content": result.content,
            "metadata": result.metadata or {},
            "_extraction_time_ms": per_file_duration_ms,
            "_batch_total_ms": total_duration_ms,
        }
        for result in results
    ]


def run_server(ocr_enabled: bool) -> None:
    """Persistent server mode: read paths from stdin, write JSON to stdout."""
    # Signal readiness after Python + FFI initialization
    print("READY", flush=True)
    for line in sys.stdin:
        file_path = line.strip()
        if not file_path:
            continue
        start = time.perf_counter()
        try:
            payload = extract_sync(file_path, ocr_enabled)
            print(json.dumps(payload), flush=True)
        except Exception as e:
            duration_ms = (time.perf_counter() - start) * 1000.0
            print(json.dumps({"error": str(e), "_extraction_time_ms": duration_ms}), flush=True)


def main() -> None:
    ocr_enabled = False
    args = []
    for arg in sys.argv[1:]:
        if arg == "--ocr":
            ocr_enabled = True
        elif arg == "--no-ocr":
            ocr_enabled = False
        else:
            args.append(arg)

    if len(args) < 1:
        print("Usage: kreuzberg_extract.py [--ocr|--no-ocr] <mode> <file_path> [additional_files...]", file=sys.stderr)
        print("Modes: sync, async, batch, server", file=sys.stderr)
        sys.exit(1)

    mode = args[0]
    file_paths = args[1:]

    try:
        if mode == "server":
            run_server(ocr_enabled)

        elif mode == "sync":
            if len(file_paths) != 1:
                print("Error: sync mode requires exactly one file", file=sys.stderr)
                sys.exit(1)
            payload = extract_sync(file_paths[0], ocr_enabled)
            print(json.dumps(payload), end="")

        elif mode == "async":
            if len(file_paths) != 1:
                print("Error: async mode requires exactly one file", file=sys.stderr)
                sys.exit(1)
            payload = asyncio.run(extract_async(file_paths[0], ocr_enabled))
            print(json.dumps(payload), end="")

        elif mode == "batch":
            if len(file_paths) < 1:
                print("Error: batch mode requires at least one file", file=sys.stderr)
                sys.exit(1)

            if len(file_paths) == 1:
                results = extract_batch_sync(file_paths, ocr_enabled)
                print(json.dumps(results[0]), end="")
            else:
                results = extract_batch_sync(file_paths, ocr_enabled)
                print(json.dumps(results), end="")

        else:
            print(f"Error: Unknown mode '{mode}'. Use sync, async, batch, or server", file=sys.stderr)
            sys.exit(1)

    except Exception as e:
        print(f"Error extracting with Kreuzberg: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()

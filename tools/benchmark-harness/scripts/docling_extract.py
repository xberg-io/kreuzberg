# /// script
# requires-python = ">=3.10"
# dependencies = [
#     "docling>=2.64.1",
# ]
# ///
"""Docling extraction wrapper for benchmark harness.

Supports two modes:
- sync: convert() - synchronous single-file extraction
- batch: convert_all() - batch extraction for multiple files
- server: persistent mode reading paths from stdin
"""

from __future__ import annotations

import json
import multiprocessing as _mp
import os
import sys
import time
from typing import Any

from docling.document_converter import DocumentConverter


def create_converter(ocr_enabled: bool) -> DocumentConverter:
    """Create a DocumentConverter with appropriate settings."""
    if not ocr_enabled:
        try:
            from docling.datamodel.pipeline_options import PipelineOptions
            options = PipelineOptions(do_ocr=False)
            return DocumentConverter(pipeline_options=options)
        except (ImportError, TypeError):
            # Fallback if PipelineOptions API not available
            return DocumentConverter()
    return DocumentConverter()


def extract_sync(file_path: str, converter: DocumentConverter) -> dict[str, Any]:
    """Extract using synchronous single-file API."""
    start = time.perf_counter()
    result = converter.convert(file_path)
    markdown = result.document.export_to_markdown()
    duration_ms = (time.perf_counter() - start) * 1000.0

    return {
        "content": markdown,
        "metadata": {"framework": "docling"},
        "_extraction_time_ms": duration_ms,
    }


def extract_batch(file_paths: list[str], converter: DocumentConverter) -> list[dict[str, Any]]:
    """Extract multiple files using batch API."""
    start = time.perf_counter()
    results = converter.convert_all(file_paths, raises_on_error=False)
    total_duration_ms = (time.perf_counter() - start) * 1000.0

    per_file_duration_ms = total_duration_ms / len(file_paths) if file_paths else 0

    outputs = []
    for result in results:
        if result.status.name == "SUCCESS":
            markdown = result.document.export_to_markdown()
            outputs.append(
                {
                    "content": markdown,
                    "metadata": {"framework": "docling"},
                    "_extraction_time_ms": per_file_duration_ms,
                    "_batch_total_ms": total_duration_ms,
                }
            )
        else:
            outputs.append(
                {
                    "content": "",
                    "metadata": {
                        "framework": "docling",
                        "error": str(result.errors) if result.errors else "Unknown error",
                        "status": result.status.name,
                    },
                    "_extraction_time_ms": per_file_duration_ms,
                    "_batch_total_ms": total_duration_ms,
                }
            )

    return outputs


def _worker(fn, args, conn):
    """Run extraction in a forked child process.

    Closes inherited stdin/stdout so the child cannot corrupt the
    parent's line-based JSON protocol.
    """
    try:
        sys.stdin.close()
        sys.stdout = open(os.devnull, "w")
    except Exception:
        pass
    try:
        result = fn(*args)
        conn.send(result)
    except Exception as e:
        conn.send({"error": str(e), "_extraction_time_ms": 0})
    finally:
        conn.close()


def _run_with_timeout(fn, args, timeout):
    """Execute fn(*args) in a forked child with a timeout.

    On timeout the child is killed but the parent stays alive —
    no expensive process restart is needed.
    """
    try:
        ctx = _mp.get_context("fork")
        parent_conn, child_conn = ctx.Pipe(duplex=False)
        p = ctx.Process(target=_worker, args=(fn, args, child_conn))
        p.start()
        child_conn.close()

        if parent_conn.poll(timeout=timeout):
            try:
                result = parent_conn.recv()
            except Exception:
                result = {"error": "worker process crashed", "_extraction_time_ms": 0}
        else:
            p.kill()
            result = {
                "error": f"extraction timed out after {timeout}s",
                "_extraction_time_ms": timeout * 1000.0,
            }

        p.join(timeout=5)
        if p.is_alive():
            p.kill()
            p.join()
        parent_conn.close()
        return result
    except Exception:
        # Fork not available — fall back to in-process extraction
        try:
            return fn(*args)
        except Exception as e:
            return {"error": str(e), "_extraction_time_ms": 0}


def run_server(converter: DocumentConverter, timeout=None) -> None:
    """Persistent server mode: read paths from stdin, write JSON to stdout."""
    print("READY", flush=True)
    for line in sys.stdin:
        file_path = line.strip()
        if not file_path:
            continue
        if timeout is not None:
            result = _run_with_timeout(extract_sync, (file_path, converter), timeout)
        else:
            try:
                result = extract_sync(file_path, converter)
            except Exception as e:
                result = {"error": str(e), "_extraction_time_ms": 0}
        print(json.dumps(result), flush=True)


def main() -> None:
    ocr_enabled = False
    timeout = None
    args = []
    for arg in sys.argv[1:]:
        if arg == "--ocr":
            ocr_enabled = True
        elif arg == "--no-ocr":
            ocr_enabled = False
        elif arg.startswith("--timeout="):
            timeout = int(arg.split("=", 1)[1])
        else:
            args.append(arg)

    if len(args) < 1:
        print("Usage: docling_extract.py [--ocr|--no-ocr] [--timeout=SECS] <mode> <file_path> [additional_files...]", file=sys.stderr)
        print("Modes: sync, batch, server", file=sys.stderr)
        sys.exit(1)

    mode = args[0]
    file_paths = args[1:]

    # Create converter once (expensive initialization)
    converter = create_converter(ocr_enabled)

    try:
        if mode == "server":
            run_server(converter, timeout=timeout)

        elif mode == "sync":
            if len(file_paths) != 1:
                print("Error: sync mode requires exactly one file", file=sys.stderr)
                sys.exit(1)
            payload = extract_sync(file_paths[0], converter)
            print(json.dumps(payload), end="")

        elif mode == "batch":
            if len(file_paths) < 1:
                print("Error: batch mode requires at least one file", file=sys.stderr)
                sys.exit(1)

            if len(file_paths) == 1:
                results = extract_batch(file_paths, converter)
                print(json.dumps(results[0]), end="")
            else:
                results = extract_batch(file_paths, converter)
                print(json.dumps(results), end="")

        else:
            print(f"Error: Unknown mode '{mode}'. Use sync, batch, or server", file=sys.stderr)
            sys.exit(1)

    except Exception as e:
        print(f"Error extracting with Docling: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()

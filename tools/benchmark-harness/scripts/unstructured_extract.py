# /// script
# requires-python = ">=3.10"
# dependencies = [
#     "unstructured[all-docs]>=0.18.21",
# ]
# ///
"""Unstructured extraction wrapper for benchmark harness."""

from __future__ import annotations

import json
import multiprocessing as _mp
import os
import sys
import time

from unstructured.partition.auto import partition


def extract_sync(file_path: str, ocr_enabled: bool) -> dict:
    """Extract using Unstructured partition API."""
    strategy = "hi_res" if ocr_enabled else "fast"
    start = time.perf_counter()
    elements = partition(filename=file_path, strategy=strategy, languages=["eng"])
    duration_ms = (time.perf_counter() - start) * 1000.0

    text = "\n\n".join(str(el) for el in elements)
    return {
        "content": text,
        "metadata": {"framework": "unstructured", "strategy": strategy},
        "_extraction_time_ms": duration_ms,
    }


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


def run_server(ocr_enabled: bool, timeout=None) -> None:
    """Persistent server mode: read paths from stdin, write JSON to stdout."""
    print("READY", flush=True)
    for line in sys.stdin:
        file_path = line.strip()
        if not file_path:
            continue
        if timeout is not None:
            result = _run_with_timeout(extract_sync, (file_path, ocr_enabled), timeout)
        else:
            try:
                result = extract_sync(file_path, ocr_enabled)
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
        print("Usage: unstructured_extract.py [--ocr|--no-ocr] [--timeout=SECS] <mode> <file_path>", file=sys.stderr)
        print("Modes: sync, server", file=sys.stderr)
        sys.exit(1)

    mode = args[0]

    if mode == "server":
        run_server(ocr_enabled, timeout=timeout)
    elif mode == "sync":
        if len(args) < 2:
            print("Error: sync mode requires a file path", file=sys.stderr)
            sys.exit(1)
        try:
            payload = extract_sync(args[1], ocr_enabled)
            print(json.dumps(payload), end="")
        except Exception as e:
            print(f"Error extracting with Unstructured: {e}", file=sys.stderr)
            sys.exit(1)
    else:
        # Legacy mode: first arg is the file path directly
        try:
            payload = extract_sync(args[0], ocr_enabled)
            print(json.dumps(payload), end="")
        except Exception as e:
            print(f"Error extracting with Unstructured: {e}", file=sys.stderr)
            sys.exit(1)


if __name__ == "__main__":
    main()

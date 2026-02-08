#!/usr/bin/env python3
"""Test script for the fork-based extraction timeout mechanism.

Simulates slow and fast extractions in persistent server mode to verify:
1. Fast extractions complete normally through the fork
2. Slow extractions are killed by the Python-side timeout
3. The parent process stays alive after a timeout (no restart needed)
4. Subsequent fast extractions still work after a timeout

Usage:
    python3 test_fork_timeout.py --timeout=3 server
    # Then send file paths via stdin. Files containing "SLOW" sleep for 10s.
"""

from __future__ import annotations

import json
import multiprocessing as _mp
import os
import sys
import time


def extract_sync(file_path: str) -> dict:
    """Simulate extraction — sleep 10s for files containing 'SLOW'."""
    start = time.perf_counter()

    if "SLOW" in file_path:
        time.sleep(10)

    try:
        with open(file_path, "r", errors="replace") as f:
            content = f.read()
    except Exception as e:
        content = f"error reading: {e}"

    duration_ms = (time.perf_counter() - start) * 1000.0
    return {
        "content": content[:500],
        "metadata": {"framework": "test-fork-timeout"},
        "_extraction_time_ms": duration_ms,
    }


def _worker(fn, args, conn):
    """Run extraction in a forked child process."""
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
    """Execute fn(*args) in a forked child with a timeout."""
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
        try:
            return fn(*args)
        except Exception as e:
            return {"error": str(e), "_extraction_time_ms": 0}


def run_server(timeout=None) -> None:
    """Persistent server mode."""
    print("READY", flush=True)
    for line in sys.stdin:
        file_path = line.strip()
        if not file_path:
            continue
        if timeout is not None:
            result = _run_with_timeout(extract_sync, (file_path,), timeout)
        else:
            try:
                result = extract_sync(file_path)
            except Exception as e:
                result = {"error": str(e), "_extraction_time_ms": 0}
        print(json.dumps(result), flush=True)


def main() -> None:
    timeout = None
    args = []
    for arg in sys.argv[1:]:
        if arg.startswith("--timeout="):
            timeout = int(arg.split("=", 1)[1])
        else:
            args.append(arg)

    if len(args) < 1:
        print("Usage: test_fork_timeout.py [--timeout=SECS] <mode>", file=sys.stderr)
        sys.exit(1)

    mode = args[0]
    if mode == "server":
        run_server(timeout=timeout)
    else:
        print(f"Unknown mode: {mode}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()

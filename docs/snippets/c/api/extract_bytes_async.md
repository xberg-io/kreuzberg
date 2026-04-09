!!! Note "Not Applicable"
    The C FFI provides synchronous extraction only. Use `kreuzberg_extract_bytes_sync`
    for in-memory extraction. For concurrent extraction, use multiple threads with
    `kreuzberg_extract_bytes_sync` — the API is fully thread-safe.

---
priority: high
---

- GIL management: use py.allow_threads() for expensive Rust operations
- Cache frequently-accessed Python data in Rust fields to minimize GIL acquisitions
- Use tokio::task::spawn_blocking for async calls to Python backends
- Python exception translation: convert Python exceptions to Rust errors with full context
- Data type mapping: Python str <-> Rust String, Python bytes <-> Rust Vec<u8>, Python dict <-> Rust HashMap
- Validate Python plugin protocol compliance on registration
- Target GIL overhead: 5-55us per acquisition

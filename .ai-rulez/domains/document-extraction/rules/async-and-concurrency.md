---
priority: high
---

- All extraction paths must be fully async using tokio
- Never block the async runtime — use spawn_blocking for CPU-intensive work
- All public types must be Send + Sync
- Use tokio::select! for timeout handling on extraction operations
- Cross-platform: test on Linux (amd64, arm64) and macOS at minimum

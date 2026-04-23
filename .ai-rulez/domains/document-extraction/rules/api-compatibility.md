---
priority: high
---

- Follow semantic versioning — breaking changes require major version bump
- Document all public API changes in CHANGELOG.md
- Maintain backward compatibility for at least one minor version before removing deprecated APIs
- All public types must be FFI-friendly or have FFI-compatible equivalents
- Version in Cargo.toml is the single source of truth for all binding packages

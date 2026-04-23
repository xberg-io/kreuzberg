---
priority: high
---

- 95% test coverage on core extraction code, 80% on bindings
- Test all format categories: text, office, PDF, images, archives, markup
- Test corrupted/malformed documents — extraction must fail gracefully, never panic
- Benchmark extraction speeds per format — track regressions in CI
- Test both success and error paths for every extractor
- Use property-based testing for parsers with wide input ranges

---
priority: critical
---

- Always use SecurityLimits validators for user content: ZipBombValidator, DepthValidator, StringGrowthValidator
- Validate MIME type before extraction — never trust file extensions alone
- Implement fallback chains: if primary extractor fails, try next-priority extractor
- Preserve partial results on failure — return what was extracted with error context
- All errors must include: operation name, input description, root cause, and suggestion
- Never expose internal file paths or system details in error messages returned to users

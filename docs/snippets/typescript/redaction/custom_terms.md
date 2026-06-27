```typescript title="TypeScript"
const output = await extract({
    kind: "uri",
    uri: "contract.pdf",
}, {
    redaction: {
        strategy: "token_replace",
        customTerms: [
            { label: "Project", value: "Project Polaris" },
            { label: "Employee", value: "EMP-7421", caseSensitive: true },
        ],
        customPatterns: [
            { label: "InternalId", pattern: "INT-\\d{6}" },
        ],
    },
});
```

```typescript title="TypeScript"
const output = await extract({
    kind: "uri",
    uri: "contract.pdf",
}, {
    ner: {
        backend: "llm",
        llm: { model: "openai/gpt-4o-mini" },
        customLabels: ["Treatment", "Vessel", "Product"],
    },
});
```

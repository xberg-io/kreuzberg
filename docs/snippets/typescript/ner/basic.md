```typescript title="TypeScript"
import { extract } from '@xberg-io/xberg';

const output = await extract({
    kind: "uri",
    uri: "contract.pdf",
}, {
    ner: {
        backend: "llm",
        llm: { model: "openai/gpt-4o-mini" },
    },
});

for (const entity of output.results[0].entities ?? []) {
    console.log(`${entity.category}: ${entity.text}`);
}
```

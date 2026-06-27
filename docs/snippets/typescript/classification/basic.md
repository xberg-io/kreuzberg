```typescript title="TypeScript"
import { extract } from '@xberg-io/xberg';

const output = await extract({
    kind: "uri",
    uri: "packet.pdf",
}, {
    pageClassification: {
        labels: ["invoice", "contract", "id_document", "receipt"],
        llm: { model: "openai/gpt-4o-mini" },
    },
});

for (const page of output.results[0].pageClassifications ?? []) {
    console.log(`page ${page.pageNumber}: ${page.labels[0]?.label}`);
}
```

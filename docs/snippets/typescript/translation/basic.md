```typescript title="TypeScript"
import { extract } from '@xberg-io/xberg';

const output = await extract({
    kind: "uri",
    uri: "contract.pdf",
}, {
    translation: {
        targetLang: "de",
        llm: { model: "openai/gpt-4o-mini" },
    },
});
if (output.results[0].translation) {
    console.log(output.results[0].translation.content);
}
```

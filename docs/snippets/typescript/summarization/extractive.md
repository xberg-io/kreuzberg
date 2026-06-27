```typescript title="TypeScript"
import { extract } from '@xberg-io/xberg';

const output = await extract({
    kind: "uri",
    uri: "report.pdf",
}, {
    summarization: {
        strategy: "extractive",
        maxTokens: 200,
    },
});
if (output.results[0].summary) {
    console.log(output.results[0].summary.text);
}
```

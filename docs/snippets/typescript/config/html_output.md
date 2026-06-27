```typescript title="TypeScript"
import { extract } from "@xberg-io/xberg";

const output = await extract({
  kind: "uri",
  uri: "document.pdf",
}, {
  outputFormat: "html",
  htmlOutput: {
    theme: "github",
    embedCss: true,
  },
});
console.log(output.results[0].content); // HTML with kb-* classes
```

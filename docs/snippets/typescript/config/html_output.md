```typescript title="TypeScript"
import { extractFile } from "xberg";

const result = await extractFile("document.pdf", {
  outputFormat: "html",
  htmlOutput: {
    theme: "github",
    embedCss: true,
  },
});
console.log(result.content); // HTML with kb-* classes
```

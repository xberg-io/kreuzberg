```typescript title="TypeScript"
import { extractFile } from "@xberg/node";

const config = {
  tokenReduction: {
    mode: "moderate",
    preserveImportantWords: true,
  },
};

const result = await extractFile("document.pdf", null, config);
console.log(result.content);
```

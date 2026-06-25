```typescript title="TypeScript"
import { extractFile } from "@xberg/node";

const config = {
  enableQualityProcessing: true,
};

const result = await extractFile("document.pdf", null, config);
console.log(result.content);
```

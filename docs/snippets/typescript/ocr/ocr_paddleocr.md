```typescript title="TypeScript"
import { extractFileSync } from "@xberg/node";

const config = {
  ocr: {
    backend: "paddle-ocr",
    language: "en",
    // modelTier: 'server', // for max accuracy
  },
};

const result = extractFileSync("scanned.pdf", null, config);
console.log(result.content);
```

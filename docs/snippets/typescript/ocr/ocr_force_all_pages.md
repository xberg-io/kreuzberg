```typescript title="TypeScript"
import { extractFileSync } from "@xberg/node";

const config = {
  ocr: {
    backend: "tesseract",
  },
  forceOcr: true,
};

const result = extractFileSync("document.pdf", null, config);
console.log(result.content);
```

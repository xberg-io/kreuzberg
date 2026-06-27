```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const config = {
  tokenReduction: {
    mode: "moderate",
    preserveImportantWords: true,
  },
};

const result = await extract({ kind: "uri", uri: "verbose_document.pdf" }, config);
console.log(`Content length: ${result.content.length}`);
console.log(`MIME type: ${result.mimeType}`);
```

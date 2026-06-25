```typescript title="WASM"
import init, { extractFile } from "xberg-wasm";

await init();

const config = {
  tokenReduction: {
    mode: "moderate",
    preserveImportantWords: true,
  },
};

const result = await extractFile("document.pdf", undefined, config);
console.log(result.content);
```

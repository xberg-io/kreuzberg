```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const config = {
  tokenReduction: {
    mode: "moderate",
    preserveImportantWords: true,
  },
};

const result = await extract({ kind: "uri", uri: "document.pdf" }, config);
console.log(result.content);
```

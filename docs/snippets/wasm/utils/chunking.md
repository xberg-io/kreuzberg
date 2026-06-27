```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const config = {
  chunking: {
    maxChars: 1500,
    chunkOverlap: 200,
  },
};

const result = await extract({ kind: "uri", uri: "document.pdf" }, config);
console.log(`Chunks created: ${result.chunks?.length ?? 0}`);
```

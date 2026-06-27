```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const data = new Uint8Array([0x25, 0x50, 0x44, 0x46]); // PDF magic bytes
const result = await extract({ kind: "bytes", bytes: data, mimeType: "application/pdf" }, undefined);
console.log(result.content);
```

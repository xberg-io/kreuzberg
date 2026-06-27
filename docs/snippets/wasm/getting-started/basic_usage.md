```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const response = await fetch("document.pdf");
const data = new Uint8Array(await response.arrayBuffer());

const result = await extract({ kind: "bytes", bytes: data, mimeType: "application/pdf" }, undefined);
console.log(result.content);
console.log(`MIME Type: ${result.mime_type}`);
```

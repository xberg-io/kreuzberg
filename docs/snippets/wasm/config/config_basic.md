```typescript title="WASM"
import { initWasm, extractBytes } from "@xberg/wasm";

await initWasm();

const config = {
  ocr: {
    backend: "tesseract-wasm",
    language: "eng",
  },
  images: {
    extractImages: true,
  },
};

const bytes = new Uint8Array(buffer);
const result = await extractBytes(bytes, "application/pdf", config);
console.log(result.content);
```

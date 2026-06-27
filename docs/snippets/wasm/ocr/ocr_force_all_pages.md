```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const documentData = await fetch("document.pdf").then((res) => res.arrayBuffer());

const result = await extract(documentData, "application/pdf", {
  force_ocr: true,
  ocr: {
    backend: "tesseract",
    language: "eng",
  },
});

console.log(result.content);
```

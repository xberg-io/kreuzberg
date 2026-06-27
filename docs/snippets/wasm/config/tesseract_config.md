```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const data = new Uint8Array(await fetch("scanned.pdf").then((r) => r.arrayBuffer()));

const config = {
  ocr: {
    backend: "tesseract",
    language: "eng+deu",
    tesseract_config: {
      psm: 6,
      oem: 3,
    },
  },
};

const result = await extract({ kind: "bytes", bytes: data, mimeType: "application/pdf" }, config);
console.log(`OCR text: ${result.content}`);
```

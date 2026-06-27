```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const multilingualData = await fetch("multilingual.pdf").then((res) => res.arrayBuffer());

const result = await extract(multilingualData, "application/pdf", {
  ocr: {
    backend: "tesseract",
    language: "eng+deu+fra",
  },
});

console.log(result.content);
```

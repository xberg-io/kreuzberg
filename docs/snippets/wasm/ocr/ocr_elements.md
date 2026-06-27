```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const documentData = await fetch("scanned.pdf").then((res) => res.arrayBuffer());

const result = await extract(documentData, "application/pdf", {
  ocr: {
    backend: "tesseract",
    language: "eng",
    element_config: {
      include_elements: true,
    },
  },
});

if (result.ocr_elements) {
  for (const element of result.ocr_elements) {
    console.log("Text:", element.text);
    console.log("Confidence:", element.confidence);
  }
}
```

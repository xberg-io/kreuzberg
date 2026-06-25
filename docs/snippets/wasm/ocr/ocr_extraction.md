```typescript title="WASM (Browser)"
import { enableOcr, extractFromFile, initWasm } from "@xberg/wasm";

await initWasm();
await enableOcr();

const fileInput = document.getElementById("file") as HTMLInputElement;
const file = fileInput.files?.[0];

if (file) {
  const result = await extractFromFile(file, file.type, {
    ocr: {
      backend: "xberg-tesseract",
      language: "eng",
    },
  });
  console.log(result.content);
}
```

```typescript title="WASM (Node.js / Deno / Bun)"
import { enableOcr, extractFile, initWasm } from "@xberg/wasm";

await initWasm();
await enableOcr(); // Uses native xberg-tesseract backend

const result = await extractFile("./scanned_document.png", "image/png", {
  ocr: {
    backend: "xberg-tesseract",
    language: "eng",
  },
});
console.log(result.content);
```

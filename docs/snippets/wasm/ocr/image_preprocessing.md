```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const documentData = await fetch("document.pdf").then((res) => res.arrayBuffer());

const result = await extract(documentData, "application/pdf", {
  images: {
    extract_images: true,
    target_dpi: 300,
    max_image_dimension: 2000,
  },
});

console.log(result.content);
```

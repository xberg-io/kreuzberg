```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const imageData = await fetch("document.pdf").then((res) => res.arrayBuffer());

const result = await extract(imageData, "application/pdf", {
  images: {
    extract_images: true,
  },
});

console.log(result.images);
```

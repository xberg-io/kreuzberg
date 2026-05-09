```typescript title="WASM"
import init, { extractBytes } from "kreuzberg-wasm";

await init();

const data = new Uint8Array(await fetch("document.pdf").then(r => r.arrayBuffer()));

const config = {
  chunking: {
    max_characters: 1000,
    overlap: 200,
    embedding: {
      model: {
        preset: {
          name: "balanced",
        },
      },
      batch_size: 16,
      normalize: true,
      show_download_progress: true,
    },
  },
};

const result = await extractBytes(data, "application/pdf", config);
console.log(`Chunks with embeddings: ${result.chunks?.length || 0}`);
```

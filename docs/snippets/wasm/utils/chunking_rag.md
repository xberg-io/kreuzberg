```typescript title="WASM"
import { initWasm, extract } from "@xberg-io/xberg-wasm";

await initWasm();

const config = {
  chunking: {
    maxChars: 500,
    chunkOverlap: 50,
  },
};

const result = await extract({ kind: "uri", uri: "research_paper.pdf" }, config);

if (result.chunks) {
  for (const chunk of result.chunks) {
    const meta = chunk.metadata;
    console.log(`Chunk ${meta.chunkIndex + 1}/${meta.totalChunks}`);
    console.log(`Position: ${meta.byteStart}-${meta.byteEnd}`);
    console.log(`Content: ${chunk.content.slice(0, 100)}...`);
  }
}
```

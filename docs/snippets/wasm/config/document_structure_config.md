```typescript title="Document Structure Config (WASM)"
import { extractBytes } from "xberg-wasm";

const config = {
  includeDocumentStructure: true,
};

const result = extractBytes(fileBuffer, "application/pdf", config);

if (result.document) {
  for (const node of result.document.nodes) {
    console.log(`[${node.content.nodeType}]`);
  }
}
```

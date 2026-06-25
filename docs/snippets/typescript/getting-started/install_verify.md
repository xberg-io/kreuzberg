```typescript title="TypeScript"
import { getVersion, extractFileSync } from "@xberg/node";

const version = getVersion();
console.log(`Xberg version: ${version}`);

const result = extractFileSync("document.pdf");
console.log(`Extraction successful: ${result.success}`);
```

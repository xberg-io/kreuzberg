```typescript title="TypeScript"
import { extractFile } from "@xberg/node";

const result = await extractFile("document.pdf");

console.log(result.content);
console.log(`Tables: ${result.tables.length}`);
console.log(`Metadata: ${JSON.stringify(result.metadata)}`);
```

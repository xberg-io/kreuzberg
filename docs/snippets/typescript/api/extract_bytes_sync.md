```typescript title="TypeScript"
import { extractBytesSync } from "xberg";
import { readFileSync } from "fs";

const content = readFileSync("document.pdf");
const result = extractBytesSync(content, "application/pdf");

console.log(result.content);
console.log(`Tables: ${result.tables?.length ?? 0}`);
```

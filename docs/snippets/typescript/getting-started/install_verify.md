```typescript title="TypeScript"
import { getVersion, extract } from "@xberg-io/xberg";

const version = getVersion();
console.log(`Xberg version: ${version}`);

const output = await extract({
  kind: "uri",
  uri: "document.pdf",
});
console.log(`Extraction successful: ${output.errors?.length === 0}`);
```

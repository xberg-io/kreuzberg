```typescript title="TypeScript"
import { extract } from "@xberg-io/xberg";

const output = await extract({
  kind: "uri",
  uri: "document.pdf",
});
console.log(output.results[0].content);
```

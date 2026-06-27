```typescript title="TypeScript"
import { extract } from "@xberg-io/xberg";

try {
  const output = await extract({
    kind: "uri",
    uri: "missing.pdf",
  });
  console.log(output.results[0].content);
} catch (error: unknown) {
  if (error instanceof Error) {
    console.error(`Extraction failed: ${error.message}`);
  }
  throw error;
}
```

```typescript title="TypeScript"
import { extract } from "@xberg-io/xberg";

const output = await extract({
  kind: "uri",
  uri: "document.pdf",
});

const doc = output.results[0];
console.log(`Content: ${doc.content}`);
console.log(`Success: ${output.errors?.length === 0}`);
console.log(`Content Length: ${doc.content.length}`);

if (doc.metadata?.page_count) {
  console.log(`Pages: ${doc.metadata.page_count}`);
}
```

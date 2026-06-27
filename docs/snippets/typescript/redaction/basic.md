```typescript title="TypeScript"
import { extract } from '@xberg-io/xberg';

const output = await extract({
    kind: "uri",
    uri: "contract.pdf",
}, {
    redaction: {
        categories: ["email", "phone", "ssn", "credit_card", "iban"],
        strategy: "mask",
    },
});
const result = output.results[0];
console.log(result.content);
console.log(`Redacted ${result.redactionReport?.totalRedacted ?? 0} spans`);
```

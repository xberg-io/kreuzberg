```typescript title="TypeScript"
import { extractFile } from '@xberg/node';

const result = await extractFile("ticket.pdf", { qrCodes: true });
for (const image of result.images ?? []) {
    for (const qr of image.qrCodes ?? []) {
        console.log(qr.payload);
    }
}
```

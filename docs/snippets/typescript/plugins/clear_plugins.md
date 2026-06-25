```typescript title="TypeScript"
import { clearOcrBackends, clearPostProcessors, clearValidators } from "@xberg/node";

clearOcrBackends();
clearPostProcessors();
clearValidators();

console.log("All plugins cleared");
```

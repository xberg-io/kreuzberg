# Clear All Registered Plugins

Clear all registered OCR backends, post-processors, or validators from the global registry.

```typescript title="WASM"
import { initWasm, clearOcrBackends, clearPostProcessors, clearValidators } from "@xberg-io/xberg-wasm";

await initWasm();

// Clear all OCR backends
clearOcrBackends();
console.log("OCR backends cleared");

// Clear all post-processors
clearPostProcessors();
console.log("Post-processors cleared");

// Clear all validators
clearValidators();
console.log("Validators cleared");
```

Use when you need to reset the plugin registries to their initial state or remove all custom plugins.

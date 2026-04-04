# @kreuzberg/test-utils

Shared test utilities for Kreuzberg e2e and integration tests.

## Features

- **Config Mapping**: Runtime-agnostic utilities for mapping plain objects to typed config objects
- **Assertion Adapters**: Unified assertion interface with adapters for Vitest and Deno
- **Fixture Helpers**: Utilities for loading and running test fixtures

## Usage

### Config Mapping

```typescript
import { buildConfig } from '@kreuzberg/test-utils/config-mapping';

const config = buildConfig({
  use_cache: true,
  max_concurrent_extractions: 4,
  ocr: {
    backend: 'tesseract',
    language: 'eng'
  }
});
```

### Assertions

```typescript
// For Vitest
import { createAssertions, VitestAdapter } from '@kreuzberg/test-utils/assertions';

const assertions = createAssertions(new VitestAdapter());

// For Deno
import { createAssertions, DenoAdapter } from '@kreuzberg/test-utils/assertions';

const assertions = createAssertions(new DenoAdapter());
```

## License

MIT

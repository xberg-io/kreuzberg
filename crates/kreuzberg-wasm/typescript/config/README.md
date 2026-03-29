# WASM Configuration Tests

Comprehensive configuration tests for the Kreuzberg WASM binding, covering all 15 configuration types required for document extraction control.

## Files Overview

### Configuration Test Files

1. **extraction-config.spec.ts** (194 LOC)
   - Tests for `ExtractionConfig` (root configuration)
   - Covers type definitions, WASM serialization, worker communication
   - Tests for nested configurations and immutability patterns

2. **chunking-config.spec.ts** (153 LOC)
   - Tests for `ChunkingConfig`
   - Maximum character limits and overlap settings
   - Type safety and edge cases (zero chunk size, very large chunks)

3. **keyword-config.spec.ts** (302 LOC)
   - Tests for `KeywordConfig` with YAKE and RAKE parameters
   - Tests `YakeParams` (windowSize) and `RakeParams` (minWordLength, maxWordsPerPhrase)
   - Algorithm selection and n-gram range handling

4. **image-extraction-config.spec.ts** (209 LOC)
   - Tests for `ImageExtractionConfig`
   - DPI settings (targetDpi, minDpi, maxDpi)
   - Image dimension limits and auto-adjustment

5. **ocr-config.spec.ts** (284 LOC)
   - Tests for `OcrConfig`
   - Backend selection and language configuration
   - Nested `TesseractConfig` within OcrConfig
   - Multiple languages array support

6. **pdf-config.spec.ts** (212 LOC)
   - Tests for `PdfConfig`
   - Image extraction and metadata extraction
   - Password handling for encrypted PDFs

7. **simple-configs.spec.ts** (382 LOC)
   - Tests for simpler configuration types in one file:
     - `TokenReductionConfig`: mode and word preservation
     - `PostProcessorConfig`: enabled processors and disabled processors
     - `PageExtractionConfig`: page extraction and marker formats
     - `LanguageDetectionConfig`: language detection enablement
     - `TesseractConfig`: PSM values, table detection, character whitelist

8. **composite-configs.spec.ts** (399 LOC)
   - Integration tests for complex hierarchical configurations
   - Full-featured extraction scenarios
   - Configuration composition and merging patterns
   - Immutable update patterns for nested structures

## Configuration Types Tested

All 15 configuration types from the requirements are covered:

1. ✓ ExtractionConfig (root config) - extraction-config.spec.ts
2. ✓ ChunkingConfig - chunking-config.spec.ts
3. ✓ EmbeddingConfig - (embedded in ChunkingConfig patterns)
4. ✓ ImageExtractionConfig - image-extraction-config.spec.ts
5. ✓ ImagePreprocessingConfig - (patterns in composite-configs.spec.ts)
6. ✓ KeywordConfig (YAKE/RAKE parameters) - keyword-config.spec.ts
7. ✓ LanguageDetectionConfig - simple-configs.spec.ts
8. ✓ OcrConfig - ocr-config.spec.ts
9. ✓ TesseractConfig - simple-configs.spec.ts (nested in OcrConfig)
10. ✓ PdfConfig - pdf-config.spec.ts
11. ✓ PageExtractionConfig - simple-configs.spec.ts
12. ✓ PostProcessorConfig - simple-configs.spec.ts
13. ✓ TokenReductionConfig - simple-configs.spec.ts
14. ✓ HierarchyConfig (hierarchy patterns) - composite-configs.spec.ts
15. ✓ FontConfig - (PDF options patterns in pdf-config.spec.ts)

## Test Coverage by Category

### Type Definitions

- Valid type creation with all fields
- Optional field support (undefined handling)
- Nested and composite types
- Algorithm selection and presets

### WASM Serialization

- JSON.stringify/parse round-trip tests
- Undefined field omission
- Nested structure serialization
- Complex type serialization (arrays, objects)

### Worker Message Passing

- structuredClone compatibility
- Deep nesting preservation
- ExtractionConfig composition in workers
- Complex nested extraction configs

### Type Safety

- Type enforcement for booleans, numbers, strings
- Array type validation
- Nested object type validation
- Union type validation (e.g., KeywordAlgorithm)

### Edge Cases

- Zero values (zero chunk size, zero DPI, zero keywords)
- Very large values (100000+ chunk sizes, 1000+ concurrent operations)
- Empty arrays and strings
- Boundary values (0.0-1.0 for scores, min-max ranges)

### Immutability Patterns

- Spread operator updates
- Nested object spreading
- Selective field overrides
- Complex nested updates

### Nesting in ExtractionConfig

- Proper nesting of all config types
- Null config handling
- Configuration composition with other options
- Multiple nested configs simultaneously

## Test Statistics

- **Total Files**: 8 spec files
- **Total Lines of Code**: 2,135 LOC
- **Test Suites**: 23+ describe blocks
- **Test Cases**: 100+ it blocks
- **Coverage Areas**:
  - Type definitions: Comprehensive
  - Serialization: Comprehensive
  - Worker communication: Full coverage
  - Type safety: Full coverage
  - Edge cases: Extensive
  - Immutability: Full coverage
  - Composition: Full coverage

## Running the Tests

```bash
# Run all WASM config tests
vitest crates/kreuzberg-wasm/typescript/config/

# Run specific config test file
vitest crates/kreuzberg-wasm/typescript/config/extraction-config.spec.ts

# Run with coverage
vitest --coverage crates/kreuzberg-wasm/typescript/config/
```

## Test Patterns Used

### Describe Block Structure

```text
- Type definitions
- WASM serialization
- Worker message passing
- Type safety
- Edge cases
- Immutability patterns
- Nesting in parent config
- Integration scenarios
```

### Common Test Patterns

1. Direct type assertion tests
2. JSON serialization round-trip tests
3. structuredClone worker communication tests
4. Type enforcement with typeof checks
5. Boundary and edge case tests
6. Immutability via spread operator tests
7. Deep nesting and composition tests

## Design Decisions

1. **File Organization**: Config types grouped by complexity
   - Simple types (5) in one file (simple-configs.spec.ts)
   - Medium complexity (3) in individual files
   - Complex types (2) in individual files
   - Integration patterns in composite-configs.spec.ts

2. **Test Scope**: Focused on WASM-specific concerns
   - JSON serialization boundaries
   - Worker communication patterns
   - Type safety for TypeScript
   - Composition and nesting

3. **Naming Convention**: "WASM: ConfigName" format for clarity
   - Distinguishes WASM tests from other binding tests
   - Consistent with project standards

4. **Coverage Focus**: Practical usage patterns
   - Real-world configuration scenarios
   - Error prevention (type safety, boundaries)
   - Performance considerations (memory efficiency)
   - Interoperability (worker communication)

## Future Enhancements

- Add snapshot tests for complex configurations
- Add performance benchmarks for large config arrays
- Add integration tests with actual WASM module
- Add E2E tests with document extraction workflows

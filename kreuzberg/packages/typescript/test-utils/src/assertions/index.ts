/**
 * Assertion adapters and utilities for cross-platform testing
 * @module assertions
 */

export { DenoAdapter } from "./deno-adapter.js";
export type { ExtractionAssertions, MetadataExpectation } from "./factory.js";
export { createAssertions } from "./factory.js";
// Export only assertion-specific types/functions, not duplicated ones like PlainRecord/isPlainRecord
export type { AssertionAdapter, ExtractionResult } from "./types.js";
export { VitestAdapter } from "./vitest-adapter.js";

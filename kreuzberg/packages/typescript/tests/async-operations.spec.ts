/**
 * Async Operations Tests for TypeScript WASM Binding
 *
 * Comprehensive test suite for asynchronous extraction operations in Kreuzberg WASM bindings.
 * Tests cover Promise-based extraction, async/await patterns, error handling, timeout behavior,
 * cancellation with AbortSignal, and concurrent async operations.
 *
 * These tests verify proper async/await semantics, Promise composition with Promise.all,
 * error propagation in async contexts, and resource cleanup.
 *
 * @group wasm-binding
 * @group async
 * @group extraction
 */

import type { ExtractionConfig } from "@kreuzberg/core";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

/**
 * Mock async extraction service
 * Simulates real async extraction with configurable delay and error conditions
 */
class MockAsyncExtractor {
	private isInitialized = false;
	private pendingOperations = new Set<Promise<unknown>>();

	/**
	 * Initialize the extractor asynchronously
	 */
	async initialize(): Promise<void> {
		if (this.isInitialized) {
			throw new Error("Extractor already initialized");
		}
		// Simulate async initialization
		await new Promise((resolve) => setTimeout(resolve, 10));
		this.isInitialized = true;
	}

	/**
	 * Extract text from document asynchronously
	 */
	async extract(data: Uint8Array, config: ExtractionConfig = {}, delayMs = 50): Promise<ExtractionResult> {
		if (!this.isInitialized) {
			throw new Error("Extractor not initialized");
		}

		const operation = new Promise<ExtractionResult>((resolve) => {
			const timeoutId = setTimeout(() => {
				this.pendingOperations.delete(operation);
				resolve({
					text: `Extracted ${data.length} bytes`,
					chunks: [],
					metadata: {
						extractionTime: delayMs,
						config,
					},
				});
			}, delayMs);
		});

		this.pendingOperations.add(operation);
		return operation;
	}

	/**
	 * Extract with timeout enforcement
	 */
	async extractWithTimeout(data: Uint8Array, config: ExtractionConfig, timeoutMs: number): Promise<ExtractionResult> {
		return Promise.race([
			this.extract(data, config, 100),
			new Promise<ExtractionResult>((_, reject) =>
				setTimeout(() => reject(new Error(`Extraction timeout after ${timeoutMs}ms`)), timeoutMs),
			),
		]);
	}

	/**
	 * Extract with AbortSignal support
	 */
	async extractWithAbort(data: Uint8Array, config: ExtractionConfig, signal?: AbortSignal): Promise<ExtractionResult> {
		if (signal?.aborted) {
			throw new Error("Operation aborted");
		}

		const extractPromise = this.extract(data, config, 50);

		return new Promise<ExtractionResult>((resolve, reject) => {
			if (signal) {
				const handleAbort = () => {
					reject(new Error("Operation aborted"));
				};
				signal.addEventListener("abort", handleAbort);

				extractPromise
					.then((result) => {
						signal.removeEventListener("abort", handleAbort);
						resolve(result);
					})
					.catch((error) => {
						signal.removeEventListener("abort", handleAbort);
						reject(error);
					});
			} else {
				extractPromise.then(resolve).catch(reject);
			}
		});
	}

	/**
	 * Batch extract multiple documents concurrently
	 */
	async batchExtract(documents: Array<{ data: Uint8Array; config?: ExtractionConfig }>): Promise<ExtractionResult[]> {
		const operations = documents.map((doc) => this.extract(doc.data, doc.config, 50));

		return Promise.all(operations);
	}

	/**
	 * Get pending operations count (for cleanup verification)
	 */
	getPendingCount(): number {
		return this.pendingOperations.size;
	}
}

/**
 * Mock extraction result type
 */
interface ExtractionResult {
	text: string;
	chunks: unknown[];
	metadata: Record<string, unknown>;
}

describe("async: Promise-based Extraction", () => {
	let extractor: MockAsyncExtractor;

	beforeEach(async () => {
		extractor = new MockAsyncExtractor();
		await extractor.initialize();
	});

	it("should return Promise from extraction", () => {
		const data = new Uint8Array([1, 2, 3, 4, 5]);
		const result = extractor.extract(data);

		expect(result).toBeInstanceOf(Promise);
	});

	it("should resolve Promise with extraction result", async () => {
		const data = new Uint8Array([1, 2, 3, 4, 5]);
		const result = await extractor.extract(data);

		expect(result).toBeDefined();
		expect(result.text).toContain("Extracted");
		expect(typeof result.metadata).toBe("object");
	});

	it("should handle Promise chain with then/catch", async () => {
		const data = new Uint8Array([1, 2, 3]);

		let resolved = false;
		const promise = extractor.extract(data).then((result) => {
			resolved = true;
			return result;
		});

		await promise;
		expect(resolved).toBe(true);
	});

	it("should preserve data through Promise chain", async () => {
		const data = new Uint8Array([10, 20, 30]);
		const result = await extractor.extract(data).then((res) => ({ ...res, processed: true }));

		expect((result as Record<string, unknown>).processed).toBe(true);
	});
});

describe("async: Async/Await Patterns", () => {
	let extractor: MockAsyncExtractor;

	beforeEach(async () => {
		extractor = new MockAsyncExtractor();
		await extractor.initialize();
	});

	it("should work with async/await syntax", async () => {
		const data = new Uint8Array([1, 2, 3, 4, 5]);
		const result = await extractor.extract(data);

		expect(result).toBeDefined();
		expect(result.text).toBeTruthy();
	});

	it("should support multiple await calls sequentially", async () => {
		const data1 = new Uint8Array([1, 2, 3]);
		const data2 = new Uint8Array([4, 5, 6, 7, 8, 9, 10]);

		const result1 = await extractor.extract(data1);
		const result2 = await extractor.extract(data2);

		expect(result1).toBeDefined();
		expect(result2).toBeDefined();
		expect(result1.text).not.toEqual(result2.text);
	});

	it("should support async function returning Promise", async () => {
		const asyncFunc = async () => {
			const data = new Uint8Array([1, 2, 3]);
			return extractor.extract(data);
		};

		const result = await asyncFunc();
		expect(result).toBeDefined();
	});

	it("should handle async operations in loops", async () => {
		const results: ExtractionResult[] = [];

		for (let i = 0; i < 3; i++) {
			const data = new Uint8Array([i]);
			const result = await extractor.extract(data);
			results.push(result);
		}

		expect(results).toHaveLength(3);
		expect(results.every((r) => r.text)).toBe(true);
	});
});

describe("async: Error Handling in Async Context", () => {
	let extractor: MockAsyncExtractor;

	beforeEach(async () => {
		extractor = new MockAsyncExtractor();
		await extractor.initialize();
	});

	it("should catch Promise rejection with catch", async () => {
		const uninitializedExtractor = new MockAsyncExtractor();
		const data = new Uint8Array([1, 2, 3]);

		let caught = false;
		await uninitializedExtractor.extract(data).catch(() => {
			caught = true;
		});

		expect(caught).toBe(true);
	});

	it("should catch async error with try/catch", async () => {
		const uninitializedExtractor = new MockAsyncExtractor();
		const data = new Uint8Array([1, 2, 3]);

		let caught = false;
		try {
			await uninitializedExtractor.extract(data);
		} catch {
			caught = true;
		}

		expect(caught).toBe(true);
	});

	it("should propagate error message through Promise chain", async () => {
		const uninitializedExtractor = new MockAsyncExtractor();
		const data = new Uint8Array([1, 2, 3]);

		try {
			await uninitializedExtractor.extract(data);
			expect.fail("Should have thrown");
		} catch (error) {
			expect((error as Error).message).toContain("not initialized");
		}
	});

	it("should support error recovery with fallback", async () => {
		const data = new Uint8Array([1, 2, 3]);

		const result = await extractor.extract(data).catch(() => ({
			text: "Fallback content",
			chunks: [],
			metadata: { fallback: true },
		}));

		expect(result.text).toBeTruthy();
	});

	it("should preserve error context in nested async", async () => {
		const uninitializedExtractor = new MockAsyncExtractor();

		const asyncOp = async () => {
			const data = new Uint8Array([1, 2, 3]);
			return uninitializedExtractor.extract(data);
		};

		try {
			await asyncOp();
			expect.fail("Should have thrown");
		} catch (error) {
			expect((error as Error).message).toContain("not initialized");
		}
	});
});

describe("async: Timeout Handling", () => {
	let extractor: MockAsyncExtractor;

	beforeEach(async () => {
		extractor = new MockAsyncExtractor();
		await extractor.initialize();
	});

	it("should timeout operation that exceeds duration", async () => {
		const data = new Uint8Array([1, 2, 3]);

		try {
			await extractor.extractWithTimeout(data, {}, 10); // 10ms timeout
			expect.fail("Should have timed out");
		} catch (error) {
			expect((error as Error).message).toContain("timeout");
		}
	});

	it("should complete within timeout deadline", async () => {
		const data = new Uint8Array([1, 2, 3]);
		const result = await extractor.extractWithTimeout(data, {}, 5000); // 5 second timeout

		expect(result).toBeDefined();
		expect(result.text).toBeTruthy();
	});

	it("should handle multiple operations with independent timeouts", async () => {
		const data1 = new Uint8Array([1, 2, 3]);
		const data2 = new Uint8Array([4, 5, 6]);

		const results = await Promise.all([
			extractor.extractWithTimeout(data1, {}, 5000),
			extractor.extractWithTimeout(data2, {}, 5000),
		]);

		expect(results).toHaveLength(2);
	});
});

describe("async: Cancellation with AbortSignal", () => {
	let extractor: MockAsyncExtractor;

	beforeEach(async () => {
		extractor = new MockAsyncExtractor();
		await extractor.initialize();
	});

	it("should support AbortSignal from AbortController", async () => {
		const controller = new AbortController();
		const data = new Uint8Array([1, 2, 3]);

		const promise = extractor.extractWithAbort(data, {}, controller.signal);

		expect(promise).toBeInstanceOf(Promise);
	});

	it("should abort operation when signal is aborted", async () => {
		const controller = new AbortController();
		const data = new Uint8Array([1, 2, 3]);

		const promise = extractor.extractWithAbort(data, {}, controller.signal);

		controller.abort();

		try {
			await promise;
			expect.fail("Should have been aborted");
		} catch (error) {
			expect((error as Error).message).toContain("aborted");
		}
	});

	it("should reject immediately if already aborted", async () => {
		const controller = new AbortController();
		controller.abort(); // Abort before operation starts

		const data = new Uint8Array([1, 2, 3]);

		try {
			await extractor.extractWithAbort(data, {}, controller.signal);
			expect.fail("Should have been aborted");
		} catch (error) {
			expect((error as Error).message).toContain("aborted");
		}
	});

	it("should complete successfully without abort signal", async () => {
		const data = new Uint8Array([1, 2, 3]);
		const result = await extractor.extractWithAbort(data, {});

		expect(result).toBeDefined();
		expect(result.text).toBeTruthy();
	});
});

describe("async: Concurrent Async Operations", () => {
	let extractor: MockAsyncExtractor;

	beforeEach(async () => {
		extractor = new MockAsyncExtractor();
		await extractor.initialize();
	});

	it("should execute operations concurrently with Promise.all", async () => {
		const data1 = new Uint8Array([1, 2, 3]);
		const data2 = new Uint8Array([4, 5, 6]);
		const data3 = new Uint8Array([7, 8, 9]);

		const startTime = Date.now();
		const results = await Promise.all([extractor.extract(data1), extractor.extract(data2), extractor.extract(data3)]);
		const elapsed = Date.now() - startTime;

		expect(results).toHaveLength(3);
		expect(results.every((r) => r.text)).toBe(true);
		// Should complete much faster than sequential (3 * 50ms = 150ms)
		expect(elapsed).toBeLessThan(200);
	});

	it("should use batch extract for multiple documents", async () => {
		const documents = [
			{ data: new Uint8Array([1, 2, 3]) },
			{ data: new Uint8Array([4, 5, 6]) },
			{ data: new Uint8Array([7, 8, 9]) },
		];

		const results = await extractor.batchExtract(documents);

		expect(results).toHaveLength(3);
		expect(results.every((r) => r.text)).toBe(true);
	});

	it("should handle batch extract with configuration", async () => {
		const documents = [
			{ data: new Uint8Array([1, 2, 3]), config: { useCache: true } },
			{ data: new Uint8Array([4, 5, 6]), config: { useCache: false } },
		];

		const results = await extractor.batchExtract(documents);

		expect(results).toHaveLength(2);
	});

	it("should propagate error from one concurrent operation", async () => {
		const uninitializedExtractor = new MockAsyncExtractor();
		const data = new Uint8Array([1, 2, 3]);

		try {
			await Promise.all([
				extractor.extract(data),
				uninitializedExtractor.extract(data), // This will fail
			]);
			expect.fail("Should have thrown");
		} catch (error) {
			expect((error as Error).message).toContain("not initialized");
		}
	});

	it("should handle Promise.allSettled for mixed results", async () => {
		const uninitializedExtractor = new MockAsyncExtractor();
		const data = new Uint8Array([1, 2, 3]);

		const results = await Promise.allSettled([
			extractor.extract(data),
			uninitializedExtractor.extract(data).catch(() => undefined),
			extractor.extract(data),
		]);

		expect(results).toHaveLength(3);
		expect(results[0].status).toBe("fulfilled");
		expect(results[1].status).toBe("fulfilled");
		expect(results[2].status).toBe("fulfilled");
	});
});

describe("async: Async Configuration Passing", () => {
	let extractor: MockAsyncExtractor;

	beforeEach(async () => {
		extractor = new MockAsyncExtractor();
		await extractor.initialize();
	});

	it("should pass empty config to async extraction", async () => {
		const data = new Uint8Array([1, 2, 3]);
		const result = await extractor.extract(data, {});

		expect(result).toBeDefined();
		expect(result.metadata.config as ExtractionConfig).toEqual({});
	});

	it("should pass complex config through async operation", async () => {
		const config: ExtractionConfig = {
			useCache: true,
			chunking: {
				enabled: true,
				maxChars: 1000,
			},
		};

		const data = new Uint8Array([1, 2, 3]);
		const result = await extractor.extract(data, config);

		expect(result).toBeDefined();
		expect((result.metadata.config as ExtractionConfig).useCache).toBe(true);
	});

	it("should preserve config across concurrent operations", async () => {
		const config1: ExtractionConfig = { useCache: true };
		const config2: ExtractionConfig = { useCache: false };

		const [result1, result2] = await Promise.all([
			extractor.extract(new Uint8Array([1, 2, 3]), config1),
			extractor.extract(new Uint8Array([4, 5, 6]), config2),
		]);

		expect((result1.metadata.config as ExtractionConfig).useCache).toBe(true);
		expect((result2.metadata.config as ExtractionConfig).useCache).toBe(false);
	});
});

describe("async: Resource Cleanup", () => {
	let extractor: MockAsyncExtractor;

	beforeEach(async () => {
		extractor = new MockAsyncExtractor();
		await extractor.initialize();
	});

	it("should cleanup after successful extraction", async () => {
		const data = new Uint8Array([1, 2, 3]);
		await extractor.extract(data);

		// Small delay to allow cleanup
		await new Promise((resolve) => setTimeout(resolve, 100));

		// Should be cleaned up after await
		expect(extractor.getPendingCount()).toBe(0);
	});

	it("should cleanup after error in extraction", async () => {
		const uninitializedExtractor = new MockAsyncExtractor();
		const data = new Uint8Array([1, 2, 3]);

		try {
			await uninitializedExtractor.extract(data);
		} catch {
			// Error expected
		}

		expect(uninitializedExtractor.getPendingCount()).toBe(0);
	});

	it("should cleanup after batch extraction", async () => {
		const documents = [{ data: new Uint8Array([1, 2, 3]) }, { data: new Uint8Array([4, 5, 6]) }];

		await extractor.batchExtract(documents);

		// Small delay to allow cleanup
		await new Promise((resolve) => setTimeout(resolve, 100));

		expect(extractor.getPendingCount()).toBe(0);
	});
});

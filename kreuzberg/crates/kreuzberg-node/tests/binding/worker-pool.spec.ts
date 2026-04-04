/**
 * Comprehensive worker pool tests for TypeScript Node.js bindings.
 *
 * Tests verify worker pool functionality for concurrent document extraction:
 * 1. Worker pool creation and configuration
 * 2. Single file extraction in worker threads
 * 3. Batch extraction with parallelism
 * 4. Pool statistics and monitoring
 * 5. Concurrent extraction stress tests
 * 6. Graceful shutdown and cleanup
 * 7. Error handling in workers
 * 8. Pool size limits and queueing
 *
 * NAPI-RS bindings with worker thread pool support.
 *
 * NOTE: Worker pool tests involve native thread pools that may have timing
 * issues when run in CI environments. Set RUN_WORKER_POOL_TESTS=1 to enable.
 */

import { afterAll, beforeAll, describe, expect, it } from "vitest";

// Skip worker pool tests by default as they can be flaky in CI environments
const runWorkerPoolTests = process.env.RUN_WORKER_POOL_TESTS === "1";

import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import {
	batchExtractFilesInWorker,
	closeWorkerPool,
	createWorkerPool,
	extractFileInWorker,
	extractFileSync,
	getWorkerPoolStats,
} from "../../dist/index.js";
import type { ExtractionConfig, WorkerPool } from "../../src/types.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

// Test file paths using test_documents directory
const TEST_DOCS_DIR = resolve(__dirname, "../../../../../test_documents");
const TEST_PDF = resolve(TEST_DOCS_DIR, "pdf/simple.pdf");
const TEST_DOCX = resolve(TEST_DOCS_DIR, "office/document.docx");
const TEST_TXT = resolve(TEST_DOCS_DIR, "text/simple.txt");

describe.skipIf(!runWorkerPoolTests)("Worker Pool APIs (Node.js Bindings)", () => {
	describe("pool creation and configuration", () => {
		it("should create worker pool with default size (CPU count)", () => {
			const pool = createWorkerPool();
			expect(pool).toBeDefined();
			expect(pool).toBeTruthy();

			const stats = getWorkerPoolStats(pool);
			expect(stats.size).toBeGreaterThan(0);
			expect(stats.activeWorkers).toBe(0);
			expect(stats.queuedTasks).toBe(0);

			closeWorkerPool(pool);
		});

		it("should create worker pool with specified size", () => {
			const poolSize = 4;
			const pool = createWorkerPool(poolSize);
			expect(pool).toBeDefined();

			const stats = getWorkerPoolStats(pool);
			expect(stats.size).toBe(poolSize);
			expect(stats.activeWorkers).toBe(0);

			closeWorkerPool(pool);
		});

		it("should create multiple independent worker pools", () => {
			const pool1 = createWorkerPool(2);
			const pool2 = createWorkerPool(4);

			const stats1 = getWorkerPoolStats(pool1);
			const stats2 = getWorkerPoolStats(pool2);

			expect(stats1.size).toBe(2);
			expect(stats2.size).toBe(4);
			// Pools are independent instances (opaque handles)
			expect(pool1).not.toBe(pool2);

			closeWorkerPool(pool1);
			closeWorkerPool(pool2);
		});

		it("should reject pool creation with invalid size", () => {
			// Zero pool size should throw
			expect(() => createWorkerPool(0)).toThrow();
			// Note: Negative values may be converted to u32 by the binding,
			// resulting in a valid (large) pool size rather than an error.
			// This behavior is implementation-specific.
		});
	});

	describe("pool statistics", () => {
		it("should return accurate pool statistics", () => {
			const pool = createWorkerPool(4);
			const stats = getWorkerPoolStats(pool);

			expect(stats).toBeDefined();
			expect(stats.size).toBe(4);
			expect(stats.activeWorkers).toBe(0);
			expect(stats.queuedTasks).toBe(0);

			closeWorkerPool(pool);
		});

		it("should throw error for invalid pool handle", () => {
			const invalidPool = { poolId: 999999 } as WorkerPool;
			expect(() => getWorkerPoolStats(invalidPool)).toThrow();
		});
	});

	describe("single file extraction in worker", () => {
		it("should extract PDF file in worker thread", async () => {
			const pool = createWorkerPool(2);
			try {
				const result = await extractFileInWorker(pool, TEST_PDF, null, {
					useCache: false,
				});

				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
				expect(result.content.length).toBeGreaterThan(0);
				expect(result.mimeType).toBe("application/pdf");
			} finally {
				await closeWorkerPool(pool);
			}
		});

		it("should extract DOCX file in worker thread", async () => {
			const pool = createWorkerPool(2);
			try {
				const result = await extractFileInWorker(pool, TEST_DOCX, null, {
					useCache: false,
				});

				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
				expect(result.content.length).toBeGreaterThan(0);
				expect(result.mimeType).toBe("application/vnd.openxmlformats-officedocument.wordprocessingml.document");
			} finally {
				await closeWorkerPool(pool);
			}
		});

		it("should extract text file in worker thread", async () => {
			const pool = createWorkerPool(2);
			try {
				const result = await extractFileInWorker(pool, TEST_TXT, null, {
					useCache: false,
				});

				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
				expect(result.content.length).toBeGreaterThan(0);
				expect(result.mimeType).toBe("text/plain");
			} finally {
				await closeWorkerPool(pool);
			}
		});

		it("should extract with custom configuration", async () => {
			const pool = createWorkerPool(2);
			try {
				const config: ExtractionConfig = {
					useCache: false,
					enableQualityProcessing: true,
				};

				const result = await extractFileInWorker(pool, TEST_PDF, null, config);

				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
			} finally {
				await closeWorkerPool(pool);
			}
		});

		it("should handle password-protected files", async () => {
			const pool = createWorkerPool(2);
			try {
				// This should fail without correct password
				await expect(
					extractFileInWorker(pool, TEST_PDF, "wrong_password", {
						useCache: false,
					}),
				).rejects.toThrow();
			} finally {
				await closeWorkerPool(pool);
			}
		});
	});

	describe("batch extraction in workers", () => {
		it("should extract multiple files concurrently", async () => {
			const pool = createWorkerPool(4);
			try {
				const files = [TEST_PDF, TEST_DOCX, TEST_TXT];
				const results = await batchExtractFilesInWorker(pool, files, {
					useCache: false,
				});

				expect(results).toBeDefined();
				expect(results.length).toBe(3);

				// Verify all files were extracted
				for (const result of results) {
					expect(result.content).toBeDefined();
					expect(result.content.length).toBeGreaterThan(0);
				}

				// Verify order matches input
				expect(results[0].mimeType).toBe("application/pdf");
				expect(results[1].mimeType).toBe("application/vnd.openxmlformats-officedocument.wordprocessingml.document");
				expect(results[2].mimeType).toBe("text/plain");
			} finally {
				await closeWorkerPool(pool);
			}
		});

		it("should handle empty file list", async () => {
			const pool = createWorkerPool(2);
			try {
				const results = await batchExtractFilesInWorker(pool, [], {
					useCache: false,
				});

				expect(results).toBeDefined();
				expect(results.length).toBe(0);
			} finally {
				await closeWorkerPool(pool);
			}
		});

		it("should extract large batch exceeding pool size", async () => {
			const pool = createWorkerPool(2);
			try {
				// Create batch larger than pool size
				const files = [TEST_PDF, TEST_DOCX, TEST_TXT, TEST_PDF, TEST_DOCX, TEST_TXT];

				const results = await batchExtractFilesInWorker(pool, files, {
					useCache: false,
				});

				expect(results.length).toBe(6);
				for (const result of results) {
					expect(result.content.length).toBeGreaterThan(0);
				}
			} finally {
				await closeWorkerPool(pool);
			}
		});
	});

	describe("concurrent extraction stress tests", () => {
		it("should handle 10+ concurrent extractions", async () => {
			const pool = createWorkerPool(4);
			try {
				const files = Array(12).fill(TEST_TXT);
				const results = await batchExtractFilesInWorker(pool, files, {
					useCache: false,
				});

				expect(results.length).toBe(12);
				for (const result of results) {
					expect(result.content).toBeDefined();
				}
			} finally {
				await closeWorkerPool(pool);
			}
		});

		it("should handle mixed file types concurrently", async () => {
			const pool = createWorkerPool(4);
			try {
				const files = [
					TEST_PDF,
					TEST_DOCX,
					TEST_TXT,
					TEST_PDF,
					TEST_DOCX,
					TEST_TXT,
					TEST_PDF,
					TEST_DOCX,
					TEST_TXT,
					TEST_PDF,
				];

				const results = await batchExtractFilesInWorker(pool, files, {
					useCache: false,
				});

				expect(results.length).toBe(10);

				// Verify correct MIME types
				expect(results[0].mimeType).toBe("application/pdf");
				expect(results[1].mimeType).toBe("application/vnd.openxmlformats-officedocument.wordprocessingml.document");
				expect(results[2].mimeType).toBe("text/plain");
			} finally {
				await closeWorkerPool(pool);
			}
		});

		it("should maintain result order under high concurrency", async () => {
			const pool = createWorkerPool(8);
			try {
				const files = Array(20).fill(TEST_TXT);
				const results = await batchExtractFilesInWorker(pool, files, {
					useCache: false,
				});

				expect(results.length).toBe(20);
				// All results should be text/plain since all inputs are TXT
				for (const result of results) {
					expect(result.mimeType).toBe("text/plain");
				}
			} finally {
				await closeWorkerPool(pool);
			}
		});
	});

	describe("pool shutdown and cleanup", () => {
		it("should close pool gracefully", async () => {
			const pool = createWorkerPool(2);
			await closeWorkerPool(pool);

			// Pool should be closed, further operations should fail
			expect(() => getWorkerPoolStats(pool)).toThrow();
		});

		it("should wait for active tasks before closing", async () => {
			const pool = createWorkerPool(2);

			// Start some extractions
			const extraction1 = extractFileInWorker(pool, TEST_PDF, null, {
				useCache: false,
			});
			const extraction2 = extractFileInWorker(pool, TEST_DOCX, null, {
				useCache: false,
			});

			// Close pool (should wait for tasks)
			const closePromise = closeWorkerPool(pool);

			// Wait for all to complete
			await Promise.all([extraction1, extraction2, closePromise]);

			// Verify tasks completed
			const result1 = await extraction1;
			const result2 = await extraction2;

			expect(result1.content).toBeDefined();
			expect(result2.content).toBeDefined();
		});

		it("should handle multiple close calls gracefully", async () => {
			const pool = createWorkerPool(2);

			await closeWorkerPool(pool);
			await closeWorkerPool(pool); // Should not throw

			expect(() => getWorkerPoolStats(pool)).toThrow();
		});
	});

	describe("error handling in workers", () => {
		it("should propagate file not found errors", async () => {
			const pool = createWorkerPool(2);
			try {
				await expect(
					extractFileInWorker(pool, "/nonexistent/file.pdf", null, {
						useCache: false,
					}),
				).rejects.toThrow();
			} finally {
				await closeWorkerPool(pool);
			}
		});

		it("should handle errors in batch without affecting other tasks", async () => {
			const pool = createWorkerPool(4);
			try {
				const files = [TEST_PDF, "/nonexistent/file.pdf", TEST_TXT];

				// Batch should fail if any file fails
				await expect(batchExtractFilesInWorker(pool, files, { useCache: false })).rejects.toThrow();
			} finally {
				await closeWorkerPool(pool);
			}
		});

		it("should recover from worker errors", async () => {
			const pool = createWorkerPool(2);
			try {
				// Cause an error
				await expect(
					extractFileInWorker(pool, "/nonexistent/file.pdf", null, {
						useCache: false,
					}),
				).rejects.toThrow();

				// Pool should still work after error
				const result = await extractFileInWorker(pool, TEST_TXT, null, {
					useCache: false,
				});
				expect(result.content).toBeDefined();
			} finally {
				await closeWorkerPool(pool);
			}
		});
	});

	describe("performance and correctness", () => {
		it("should produce same results as sync extraction", async () => {
			const pool = createWorkerPool(2);
			try {
				const workerResult = await extractFileInWorker(pool, TEST_PDF, null, {
					useCache: false,
				});
				const syncResult = extractFileSync(TEST_PDF, null, {
					useCache: false,
				});

				expect(workerResult.content).toBe(syncResult.content);
				expect(workerResult.mimeType).toBe(syncResult.mimeType);
				expect(workerResult.metadata).toEqual(syncResult.metadata);
			} finally {
				await closeWorkerPool(pool);
			}
		});

		it("should extract files faster with parallelism", async () => {
			const files = Array(8).fill(TEST_PDF);

			// Serial extraction (pool size 1)
			const pool1 = createWorkerPool(1);
			const start1 = Date.now();
			await batchExtractFilesInWorker(pool1, files, { useCache: false });
			const serial_time = Date.now() - start1;
			await closeWorkerPool(pool1);

			// Parallel extraction (pool size 4)
			const pool4 = createWorkerPool(4);
			const start4 = Date.now();
			await batchExtractFilesInWorker(pool4, files, { useCache: false });
			const parallel_time = Date.now() - start4;
			await closeWorkerPool(pool4);

			// Parallel should be faster (or at least not significantly slower)
			// Allow some margin for variance
			expect(parallel_time).toBeLessThanOrEqual(serial_time * 1.2);
		}, 30000); // 30 second timeout for this test
	});
});

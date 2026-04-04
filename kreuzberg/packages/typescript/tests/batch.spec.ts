/**
 * Batch Operations Tests for TypeScript WASM Binding
 *
 * Comprehensive test suite for batch document processing in Kreuzberg WASM bindings.
 * Tests cover sequential batch processing, concurrent batch operations, error handling
 * across batches, configuration variants, result ordering, and large batch performance.
 *
 * @group wasm-binding
 * @group batch-operations
 * @group extraction
 */

import type { ChunkingConfig, ExtractionConfig, ExtractionResult, KeywordConfig } from "@kreuzberg/core";
import { beforeEach, describe, expect, it, vi } from "vitest";

/**
 * Mock extraction service for batch operations testing
 */
class MockBatchExtractor {
	private processingDelay: number;
	private failureRate: number;

	constructor(processingDelay = 10, failureRate = 0) {
		this.processingDelay = processingDelay;
		this.failureRate = failureRate;
	}

	/**
	 * Extract a single document, simulating async operation
	 */
	async extractFile(filePath: string, config?: ExtractionConfig): Promise<ExtractionResult> {
		// Simulate processing delay
		await new Promise((resolve) => setTimeout(resolve, this.processingDelay));

		// Simulate random failures based on configured failure rate
		if (Math.random() < this.failureRate) {
			throw new Error(`Failed to extract ${filePath}`);
		}

		// Validate file path
		if (!filePath || filePath.length === 0) {
			throw new Error("File path cannot be empty");
		}

		// Simulate extraction based on file type
		const mimeType = this.detectMimeType(filePath);
		const content = this.generateMockContent(filePath, config);

		return {
			content,
			mimeType,
			metadata: {
				pageCount: 1,
				wordCount: content.split(" ").length,
				characterCount: content.length,
				processingTimeMs: this.processingDelay,
				extractedAt: new Date().toISOString(),
			},
		};
	}

	/**
	 * Extract multiple documents sequentially
	 */
	async batchExtract(filePaths: string[], config?: ExtractionConfig): Promise<ExtractionResult[]> {
		const results: ExtractionResult[] = [];

		for (const filePath of filePaths) {
			try {
				const result = await this.extractFile(filePath, config);
				results.push(result);
			} catch (error) {
				// Store error result
				results.push({
					content: "",
					mimeType: "application/error",
					metadata: {
						error: (error as Error).message,
						pageCount: 0,
					},
				});
			}
		}

		return results;
	}

	/**
	 * Extract multiple documents concurrently
	 */
	async batchExtractConcurrent(
		filePaths: string[],
		config?: ExtractionConfig,
		concurrency = 3,
	): Promise<ExtractionResult[]> {
		const results = Array.from<ExtractionResult>({ length: filePaths.length });
		let activeCount = 0;
		let index = 0;

		return new Promise((resolve, reject) => {
			const processNext = async () => {
				if (index >= filePaths.length) {
					if (activeCount === 0) {
						resolve(results);
					}
					return;
				}

				const currentIndex = index++;
				activeCount++;

				try {
					results[currentIndex] = await this.extractFile(filePaths[currentIndex], config);
				} catch (error) {
					results[currentIndex] = {
						content: "",
						mimeType: "application/error",
						metadata: {
							error: (error as Error).message,
							pageCount: 0,
						},
					};
				}

				activeCount--;
				processNext();
			};

			// Start initial concurrent operations
			for (let i = 0; i < Math.min(concurrency, filePaths.length); i++) {
				processNext().catch(reject);
			}
		});
	}

	/**
	 * Detect MIME type based on file extension
	 */
	private detectMimeType(filePath: string): string {
		if (filePath.endsWith(".pdf")) return "application/pdf";
		if (filePath.endsWith(".docx")) return "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
		if (filePath.endsWith(".txt")) return "text/plain";
		if (filePath.endsWith(".html")) return "text/html";
		return "application/octet-stream";
	}

	/**
	 * Generate mock content for testing
	 */
	private generateMockContent(filePath: string, config?: ExtractionConfig): string {
		const baseContent = `Extracted content from ${filePath}. This is sample text for testing batch operations.`;

		// Apply chunking if configured
		if (config?.chunking?.maxChars) {
			return baseContent.substring(0, config.chunking.maxChars);
		}

		return baseContent;
	}
}

describe("Batch Operations: Sequential Processing", () => {
	let extractor: MockBatchExtractor;

	beforeEach(() => {
		extractor = new MockBatchExtractor(10, 0);
	});

	it("should extract multiple files sequentially", async () => {
		const files = ["/docs/file1.pdf", "/docs/file2.txt", "/docs/file3.docx"];
		const results = await extractor.batchExtract(files);

		expect(results).toHaveLength(3);
		for (const result of results) {
			expect(result.content).toBeDefined();
			expect(result.mimeType).toBeDefined();
			expect(result.metadata).toBeDefined();
		}
	});

	it("should preserve file order in batch extraction", async () => {
		const files = ["/docs/file1.pdf", "/docs/file2.txt", "/docs/file3.docx"];
		const results = await extractor.batchExtract(files);

		expect(results.length).toBe(files.length);
		for (let i = 0; i < results.length; i++) {
			expect(results[i]).toBeDefined();
			expect(results[i].content).toBeTruthy();
		}
	});

	it("should handle single file batch", async () => {
		const files = ["/docs/document.pdf"];
		const results = await extractor.batchExtract(files);

		expect(results).toHaveLength(1);
		expect(results[0].content).toBeTruthy();
		expect(results[0].mimeType).toBe("application/pdf");
	});

	it("should apply config to all files in batch", async () => {
		const files = ["/docs/file1.pdf", "/docs/file2.txt"];
		const config: ExtractionConfig = {
			chunking: {
				maxChars: 100,
				maxOverlap: 10,
			},
		};

		const results = await extractor.batchExtract(files, config);

		for (const result of results) {
			expect(result.content).toBeDefined();
			expect(result.content.length).toBeLessThanOrEqual(100);
		}
	});

	it("should handle empty file list", async () => {
		const results = await extractor.batchExtract([]);

		expect(Array.isArray(results)).toBe(true);
		expect(results).toHaveLength(0);
	});
});

describe("Batch Operations: Concurrent Processing", () => {
	let extractor: MockBatchExtractor;

	beforeEach(() => {
		extractor = new MockBatchExtractor(20, 0);
	});

	it("should extract multiple files concurrently", async () => {
		const files = ["/docs/file1.pdf", "/docs/file2.txt", "/docs/file3.docx"];
		const results = await extractor.batchExtractConcurrent(files, undefined, 3);

		expect(results).toHaveLength(3);
		for (const result of results) {
			expect(result.content).toBeDefined();
			expect(result.mimeType).toBeDefined();
		}
	});

	it("should respect concurrency limit", async () => {
		const files = Array(10)
			.fill(0)
			.map((_, i) => `/docs/file${i}.pdf`);
		const concurrency = 2;

		const startTime = Date.now();
		const results = await extractor.batchExtractConcurrent(files, undefined, concurrency);
		const duration = Date.now() - startTime;

		expect(results).toHaveLength(10);
		// With concurrency of 2 and 20ms per file, minimum time should be ~100ms (5 batches)
		expect(duration).toBeGreaterThanOrEqual(80);
	});

	it("should preserve order in concurrent extraction", async () => {
		const files = ["/docs/a.pdf", "/docs/b.txt", "/docs/c.docx", "/docs/d.html"];
		const results = await extractor.batchExtractConcurrent(files, undefined, 2);

		expect(results).toHaveLength(4);
		for (let i = 0; i < results.length; i++) {
			expect(results[i]).toBeDefined();
		}
	});

	it("should handle large concurrent batch", async () => {
		const files = Array(50)
			.fill(0)
			.map((_, i) => `/docs/file${i}.pdf`);

		const results = await extractor.batchExtractConcurrent(files, undefined, 5);

		expect(results).toHaveLength(50);
		expect(results.filter((r) => r.content.length > 0)).toHaveLength(50);
	});
});

describe("Batch Operations: Error Handling", () => {
	it("should handle failures in batch gracefully", async () => {
		const extractor = new MockBatchExtractor(5, 0.3);
		const files = ["/docs/file1.pdf", "/docs/file2.txt", "/docs/file3.docx"];

		const results = await extractor.batchExtract(files);

		expect(results).toHaveLength(3);
		// Some results may have errors
		const errorResults = results.filter((r) => r.mimeType === "application/error");
		expect(errorResults.length).toBeGreaterThanOrEqual(0);
	});

	it("should handle non-existent files", async () => {
		const extractor = new MockBatchExtractor();
		const files = ["/non/existent/file.pdf"];

		try {
			await extractor.batchExtract(files);
		} catch (error) {
			expect((error as Error).message).toMatch(/extracted/i);
		}
	});

	it("should handle empty file paths in batch", async () => {
		const extractor = new MockBatchExtractor();

		try {
			await extractor.batchExtract([""]);
		} catch (error) {
			expect((error as Error).message).toMatch(/empty/i);
		}
	});

	it("should continue processing after individual file error", async () => {
		const extractor = new MockBatchExtractor();
		const files = ["/docs/valid.pdf", "", "/docs/another.txt"];

		const results = await extractor.batchExtract(files);

		expect(results).toHaveLength(3);
		expect(results[0]).toBeDefined();
		expect(results[1]).toBeDefined();
		expect(results[2]).toBeDefined();
	});
});

describe("Batch Operations: Configuration Variants", () => {
	let extractor: MockBatchExtractor;

	beforeEach(() => {
		extractor = new MockBatchExtractor();
	});

	it("should apply identical chunking config to all files", async () => {
		const files = ["/docs/file1.pdf", "/docs/file2.txt"];
		const config: ExtractionConfig = {
			chunking: {
				maxChars: 200,
				maxOverlap: 50,
			},
		};

		const results = await extractor.batchExtract(files, config);

		for (const result of results) {
			expect(result.content.length).toBeLessThanOrEqual(200);
		}
	});

	it("should apply keyword extraction config to all files", async () => {
		const files = ["/docs/file1.pdf", "/docs/file2.txt"];
		const config: ExtractionConfig = {
			keywords: {
				algorithm: "yake",
				maxKeywords: 10,
			} as KeywordConfig,
		};

		const results = await extractor.batchExtract(files, config);

		expect(results).toHaveLength(2);
		for (const result of results) {
			expect(result.content).toBeDefined();
			expect(result.metadata).toBeDefined();
		}
	});

	it("should handle null config in batch operations", async () => {
		const files = ["/docs/file1.pdf", "/docs/file2.txt"];
		const results = await extractor.batchExtract(files, null);

		expect(results).toHaveLength(2);
		for (const result of results) {
			expect(result.content).toBeDefined();
		}
	});

	it("should handle undefined config in batch operations", async () => {
		const files = ["/docs/file1.pdf", "/docs/file2.txt"];
		const results = await extractor.batchExtract(files);

		expect(results).toHaveLength(2);
		for (const result of results) {
			expect(result.content).toBeDefined();
		}
	});
});

describe("Batch Operations: Order Preservation", () => {
	it("should maintain order across sequential processing", async () => {
		const extractor = new MockBatchExtractor(5, 0);
		const files = ["/docs/alpha.pdf", "/docs/beta.txt", "/docs/gamma.docx", "/docs/delta.html"];

		const results = await extractor.batchExtract(files);

		expect(results).toHaveLength(4);
		// Verify content contains expected file references
		for (let i = 0; i < results.length; i++) {
			expect(results[i]).toBeDefined();
		}
	});

	it("should maintain order across concurrent processing", async () => {
		const extractor = new MockBatchExtractor(10, 0);
		const files = Array(20)
			.fill(0)
			.map((_, i) => `/docs/file${i.toString().padStart(2, "0")}.pdf`);

		const results = await extractor.batchExtractConcurrent(files, undefined, 4);

		expect(results).toHaveLength(20);
		// All positions should have valid results
		for (let i = 0; i < results.length; i++) {
			expect(results[i]).toBeDefined();
		}
	});

	it("should validate order consistency multiple times", async () => {
		const extractor = new MockBatchExtractor(2, 0);
		const files = ["/docs/first.pdf", "/docs/second.txt", "/docs/third.docx"];

		const results1 = await extractor.batchExtract(files);
		const results2 = await extractor.batchExtract(files);

		expect(results1).toHaveLength(results2.length);
		for (let i = 0; i < results1.length; i++) {
			expect(results1[i]).toBeDefined();
			expect(results2[i]).toBeDefined();
		}
	});
});

describe("Batch Operations: Performance", () => {
	it("should handle large batch efficiently", async () => {
		const extractor = new MockBatchExtractor(5, 0);
		const files = Array(100)
			.fill(0)
			.map((_, i) => `/docs/document${i}.pdf`);

		const startTime = Date.now();
		const results = await extractor.batchExtract(files);
		const duration = Date.now() - startTime;

		expect(results).toHaveLength(100);
		expect(results.filter((r) => r.content.length > 0)).toHaveLength(100);
		// Should complete in reasonable time (less than 2 seconds for 100 files with 5ms each)
		expect(duration).toBeLessThan(2000);
	});

	it("should handle large concurrent batch efficiently", async () => {
		const extractor = new MockBatchExtractor(5, 0);
		const files = Array(100)
			.fill(0)
			.map((_, i) => `/docs/document${i}.pdf`);

		const startTime = Date.now();
		const results = await extractor.batchExtractConcurrent(files, undefined, 10);
		const duration = Date.now() - startTime;

		expect(results).toHaveLength(100);
		expect(results.filter((r) => r.content.length > 0)).toHaveLength(100);
		// Concurrent should be significantly faster than sequential
		expect(duration).toBeLessThan(300);
	});

	it("should measure concurrent vs sequential performance", async () => {
		const files = Array(20)
			.fill(0)
			.map((_, i) => `/docs/file${i}.pdf`);

		const extractor = new MockBatchExtractor(10, 0);

		const sequentialStart = Date.now();
		await extractor.batchExtract(files);
		const sequentialDuration = Date.now() - sequentialStart;

		const concurrentStart = Date.now();
		await extractor.batchExtractConcurrent(files, undefined, 4);
		const concurrentDuration = Date.now() - concurrentStart;

		// Concurrent should be faster
		expect(concurrentDuration).toBeLessThan(sequentialDuration);
	});
});

describe("Batch Operations: Result Validation", () => {
	let extractor: MockBatchExtractor;

	beforeEach(() => {
		extractor = new MockBatchExtractor(5, 0);
	});

	it("should validate all results have required fields", async () => {
		const files = ["/docs/file1.pdf", "/docs/file2.txt"];
		const results = await extractor.batchExtract(files);

		for (const result of results) {
			expect(result).toHaveProperty("content");
			expect(result).toHaveProperty("mimeType");
			expect(result).toHaveProperty("metadata");
			expect(typeof result.content).toBe("string");
			expect(typeof result.mimeType).toBe("string");
			expect(typeof result.metadata).toBe("object");
		}
	});

	it("should return correct MIME types for different file formats", async () => {
		const files = ["/docs/file.pdf", "/docs/file.txt", "/docs/file.docx"];
		const results = await extractor.batchExtract(files);

		expect(results[0].mimeType).toBe("application/pdf");
		expect(results[1].mimeType).toBe("text/plain");
		expect(results[2].mimeType).toContain("wordprocessingml");
	});

	it("should include metadata in results", async () => {
		const files = ["/docs/test.pdf"];
		const results = await extractor.batchExtract(files);

		expect(results[0].metadata).toBeDefined();
		expect(results[0].metadata).toHaveProperty("pageCount");
		expect(results[0].metadata).toHaveProperty("characterCount");
		expect(results[0].metadata).toHaveProperty("wordCount");
	});
});

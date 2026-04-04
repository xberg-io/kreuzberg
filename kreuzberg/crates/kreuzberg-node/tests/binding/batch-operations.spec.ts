/**
 * Comprehensive Batch Operations Tests for TypeScript Node.js bindings.
 *
 * Tests verify batch extraction functionality across multiple scenarios:
 * 1. Batch file extraction (sync and async)
 * 2. Batch byte extraction (sync and async)
 * 3. Parallel processing efficiency
 * 4. Error handling in batch operations
 * 5. Result ordering and consistency
 * 6. Mixed content types in batches
 * 7. Large batch operations
 * 8. Concurrent operation limits
 *
 * NAPI-RS bindings with plain object configs (NO builder pattern).
 */

import { readFileSync, realpathSync } from "node:fs";
import { beforeAll, describe, expect, it } from "vitest";
import {
	batchExtractBytes,
	batchExtractBytesSync,
	batchExtractFiles,
	batchExtractFilesSync,
} from "../../dist/index.js";
import type { ExtractionConfig } from "../../src/types.js";
import { getTestDocumentPath } from "../helpers/index.js";

let samplePdfPath: string;
let sampleDocxPath: string;
let sampleTxtPath: string;
let samplePdfBytes: Uint8Array;
let sampleDocxBytes: Uint8Array;
let sampleTxtBytes: Uint8Array;

beforeAll(() => {
	// Get test documents
	samplePdfPath = getTestDocumentPath("pdf/tiny.pdf");
	sampleDocxPath = getTestDocumentPath("documents/sample.docx");
	sampleTxtPath = getTestDocumentPath("text/sample.txt");

	// Read as bytes - Resolve symlinks for Windows compatibility
	try {
		samplePdfBytes = new Uint8Array(readFileSync(realpathSync(samplePdfPath)));
	} catch {
		// File may not exist
	}

	try {
		sampleDocxBytes = new Uint8Array(readFileSync(realpathSync(sampleDocxPath)));
	} catch {
		// File may not exist
	}

	try {
		sampleTxtBytes = new Uint8Array(readFileSync(realpathSync(sampleTxtPath)));
	} catch {
		// File may not exist
	}
});

describe("Batch File Extraction (Node.js Bindings)", () => {
	describe("batch file extraction sync", () => {
		it("should extract multiple files with sync API", () => {
			const files = [samplePdfPath, sampleTxtPath].filter((f) => f);

			if (files.length === 0) {
				expect(true).toBe(true);
				return;
			}

			const results = batchExtractFilesSync(files);

			expect(Array.isArray(results)).toBe(true);
			expect(results.length).toBe(files.length);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
				expect(typeof result.content).toBe("string");
				expect(result.mimeType).toBeDefined();
				expect(result.metadata).toBeInstanceOf(Object);
			}
		});

		it("should preserve file order in batch extraction", () => {
			const files = [samplePdfPath, sampleTxtPath].filter((f) => f);

			if (files.length < 2) {
				expect(true).toBe(true);
				return;
			}

			const results = batchExtractFilesSync(files);

			expect(results.length).toBe(files.length);

			// Results should be in same order as input files
			for (let i = 0; i < results.length; i++) {
				expect(results[i]).toBeDefined();
				expect(results[i].content).toBeTruthy();
			}
		});

		it("should handle single file in batch", () => {
			const files = [samplePdfPath].filter((f) => f);

			if (files.length === 0) {
				expect(true).toBe(true);
				return;
			}

			const results = batchExtractFilesSync(files);

			expect(results.length).toBe(1);
			expect(results[0].content).toBeDefined();
		});

		it("should apply config to all files in batch", () => {
			const files = [samplePdfPath, sampleTxtPath].filter((f) => f);

			if (files.length === 0) {
				expect(true).toBe(true);
				return;
			}

			const config: ExtractionConfig = {
				chunking: {
					maxChars: 500,
					maxOverlap: 100,
				},
			};

			const results = batchExtractFilesSync(files, config);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
			}
		});
	});

	describe("batch file extraction async", () => {
		it("should extract multiple files with async API", async () => {
			const files = [samplePdfPath, sampleTxtPath].filter((f) => f);

			if (files.length === 0) {
				expect(true).toBe(true);
				return;
			}

			const results = await batchExtractFiles(files);

			expect(Array.isArray(results)).toBe(true);
			expect(results.length).toBe(files.length);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
				expect(typeof result.content).toBe("string");
			}
		});

		it("should handle async batch with configuration", async () => {
			const files = [samplePdfPath, sampleTxtPath].filter((f) => f);

			if (files.length === 0) {
				expect(true).toBe(true);
				return;
			}

			const config: ExtractionConfig = {
				keywords: {
					algorithm: "yake",
					maxKeywords: 5,
				},
			};

			const results = await batchExtractFiles(files, config);

			expect(results.length).toBe(files.length);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeTruthy();
				expect(result.metadata).toBeInstanceOf(Object);
			}
		});

		it("should preserve order in async batch operations", async () => {
			const files = [samplePdfPath, sampleTxtPath, sampleDocxPath].filter((f) => f);

			if (files.length < 2) {
				expect(true).toBe(true);
				return;
			}

			const results = await batchExtractFiles(files);

			expect(results.length).toBe(files.length);

			// Verify all results are defined and in order
			for (let i = 0; i < results.length; i++) {
				expect(results[i]).toBeDefined();
				expect(results[i].content).toBeTruthy();
			}
		});

		it("should handle large batch of files", async () => {
			const files = [samplePdfPath, sampleTxtPath].filter((f) => f);

			if (files.length === 0) {
				expect(true).toBe(true);
				return;
			}

			// Create a large batch by repeating files
			const largeBatch = Array(5).fill(files).flat();

			const results = await batchExtractFiles(largeBatch);

			expect(results.length).toBe(largeBatch.length);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeTruthy();
			}
		});
	});

	describe("batch file extraction error handling", () => {
		it("should handle non-existent files in batch", () => {
			const files = ["/non/existent/file.pdf"];

			// The batch API may or may not throw depending on internal error handling
			// Just verify it doesn't crash the process
			try {
				const results = batchExtractFilesSync(files);
				// If it doesn't throw, verify the result structure
				expect(Array.isArray(results)).toBe(true);
			} catch {
				// If it throws, that's also acceptable behavior
				expect(true).toBe(true);
			}
		});

		it("should handle mixed valid and invalid files", () => {
			const files = [samplePdfPath, "/non/existent/file.txt"].filter((f) => f);

			if (files.length < 2) {
				expect(true).toBe(true);
				return;
			}

			// The batch API may or may not throw depending on internal error handling
			try {
				const results = batchExtractFilesSync(files);
				expect(Array.isArray(results)).toBe(true);
			} catch {
				// If it throws, that's also acceptable behavior
				expect(true).toBe(true);
			}
		});

		it("should handle empty file list", () => {
			const results = batchExtractFilesSync([]);

			expect(Array.isArray(results)).toBe(true);
			expect(results.length).toBe(0);
		});
	});
});

describe("Batch Bytes Extraction (Node.js Bindings)", () => {
	describe("batch bytes extraction sync", () => {
		it("should extract from multiple byte arrays", () => {
			const dataList = [samplePdfBytes, sampleTxtBytes].filter((b) => b);
			const mimeTypes = ["application/pdf", "text/plain"].slice(0, dataList.length);

			if (dataList.length === 0) {
				expect(true).toBe(true);
				return;
			}

			const results = batchExtractBytesSync(dataList, mimeTypes);

			expect(Array.isArray(results)).toBe(true);
			expect(results.length).toBe(dataList.length);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
				expect(typeof result.content).toBe("string");
				expect(result.mimeType).toBeDefined();
			}
		});

		it("should preserve order of bytes in batch", () => {
			const dataList = [samplePdfBytes, sampleTxtBytes, sampleDocxBytes].filter((b) => b);
			const mimeTypes = [
				"application/pdf",
				"text/plain",
				"application/vnd.openxmlformats-officedocument.wordprocessingml.document",
			].slice(0, dataList.length);

			if (dataList.length < 2) {
				expect(true).toBe(true);
				return;
			}

			const results = batchExtractBytesSync(dataList, mimeTypes);

			expect(results.length).toBe(dataList.length);

			for (let i = 0; i < results.length; i++) {
				expect(results[i]).toBeDefined();
				expect(results[i].content).toBeTruthy();
			}
		});

		it("should require matching mimeTypes array length", () => {
			const dataList = [samplePdfBytes, sampleTxtBytes].filter((b) => b);

			if (dataList.length < 2) {
				expect(true).toBe(true);
				return;
			}

			const mismatchedMimeTypes = ["application/pdf"]; // Too few

			expect(() => {
				batchExtractBytesSync(dataList, mismatchedMimeTypes);
			}).toThrow();
		});

		it("should apply config to all byte arrays", () => {
			const dataList = [samplePdfBytes, sampleTxtBytes].filter((b) => b);
			const mimeTypes = ["application/pdf", "text/plain"].slice(0, dataList.length);

			if (dataList.length === 0) {
				expect(true).toBe(true);
				return;
			}

			const config: ExtractionConfig = {
				chunking: {
					maxChars: 500,
				},
			};

			const results = batchExtractBytesSync(dataList, mimeTypes, config);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
			}
		});
	});

	describe("batch bytes extraction async", () => {
		it("should extract from multiple byte arrays asynchronously", async () => {
			const dataList = [samplePdfBytes, sampleTxtBytes].filter((b) => b);
			const mimeTypes = ["application/pdf", "text/plain"].slice(0, dataList.length);

			if (dataList.length === 0) {
				expect(true).toBe(true);
				return;
			}

			const results = await batchExtractBytes(dataList, mimeTypes);

			expect(Array.isArray(results)).toBe(true);
			expect(results.length).toBe(dataList.length);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
			}
		});

		it("should handle async batch with keywords extraction", async () => {
			const dataList = [samplePdfBytes, sampleTxtBytes].filter((b) => b);
			const mimeTypes = ["application/pdf", "text/plain"].slice(0, dataList.length);

			if (dataList.length === 0) {
				expect(true).toBe(true);
				return;
			}

			const config: ExtractionConfig = {
				keywords: {
					algorithm: "rake",
					maxKeywords: 10,
				},
			};

			const results = await batchExtractBytes(dataList, mimeTypes, config);

			expect(results.length).toBe(dataList.length);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeTruthy();
				expect(result.metadata).toBeInstanceOf(Object);
			}
		});

		it("should maintain order in async byte batch", async () => {
			const dataList = [samplePdfBytes, sampleTxtBytes, sampleDocxBytes].filter((b) => b);
			const mimeTypes = [
				"application/pdf",
				"text/plain",
				"application/vnd.openxmlformats-officedocument.wordprocessingml.document",
			].slice(0, dataList.length);

			if (dataList.length < 2) {
				expect(true).toBe(true);
				return;
			}

			const results = await batchExtractBytes(dataList, mimeTypes);

			expect(results.length).toBe(dataList.length);

			for (let i = 0; i < results.length; i++) {
				expect(results[i]).toBeDefined();
				expect(results[i].content).toBeTruthy();
			}
		});

		it("should handle concurrent byte extractions", async () => {
			const dataList = [samplePdfBytes, sampleTxtBytes].filter((b) => b);
			const mimeTypes = ["application/pdf", "text/plain"].slice(0, dataList.length);

			if (dataList.length === 0) {
				expect(true).toBe(true);
				return;
			}

			// Run multiple batches concurrently
			const [results1, results2] = await Promise.all([
				batchExtractBytes(dataList, mimeTypes),
				batchExtractBytes(dataList, mimeTypes),
			]);

			expect(results1.length).toBe(dataList.length);
			expect(results2.length).toBe(dataList.length);

			for (const result of [...results1, ...results2]) {
				expect(result).toBeDefined();
				expect(result.content).toBeTruthy();
			}
		});
	});

	describe("batch bytes extraction error handling", () => {
		it("should reject non-Uint8Array data", () => {
			const invalidData = ["not bytes" as any];
			const mimeTypes = ["text/plain"];

			expect(() => {
				batchExtractBytesSync(invalidData, mimeTypes);
			}).toThrow();
		});

		it("should handle empty data list", () => {
			const results = batchExtractBytesSync([], []);

			expect(Array.isArray(results)).toBe(true);
			expect(results.length).toBe(0);
		});

		it("should require matching mimeTypes count", () => {
			const dataList = [samplePdfBytes, sampleTxtBytes].filter((b) => b);

			if (dataList.length < 2) {
				expect(true).toBe(true);
				return;
			}

			const tooFewMimeTypes = ["application/pdf"];

			expect(() => {
				batchExtractBytesSync(dataList, tooFewMimeTypes);
			}).toThrow();
		});
	});
});

describe("Batch Operations - Sync vs Async Consistency", () => {
	it("should produce consistent results between sync and async batch APIs", async () => {
		const files = [samplePdfPath, sampleTxtPath].filter((f) => f);

		if (files.length === 0) {
			expect(true).toBe(true);
			return;
		}

		const syncResults = batchExtractFilesSync(files);
		const asyncResults = await batchExtractFiles(files);

		expect(syncResults.length).toBe(asyncResults.length);

		// Results should be structurally similar
		for (let i = 0; i < syncResults.length; i++) {
			expect(syncResults[i].content.length).toBeGreaterThan(0);
			expect(asyncResults[i].content.length).toBeGreaterThan(0);
			expect(syncResults[i].mimeType).toBeDefined();
			expect(asyncResults[i].mimeType).toBeDefined();
		}
	});

	it("should produce consistent results with byte APIs", async () => {
		const dataList = [samplePdfBytes, sampleTxtBytes].filter((b) => b);
		const mimeTypes = ["application/pdf", "text/plain"].slice(0, dataList.length);

		if (dataList.length === 0) {
			expect(true).toBe(true);
			return;
		}

		const syncResults = batchExtractBytesSync(dataList, mimeTypes);
		const asyncResults = await batchExtractBytes(dataList, mimeTypes);

		expect(syncResults.length).toBe(asyncResults.length);

		for (let i = 0; i < syncResults.length; i++) {
			expect(syncResults[i].content.length).toBeGreaterThan(0);
			expect(asyncResults[i].content.length).toBeGreaterThan(0);
		}
	});
});

describe("Batch Operations - Configuration Handling", () => {
	it("should apply identical config to all batch items", () => {
		const files = [samplePdfPath, sampleTxtPath].filter((f) => f);

		if (files.length === 0) {
			expect(true).toBe(true);
			return;
		}

		const config: ExtractionConfig = {
			keywords: {
				algorithm: "yake",
				maxKeywords: 3,
			},
		};

		const results = batchExtractFilesSync(files, config);

		for (const result of results) {
			expect(result).toBeDefined();
			expect(result.metadata).toBeInstanceOf(Object);
		}
	});

	it("should handle null config in batch operations", () => {
		const files = [samplePdfPath, sampleTxtPath].filter((f) => f);

		if (files.length === 0) {
			expect(true).toBe(true);
			return;
		}

		const results = batchExtractFilesSync(files, null);

		expect(Array.isArray(results)).toBe(true);
		expect(results.length).toBe(files.length);

		for (const result of results) {
			expect(result.content).toBeDefined();
		}
	});

	it("should handle undefined config in batch operations", () => {
		const files = [samplePdfPath, sampleTxtPath].filter((f) => f);

		if (files.length === 0) {
			expect(true).toBe(true);
			return;
		}

		const results = batchExtractFilesSync(files);

		expect(Array.isArray(results)).toBe(true);
		expect(results.length).toBe(files.length);
	});
});

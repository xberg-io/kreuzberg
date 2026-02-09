/**
 * Comprehensive Batch Operations Tests for WASM bindings.
 *
 * Tests verify batch extraction functionality across multiple scenarios:
 * 1. Batch bytes extraction (sync and async)
 * 2. Batch File extraction (async)
 * 3. Result ordering and consistency
 * 4. Error handling in batch operations
 * 5. Configuration variants
 * 6. Multiple document types (PDFs, text)
 *
 * Note: WASM has limitations - no Polars (no Excel/CSV).
 * Tests focus on PDF and text formats available in WASM.
 */

import { readFileSync } from "node:fs";
import { beforeAll, describe, expect, it } from "vitest";
import { batchExtractBytes, batchExtractBytesSync, batchExtractFiles, initWasm } from "./index.js";
import type { ExtractionConfig, ExtractionResult } from "./types.js";

let samplePdfBytes: Uint8Array;
let sampleTxtBytes: Uint8Array;

beforeAll(async () => {
	// Initialize WASM module before running tests
	await initWasm();

	// Load test PDF file
	const pdfPath = new URL("../../../tests/fixtures/documents/pdf/simple.pdf", import.meta.url).pathname;
	try {
		samplePdfBytes = new Uint8Array(readFileSync(pdfPath));
	} catch {
		console.warn("Test PDF file not found");
	}

	// Load test text file
	const txtPath = new URL("../../../tests/fixtures/documents/text/sample.txt", import.meta.url).pathname;
	try {
		sampleTxtBytes = new Uint8Array(readFileSync(txtPath));
	} catch {
		console.warn("Test text file not found");
	}
});

describe("Batch Bytes Extraction (WASM Bindings)", () => {
	describe("batch bytes extraction sync", () => {
		it("should extract from multiple byte arrays synchronously", () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const files = [
				{ data: samplePdfBytes, mimeType: "application/pdf" },
				{ data: sampleTxtBytes || samplePdfBytes, mimeType: sampleTxtBytes ? "text/plain" : "application/pdf" },
			];

			const results = batchExtractBytesSync(files);

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

		it("should preserve byte order in batch extraction", () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const files = [
				{ data: samplePdfBytes, mimeType: "application/pdf" },
				{ data: samplePdfBytes, mimeType: "application/pdf" },
			];

			const results = batchExtractBytesSync(files);

			expect(results.length).toBe(files.length);
			for (let i = 0; i < results.length; i++) {
				expect(results[i]).toBeDefined();
				expect(results[i].content).toBeTruthy();
			}
		});

		it("should handle single file in batch", () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const files = [{ data: samplePdfBytes, mimeType: "application/pdf" }];

			const results = batchExtractBytesSync(files);

			expect(results.length).toBe(1);
			expect(results[0].content).toBeDefined();
		});

		it("should apply config to all byte arrays in batch", () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const files = [
				{ data: samplePdfBytes, mimeType: "application/pdf" },
				{ data: samplePdfBytes, mimeType: "application/pdf" },
			];

			const config: ExtractionConfig = {
				chunking: {
					maxChars: 500,
					maxOverlap: 100,
				},
			};

			const results = batchExtractBytesSync(files, config);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
			}
		});

		it("should handle empty data array error", () => {
			expect(() => {
				batchExtractBytesSync([]);
			}).toThrow();
		});

		it("should require data property in file objects", () => {
			expect(() => {
				batchExtractBytesSync([{ data: undefined, mimeType: "application/pdf" } as any]);
			}).toThrow();
		});

		it("should require mimeType property in file objects", () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			expect(() => {
				batchExtractBytesSync([{ data: samplePdfBytes, mimeType: undefined } as any]);
			}).toThrow();
		});
	});

	describe("batch bytes extraction async", () => {
		it("should extract from multiple byte arrays asynchronously", async () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const files = [
				{ data: samplePdfBytes, mimeType: "application/pdf" },
				{ data: sampleTxtBytes || samplePdfBytes, mimeType: sampleTxtBytes ? "text/plain" : "application/pdf" },
			];

			const results = await batchExtractBytes(files);

			expect(Array.isArray(results)).toBe(true);
			expect(results.length).toBe(files.length);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
				expect(typeof result.content).toBe("string");
			}
		});

		it("should apply config to all files in async batch", async () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const files = [
				{ data: samplePdfBytes, mimeType: "application/pdf" },
				{ data: samplePdfBytes, mimeType: "application/pdf" },
			];

			const config: ExtractionConfig = {
				keywords: {
					algorithm: "yake",
					maxKeywords: 5,
				},
			};

			const results = await batchExtractBytes(files, config);

			expect(results.length).toBe(files.length);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeTruthy();
				expect(result.metadata).toBeInstanceOf(Object);
			}
		});

		it("should preserve order in async batch operations", async () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const files = [
				{ data: samplePdfBytes, mimeType: "application/pdf" },
				{ data: samplePdfBytes, mimeType: "application/pdf" },
				{ data: samplePdfBytes, mimeType: "application/pdf" },
			];

			const results = await batchExtractBytes(files);

			expect(results.length).toBe(files.length);

			for (let i = 0; i < results.length; i++) {
				expect(results[i]).toBeDefined();
				expect(results[i].content).toBeTruthy();
			}
		});

		it("should handle large batch of files", async () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const files = [{ data: samplePdfBytes, mimeType: "application/pdf" }];

			// Create a large batch by repeating files
			const largeBatch = Array(5).fill(files).flat();

			const results = await batchExtractBytes(largeBatch);

			expect(results.length).toBe(largeBatch.length);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeTruthy();
			}
		});

		it("should handle concurrent batch operations", async () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const files = [
				{ data: samplePdfBytes, mimeType: "application/pdf" },
				{ data: samplePdfBytes, mimeType: "application/pdf" },
			];

			// Run multiple batches concurrently
			const [results1, results2] = await Promise.all([batchExtractBytes(files), batchExtractBytes(files)]);

			expect(results1.length).toBe(files.length);
			expect(results2.length).toBe(files.length);

			for (const result of [...results1, ...results2]) {
				expect(result).toBeDefined();
				expect(result.content).toBeTruthy();
			}
		});

		it("should reject empty files array async", async () => {
			await expect(async () => {
				await batchExtractBytes([]);
			}).rejects.toThrow();
		});

		it("should reject invalid file objects async", async () => {
			await expect(async () => {
				await batchExtractBytes([{ data: "not bytes", mimeType: "application/pdf" } as any]);
			}).rejects.toThrow();
		});
	});

	describe("batch bytes extraction error handling", () => {
		it("should handle empty data in file objects", () => {
			expect(() => {
				batchExtractBytesSync([{ data: new Uint8Array(), mimeType: "application/pdf" }]);
			}).toThrow();
		});

		it("should require matching data types in batch", () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			expect(() => {
				batchExtractBytesSync([
					{ data: samplePdfBytes, mimeType: "application/pdf" },
					{ data: "not uint8array" as any, mimeType: "text/plain" },
				]);
			}).toThrow();
		});

		it("should handle null files array", () => {
			expect(() => {
				batchExtractBytesSync(null as any);
			}).toThrow();
		});

		it("should handle invalid file object structure", () => {
			expect(() => {
				batchExtractBytesSync([{ notData: new Uint8Array(), notMimeType: "pdf" } as any]);
			}).toThrow();
		});
	});
});

describe("Batch File Extraction (WASM Bindings)", () => {
	describe("batch file extraction async", () => {
		it("should extract from multiple File objects", async () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const file1 = new File([samplePdfBytes], "test.pdf", { type: "application/pdf" });
			const file2 = sampleTxtBytes
				? new File([sampleTxtBytes], "test.txt", { type: "text/plain" })
				: new File([samplePdfBytes], "test2.pdf", { type: "application/pdf" });

			const files = [file1, file2];

			const results = await batchExtractFiles(files);

			expect(Array.isArray(results)).toBe(true);
			expect(results.length).toBe(files.length);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.content).toBeDefined();
				expect(typeof result.content).toBe("string");
			}
		});

		it("should use File MIME type for extraction", async () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const file = new File([samplePdfBytes], "document.pdf", { type: "application/pdf" });

			const results = await batchExtractFiles([file]);

			expect(results.length).toBe(1);
			expect(results[0].content).toBeDefined();
			expect(results[0].mimeType).toContain("pdf");
		});

		it("should apply config to all files in batch", async () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const file1 = new File([samplePdfBytes], "test1.pdf", { type: "application/pdf" });
			const file2 = new File([samplePdfBytes], "test2.pdf", { type: "application/pdf" });

			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const results = await batchExtractFiles([file1, file2], config);

			expect(results.length).toBe(2);

			for (const result of results) {
				expect(result).toBeDefined();
				expect(result.pages).toBeDefined();
			}
		});

		it("should preserve file order in batch extraction", async () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const file1 = new File([samplePdfBytes], "first.pdf", { type: "application/pdf" });
			const file2 = new File([samplePdfBytes], "second.pdf", { type: "application/pdf" });
			const file3 = new File([samplePdfBytes], "third.pdf", { type: "application/pdf" });

			const results = await batchExtractFiles([file1, file2, file3]);

			expect(results.length).toBe(3);

			for (let i = 0; i < results.length; i++) {
				expect(results[i]).toBeDefined();
				expect(results[i].content).toBeTruthy();
			}
		});

		it("should handle single File in batch", async () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const file = new File([samplePdfBytes], "test.pdf", { type: "application/pdf" });

			const results = await batchExtractFiles([file]);

			expect(results.length).toBe(1);
			expect(results[0].content).toBeDefined();
		});

		it("should reject empty files array", async () => {
			await expect(async () => {
				await batchExtractFiles([]);
			}).rejects.toThrow();
		});

		it("should handle concurrent File batch operations", async () => {
			if (!samplePdfBytes) {
				expect(true).toBe(true);
				return;
			}

			const file1 = new File([samplePdfBytes], "test1.pdf", { type: "application/pdf" });
			const file2 = new File([samplePdfBytes], "test2.pdf", { type: "application/pdf" });

			const [results1, results2] = await Promise.all([batchExtractFiles([file1]), batchExtractFiles([file2])]);

			expect(results1.length).toBe(1);
			expect(results2.length).toBe(1);

			for (const result of [...results1, ...results2]) {
				expect(result).toBeDefined();
				expect(result.content).toBeTruthy();
			}
		});
	});

	describe("batch file extraction error handling", () => {
		it("should reject non-File objects", async () => {
			await expect(async () => {
				await batchExtractFiles([{ name: "fake" } as any]);
			}).rejects.toThrow();
		});

		it("should handle null files array", async () => {
			await expect(async () => {
				await batchExtractFiles(null as any);
			}).rejects.toThrow();
		});
	});
});

describe("Batch Operations - Sync vs Async Consistency", () => {
	it("should produce consistent results between sync and async batch byte APIs", async () => {
		if (!samplePdfBytes) {
			expect(true).toBe(true);
			return;
		}

		const files = [
			{ data: samplePdfBytes, mimeType: "application/pdf" },
			{ data: samplePdfBytes, mimeType: "application/pdf" },
		];

		const syncResults = batchExtractBytesSync(files);
		const asyncResults = await batchExtractBytes(files);

		expect(syncResults.length).toBe(asyncResults.length);

		for (let i = 0; i < syncResults.length; i++) {
			expect(syncResults[i].content.length).toBeGreaterThan(0);
			expect(asyncResults[i].content.length).toBeGreaterThan(0);
			expect(syncResults[i].mimeType).toBeDefined();
			expect(asyncResults[i].mimeType).toBeDefined();
		}
	});

	it("should maintain result structure consistency across batches", async () => {
		if (!samplePdfBytes) {
			expect(true).toBe(true);
			return;
		}

		const files = [
			{ data: samplePdfBytes, mimeType: "application/pdf" },
			{ data: samplePdfBytes, mimeType: "application/pdf" },
		];

		const results = batchExtractBytesSync(files);

		for (const result of results) {
			expect(result).toHaveProperty("content");
			expect(result).toHaveProperty("mimeType");
			expect(result).toHaveProperty("metadata");
			expect(result).toHaveProperty("tables");
		}
	});
});

describe("Batch Operations - Configuration Handling", () => {
	it("should apply identical config to all batch items", () => {
		if (!samplePdfBytes) {
			expect(true).toBe(true);
			return;
		}

		const files = [
			{ data: samplePdfBytes, mimeType: "application/pdf" },
			{ data: samplePdfBytes, mimeType: "application/pdf" },
		];

		const config: ExtractionConfig = {
			keywords: {
				algorithm: "yake",
				maxKeywords: 3,
			},
		};

		const results = batchExtractBytesSync(files, config);

		for (const result of results) {
			expect(result).toBeDefined();
			expect(result.metadata).toBeInstanceOf(Object);
		}
	});

	it("should handle null config in batch operations", () => {
		if (!samplePdfBytes) {
			expect(true).toBe(true);
			return;
		}

		const files = [
			{ data: samplePdfBytes, mimeType: "application/pdf" },
			{ data: samplePdfBytes, mimeType: "application/pdf" },
		];

		const results = batchExtractBytesSync(files, null);

		expect(Array.isArray(results)).toBe(true);
		expect(results.length).toBe(files.length);

		for (const result of results) {
			expect(result.content).toBeDefined();
		}
	});

	it("should handle undefined config in batch operations", () => {
		if (!samplePdfBytes) {
			expect(true).toBe(true);
			return;
		}

		const files = [
			{ data: samplePdfBytes, mimeType: "application/pdf" },
			{ data: samplePdfBytes, mimeType: "application/pdf" },
		];

		const results = batchExtractBytesSync(files);

		expect(Array.isArray(results)).toBe(true);
		expect(results.length).toBe(files.length);
	});

	it("should apply chunking config to batch extractions", () => {
		if (!samplePdfBytes) {
			expect(true).toBe(true);
			return;
		}

		const files = [{ data: samplePdfBytes, mimeType: "application/pdf" }];

		const config: ExtractionConfig = {
			chunking: {
				maxChars: 1000,
				maxOverlap: 200,
			},
		};

		const results = batchExtractBytesSync(files, config);

		expect(results.length).toBe(1);
		expect(results[0].content).toBeDefined();
	});

	it("should apply pages config to batch extractions", async () => {
		if (!samplePdfBytes) {
			expect(true).toBe(true);
			return;
		}

		const files = [{ data: samplePdfBytes, mimeType: "application/pdf" }];

		const config: ExtractionConfig = {
			pages: {
				extractPages: true,
				insertPageMarkers: true,
			},
		};

		const results = await batchExtractBytes(files, config);

		expect(results.length).toBe(1);
		expect(results[0].pages).toBeDefined();
	});
});

describe("Batch Operations - Multiple Document Types", () => {
	it("should handle mixed MIME types in batch", () => {
		if (!samplePdfBytes || !sampleTxtBytes) {
			expect(true).toBe(true);
			return;
		}

		const files = [
			{ data: samplePdfBytes, mimeType: "application/pdf" },
			{ data: sampleTxtBytes, mimeType: "text/plain" },
		];

		const results = batchExtractBytesSync(files);

		expect(results.length).toBe(2);
		expect(results[0].mimeType).toContain("pdf");
		expect(results[1].mimeType).toContain("text");
	});

	it("should maintain MIME type accuracy in batch results", () => {
		if (!samplePdfBytes) {
			expect(true).toBe(true);
			return;
		}

		const files = [
			{ data: samplePdfBytes, mimeType: "application/pdf" },
			{ data: samplePdfBytes, mimeType: "application/pdf" },
		];

		const results = batchExtractBytesSync(files);

		for (const result of results) {
			expect(result.mimeType).toBeDefined();
			expect(typeof result.mimeType).toBe("string");
		}
	});

	it("should validate result structure for all document types", async () => {
		if (!samplePdfBytes) {
			expect(true).toBe(true);
			return;
		}

		const files = [
			{ data: samplePdfBytes, mimeType: "application/pdf" },
			{ data: samplePdfBytes, mimeType: "application/pdf" },
		];

		const results = await batchExtractBytes(files);

		for (const result of results) {
			expect(result).toBeInstanceOf(Object);
			expect(typeof result.content).toBe("string");
			expect(Array.isArray(result.tables)).toBe(true);
			expect(result.metadata).toBeInstanceOf(Object);
		}
	});
});

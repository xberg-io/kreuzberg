/**
 * ExtractionConfig configuration tests for WASM binding
 *
 * Tests for ExtractionConfig root configuration that orchestrates all other
 * configuration types for comprehensive document extraction control.
 */

import { describe, expect, it } from "vitest";
import type { ExtractionConfig } from "../types";

describe("WASM: ExtractionConfig", () => {
	describe("type definitions", () => {
		it("should define valid ExtractionConfig type", () => {
			const config: ExtractionConfig = {
				useCache: true,
				enableQualityProcessing: true,
				forceOcr: false,
				maxConcurrentExtractions: 4,
			};

			expect(config.useCache).toBe(true);
			expect(config.enableQualityProcessing).toBe(true);
			expect(config.forceOcr).toBe(false);
		});

		it("should support optional fields", () => {
			const minimalConfig: ExtractionConfig = {};
			expect(minimalConfig.useCache).toBeUndefined();
			expect(minimalConfig.ocr).toBeUndefined();
		});

		it("should support all nested configurations", () => {
			const config: ExtractionConfig = {
				useCache: true,
				ocr: { backend: "tesseract" },
				chunking: { maxChars: 512 },
				images: { enabled: true },
				keywords: { algorithm: "yake" },
				languageDetection: { enabled: true },
			};

			expect(config.ocr).toBeDefined();
			expect(config.chunking).toBeDefined();
			expect(config.images).toBeDefined();
			expect(config.keywords).toBeDefined();
			expect(config.languageDetection).toBeDefined();
		});
	});

	describe("WASM serialization", () => {
		it("should serialize for WASM boundary", () => {
			const config: ExtractionConfig = {
				useCache: true,
				enableQualityProcessing: false,
				forceOcr: true,
			};

			const json = JSON.stringify(config);
			const parsed: ExtractionConfig = JSON.parse(json);

			expect(parsed.useCache).toBe(true);
			expect(parsed.enableQualityProcessing).toBe(false);
			expect(parsed.forceOcr).toBe(true);
		});

		it("should handle undefined fields in serialization", () => {
			const config: ExtractionConfig = {
				useCache: true,
				enableQualityProcessing: undefined,
			};

			const json = JSON.stringify(config);
			expect(json).not.toContain("enableQualityProcessing");
			expect(json).toContain("useCache");
		});

		it("should serialize complex nested structures", () => {
			const config: ExtractionConfig = {
				useCache: true,
				ocr: { backend: "tesseract", language: "eng" },
				chunking: { maxChars: 256 },
				images: { enabled: true, targetDpi: 150 },
			};

			const json = JSON.stringify(config);
			const parsed: ExtractionConfig = JSON.parse(json);

			expect(parsed.ocr?.backend).toBe("tesseract");
			expect(parsed.chunking?.maxChars).toBe(256);
			expect(parsed.images?.targetDpi).toBe(150);
		});
	});

	describe("worker message passing", () => {
		it("should serialize for worker communication", () => {
			const config: ExtractionConfig = {
				useCache: true,
				forceOcr: false,
				maxConcurrentExtractions: 4,
			};

			const cloned = structuredClone(config);
			expect(cloned.useCache).toBe(true);
			expect(cloned.forceOcr).toBe(false);
		});

		it("should preserve nested configs in workers", () => {
			const config: ExtractionConfig = {
				useCache: true,
				ocr: { backend: "tesseract", language: "fra" },
				chunking: { maxChars: 512, maxOverlap: 128 },
			};

			const cloned = structuredClone(config);
			expect(cloned.ocr?.backend).toBe("tesseract");
			expect(cloned.chunking?.maxChars).toBe(512);
		});

		it("should handle deeply nested configurations", () => {
			const config: ExtractionConfig = {
				useCache: true,
				pdfOptions: { extractImages: true },
				ocr: { backend: "tesseract", language: "eng" },
				keywords: { algorithm: "rake", maxKeywords: 10 },
			};

			const cloned = structuredClone(config);
			expect(cloned.pdfOptions?.extractImages).toBe(true);
			expect(cloned.keywords?.algorithm).toBe("rake");
		});
	});

	describe("edge cases", () => {
		it("should handle zero concurrent extractions", () => {
			const config: ExtractionConfig = {
				maxConcurrentExtractions: 0,
			};
			expect(config.maxConcurrentExtractions).toBe(0);
		});

		it("should handle all boolean combinations", () => {
			const combinations = [
				{ useCache: true, enableQualityProcessing: true, forceOcr: true },
				{ useCache: false, enableQualityProcessing: false, forceOcr: false },
			];

			combinations.forEach((combo) => {
				const config: ExtractionConfig = combo;
				expect(config.useCache).toBeDefined();
			});
		});

		it("should support minimal configuration", () => {
			const config: ExtractionConfig = { useCache: true };
			expect(config.useCache).toBe(true);
			expect(config.ocr).toBeUndefined();
		});
	});

	describe("immutability patterns", () => {
		it("should support spread operator updates", () => {
			const original: ExtractionConfig = {
				useCache: true,
				forceOcr: false,
			};

			const updated: ExtractionConfig = {
				...original,
				forceOcr: true,
			};

			expect(original.forceOcr).toBe(false);
			expect(updated.forceOcr).toBe(true);
		});

		it("should support nested config updates", () => {
			const original: ExtractionConfig = {
				useCache: true,
				ocr: { backend: "tesseract", language: "eng" },
			};

			const updated: ExtractionConfig = {
				...original,
				ocr: {
					...original.ocr,
					language: "fra",
				},
			};

			expect(original.ocr?.language).toBe("eng");
			expect(updated.ocr?.language).toBe("fra");
		});
	});
});

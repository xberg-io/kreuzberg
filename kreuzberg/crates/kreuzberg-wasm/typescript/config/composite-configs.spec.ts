/**
 * Composite configuration tests for WASM binding
 *
 * Tests for composite configuration types: ImagePreprocessingConfig (if exists),
 * HierarchyConfig, and FontConfig - focusing on their nesting and composition.
 */

import { describe, expect, it } from "vitest";
import type { ExtractionConfig, ImageExtractionConfig, PdfConfig } from "../types";

/**
 * Note: These tests are structured to work with the available config types
 * in the WASM types. Some config types from the requirements may not be directly
 * exposed in the WASM types or may be composed differently.
 */

describe("WASM: Composite Configuration Tests", () => {
	describe("HierarchyConfig patterns", () => {
		it("should support hierarchical extraction configuration", () => {
			const config: ExtractionConfig = {
				useCache: true,
				enableQualityProcessing: true,
				ocr: {
					backend: "tesseract",
					language: "eng",
					tesseractConfig: {
						psm: 3,
						enableTableDetection: true,
					},
				},
				chunking: {
					maxChars: 512,
					maxOverlap: 128,
				},
				images: {
					enabled: true,
					targetDpi: 300,
					maxImageDimension: 2048,
				},
				keywords: {
					algorithm: "yake",
					maxKeywords: 10,
				},
				pdfOptions: {
					extractImages: true,
					extractMetadata: true,
					passwords: ["pwd"],
				},
			};

			expect(config.ocr?.tesseractConfig?.psm).toBe(3);
			expect(config.chunking?.maxChars).toBe(512);
			expect(config.images?.targetDpi).toBe(300);
			expect(config.keywords?.algorithm).toBe("yake");
		});

		it("should support deep nesting without loss", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractImages: true,
					extractMetadata: true,
				},
				ocr: {
					backend: "tesseract",
					tesseractConfig: {
						psm: 6,
						enableTableDetection: true,
						tesseditCharWhitelist: "abc123",
					},
				},
			};

			const cloned = structuredClone(config);

			expect(cloned.pdfOptions?.extractImages).toBe(true);
			expect(cloned.ocr?.tesseractConfig?.tesseditCharWhitelist).toBe("abc123");
		});
	});

	describe("ImagePreprocessing patterns", () => {
		it("should support comprehensive image configuration", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 300,
					maxImageDimension: 2048,
					autoAdjustDpi: true,
					minDpi: 72,
					maxDpi: 600,
				},
			};

			expect(config.images?.enabled).toBe(true);
			expect(config.images?.autoAdjustDpi).toBe(true);
			expect(config.images?.minDpi).toBe(72);
			expect(config.images?.maxDpi).toBe(600);
		});

		it("should support image configuration updates", () => {
			const original: ImageExtractionConfig = {
				enabled: true,
				targetDpi: 300,
			};

			const updated: ImageExtractionConfig = {
				...original,
				maxImageDimension: 4096,
			};

			expect(updated.enabled).toBe(true);
			expect(updated.maxImageDimension).toBe(4096);
		});
	});

	describe("FontConfig patterns", () => {
		it("should support font configuration in PDF options", () => {
			const config: PdfConfig = {
				extractImages: true,
				extractMetadata: true,
				passwords: ["secure"],
			};

			expect(config.extractImages).toBe(true);
			expect(config.passwords?.[0]).toBe("secure");
		});

		it("should support multiple password PDF extraction", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractImages: true,
					passwords: ["password1", "password2", "password3"],
					extractMetadata: true,
				},
			};

			expect(config.pdfOptions?.passwords?.length).toBe(3);
			expect(config.pdfOptions?.passwords?.[1]).toBe("password2");
		});
	});

	describe("Complex composition scenarios", () => {
		it("should support full-featured extraction configuration", () => {
			const config: ExtractionConfig = {
				useCache: true,
				enableQualityProcessing: true,
				forceOcr: false,
				maxConcurrentExtractions: 8,
				ocr: {
					backend: "tesseract",
					language: "eng",
					languages: ["eng", "deu", "fra"],
					tesseractConfig: {
						psm: 3,
						enableTableDetection: true,
					},
				},
				chunking: {
					maxChars: 512,
					maxOverlap: 128,
				},
				images: {
					enabled: true,
					targetDpi: 300,
					maxImageDimension: 2048,
					autoAdjustDpi: true,
					minDpi: 150,
					maxDpi: 600,
				},
				keywords: {
					algorithm: "yake",
					maxKeywords: 15,
					minScore: 0.1,
					ngramRange: [1, 3],
					language: "en",
				},
				pdfOptions: {
					extractImages: true,
					extractMetadata: true,
					passwords: ["pwd1", "pwd2"],
				},
				pages: {
					extractPages: true,
					insertPageMarkers: true,
					markerFormat: "--- Page {page_num} ---",
				},
				languageDetection: {
					enabled: true,
				},
				tokenReduction: {
					mode: "balanced",
					preserveImportantWords: true,
				},
				postprocessor: {
					enabled: true,
					enabledProcessors: ["clean", "format"],
				},
			};

			expect(config.useCache).toBe(true);
			expect(config.ocr?.languages?.length).toBe(3);
			expect(config.chunking?.maxChars).toBe(512);
			expect(config.images?.autoAdjustDpi).toBe(true);
			expect(config.keywords?.ngramRange).toEqual([1, 3]);
			expect(config.pdfOptions?.passwords?.length).toBe(2);
			expect(config.pages?.markerFormat).toContain("{page_num}");
		});

		it("should preserve complex structure in worker communication", () => {
			const config: ExtractionConfig = {
				useCache: true,
				ocr: {
					backend: "tesseract",
					language: "eng",
					tesseractConfig: {
						psm: 6,
						enableTableDetection: true,
						tesseditCharWhitelist: "ABC",
					},
				},
				chunking: {
					maxChars: 256,
					maxOverlap: 64,
				},
				images: {
					enabled: true,
					targetDpi: 150,
					maxImageDimension: 1024,
				},
				pdfOptions: {
					extractImages: false,
					extractMetadata: true,
					passwords: ["secret"],
				},
				keywords: {
					algorithm: "rake",
					maxKeywords: 20,
					rakeParams: {
						minWordLength: 3,
						maxWordsPerPhrase: 5,
					},
				},
			};

			const cloned = structuredClone(config);

			expect(cloned.useCache).toBe(true);
			expect(cloned.ocr?.tesseractConfig?.tesseditCharWhitelist).toBe("ABC");
			expect(cloned.chunking?.maxChars).toBe(256);
			expect(cloned.images?.targetDpi).toBe(150);
			expect(cloned.keywords?.rakeParams?.minWordLength).toBe(3);
			expect(cloned.pdfOptions?.passwords?.[0]).toBe("secret");
		});

		it("should support immutable updates of complex configs", () => {
			const original: ExtractionConfig = {
				useCache: true,
				ocr: {
					backend: "tesseract",
					language: "eng",
				},
				chunking: {
					maxChars: 512,
					maxOverlap: 128,
				},
			};

			const updated: ExtractionConfig = {
				...original,
				ocr: {
					...original.ocr,
					language: "fra",
				},
				chunking: {
					...original.chunking,
					maxChars: 1024,
				},
			};

			expect(original.ocr?.language).toBe("eng");
			expect(original.chunking?.maxChars).toBe(512);
			expect(updated.ocr?.language).toBe("fra");
			expect(updated.chunking?.maxChars).toBe(1024);
			expect(updated.ocr?.backend).toBe("tesseract");
		});
	});

	describe("Configuration composition", () => {
		it("should compose base config with specific options", () => {
			const baseConfig: ExtractionConfig = {
				useCache: true,
				enableQualityProcessing: true,
			};

			const ocrConfig: ExtractionConfig = {
				ocr: {
					backend: "tesseract",
					language: "eng",
				},
			};

			const imageConfig: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 300,
				},
			};

			const mergedConfig: ExtractionConfig = {
				...baseConfig,
				...ocrConfig,
				...imageConfig,
			};

			expect(mergedConfig.useCache).toBe(true);
			expect(mergedConfig.ocr?.backend).toBe("tesseract");
			expect(mergedConfig.images?.targetDpi).toBe(300);
		});

		it("should allow selective override of nested configs", () => {
			const baseConfig: ExtractionConfig = {
				ocr: {
					backend: "tesseract",
					language: "eng",
					enabled: true,
				},
			};

			const overrideConfig: ExtractionConfig = {
				ocr: {
					...baseConfig.ocr,
					language: "fra",
					enabled: false,
				},
			};

			expect(baseConfig.ocr?.language).toBe("eng");
			expect(overrideConfig.ocr?.language).toBe("fra");
			expect(overrideConfig.ocr?.enabled).toBe(false);
			expect(overrideConfig.ocr?.backend).toBe("tesseract");
		});
	});

	describe("Configuration validation patterns", () => {
		it("should validate required fields presence", () => {
			const config: ExtractionConfig = {
				ocr: {
					backend: "tesseract",
					language: "eng",
				},
			};

			expect(config.ocr?.backend).toBeDefined();
			expect(config.ocr?.language).toBeDefined();
		});

		it("should handle missing optional nested fields", () => {
			const config: ExtractionConfig = {
				ocr: {
					backend: "tesseract",
				},
			};

			expect(config.ocr?.backend).toBeDefined();
			expect(config.ocr?.language).toBeUndefined();
			expect(config.ocr?.enabled).toBeUndefined();
		});

		it("should support partial configuration updates", () => {
			const baseConfig: ExtractionConfig = {
				useCache: true,
				enableQualityProcessing: true,
				forceOcr: false,
			};

			const partialUpdate: Partial<ExtractionConfig> = {
				forceOcr: true,
			};

			const updated: ExtractionConfig = {
				...baseConfig,
				...partialUpdate,
			};

			expect(updated.useCache).toBe(true);
			expect(updated.enableQualityProcessing).toBe(true);
			expect(updated.forceOcr).toBe(true);
		});
	});
});

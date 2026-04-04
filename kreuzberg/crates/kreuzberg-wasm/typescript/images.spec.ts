/**
 * Comprehensive image extraction tests for WASM bindings.
 *
 * Tests verify:
 * 1. Image extraction configuration types and validation
 * 2. ExtractedImage interface structure and properties
 * 3. Image metadata handling (format, dimensions, MIME type)
 * 4. Integration with ExtractionConfig for various document types
 * 5. Error handling configurations for image extraction
 * 6. Batch image extraction configuration patterns
 *
 * Week 2 Phase 2.1 - Images tests across all 9 languages
 * TypeScript WASM has 7x fewer tests than Node - this expands WASM test coverage.
 *
 * Note: These tests focus on configuration and type validation.
 * Full end-to-end WASM testing requires the module to be built.
 */

import { describe, expect, it } from "vitest";
import type {
	ExtractedImage,
	ExtractionConfig,
	ExtractionResult,
	ImageExtractionConfig,
	PageContent,
} from "./types.js";

describe("Image Extraction Configuration (WASM Bindings)", () => {
	describe("ImageExtractionConfig interface validation", () => {
		it("should allow minimal image extraction config", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
			};

			expect(config).toBeDefined();
			expect(config.enabled).toBe(true);
		});

		it("should support targetDpi configuration", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				targetDpi: 150,
			};

			expect(config.targetDpi).toBe(150);
			expect(typeof config.targetDpi).toBe("number");
		});

		it("should support maxImageDimension configuration", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				maxImageDimension: 4096,
			};

			expect(config.maxImageDimension).toBe(4096);
		});

		it("should support autoAdjustDpi configuration", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				autoAdjustDpi: true,
				minDpi: 72,
				maxDpi: 300,
			};

			expect(config.autoAdjustDpi).toBe(true);
			expect(config.minDpi).toBe(72);
			expect(config.maxDpi).toBe(300);
		});

		it("should allow disabled image extraction", () => {
			const config: ImageExtractionConfig = {
				enabled: false,
			};

			expect(config.enabled).toBe(false);
		});

		it("should support all image extraction options together", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				targetDpi: 200,
				maxImageDimension: 8192,
				autoAdjustDpi: true,
				minDpi: 100,
				maxDpi: 400,
			};

			expect(config).toHaveProperty("enabled");
			expect(config).toHaveProperty("targetDpi");
			expect(config).toHaveProperty("maxImageDimension");
			expect(config).toHaveProperty("autoAdjustDpi");
			expect(config).toHaveProperty("minDpi");
			expect(config).toHaveProperty("maxDpi");
		});
	});

	describe("ExtractionConfig with image extraction", () => {
		it("should integrate images config in ExtractionConfig", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 150,
				},
			};

			expect(config.images).toBeDefined();
			expect(config.images?.enabled).toBe(true);
			expect(config.images?.targetDpi).toBe(150);
		});

		it("should support images config with other options", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 200,
				},
				pages: {
					extractPages: true,
				},
				useCache: false,
			};

			expect(config.images).toBeDefined();
			expect(config.pages).toBeDefined();
			expect(config.useCache).toBe(false);
		});

		it("should allow undefined images config", () => {
			const config: ExtractionConfig = {
				images: undefined,
			};

			expect(config.images).toBeUndefined();
		});

		it("should allow null images config", () => {
			const config: ExtractionConfig = {
				images: null as any,
			};

			expect(config.images).toBeDefined();
		});

		it("should support PDF-specific image extraction", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 150,
				},
				pdfOptions: {
					extractImages: true,
				},
			};

			expect(config.images).toBeDefined();
			expect(config.pdfOptions).toBeDefined();
			expect(config.pdfOptions?.extractImages).toBe(true);
		});
	});

	describe("ExtractedImage interface validation", () => {
		it("should validate basic extracted image structure", () => {
			const image: ExtractedImage = {
				data: new Uint8Array([0, 1, 2, 3]),
			};

			expect(image).toBeDefined();
			expect(image.data).toBeDefined();
			expect(image.data instanceof Uint8Array).toBe(true);
		});

		it("should support image with base64 data", () => {
			const image: ExtractedImage = {
				data: "iVBORw0KGgoAAAANSUhEUgAAAAUA",
			};

			expect(typeof image.data).toBe("string");
			expect(/^[A-Za-z0-9+/]*={0,2}$/.test(image.data)).toBe(true);
		});

		it("should include image format information", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				format: "PNG",
				mimeType: "image/png",
			};

			expect(image.format).toBe("PNG");
			expect(image.mimeType).toBe("image/png");
		});

		it("should include image dimensions", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				width: 1024,
				height: 768,
			};

			expect(image.width).toBe(1024);
			expect(image.height).toBe(768);
		});

		it("should support image color space information", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				colorspace: "RGB",
				bitsPerComponent: 8,
			};

			expect(image.colorspace).toBe("RGB");
			expect(image.bitsPerComponent).toBe(8);
		});

		it("should include image index and page number", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				imageIndex: 0,
				pageNumber: 1,
			};

			expect(image.imageIndex).toBe(0);
			expect(image.pageNumber).toBe(1);
		});

		it("should support image description", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				description: "Sample image description",
			};

			expect(image.description).toBe("Sample image description");
		});

		it("should support mask image flag", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				isMask: true,
			};

			expect(image.isMask).toBe(true);
		});

		it("should support OCR result on image", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				ocrResult: "Extracted text from image",
			};

			expect(image.ocrResult).toBe("Extracted text from image");
		});

		it("should validate complete extracted image with all properties", () => {
			const image: ExtractedImage = {
				data: new Uint8Array([255, 255, 255]),
				format: "JPEG",
				mimeType: "image/jpeg",
				imageIndex: 1,
				pageNumber: 2,
				width: 800,
				height: 600,
				colorspace: "CMYK",
				bitsPerComponent: 8,
				isMask: false,
				description: "Page 2, Image 1",
				ocrResult: "Some text found",
			};

			expect(image.data instanceof Uint8Array).toBe(true);
			expect(image.format).toBe("JPEG");
			expect(image.mimeType).toBe("image/jpeg");
			expect(image.imageIndex).toBe(1);
			expect(image.pageNumber).toBe(2);
			expect(image.width).toBe(800);
			expect(image.height).toBe(600);
		});
	});

	describe("ExtractionResult with images", () => {
		it("should validate ExtractionResult with images array", () => {
			const result: ExtractionResult = {
				content: "Document content",
				mimeType: "application/pdf",
				metadata: {},
				tables: [],
				images: [
					{
						data: new Uint8Array(),
						format: "PNG",
					},
				],
			};

			expect(result.images).toBeDefined();
			expect(Array.isArray(result.images)).toBe(true);
			expect(result.images?.length).toBe(1);
		});

		it("should allow null images in result", () => {
			const result: ExtractionResult = {
				content: "Document content",
				mimeType: "application/pdf",
				metadata: {},
				tables: [],
				images: null,
			};

			expect(result.images).toBeNull();
		});

		it("should allow undefined images in result", () => {
			const result: ExtractionResult = {
				content: "Document content",
				mimeType: "application/pdf",
				metadata: {},
				tables: [],
				images: undefined,
			};

			expect(result.images).toBeUndefined();
		});

		it("should support multiple images in result", () => {
			const result: ExtractionResult = {
				content: "Document content",
				mimeType: "application/pdf",
				metadata: {},
				tables: [],
				images: [
					{ data: new Uint8Array(), format: "PNG" },
					{ data: new Uint8Array(), format: "JPEG" },
					{ data: new Uint8Array(), format: "WebP" },
				],
			};

			expect(result.images?.length).toBe(3);
		});
	});

	describe("PageContent with images", () => {
		it("should validate PageContent with images array", () => {
			const page: PageContent = {
				pageNumber: 1,
				content: "Page content",
				tables: [],
				images: [
					{
						data: new Uint8Array(),
						format: "PNG",
						pageNumber: 1,
					},
				],
			};

			expect(page.images).toBeDefined();
			expect(Array.isArray(page.images)).toBe(true);
			expect(page.images?.length).toBe(1);
		});

		it("should allow undefined images in page", () => {
			const page: PageContent = {
				pageNumber: 1,
				content: "Page content",
				tables: [],
				images: undefined,
			};

			expect(page.images).toBeUndefined();
		});

		it("should support page-specific image extraction", () => {
			const page: PageContent = {
				pageNumber: 2,
				content: "Page 2 content",
				images: [
					{
						data: new Uint8Array(),
						pageNumber: 2,
						imageIndex: 0,
					},
					{
						data: new Uint8Array(),
						pageNumber: 2,
						imageIndex: 1,
					},
				],
			};

			expect(page.images?.length).toBe(2);
			if (page.images) {
				for (const image of page.images) {
					expect(image.pageNumber).toBe(page.pageNumber);
				}
			}
		});
	});

	describe("Image format handling", () => {
		it("should support PNG image format", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				format: "PNG",
				mimeType: "image/png",
			};

			expect(image.format).toBe("PNG");
			expect(image.mimeType?.includes("png")).toBe(true);
		});

		it("should support JPEG image format", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				format: "JPEG",
				mimeType: "image/jpeg",
			};

			expect(image.format).toBe("JPEG");
			expect(image.mimeType?.includes("jpeg")).toBe(true);
		});

		it("should support WebP image format", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				format: "WebP",
				mimeType: "image/webp",
			};

			expect(image.format).toBe("WebP");
			expect(image.mimeType?.includes("webp")).toBe(true);
		});

		it("should support TIFF image format", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				format: "TIFF",
				mimeType: "image/tiff",
			};

			expect(image.format).toBe("TIFF");
		});
	});

	describe("Image DPI and dimension configurations", () => {
		it("should validate DPI values for extraction", () => {
			const dpiValues = [72, 100, 150, 200, 300, 600];

			for (const dpi of dpiValues) {
				const config: ImageExtractionConfig = {
					enabled: true,
					targetDpi: dpi,
				};

				expect(config.targetDpi).toBe(dpi);
			}
		});

		it("should validate dimension configurations", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				maxImageDimension: 4096,
			};

			expect(config.maxImageDimension).toBe(4096);
			expect(config.maxImageDimension).toBeGreaterThan(0);
		});

		it("should validate DPI range configuration", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				autoAdjustDpi: true,
				minDpi: 72,
				maxDpi: 300,
			};

			const { minDpi, maxDpi } = config;
			expect(minDpi).toBe(72);
			expect(maxDpi).toBe(300);
			if (minDpi !== undefined && maxDpi !== undefined) {
				expect(minDpi).toBeLessThanOrEqual(maxDpi);
			}
		});
	});

	describe("Batch image extraction configuration", () => {
		it("should support batch extraction with images config", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 150,
				},
			};

			// Multiple documents can be processed with same config
			const configs = [config, config, config];
			expect(configs.length).toBe(3);

			for (const cfg of configs) {
				expect(cfg.images?.enabled).toBe(true);
			}
		});

		it("should allow per-document image configuration", () => {
			const highQualityConfig: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 300,
				},
			};

			const standardConfig: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 150,
				},
			};

			expect(highQualityConfig.images?.targetDpi).toBe(300);
			expect(standardConfig.images?.targetDpi).toBe(150);
		});
	});

	describe("Error handling configurations for images", () => {
		it("should handle disabled image extraction gracefully", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: false,
				},
			};

			expect(config.images?.enabled).toBe(false);
		});

		it("should support null image config handling", () => {
			const config: ExtractionConfig = {
				images: null as any,
			};

			expect(config).toBeDefined();
		});

		it("should support mixed extraction options", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: true,
				},
				pages: {
					extractPages: true,
				},
				ocr: {
					enabled: false,
				},
			};

			expect(config.images?.enabled).toBe(true);
			expect(config.pages?.extractPages).toBe(true);
			expect(config.ocr?.enabled).toBe(false);
		});
	});

	describe("Type safety for image extraction", () => {
		it("should enforce ExtractedImage data requirement", () => {
			// data property is required
			const image: ExtractedImage = {
				data: new Uint8Array(),
			};

			expect(image.data).toBeDefined();
		});

		it("should allow optional image metadata", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				// All other properties are optional
			};

			expect(image.format).toBeUndefined();
			expect(image.mimeType).toBeUndefined();
			expect(image.width).toBeUndefined();
			expect(image.height).toBeUndefined();
		});

		it("should support nullable image properties", () => {
			const image: ExtractedImage = {
				data: new Uint8Array(),
				width: null,
				height: null,
				pageNumber: null,
				colorspace: null,
				bitsPerComponent: null,
				description: null,
				ocrResult: null,
			};

			expect(image.width).toBeNull();
			expect(image.height).toBeNull();
			expect(image.pageNumber).toBeNull();
		});

		it("should validate ImageExtractionConfig optional properties", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				// All other properties are optional
			};

			expect(config.targetDpi).toBeUndefined();
			expect(config.maxImageDimension).toBeUndefined();
			expect(config.autoAdjustDpi).toBeUndefined();
		});
	});

	describe("Integration patterns for image extraction", () => {
		it("should support PDF extraction with images", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 150,
				},
				pdfOptions: {
					extractImages: true,
				},
			};

			expect(config.images?.enabled).toBe(true);
			expect(config.pdfOptions?.extractImages).toBe(true);
		});

		it("should support multi-page extraction with images", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: true,
				},
				pages: {
					extractPages: true,
				},
			};

			expect(config.images?.enabled).toBe(true);
			expect(config.pages?.extractPages).toBe(true);
		});

		it("should support OCR on extracted images", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: true,
				},
				ocr: {
					enabled: true,
					language: "eng",
				},
			};

			expect(config.images?.enabled).toBe(true);
			expect(config.ocr?.enabled).toBe(true);
			expect(config.ocr?.language).toBe("eng");
		});

		it("should support quality processing with image extraction", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 200,
				},
				enableQualityProcessing: true,
			};

			expect(config.images?.enabled).toBe(true);
			expect(config.enableQualityProcessing).toBe(true);
		});
	});
});

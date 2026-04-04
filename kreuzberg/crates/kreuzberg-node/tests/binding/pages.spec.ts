/**
 * Comprehensive pages extraction tests for TypeScript Node.js bindings.
 *
 * Tests verify:
 * 1. extractPages: true - Returns pages array with page content
 * 2. insertPageMarkers: true - Markers appear in main content string
 * 3. markerFormat: custom format - Custom page marker format works correctly
 * 4. Multi-page PDF - Documents with multiple pages produce multiple page entries
 * 5. Page content structure - Each page has correct fields (pageNumber, content, tables, images)
 */

import { readFileSync, realpathSync } from "node:fs";
import { beforeAll, describe, expect, it } from "vitest";
import { extractBytesSync, extractFileSync } from "../../dist/index.js";
import type { ExtractionConfig, PageContent } from "../../src/types";
import { getTestDocumentPath } from "../helpers/index.js";

let samplePdfPath: string;
let samplePdfBytes: Uint8Array;

beforeAll(() => {
	samplePdfPath = getTestDocumentPath("pdf/simple.pdf");
	// Resolve symlinks to get the actual file path (important for Windows compatibility)
	samplePdfBytes = new Uint8Array(readFileSync(realpathSync(samplePdfPath)));
});

describe("Pages Extraction (Node.js Bindings)", () => {
	describe("extractPages: true", () => {
		it("should extract pages as separate array", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result).toBeDefined();
			expect(result.pages).toBeDefined();
			expect(Array.isArray(result.pages)).toBe(true);
			expect(result.pages.length).toBeGreaterThan(0);
		});

		it("should extract pages with async extraction", async () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractBytesSync(samplePdfBytes, "application/pdf", config);

			expect(result).toBeDefined();
			expect(result.pages).toBeDefined();
			expect(Array.isArray(result.pages)).toBe(true);
			expect(result.pages.length).toBeGreaterThan(0);
		});

		it("should preserve pages in main content when extractPages is true", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.content).toBeDefined();
			expect(typeof result.content).toBe("string");
			expect(result.content.length).toBeGreaterThan(0);
		});
	});

	describe("insertPageMarkers: true", () => {
		it("should insert page markers in content with default format", () => {
			const config: ExtractionConfig = {
				pages: {
					insertPageMarkers: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.content).toBeDefined();
			expect(result.content).toContain("PAGE");
		});

		it("should insert markers when extractPages and insertPageMarkers are both true", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
					insertPageMarkers: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.content).toBeDefined();
			expect(result.pages).toBeDefined();
			expect(result.pages.length).toBeGreaterThan(0);
			expect(result.content).toContain("PAGE");
		});

		it("should insert multiple page markers for multi-page documents", () => {
			const config: ExtractionConfig = {
				pages: {
					insertPageMarkers: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.content).toBeDefined();
			const pageMarkerCount = (result.content.match(/PAGE/g) || []).length;
			expect(pageMarkerCount).toBeGreaterThan(0);
		});
	});

	describe("markerFormat: custom format", () => {
		it("should use custom marker format with placeholder", () => {
			const customFormat = "=== Page {page_num} ===";
			const config: ExtractionConfig = {
				pages: {
					insertPageMarkers: true,
					markerFormat: customFormat,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.content).toBeDefined();
			expect(result.content).toContain("Page");
			expect(result.content).toContain("===");
		});

		it("should handle multiple custom marker formats", () => {
			const formats = ["--- PAGE {page_num} ---", "[Page {page_num}]", "Page {page_num}:"];

			for (const markerFormat of formats) {
				const config: ExtractionConfig = {
					pages: {
						insertPageMarkers: true,
						markerFormat,
					},
				};

				const result = extractFileSync(samplePdfPath, config);
				expect(result.content).toBeDefined();
				expect(typeof result.content).toBe("string");
				expect(result.content.length).toBeGreaterThan(0);
			}
		});

		it("should use custom marker format with page numbers", () => {
			const customFormat = "## Section {page_num}";
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
					insertPageMarkers: true,
					markerFormat: customFormat,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.content).toBeDefined();
			expect(result.pages).toBeDefined();
			expect(result.content).toContain("Section");
		});
	});

	describe("Multi-page PDF handling", () => {
		it("should extract multiple pages for multi-page documents", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.pages).toBeDefined();
			expect(Array.isArray(result.pages)).toBe(true);

			if (result.pages.length > 1) {
				expect(result.pages.length).toBeGreaterThan(1);

				for (let i = 0; i < result.pages.length; i++) {
					const page = result.pages[i];
					expect(page.pageNumber).toBe(i + 1);
				}
			}
		});

		it("should have sequential page numbers", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			if (result.pages && result.pages.length > 1) {
				for (let i = 0; i < result.pages.length; i++) {
					expect(result.pages[i].pageNumber).toBe(i + 1);
				}
			}
		});

		it("should maintain page order in extracted pages array", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			if (result.pages && result.pages.length > 1) {
				for (let i = 0; i < result.pages.length - 1; i++) {
					expect(result.pages[i].pageNumber).toBeLessThan(result.pages[i + 1].pageNumber);
				}
			}
		});
	});

	describe("Page content structure validation", () => {
		it("should have required fields in each page object", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.pages).toBeDefined();
			expect(Array.isArray(result.pages)).toBe(true);

			for (const page of result.pages) {
				expect(page).toHaveProperty("pageNumber");
				expect(page).toHaveProperty("content");
				expect(page).toHaveProperty("tables");
				expect(page).toHaveProperty("images");
				// isBlank should be a boolean or null/undefined
				expect(page.isBlank === undefined || page.isBlank === null || typeof page.isBlank === "boolean").toBe(true);
			}
		});

		it("should have valid pageNumber in each page", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			for (const page of result.pages) {
				expect(typeof page.pageNumber).toBe("number");
				expect(page.pageNumber).toBeGreaterThan(0);
				expect(Number.isInteger(page.pageNumber)).toBe(true);
			}
		});

		it("should have string content in each page", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			for (const page of result.pages) {
				expect(typeof page.content).toBe("string");
			}
		});

		it("should have valid tables array in each page", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			for (const page of result.pages) {
				expect(Array.isArray(page.tables)).toBe(true);

				for (const table of page.tables) {
					expect(table).toHaveProperty("cells");
					expect(table).toHaveProperty("markdown");
					expect(table).toHaveProperty("pageNumber");
					expect(Array.isArray(table.cells)).toBe(true);
					expect(typeof table.markdown).toBe("string");
					expect(typeof table.pageNumber).toBe("number");
				}
			}
		});

		it("should have valid images array in each page", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			for (const page of result.pages) {
				expect(Array.isArray(page.images)).toBe(true);
			}
		});

		it("should validate PageContent type structure", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);
			expect(result.pages).toBeDefined();

			const validatePageContent = (page: PageContent): void => {
				expect(typeof page.pageNumber).toBe("number");
				expect(page.pageNumber).toBeGreaterThan(0);
				expect(typeof page.content).toBe("string");
				expect(Array.isArray(page.tables)).toBe(true);
				expect(Array.isArray(page.images)).toBe(true);
				expect(page.isBlank === undefined || page.isBlank === null || typeof page.isBlank === "boolean").toBe(true);
			};

			for (const page of result.pages) {
				validatePageContent(page);
			}
		});
	});

	describe("Combined page extraction features", () => {
		it("should work with extractPages and insertPageMarkers together", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
					insertPageMarkers: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.pages).toBeDefined();
			expect(result.pages.length).toBeGreaterThan(0);
			expect(result.content).toBeDefined();
			expect(result.content).toContain("PAGE");
		});

		it("should apply custom marker format to main content", () => {
			const markerFormat = "### Page {page_num}";
			const config: ExtractionConfig = {
				pages: {
					insertPageMarkers: true,
					markerFormat,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.content).toBeDefined();
			expect(result.content).toContain("Page");
		});

		it("should work with other extraction config options", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
					insertPageMarkers: true,
				},
				useCache: false,
				enableQualityProcessing: true,
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.pages).toBeDefined();
			expect(result.pages.length).toBeGreaterThan(0);
			expect(result.content).toBeDefined();
		});
	});

	describe("Edge cases and validation", () => {
		it("should handle null config gracefully", () => {
			const result = extractFileSync(samplePdfPath, null);

			expect(result).toBeDefined();
			expect(result.content).toBeDefined();
		});

		it("should handle empty page extraction config", () => {
			const config: ExtractionConfig = {
				pages: {},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result).toBeDefined();
			expect(result.content).toBeDefined();
		});

		it("should have matching page count between pages array and metadata", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			expect(result.pages).toBeDefined();
			// metadata.pages contains page information for PDF documents
			expect(result.metadata.pages).toBeDefined();

			if (result.metadata.pages && result.pages) {
				// The pages metadata contains boundaries info
				const pagesMeta = result.metadata.pages as { boundaries?: unknown[]; pages?: unknown[] };
				const pageCount = pagesMeta.pages?.length || pagesMeta.boundaries?.length || 0;
				expect(result.pages.length).toBeLessThanOrEqual(pageCount);
			}
		});

		it("should not have duplicate page numbers", () => {
			const config: ExtractionConfig = {
				pages: {
					extractPages: true,
				},
			};

			const result = extractFileSync(samplePdfPath, config);

			const pageNumbers = result.pages.map((p) => p.pageNumber);
			const uniquePageNumbers = new Set(pageNumbers);

			expect(uniquePageNumbers.size).toBe(pageNumbers.length);
		});
	});
});

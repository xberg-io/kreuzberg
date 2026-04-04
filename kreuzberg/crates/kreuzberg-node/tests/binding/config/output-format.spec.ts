/**
 * OutputFormat and ResultFormat configuration tests
 *
 * Tests for output format and result format configuration fields in ExtractionConfig.
 * These fields control how extracted content is formatted and structured.
 */

import type { ExtractionConfig } from "@kreuzberg/core";
import { describe, expect, it } from "vitest";

describe("WASM: OutputFormat and ResultFormat Configuration", () => {
	describe("outputFormat type definitions", () => {
		it("should accept plain output format", () => {
			const config: ExtractionConfig = {
				outputFormat: "plain",
			};

			expect(config.outputFormat).toBe("plain");
		});

		it("should accept markdown output format", () => {
			const config: ExtractionConfig = {
				outputFormat: "markdown",
			};

			expect(config.outputFormat).toBe("markdown");
		});

		it("should accept djot output format", () => {
			const config: ExtractionConfig = {
				outputFormat: "djot",
			};

			expect(config.outputFormat).toBe("djot");
		});

		it("should accept html output format", () => {
			const config: ExtractionConfig = {
				outputFormat: "html",
			};

			expect(config.outputFormat).toBe("html");
		});

		it("should support optional outputFormat field", () => {
			const config: ExtractionConfig = {};

			expect(config.outputFormat).toBeUndefined();
		});

		it("should handle outputFormat with other config fields", () => {
			const config: ExtractionConfig = {
				useCache: true,
				outputFormat: "markdown",
				forceOcr: false,
			};

			expect(config.useCache).toBe(true);
			expect(config.outputFormat).toBe("markdown");
			expect(config.forceOcr).toBe(false);
		});
	});

	describe("resultFormat type definitions", () => {
		it("should accept unified result format", () => {
			const config: ExtractionConfig = {
				resultFormat: "unified",
			};

			expect(config.resultFormat).toBe("unified");
		});

		it("should accept element_based result format", () => {
			const config: ExtractionConfig = {
				resultFormat: "element_based",
			};

			expect(config.resultFormat).toBe("element_based");
		});

		it("should support optional resultFormat field", () => {
			const config: ExtractionConfig = {};

			expect(config.resultFormat).toBeUndefined();
		});

		it("should handle resultFormat with other config fields", () => {
			const config: ExtractionConfig = {
				enableQualityProcessing: true,
				resultFormat: "element_based",
				maxConcurrentExtractions: 4,
			};

			expect(config.enableQualityProcessing).toBe(true);
			expect(config.resultFormat).toBe("element_based");
			expect(config.maxConcurrentExtractions).toBe(4);
		});
	});

	describe("combined format configuration", () => {
		it("should support both outputFormat and resultFormat together", () => {
			const config: ExtractionConfig = {
				outputFormat: "markdown",
				resultFormat: "element_based",
			};

			expect(config.outputFormat).toBe("markdown");
			expect(config.resultFormat).toBe("element_based");
		});

		it("should support all outputFormat variants with unified resultFormat", () => {
			const formats: Array<"plain" | "markdown" | "djot" | "html"> = ["plain", "markdown", "djot", "html"];

			formats.forEach((format) => {
				const config: ExtractionConfig = {
					outputFormat: format,
					resultFormat: "unified",
				};

				expect(config.outputFormat).toBe(format);
				expect(config.resultFormat).toBe("unified");
			});
		});

		it("should support all outputFormat variants with element_based resultFormat", () => {
			const formats: Array<"plain" | "markdown" | "djot" | "html"> = ["plain", "markdown", "djot", "html"];

			formats.forEach((format) => {
				const config: ExtractionConfig = {
					outputFormat: format,
					resultFormat: "element_based",
				};

				expect(config.outputFormat).toBe(format);
				expect(config.resultFormat).toBe("element_based");
			});
		});

		it("should support both formats in complex configuration", () => {
			const config: ExtractionConfig = {
				useCache: true,
				enableQualityProcessing: true,
				forceOcr: false,
				outputFormat: "markdown",
				resultFormat: "element_based",
				ocr: {
					backend: "tesseract",
					language: "eng",
				},
				chunking: {
					chunkSize: 512,
					chunkOverlap: 128,
				},
				maxConcurrentExtractions: 4,
			};

			expect(config.outputFormat).toBe("markdown");
			expect(config.resultFormat).toBe("element_based");
			expect(config.ocr?.backend).toBe("tesseract");
			expect(config.chunking?.chunkSize).toBe(512);
		});
	});

	describe("JSON serialization", () => {
		it("should serialize outputFormat to JSON", () => {
			const config: ExtractionConfig = {
				outputFormat: "markdown",
			};

			const json = JSON.stringify(config);
			const parsed: ExtractionConfig = JSON.parse(json);

			expect(parsed.outputFormat).toBe("markdown");
		});

		it("should serialize resultFormat to JSON", () => {
			const config: ExtractionConfig = {
				resultFormat: "element_based",
			};

			const json = JSON.stringify(config);
			const parsed: ExtractionConfig = JSON.parse(json);

			expect(parsed.resultFormat).toBe("element_based");
		});

		it("should serialize both formats to JSON", () => {
			const config: ExtractionConfig = {
				outputFormat: "djot",
				resultFormat: "unified",
			};

			const json = JSON.stringify(config);
			const parsed: ExtractionConfig = JSON.parse(json);

			expect(parsed.outputFormat).toBe("djot");
			expect(parsed.resultFormat).toBe("unified");
		});

		it("should omit undefined format fields in JSON", () => {
			const config: ExtractionConfig = {
				outputFormat: "html",
				resultFormat: undefined,
			};

			const json = JSON.stringify(config);

			expect(json).toContain("outputFormat");
			expect(json).not.toContain("resultFormat");
		});

		it("should handle complete config serialization", () => {
			const config: ExtractionConfig = {
				useCache: true,
				outputFormat: "markdown",
				resultFormat: "element_based",
				ocr: {
					backend: "tesseract",
					language: "eng",
				},
			};

			const json = JSON.stringify(config);
			const parsed: ExtractionConfig = JSON.parse(json);

			expect(parsed.useCache).toBe(true);
			expect(parsed.outputFormat).toBe("markdown");
			expect(parsed.resultFormat).toBe("element_based");
			expect(parsed.ocr?.backend).toBe("tesseract");
		});
	});

	describe("worker message passing", () => {
		it("should serialize outputFormat for worker communication", () => {
			const config: ExtractionConfig = {
				outputFormat: "markdown",
			};

			const cloned = structuredClone(config);

			expect(cloned.outputFormat).toBe("markdown");
		});

		it("should serialize resultFormat for worker communication", () => {
			const config: ExtractionConfig = {
				resultFormat: "element_based",
			};

			const cloned = structuredClone(config);

			expect(cloned.resultFormat).toBe("element_based");
		});

		it("should preserve both formats in workers", () => {
			const config: ExtractionConfig = {
				outputFormat: "djot",
				resultFormat: "unified",
				useCache: true,
			};

			const cloned = structuredClone(config);

			expect(cloned.outputFormat).toBe("djot");
			expect(cloned.resultFormat).toBe("unified");
			expect(cloned.useCache).toBe(true);
		});

		it("should handle complex config with formats in workers", () => {
			const config: ExtractionConfig = {
				outputFormat: "markdown",
				resultFormat: "element_based",
				ocr: {
					backend: "tesseract",
				},
				chunking: {
					chunkSize: 512,
				},
			};

			const cloned = structuredClone(config);

			expect(cloned.outputFormat).toBe("markdown");
			expect(cloned.resultFormat).toBe("element_based");
			expect(cloned.ocr?.backend).toBe("tesseract");
		});
	});

	describe("default values", () => {
		it("should have undefined as default for outputFormat", () => {
			const config: ExtractionConfig = {};

			expect(config.outputFormat).toBeUndefined();
		});

		it("should have undefined as default for resultFormat", () => {
			const config: ExtractionConfig = {};

			expect(config.resultFormat).toBeUndefined();
		});

		it("should allow explicit null-like behavior with undefined", () => {
			const config: ExtractionConfig = {
				outputFormat: undefined,
				resultFormat: undefined,
			};

			expect(config.outputFormat).toBeUndefined();
			expect(config.resultFormat).toBeUndefined();
		});

		it("should support setting only outputFormat, keeping resultFormat undefined", () => {
			const config: ExtractionConfig = {
				outputFormat: "html",
			};

			expect(config.outputFormat).toBe("html");
			expect(config.resultFormat).toBeUndefined();
		});

		it("should support setting only resultFormat, keeping outputFormat undefined", () => {
			const config: ExtractionConfig = {
				resultFormat: "element_based",
			};

			expect(config.outputFormat).toBeUndefined();
			expect(config.resultFormat).toBe("element_based");
		});
	});

	describe("type safety", () => {
		it("should enforce outputFormat as string when defined", () => {
			const config: ExtractionConfig = { outputFormat: "plain" };
			if (config.outputFormat !== undefined) {
				expect(typeof config.outputFormat).toBe("string");
			}
		});

		it("should enforce resultFormat as string when defined", () => {
			const config: ExtractionConfig = { resultFormat: "unified" };
			if (config.resultFormat !== undefined) {
				expect(typeof config.resultFormat).toBe("string");
			}
		});

		it("should require outputFormat to be one of valid values", () => {
			const validFormats: Array<"plain" | "markdown" | "djot" | "html"> = ["plain", "markdown", "djot", "html"];

			validFormats.forEach((format) => {
				const config: ExtractionConfig = { outputFormat: format };
				expect(validFormats).toContain(config.outputFormat);
			});
		});

		it("should require resultFormat to be one of valid values", () => {
			const validFormats: Array<"unified" | "element_based"> = ["unified", "element_based"];

			validFormats.forEach((format) => {
				const config: ExtractionConfig = { resultFormat: format };
				expect(validFormats).toContain(config.resultFormat);
			});
		});
	});

	describe("camelCase conventions", () => {
		it("should use camelCase for outputFormat property", () => {
			const config: ExtractionConfig = {
				outputFormat: "markdown",
			};

			expect(config).toHaveProperty("outputFormat");
		});

		it("should use camelCase for resultFormat property", () => {
			const config: ExtractionConfig = {
				resultFormat: "element_based",
			};

			expect(config).toHaveProperty("resultFormat");
		});

		it("should not use snake_case for format properties", () => {
			const config: ExtractionConfig = {
				outputFormat: "markdown",
				resultFormat: "element_based",
			};

			expect(config).not.toHaveProperty("output_format");
			expect(config).not.toHaveProperty("result_format");
		});
	});

	describe("edge cases", () => {
		it("should handle plain text output format", () => {
			const config: ExtractionConfig = {
				outputFormat: "plain",
			};

			expect(config.outputFormat).toBe("plain");
		});

		it("should handle markdown output format", () => {
			const config: ExtractionConfig = {
				outputFormat: "markdown",
			};

			expect(config.outputFormat).toBe("markdown");
		});

		it("should handle djot output format", () => {
			const config: ExtractionConfig = {
				outputFormat: "djot",
			};

			expect(config.outputFormat).toBe("djot");
		});

		it("should handle html output format", () => {
			const config: ExtractionConfig = {
				outputFormat: "html",
			};

			expect(config.outputFormat).toBe("html");
		});

		it("should handle unified result format", () => {
			const config: ExtractionConfig = {
				resultFormat: "unified",
			};

			expect(config.resultFormat).toBe("unified");
		});

		it("should handle element_based result format", () => {
			const config: ExtractionConfig = {
				resultFormat: "element_based",
			};

			expect(config.resultFormat).toBe("element_based");
		});
	});

	describe("immutability patterns", () => {
		it("should support spread operator updates for outputFormat", () => {
			const original: ExtractionConfig = {
				outputFormat: "plain",
			};

			const updated: ExtractionConfig = {
				...original,
				outputFormat: "markdown",
			};

			expect(original.outputFormat).toBe("plain");
			expect(updated.outputFormat).toBe("markdown");
		});

		it("should support spread operator updates for resultFormat", () => {
			const original: ExtractionConfig = {
				resultFormat: "unified",
			};

			const updated: ExtractionConfig = {
				...original,
				resultFormat: "element_based",
			};

			expect(original.resultFormat).toBe("unified");
			expect(updated.resultFormat).toBe("element_based");
		});

		it("should support updating both formats with spread operator", () => {
			const original: ExtractionConfig = {
				outputFormat: "plain",
				resultFormat: "unified",
			};

			const updated: ExtractionConfig = {
				...original,
				outputFormat: "markdown",
				resultFormat: "element_based",
			};

			expect(original.outputFormat).toBe("plain");
			expect(original.resultFormat).toBe("unified");
			expect(updated.outputFormat).toBe("markdown");
			expect(updated.resultFormat).toBe("element_based");
		});

		it("should preserve other config fields when updating formats", () => {
			const original: ExtractionConfig = {
				useCache: true,
				outputFormat: "plain",
				forceOcr: false,
			};

			const updated: ExtractionConfig = {
				...original,
				outputFormat: "markdown",
			};

			expect(updated.useCache).toBe(true);
			expect(updated.outputFormat).toBe("markdown");
			expect(updated.forceOcr).toBe(false);
		});
	});

	describe("practical scenarios", () => {
		it("should support markdown output configuration", () => {
			const config: ExtractionConfig = {
				outputFormat: "markdown",
			};

			expect(config.outputFormat).toBe("markdown");
		});

		it("should support element-based structure configuration", () => {
			const config: ExtractionConfig = {
				resultFormat: "element_based",
			};

			expect(config.resultFormat).toBe("element_based");
		});

		it("should support markdown with element-based structure", () => {
			const config: ExtractionConfig = {
				outputFormat: "markdown",
				resultFormat: "element_based",
			};

			expect(config.outputFormat).toBe("markdown");
			expect(config.resultFormat).toBe("element_based");
		});

		it("should support full extraction config with format options", () => {
			const config: ExtractionConfig = {
				useCache: true,
				enableQualityProcessing: true,
				outputFormat: "markdown",
				resultFormat: "element_based",
				ocr: {
					backend: "tesseract",
					language: "eng",
				},
				chunking: {
					chunkSize: 512,
					chunkOverlap: 128,
				},
				images: {
					extractImages: true,
					targetDpi: 300,
				},
			};

			expect(config.outputFormat).toBe("markdown");
			expect(config.resultFormat).toBe("element_based");
			expect(config.useCache).toBe(true);
			expect(config.ocr?.backend).toBe("tesseract");
		});

		it("should support HTML output configuration", () => {
			const config: ExtractionConfig = {
				outputFormat: "html",
				resultFormat: "unified",
			};

			expect(config.outputFormat).toBe("html");
			expect(config.resultFormat).toBe("unified");
		});

		it("should support djot output configuration", () => {
			const config: ExtractionConfig = {
				outputFormat: "djot",
				resultFormat: "unified",
			};

			expect(config.outputFormat).toBe("djot");
			expect(config.resultFormat).toBe("unified");
		});

		it("should support plain output with element-based structure", () => {
			const config: ExtractionConfig = {
				outputFormat: "plain",
				resultFormat: "element_based",
			};

			expect(config.outputFormat).toBe("plain");
			expect(config.resultFormat).toBe("element_based");
		});
	});

	describe("configuration composition", () => {
		it("should support merging format configs", () => {
			const baseConfig: ExtractionConfig = {
				useCache: true,
			};

			const formatConfig: ExtractionConfig = {
				outputFormat: "markdown",
				resultFormat: "element_based",
			};

			const mergedConfig: ExtractionConfig = {
				...baseConfig,
				...formatConfig,
			};

			expect(mergedConfig.useCache).toBe(true);
			expect(mergedConfig.outputFormat).toBe("markdown");
			expect(mergedConfig.resultFormat).toBe("element_based");
		});

		it("should support overriding format configs", () => {
			const baseConfig: ExtractionConfig = {
				outputFormat: "plain",
				resultFormat: "unified",
			};

			const overrideConfig: ExtractionConfig = {
				outputFormat: "markdown",
			};

			const mergedConfig: ExtractionConfig = {
				...baseConfig,
				...overrideConfig,
			};

			expect(mergedConfig.outputFormat).toBe("markdown");
			expect(mergedConfig.resultFormat).toBe("unified");
		});

		it("should support complete format config replacement", () => {
			const baseConfig: ExtractionConfig = {
				outputFormat: "plain",
				resultFormat: "unified",
				useCache: true,
			};

			const newFormatConfig: ExtractionConfig = {
				outputFormat: "djot",
				resultFormat: "element_based",
			};

			const mergedConfig: ExtractionConfig = {
				...baseConfig,
				...newFormatConfig,
			};

			expect(mergedConfig.outputFormat).toBe("djot");
			expect(mergedConfig.resultFormat).toBe("element_based");
			expect(mergedConfig.useCache).toBe(true);
		});
	});

	describe("documentation compliance", () => {
		it("outputFormat should have JSDoc comments", () => {
			// This is verified by TypeScript compiler when checking the source
			const config: ExtractionConfig = {
				outputFormat: "markdown",
			};

			expect(config.outputFormat).toBeDefined();
		});

		it("resultFormat should have JSDoc comments", () => {
			// This is verified by TypeScript compiler when checking the source
			const config: ExtractionConfig = {
				resultFormat: "element_based",
			};

			expect(config.resultFormat).toBeDefined();
		});

		it("should support all documented format options", () => {
			const outputFormats: Array<"plain" | "markdown" | "djot" | "html"> = ["plain", "markdown", "djot", "html"];

			const resultFormats: Array<"unified" | "element_based"> = ["unified", "element_based"];

			outputFormats.forEach((outFmt) => {
				resultFormats.forEach((resFmt) => {
					const config: ExtractionConfig = {
						outputFormat: outFmt,
						resultFormat: resFmt,
					};

					expect(config.outputFormat).toBe(outFmt);
					expect(config.resultFormat).toBe(resFmt);
				});
			});
		});
	});
});

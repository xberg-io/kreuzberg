/**
 * OcrConfig configuration tests for WASM binding
 *
 * Tests for OcrConfig feature that allows users to configure OCR
 * backends, languages, and backend-specific parameters.
 */

import { describe, expect, it } from "vitest";
import type { ExtractionConfig, OcrConfig } from "../types";

describe("WASM: OcrConfig", () => {
	describe("type definitions", () => {
		it("should define valid OcrConfig type", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				language: "eng",
				enabled: true,
			};

			expect(config.backend).toBe("tesseract");
			expect(config.language).toBe("eng");
			expect(config.enabled).toBe(true);
		});

		it("should support optional fields", () => {
			const minimalConfig: OcrConfig = {};
			expect(minimalConfig.backend).toBeUndefined();
			expect(minimalConfig.language).toBeUndefined();
		});

		it("should support multiple languages array", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				languages: ["eng", "deu", "fra"],
			};

			expect(config.languages).toEqual(["eng", "deu", "fra"]);
		});

		it("should support tesseract configuration", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				language: "eng",
				tesseractConfig: {
					psm: 3,
					enableTableDetection: true,
				},
			};

			expect(config.tesseractConfig?.psm).toBe(3);
			expect(config.tesseractConfig?.enableTableDetection).toBe(true);
		});
	});

	describe("WASM serialization", () => {
		it("should serialize for WASM boundary", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				language: "eng",
				enabled: true,
			};

			const json = JSON.stringify(config);
			const parsed: OcrConfig = JSON.parse(json);

			expect(parsed.backend).toBe("tesseract");
			expect(parsed.language).toBe("eng");
			expect(parsed.enabled).toBe(true);
		});

		it("should handle undefined fields in serialization", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				language: undefined,
			};

			const json = JSON.stringify(config);
			expect(json).not.toContain("language");
			expect(json).toContain("backend");
		});

		it("should serialize tesseract configuration", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				language: "eng",
				tesseractConfig: {
					psm: 6,
					enableTableDetection: true,
					tesseditCharWhitelist: "abc123",
				},
			};

			const json = JSON.stringify(config);
			const parsed: OcrConfig = JSON.parse(json);

			expect(parsed.tesseractConfig?.psm).toBe(6);
			expect(parsed.tesseractConfig?.tesseditCharWhitelist).toBe("abc123");
		});
	});

	describe("worker message passing", () => {
		it("should serialize for worker communication", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				language: "eng",
				enabled: true,
			};

			const cloned = structuredClone(config);
			expect(cloned.backend).toBe("tesseract");
			expect(cloned.language).toBe("eng");
		});

		it("should handle nested configs in ExtractionConfig", () => {
			const extractionConfig: ExtractionConfig = {
				ocr: {
					backend: "tesseract",
					language: "fra",
					enabled: true,
				},
			};

			const cloned = structuredClone(extractionConfig);
			expect(cloned.ocr?.backend).toBe("tesseract");
			expect(cloned.ocr?.language).toBe("fra");
		});

		it("should preserve complex OCR configs", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				language: "eng",
				languages: ["eng", "deu"],
				tesseractConfig: {
					psm: 3,
					enableTableDetection: true,
				},
			};

			const cloned = structuredClone(config);
			expect(cloned.languages).toEqual(["eng", "deu"]);
			expect(cloned.tesseractConfig?.psm).toBe(3);
		});
	});

	describe("type safety", () => {
		it("should enforce backend as string when defined", () => {
			const config: OcrConfig = { backend: "tesseract" };
			if (config.backend !== undefined) {
				expect(typeof config.backend).toBe("string");
			}
		});

		it("should enforce language as string when defined", () => {
			const config: OcrConfig = { language: "eng" };
			if (config.language !== undefined) {
				expect(typeof config.language).toBe("string");
			}
		});

		it("should enforce enabled as boolean when defined", () => {
			const config: OcrConfig = { enabled: true };
			if (config.enabled !== undefined) {
				expect(typeof config.enabled).toBe("boolean");
			}
		});

		it("should enforce languages as string array when defined", () => {
			const config: OcrConfig = { languages: ["eng"] };
			if (config.languages !== undefined) {
				expect(Array.isArray(config.languages)).toBe(true);
				config.languages.forEach((lang) => {
					expect(typeof lang).toBe("string");
				});
			}
		});
	});

	describe("edge cases", () => {
		it("should handle single language", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				language: "eng",
			};
			expect(config.language).toBe("eng");
		});

		it("should handle multiple languages array", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				languages: ["eng", "deu", "fra", "spa"],
			};
			expect(config.languages?.length).toBe(4);
		});

		it("should handle zero PSM value", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				tesseractConfig: {
					psm: 0,
				},
			};
			expect(config.tesseractConfig?.psm).toBe(0);
		});

		it("should handle empty char whitelist", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				tesseractConfig: {
					tesseditCharWhitelist: "",
				},
			};
			expect(config.tesseractConfig?.tesseditCharWhitelist).toBe("");
		});
	});

	describe("immutability patterns", () => {
		it("should support spread operator updates", () => {
			const original: OcrConfig = {
				backend: "tesseract",
				language: "eng",
			};

			const updated: OcrConfig = {
				...original,
				language: "fra",
			};

			expect(original.language).toBe("eng");
			expect(updated.language).toBe("fra");
			expect(updated.backend).toBe("tesseract");
		});

		it("should support nested tesseract config updates", () => {
			const original: OcrConfig = {
				backend: "tesseract",
				tesseractConfig: {
					psm: 3,
				},
			};

			const updated: OcrConfig = {
				...original,
				tesseractConfig: {
					...original.tesseractConfig,
					psm: 6,
				},
			};

			expect(original.tesseractConfig?.psm).toBe(3);
			expect(updated.tesseractConfig?.psm).toBe(6);
		});
	});

	describe("nesting in ExtractionConfig", () => {
		it("should nest properly in ExtractionConfig", () => {
			const config: ExtractionConfig = {
				ocr: {
					backend: "tesseract",
					language: "eng",
					enabled: true,
				},
			};

			expect(config.ocr).toBeDefined();
			expect(config.ocr?.backend).toBe("tesseract");
			expect(config.ocr?.enabled).toBe(true);
		});

		it("should support OCR with other extraction options", () => {
			const config: ExtractionConfig = {
				useCache: true,
				ocr: {
					backend: "tesseract",
					language: "eng",
				},
				forceOcr: true,
			};

			expect(config.useCache).toBe(true);
			expect(config.ocr?.backend).toBe("tesseract");
			expect(config.forceOcr).toBe(true);
		});
	});
});

/**
 * Simple configuration tests for WASM binding
 *
 * Tests for simpler configuration types: TokenReductionConfig, PostProcessorConfig,
 * PageExtractionConfig, LanguageDetectionConfig, and TesseractConfig.
 */

import { describe, expect, it } from "vitest";
import type {
	ExtractionConfig,
	LanguageDetectionConfig,
	OcrConfig,
	PageExtractionConfig,
	PostProcessorConfig,
	TesseractConfig,
	TokenReductionConfig,
} from "../types";

describe("WASM: TokenReductionConfig", () => {
	describe("type definitions", () => {
		it("should define valid TokenReductionConfig type", () => {
			const config: TokenReductionConfig = {
				mode: "balanced",
				preserveImportantWords: true,
			};

			expect(config.mode).toBe("balanced");
			expect(config.preserveImportantWords).toBe(true);
		});

		it("should support optional fields", () => {
			const minimalConfig: TokenReductionConfig = {};
			expect(minimalConfig.mode).toBeUndefined();
			expect(minimalConfig.preserveImportantWords).toBeUndefined();
		});
	});

	describe("WASM serialization", () => {
		it("should serialize for WASM boundary", () => {
			const config: TokenReductionConfig = {
				mode: "aggressive",
				preserveImportantWords: false,
			};

			const json = JSON.stringify(config);
			const parsed: TokenReductionConfig = JSON.parse(json);

			expect(parsed.mode).toBe("aggressive");
			expect(parsed.preserveImportantWords).toBe(false);
		});
	});

	describe("worker message passing", () => {
		it("should serialize for worker communication", () => {
			const config: TokenReductionConfig = {
				mode: "conservative",
				preserveImportantWords: true,
			};

			const cloned = structuredClone(config);
			expect(cloned.mode).toBe("conservative");
			expect(cloned.preserveImportantWords).toBe(true);
		});
	});
});

describe("WASM: PostProcessorConfig", () => {
	describe("type definitions", () => {
		it("should define valid PostProcessorConfig type", () => {
			const config: PostProcessorConfig = {
				enabled: true,
				enabledProcessors: ["processor1", "processor2"],
				disabledProcessors: ["processor3"],
			};

			expect(config.enabled).toBe(true);
			expect(config.enabledProcessors).toEqual(["processor1", "processor2"]);
			expect(config.disabledProcessors).toEqual(["processor3"]);
		});

		it("should support optional fields", () => {
			const minimalConfig: PostProcessorConfig = {};
			expect(minimalConfig.enabled).toBeUndefined();
			expect(minimalConfig.enabledProcessors).toBeUndefined();
		});
	});

	describe("WASM serialization", () => {
		it("should serialize for WASM boundary", () => {
			const config: PostProcessorConfig = {
				enabled: true,
				enabledProcessors: ["clean", "format"],
			};

			const json = JSON.stringify(config);
			const parsed: PostProcessorConfig = JSON.parse(json);

			expect(parsed.enabled).toBe(true);
			expect(parsed.enabledProcessors).toEqual(["clean", "format"]);
		});
	});

	describe("worker message passing", () => {
		it("should serialize for worker communication", () => {
			const config: PostProcessorConfig = {
				enabled: false,
				disabledProcessors: ["unwanted"],
			};

			const cloned = structuredClone(config);
			expect(cloned.enabled).toBe(false);
			expect(cloned.disabledProcessors).toEqual(["unwanted"]);
		});
	});
});

describe("WASM: PageExtractionConfig", () => {
	describe("type definitions", () => {
		it("should define valid PageExtractionConfig type", () => {
			const config: PageExtractionConfig = {
				extractPages: true,
				insertPageMarkers: true,
				markerFormat: "--- Page {page_num} ---",
			};

			expect(config.extractPages).toBe(true);
			expect(config.insertPageMarkers).toBe(true);
			expect(config.markerFormat).toBe("--- Page {page_num} ---");
		});

		it("should support optional fields", () => {
			const minimalConfig: PageExtractionConfig = {};
			expect(minimalConfig.extractPages).toBeUndefined();
			expect(minimalConfig.markerFormat).toBeUndefined();
		});
	});

	describe("WASM serialization", () => {
		it("should serialize for WASM boundary", () => {
			const config: PageExtractionConfig = {
				extractPages: true,
				insertPageMarkers: false,
			};

			const json = JSON.stringify(config);
			const parsed: PageExtractionConfig = JSON.parse(json);

			expect(parsed.extractPages).toBe(true);
			expect(parsed.insertPageMarkers).toBe(false);
		});
	});

	describe("worker message passing", () => {
		it("should serialize for worker communication", () => {
			const config: PageExtractionConfig = {
				extractPages: false,
				insertPageMarkers: true,
				markerFormat: "[PAGE {page_num}]",
			};

			const cloned = structuredClone(config);
			expect(cloned.markerFormat).toBe("[PAGE {page_num}]");
		});
	});
});

describe("WASM: LanguageDetectionConfig", () => {
	describe("type definitions", () => {
		it("should define valid LanguageDetectionConfig type", () => {
			const config: LanguageDetectionConfig = {
				enabled: true,
			};

			expect(config.enabled).toBe(true);
		});

		it("should support optional fields", () => {
			const minimalConfig: LanguageDetectionConfig = {};
			expect(minimalConfig.enabled).toBeUndefined();
		});
	});

	describe("WASM serialization", () => {
		it("should serialize for WASM boundary", () => {
			const config: LanguageDetectionConfig = {
				enabled: true,
			};

			const json = JSON.stringify(config);
			const parsed: LanguageDetectionConfig = JSON.parse(json);

			expect(parsed.enabled).toBe(true);
		});

		it("should handle false value", () => {
			const config: LanguageDetectionConfig = {
				enabled: false,
			};

			const json = JSON.stringify(config);
			const parsed: LanguageDetectionConfig = JSON.parse(json);

			expect(parsed.enabled).toBe(false);
		});
	});

	describe("worker message passing", () => {
		it("should serialize for worker communication", () => {
			const config: LanguageDetectionConfig = {
				enabled: true,
			};

			const cloned = structuredClone(config);
			expect(cloned.enabled).toBe(true);
		});

		it("should handle nested in ExtractionConfig", () => {
			const extractionConfig: ExtractionConfig = {
				languageDetection: {
					enabled: true,
				},
			};

			const cloned = structuredClone(extractionConfig);
			expect(cloned.languageDetection?.enabled).toBe(true);
		});
	});
});

describe("WASM: TesseractConfig", () => {
	describe("type definitions", () => {
		it("should define valid TesseractConfig type", () => {
			const config: TesseractConfig = {
				psm: 3,
				enableTableDetection: true,
				tesseditCharWhitelist: "abc123",
			};

			expect(config.psm).toBe(3);
			expect(config.enableTableDetection).toBe(true);
			expect(config.tesseditCharWhitelist).toBe("abc123");
		});

		it("should support optional fields", () => {
			const minimalConfig: TesseractConfig = {};
			expect(minimalConfig.psm).toBeUndefined();
			expect(minimalConfig.enableTableDetection).toBeUndefined();
		});
	});

	describe("WASM serialization", () => {
		it("should serialize for WASM boundary", () => {
			const config: TesseractConfig = {
				psm: 6,
				enableTableDetection: false,
				tesseditCharWhitelist: "0123456789",
			};

			const json = JSON.stringify(config);
			const parsed: TesseractConfig = JSON.parse(json);

			expect(parsed.psm).toBe(6);
			expect(parsed.enableTableDetection).toBe(false);
			expect(parsed.tesseditCharWhitelist).toBe("0123456789");
		});

		it("should handle undefined fields in serialization", () => {
			const config: TesseractConfig = {
				psm: 3,
				enableTableDetection: undefined,
			};

			const json = JSON.stringify(config);
			expect(json).not.toContain("enableTableDetection");
			expect(json).toContain("psm");
		});
	});

	describe("worker message passing", () => {
		it("should serialize for worker communication", () => {
			const config: TesseractConfig = {
				psm: 11,
				enableTableDetection: true,
			};

			const cloned = structuredClone(config);
			expect(cloned.psm).toBe(11);
			expect(cloned.enableTableDetection).toBe(true);
		});

		it("should handle nested in OcrConfig", () => {
			const ocrConfig: OcrConfig = {
				backend: "tesseract",
				tesseractConfig: {
					psm: 3,
					enableTableDetection: true,
				},
			};

			const cloned = structuredClone(ocrConfig);
			expect(cloned.tesseractConfig?.psm).toBe(3);
		});
	});

	describe("type safety", () => {
		it("should enforce psm as number when defined", () => {
			const config: TesseractConfig = { psm: 3 };
			if (config.psm !== undefined) {
				expect(typeof config.psm).toBe("number");
			}
		});

		it("should enforce enableTableDetection as boolean when defined", () => {
			const config: TesseractConfig = { enableTableDetection: true };
			if (config.enableTableDetection !== undefined) {
				expect(typeof config.enableTableDetection).toBe("boolean");
			}
		});
	});

	describe("edge cases", () => {
		it("should handle various PSM values", () => {
			const configs: TesseractConfig[] = [{ psm: 0 }, { psm: 3 }, { psm: 6 }, { psm: 11 }];

			expect(configs).toHaveLength(4);
			expect(configs[3].psm).toBe(11);
		});

		it("should handle empty char whitelist", () => {
			const config: TesseractConfig = {
				tesseditCharWhitelist: "",
			};
			expect(config.tesseditCharWhitelist).toBe("");
		});

		it("should handle special characters in whitelist", () => {
			const config: TesseractConfig = {
				tesseditCharWhitelist: "!@#$%^&*()",
			};
			expect(config.tesseditCharWhitelist).toBe("!@#$%^&*()");
		});
	});

	describe("immutability patterns", () => {
		it("should support spread operator updates", () => {
			const original: TesseractConfig = {
				psm: 3,
				enableTableDetection: true,
			};

			const updated: TesseractConfig = {
				...original,
				psm: 6,
			};

			expect(original.psm).toBe(3);
			expect(updated.psm).toBe(6);
			expect(updated.enableTableDetection).toBe(true);
		});
	});

	describe("nesting in OcrConfig", () => {
		it("should nest properly in OcrConfig", () => {
			const config: OcrConfig = {
				backend: "tesseract",
				language: "eng",
				tesseractConfig: {
					psm: 3,
					enableTableDetection: true,
				},
			};

			expect(config.tesseractConfig).toBeDefined();
			expect(config.tesseractConfig?.psm).toBe(3);
		});
	});
});

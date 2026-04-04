/**
 * PdfConfig configuration tests for WASM binding
 *
 * Tests for PdfConfig feature that allows users to configure PDF-specific
 * extraction options including image extraction and passwords.
 */

import { describe, expect, it } from "vitest";
import type { ExtractionConfig, PdfConfig } from "../types";

describe("WASM: PdfConfig", () => {
	describe("type definitions", () => {
		it("should define valid PdfConfig type", () => {
			const config: PdfConfig = {
				extractImages: true,
				passwords: ["password123"],
				extractMetadata: true,
			};

			expect(config.extractImages).toBe(true);
			expect(config.passwords).toEqual(["password123"]);
			expect(config.extractMetadata).toBe(true);
		});

		it("should support optional fields", () => {
			const minimalConfig: PdfConfig = {};
			expect(minimalConfig.extractImages).toBeUndefined();
			expect(minimalConfig.passwords).toBeUndefined();
		});
	});

	describe("WASM serialization", () => {
		it("should serialize for WASM boundary", () => {
			const config: PdfConfig = {
				extractImages: true,
				passwords: ["pass1", "pass2"],
				extractMetadata: false,
			};

			const json = JSON.stringify(config);
			const parsed: PdfConfig = JSON.parse(json);

			expect(parsed.extractImages).toBe(true);
			expect(parsed.passwords).toEqual(["pass1", "pass2"]);
			expect(parsed.extractMetadata).toBe(false);
		});

		it("should handle undefined fields in serialization", () => {
			const config: PdfConfig = {
				extractImages: true,
				passwords: undefined,
			};

			const json = JSON.stringify(config);
			expect(json).not.toContain("passwords");
			expect(json).toContain("extractImages");
		});
	});

	describe("worker message passing", () => {
		it("should serialize for worker communication", () => {
			const config: PdfConfig = {
				extractImages: true,
				passwords: ["secret"],
				extractMetadata: true,
			};

			const cloned = structuredClone(config);
			expect(cloned.extractImages).toBe(true);
			expect(cloned.passwords).toEqual(["secret"]);
		});

		it("should handle nested configs in ExtractionConfig", () => {
			const extractionConfig: ExtractionConfig = {
				pdfOptions: {
					extractImages: true,
					passwords: ["pwd1"],
					extractMetadata: true,
				},
			};

			const cloned = structuredClone(extractionConfig);
			expect(cloned.pdfOptions?.extractImages).toBe(true);
			expect(cloned.pdfOptions?.passwords).toEqual(["pwd1"]);
		});
	});

	describe("type safety", () => {
		it("should enforce extractImages as boolean when defined", () => {
			const config: PdfConfig = { extractImages: true };
			if (config.extractImages !== undefined) {
				expect(typeof config.extractImages).toBe("boolean");
			}
		});

		it("should enforce passwords as string array when defined", () => {
			const config: PdfConfig = { passwords: ["pass"] };
			if (config.passwords !== undefined) {
				expect(Array.isArray(config.passwords)).toBe(true);
				config.passwords.forEach((pwd) => {
					expect(typeof pwd).toBe("string");
				});
			}
		});

		it("should enforce extractMetadata as boolean when defined", () => {
			const config: PdfConfig = { extractMetadata: true };
			if (config.extractMetadata !== undefined) {
				expect(typeof config.extractMetadata).toBe("boolean");
			}
		});
	});

	describe("edge cases", () => {
		it("should handle empty passwords array", () => {
			const config: PdfConfig = {
				extractImages: true,
				passwords: [],
			};
			expect(config.passwords).toEqual([]);
		});

		it("should handle single password", () => {
			const config: PdfConfig = {
				extractImages: true,
				passwords: ["onlypassword"],
			};
			expect(config.passwords).toEqual(["onlypassword"]);
		});

		it("should handle very long password strings", () => {
			const longPassword = "a".repeat(1000);
			const config: PdfConfig = {
				extractImages: true,
				passwords: [longPassword],
			};
			expect(config.passwords?.[0]).toBe(longPassword);
		});

		it("should handle undefined passwords", () => {
			const config: PdfConfig = {
				extractImages: true,
				passwords: undefined,
			};
			expect(config.passwords).toBeUndefined();
		});
	});

	describe("immutability patterns", () => {
		it("should support spread operator updates", () => {
			const original: PdfConfig = {
				extractImages: true,
				extractMetadata: false,
			};

			const updated: PdfConfig = {
				...original,
				extractMetadata: true,
			};

			expect(original.extractMetadata).toBe(false);
			expect(updated.extractMetadata).toBe(true);
			expect(updated.extractImages).toBe(true);
		});

		it("should support password array updates", () => {
			const original: PdfConfig = {
				extractImages: true,
				passwords: ["pass1"],
			};

			const updated: PdfConfig = {
				...original,
				passwords: [...(original.passwords || []), "pass2"],
			};

			expect(original.passwords).toEqual(["pass1"]);
			expect(updated.passwords).toEqual(["pass1", "pass2"]);
		});
	});

	describe("nesting in ExtractionConfig", () => {
		it("should nest properly in ExtractionConfig", () => {
			const config: ExtractionConfig = {
				pdfOptions: {
					extractImages: true,
					passwords: ["secret"],
					extractMetadata: true,
				},
			};

			expect(config.pdfOptions).toBeDefined();
			expect(config.pdfOptions?.extractImages).toBe(true);
			expect(config.pdfOptions?.passwords).toEqual(["secret"]);
		});

		it("should support PDF options with other extraction options", () => {
			const config: ExtractionConfig = {
				useCache: true,
				pdfOptions: {
					extractImages: true,
					extractMetadata: false,
				},
				forceOcr: false,
			};

			expect(config.useCache).toBe(true);
			expect(config.pdfOptions?.extractImages).toBe(true);
			expect(config.forceOcr).toBe(false);
		});
	});
});

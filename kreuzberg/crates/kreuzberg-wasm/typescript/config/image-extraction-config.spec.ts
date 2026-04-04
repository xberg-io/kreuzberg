/**
 * ImageExtractionConfig configuration tests for WASM binding
 *
 * Tests for ImageExtractionConfig feature that allows users to configure
 * image extraction parameters including DPI and dimensions.
 */

import { describe, expect, it } from "vitest";
import type { ExtractionConfig, ImageExtractionConfig } from "../types";

describe("WASM: ImageExtractionConfig", () => {
	describe("type definitions", () => {
		it("should define valid ImageExtractionConfig type", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				targetDpi: 300,
				maxImageDimension: 2048,
			};

			expect(config.enabled).toBe(true);
			expect(config.targetDpi).toBe(300);
			expect(config.maxImageDimension).toBe(2048);
		});

		it("should support optional fields", () => {
			const minimalConfig: ImageExtractionConfig = {};
			expect(minimalConfig.enabled).toBeUndefined();
			expect(minimalConfig.targetDpi).toBeUndefined();
		});

		it("should support auto DPI adjustment options", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				autoAdjustDpi: true,
				minDpi: 72,
				maxDpi: 600,
			};

			expect(config.autoAdjustDpi).toBe(true);
			expect(config.minDpi).toBe(72);
			expect(config.maxDpi).toBe(600);
		});
	});

	describe("WASM serialization", () => {
		it("should serialize for WASM boundary", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				targetDpi: 300,
				maxImageDimension: 1024,
			};

			const json = JSON.stringify(config);
			const parsed: ImageExtractionConfig = JSON.parse(json);

			expect(parsed.enabled).toBe(true);
			expect(parsed.targetDpi).toBe(300);
			expect(parsed.maxImageDimension).toBe(1024);
		});

		it("should handle undefined fields in serialization", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				targetDpi: undefined,
			};

			const json = JSON.stringify(config);
			expect(json).not.toContain("targetDpi");
			expect(json).toContain("enabled");
		});

		it("should serialize DPI range settings", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				autoAdjustDpi: true,
				minDpi: 150,
				maxDpi: 500,
			};

			const json = JSON.stringify(config);
			const parsed: ImageExtractionConfig = JSON.parse(json);

			expect(parsed.minDpi).toBe(150);
			expect(parsed.maxDpi).toBe(500);
		});
	});

	describe("worker message passing", () => {
		it("should serialize for worker communication", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				targetDpi: 300,
				maxImageDimension: 2048,
			};

			const cloned = structuredClone(config);
			expect(cloned.enabled).toBe(true);
			expect(cloned.targetDpi).toBe(300);
		});

		it("should handle nested configs in ExtractionConfig", () => {
			const extractionConfig: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 300,
					maxImageDimension: 2048,
				},
			};

			const cloned = structuredClone(extractionConfig);
			expect(cloned.images?.enabled).toBe(true);
			expect(cloned.images?.targetDpi).toBe(300);
		});
	});

	describe("type safety", () => {
		it("should enforce enabled as boolean when defined", () => {
			const config: ImageExtractionConfig = { enabled: true };
			if (config.enabled !== undefined) {
				expect(typeof config.enabled).toBe("boolean");
			}
		});

		it("should enforce targetDpi as number when defined", () => {
			const config: ImageExtractionConfig = { targetDpi: 300 };
			if (config.targetDpi !== undefined) {
				expect(typeof config.targetDpi).toBe("number");
			}
		});

		it("should enforce maxImageDimension as number when defined", () => {
			const config: ImageExtractionConfig = { maxImageDimension: 4096 };
			if (config.maxImageDimension !== undefined) {
				expect(typeof config.maxImageDimension).toBe("number");
			}
		});
	});

	describe("edge cases", () => {
		it("should handle zero DPI", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				targetDpi: 0,
			};
			expect(config.targetDpi).toBe(0);
		});

		it("should handle high DPI values", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				targetDpi: 1200,
			};
			expect(config.targetDpi).toBe(1200);
		});

		it("should handle large image dimensions", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				maxImageDimension: 8192,
			};
			expect(config.maxImageDimension).toBe(8192);
		});

		it("should handle DPI thresholds", () => {
			const config: ImageExtractionConfig = {
				enabled: true,
				minDpi: 72,
				maxDpi: 1200,
			};

			expect(config.minDpi).toBe(72);
			expect(config.maxDpi).toBe(1200);
		});
	});

	describe("immutability patterns", () => {
		it("should support spread operator updates", () => {
			const original: ImageExtractionConfig = {
				enabled: true,
				targetDpi: 300,
			};

			const updated: ImageExtractionConfig = {
				...original,
				targetDpi: 150,
			};

			expect(original.targetDpi).toBe(300);
			expect(updated.targetDpi).toBe(150);
			expect(updated.enabled).toBe(true);
		});
	});

	describe("nesting in ExtractionConfig", () => {
		it("should nest properly in ExtractionConfig", () => {
			const config: ExtractionConfig = {
				images: {
					enabled: true,
					targetDpi: 300,
					maxImageDimension: 2048,
				},
			};

			expect(config.images).toBeDefined();
			expect(config.images?.enabled).toBe(true);
			expect(config.images?.targetDpi).toBe(300);
		});
	});
});

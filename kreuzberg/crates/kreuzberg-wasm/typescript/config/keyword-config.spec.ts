/**
 * KeywordConfig configuration tests for WASM binding
 *
 * Tests for KeywordConfig feature that allows users to configure
 * keyword extraction algorithms and parameters.
 */

import { describe, expect, it } from "vitest";
import type { ExtractionConfig, KeywordAlgorithm, KeywordConfig } from "../types";

describe("WASM: KeywordConfig", () => {
	describe("type definitions", () => {
		it("should define valid KeywordConfig type", () => {
			const config: KeywordConfig = {
				algorithm: "yake",
				maxKeywords: 10,
				minScore: 0.0,
				ngramRange: [1, 3],
				language: "en",
			};

			expect(config.algorithm).toBe("yake");
			expect(config.maxKeywords).toBe(10);
			expect(config.minScore).toBe(0.0);
			expect(config.ngramRange).toEqual([1, 3]);
			expect(config.language).toBe("en");
		});

		it("should support optional fields", () => {
			const minimalConfig: KeywordConfig = {};
			expect(minimalConfig.algorithm).toBeUndefined();
			expect(minimalConfig.maxKeywords).toBeUndefined();
		});

		it("should support different algorithms", () => {
			const yakeConfig: KeywordConfig = { algorithm: "yake" };
			const rakeConfig: KeywordConfig = { algorithm: "rake" };

			expect(yakeConfig.algorithm).toBe("yake");
			expect(rakeConfig.algorithm).toBe("rake");
		});
	});

	describe("WASM serialization", () => {
		it("should serialize for WASM boundary", () => {
			const config: KeywordConfig = {
				algorithm: "yake",
				maxKeywords: 15,
				minScore: 0.1,
				ngramRange: [1, 2],
				language: "en",
			};

			const json = JSON.stringify(config);
			const parsed: KeywordConfig = JSON.parse(json);

			expect(parsed.algorithm).toBe("yake");
			expect(parsed.maxKeywords).toBe(15);
			expect(parsed.minScore).toBe(0.1);
			expect(parsed.ngramRange).toEqual([1, 2]);
		});

		it("should handle undefined fields in serialization", () => {
			const config: KeywordConfig = {
				algorithm: "yake",
				maxKeywords: undefined,
			};

			const json = JSON.stringify(config);
			expect(json).not.toContain("maxKeywords");
			expect(json).toContain("algorithm");
		});

		it("should serialize YAKE algorithm parameters", () => {
			const config: KeywordConfig = {
				algorithm: "yake",
				yakeParams: {
					windowSize: 3,
				},
			};

			const json = JSON.stringify(config);
			const parsed: KeywordConfig = JSON.parse(json);

			expect(parsed.yakeParams?.windowSize).toBe(3);
		});

		it("should serialize RAKE algorithm parameters", () => {
			const config: KeywordConfig = {
				algorithm: "rake",
				rakeParams: {
					minWordLength: 2,
					maxWordsPerPhrase: 4,
				},
			};

			const json = JSON.stringify(config);
			const parsed: KeywordConfig = JSON.parse(json);

			expect(parsed.rakeParams?.minWordLength).toBe(2);
			expect(parsed.rakeParams?.maxWordsPerPhrase).toBe(4);
		});
	});

	describe("worker message passing", () => {
		it("should serialize for worker communication", () => {
			const config: KeywordConfig = {
				algorithm: "yake",
				maxKeywords: 10,
				minScore: 0.1,
				ngramRange: [1, 3],
			};

			const cloned = structuredClone(config);
			expect(cloned.algorithm).toBe("yake");
			expect(cloned.maxKeywords).toBe(10);
			expect(cloned.ngramRange).toEqual([1, 3]);
		});

		it("should handle nested configs in ExtractionConfig", () => {
			const extractionConfig: ExtractionConfig = {
				keywords: {
					algorithm: "rake",
					maxKeywords: 20,
					language: "de",
				},
			};

			const cloned = structuredClone(extractionConfig);
			expect(cloned.keywords?.algorithm).toBe("rake");
			expect(cloned.keywords?.maxKeywords).toBe(20);
		});

		it("should preserve complex keyword configs with parameters", () => {
			const config: KeywordConfig = {
				algorithm: "yake",
				maxKeywords: 25,
				minScore: 0.5,
				ngramRange: [1, 4],
				language: "fr",
				yakeParams: {
					windowSize: 4,
				},
			};

			const cloned = structuredClone(config);
			expect(cloned.yakeParams?.windowSize).toBe(4);
			expect(cloned.ngramRange).toEqual([1, 4]);
		});
	});

	describe("type safety", () => {
		it("should enforce algorithm as valid string when defined", () => {
			const config: KeywordConfig = { algorithm: "yake" };
			if (config.algorithm !== undefined) {
				expect(["yake", "rake"]).toContain(config.algorithm);
			}
		});

		it("should enforce maxKeywords as number when defined", () => {
			const config: KeywordConfig = { maxKeywords: 10 };
			if (config.maxKeywords !== undefined) {
				expect(typeof config.maxKeywords).toBe("number");
			}
		});

		it("should enforce minScore as number when defined", () => {
			const config: KeywordConfig = { minScore: 0.5 };
			if (config.minScore !== undefined) {
				expect(typeof config.minScore).toBe("number");
			}
		});

		it("should enforce ngramRange as tuple when defined", () => {
			const config: KeywordConfig = { ngramRange: [1, 3] };
			if (config.ngramRange !== undefined) {
				expect(Array.isArray(config.ngramRange)).toBe(true);
				expect(config.ngramRange).toHaveLength(2);
			}
		});
	});

	describe("edge cases", () => {
		it("should handle zero maxKeywords", () => {
			const config: KeywordConfig = {
				algorithm: "yake",
				maxKeywords: 0,
			};
			expect(config.maxKeywords).toBe(0);
		});

		it("should handle very large maxKeywords", () => {
			const config: KeywordConfig = {
				algorithm: "yake",
				maxKeywords: 10000,
			};
			expect(config.maxKeywords).toBe(10000);
		});

		it("should handle zero minScore", () => {
			const config: KeywordConfig = {
				algorithm: "yake",
				minScore: 0.0,
			};
			expect(config.minScore).toBe(0.0);
		});

		it("should handle 1.0 minScore", () => {
			const config: KeywordConfig = {
				algorithm: "yake",
				minScore: 1.0,
			};
			expect(config.minScore).toBe(1.0);
		});

		it("should handle ngram ranges", () => {
			const configs: KeywordConfig[] = [{ ngramRange: [1, 1] }, { ngramRange: [1, 2] }, { ngramRange: [2, 4] }];

			expect(configs).toHaveLength(3);
			expect(configs[0].ngramRange).toEqual([1, 1]);
			expect(configs[2].ngramRange).toEqual([2, 4]);
		});
	});

	describe("immutability patterns", () => {
		it("should support spread operator updates", () => {
			const original: KeywordConfig = {
				algorithm: "yake",
				maxKeywords: 10,
				minScore: 0.1,
			};

			const updated: KeywordConfig = {
				...original,
				maxKeywords: 20,
			};

			expect(original.maxKeywords).toBe(10);
			expect(updated.maxKeywords).toBe(20);
			expect(updated.algorithm).toBe("yake");
		});

		it("should support nested parameter updates", () => {
			const original: KeywordConfig = {
				algorithm: "yake",
				yakeParams: {
					windowSize: 2,
				},
			};

			const updated: KeywordConfig = {
				...original,
				yakeParams: {
					...original.yakeParams,
					windowSize: 3,
				},
			};

			expect(original.yakeParams?.windowSize).toBe(2);
			expect(updated.yakeParams?.windowSize).toBe(3);
		});
	});

	describe("nesting in ExtractionConfig", () => {
		it("should nest properly in ExtractionConfig", () => {
			const config: ExtractionConfig = {
				keywords: {
					algorithm: "yake",
					maxKeywords: 15,
					minScore: 0.1,
				},
			};

			expect(config.keywords).toBeDefined();
			expect(config.keywords?.algorithm).toBe("yake");
			expect(config.keywords?.maxKeywords).toBe(15);
		});

		it("should support keywords with other extraction options", () => {
			const config: ExtractionConfig = {
				useCache: true,
				keywords: {
					algorithm: "rake",
					maxKeywords: 20,
				},
				enableQualityProcessing: true,
			};

			expect(config.useCache).toBe(true);
			expect(config.keywords?.algorithm).toBe("rake");
			expect(config.enableQualityProcessing).toBe(true);
		});
	});
});

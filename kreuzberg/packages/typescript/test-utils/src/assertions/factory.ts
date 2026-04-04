import type { AssertionAdapter, ExtractionResult, PlainRecord } from "./types.js";
import { isPlainRecord } from "./types.js";

/**
 * Metadata expectation types
 */
export type MetadataExpectation =
	| PlainRecord
	| string
	| number
	| boolean
	| {
			eq?: unknown;
			gte?: number;
			lte?: number;
			contains?: string | string[];
	  };

/**
 * High-level assertions for extraction results
 */
export interface ExtractionAssertions<TResult extends ExtractionResult> {
	assertExpectedMime(result: TResult, expected: string[]): void;
	assertMinContentLength(result: TResult, minimum: number): void;
	assertMaxContentLength(result: TResult, maximum: number): void;
	assertContentContainsAny(result: TResult, snippets: string[]): void;
	assertContentContainsAll(result: TResult, snippets: string[]): void;
	assertTableCount(result: TResult, minimum?: number | null, maximum?: number | null): void;
	assertDetectedLanguages(result: TResult, expected: string[], minConfidence?: number | null): void;
	assertMetadataExpectation(result: TResult, path: string, expectation: MetadataExpectation): void;
}

/**
 * Helper function to lookup metadata path
 */
function lookupMetadataPath(metadata: PlainRecord, path: string): unknown {
	const segments = path.split(".");
	let current: unknown = metadata;
	for (const segment of segments) {
		if (!isPlainRecord(current) || !(segment in current)) {
			return undefined;
		}
		current = current[segment];
	}
	return current;
}

/**
 * Helper function to get metadata path with fallback to format field
 */
function getMetadataPath(metadata: PlainRecord, path: string): unknown {
	const direct = lookupMetadataPath(metadata, path);
	if (direct !== undefined) {
		return direct;
	}
	const format = metadata["format"];
	if (isPlainRecord(format)) {
		return lookupMetadataPath(format, path);
	}
	return undefined;
}

/**
 * Helper function to check value equality
 */
function valuesEqual(lhs: unknown, rhs: unknown): boolean {
	if (typeof lhs === "string" && typeof rhs === "string") {
		return lhs === rhs;
	}
	if (typeof lhs === "number" && typeof rhs === "number") {
		return lhs === rhs;
	}
	if (typeof lhs === "boolean" && typeof rhs === "boolean") {
		return lhs === rhs;
	}
	return JSON.stringify(lhs) === JSON.stringify(rhs);
}

/**
 * Creates extraction assertions using the provided adapter
 */
export function createAssertions<TResult extends ExtractionResult>(
	adapter: AssertionAdapter,
): ExtractionAssertions<TResult> {
	return {
		assertExpectedMime(result: TResult, expected: string[]): void {
			if (!expected.length) {
				return;
			}
			const matches = expected.some((token) => result.mimeType.includes(token));
			adapter.assertTrue(
				matches,
				`Expected MIME type to contain one of ${JSON.stringify(expected)}, got ${result.mimeType}`,
			);
		},

		assertMinContentLength(result: TResult, minimum: number): void {
			adapter.assertGreaterThanOrEqual(
				result.content.length,
				minimum,
				`Expected content length to be at least ${minimum}, got ${result.content.length}`,
			);
		},

		assertMaxContentLength(result: TResult, maximum: number): void {
			adapter.assertLessThanOrEqual(
				result.content.length,
				maximum,
				`Expected content length to be at most ${maximum}, got ${result.content.length}`,
			);
		},

		assertContentContainsAny(result: TResult, snippets: string[]): void {
			if (!snippets.length) {
				return;
			}
			const lowered = result.content.toLowerCase();
			const matches = snippets.some((snippet) => lowered.includes(snippet.toLowerCase()));
			adapter.assertTrue(matches, `Expected content to contain one of ${JSON.stringify(snippets)}`);
		},

		assertContentContainsAll(result: TResult, snippets: string[]): void {
			if (!snippets.length) {
				return;
			}
			const lowered = result.content.toLowerCase();
			const allMatch = snippets.every((snippet) => lowered.includes(snippet.toLowerCase()));
			adapter.assertTrue(allMatch, `Expected content to contain all of ${JSON.stringify(snippets)}`);
		},

		assertTableCount(result: TResult, minimum?: number | null, maximum?: number | null): void {
			const tables = Array.isArray(result.tables) ? result.tables : [];
			if (typeof minimum === "number") {
				adapter.assertGreaterThanOrEqual(
					tables.length,
					minimum,
					`Expected at least ${minimum} tables, got ${tables.length}`,
				);
			}
			if (typeof maximum === "number") {
				adapter.assertLessThanOrEqual(
					tables.length,
					maximum,
					`Expected at most ${maximum} tables, got ${tables.length}`,
				);
			}
		},

		assertDetectedLanguages(result: TResult, expected: string[], minConfidence?: number | null): void {
			if (!expected.length) {
				return;
			}
			adapter.assertDefined(result.detectedLanguages, "Expected detectedLanguages to be defined");
			const languages = result.detectedLanguages ?? [];
			const allPresent = expected.every((lang) => languages.includes(lang));
			adapter.assertTrue(
				allPresent,
				`Expected detected languages to include all of ${JSON.stringify(expected)}, got ${JSON.stringify(languages)}`,
			);

			if (typeof minConfidence === "number" && isPlainRecord(result.metadata)) {
				const confidence = result.metadata["confidence"];
				if (typeof confidence === "number") {
					adapter.assertGreaterThanOrEqual(
						confidence,
						minConfidence,
						`Expected confidence to be at least ${minConfidence}, got ${confidence}`,
					);
				}
			}
		},

		assertMetadataExpectation(result: TResult, path: string, expectation: MetadataExpectation): void {
			if (!isPlainRecord(result.metadata)) {
				adapter.fail(`Metadata is not a record for path ${path}`);
			}

			const value = getMetadataPath(result.metadata, path);
			if (value === undefined || value === null) {
				adapter.fail(`Metadata path '${path}' missing in ${JSON.stringify(result.metadata)}`);
			}

			if (!isPlainRecord(expectation)) {
				adapter.assertTrue(
					valuesEqual(value, expectation),
					`Expected metadata at '${path}' to equal ${JSON.stringify(expectation)}, got ${JSON.stringify(value)}`,
				);
				return;
			}

			if ("eq" in expectation) {
				adapter.assertTrue(
					valuesEqual(value, expectation.eq),
					`Expected metadata at '${path}' to equal ${JSON.stringify(expectation.eq)}, got ${JSON.stringify(value)}`,
				);
			}

			if ("gte" in expectation && typeof expectation.gte === "number") {
				adapter.assertGreaterThanOrEqual(
					Number(value),
					expectation.gte,
					`Expected metadata at '${path}' to be >= ${expectation.gte}, got ${value}`,
				);
			}

			if ("lte" in expectation && typeof expectation.lte === "number") {
				adapter.assertLessThanOrEqual(
					Number(value),
					expectation.lte,
					`Expected metadata at '${path}' to be <= ${expectation.lte}, got ${value}`,
				);
			}

			if ("contains" in expectation) {
				const contains = expectation.contains;
				if (typeof value === "string" && typeof contains === "string") {
					adapter.assertTrue(
						value.includes(contains),
						`Expected metadata at '${path}' to contain "${contains}", got "${value}"`,
					);
				} else if (Array.isArray(value) && typeof contains === "string") {
					adapter.assertTrue(
						value.includes(contains),
						`Expected metadata at '${path}' to contain "${contains}", got ${JSON.stringify(value)}`,
					);
				} else if (Array.isArray(value) && Array.isArray(contains)) {
					const allPresent = contains.every((item) => value.includes(item));
					adapter.assertTrue(
						allPresent,
						`Expected metadata at '${path}' to contain all of ${JSON.stringify(contains)}, got ${JSON.stringify(value)}`,
					);
				} else {
					adapter.fail(`Unsupported contains expectation for path '${path}'`);
				}
			}
		},
	};
}

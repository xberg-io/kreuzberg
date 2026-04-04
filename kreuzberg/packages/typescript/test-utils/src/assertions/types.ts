/**
 * Generic plain object record type (duplicated from config-mapping to avoid circular deps)
 */
export type PlainRecord = Record<string, unknown>;

/**
 * Type guard to check if a value is a plain object record
 */
export function isPlainRecord(value: unknown): value is PlainRecord {
	return typeof value === "object" && value !== null && !Array.isArray(value);
}

/**
 * Generic result type that can be specialized per test suite
 */
export interface ExtractionResult {
	content: string;
	mimeType: string;
	tables?: unknown[];
	detectedLanguages?: string[] | null;
	metadata?: PlainRecord | null;
	[key: string]: unknown;
}

/**
 * Basic assertion adapter interface
 * Implementations provide platform-specific assertion logic
 */
export interface AssertionAdapter {
	/**
	 * Assert that a condition is true
	 */
	assertTrue(value: boolean, message?: string): void;

	/**
	 * Assert that two values are equal
	 */
	assertEqual<T>(actual: T, expected: T, message?: string): void;

	/**
	 * Assert that a value is defined (not null or undefined)
	 */
	assertDefined<T>(value: T | null | undefined, message?: string): void;

	/**
	 * Assert that a number is greater than or equal to a minimum
	 */
	assertGreaterThanOrEqual(actual: number, minimum: number, message?: string): void;

	/**
	 * Assert that a number is less than or equal to a maximum
	 */
	assertLessThanOrEqual(actual: number, maximum: number, message?: string): void;

	/**
	 * Throw an error with the given message
	 */
	fail(message: string): never;
}

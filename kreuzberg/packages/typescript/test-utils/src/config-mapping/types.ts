/**
 * Generic plain object record type
 */
export type PlainRecord = Record<string, unknown>;

/**
 * Type guard to check if a value is a plain object record
 */
export function isPlainRecord(value: unknown): value is PlainRecord {
	return typeof value === "object" && value !== null && !Array.isArray(value);
}

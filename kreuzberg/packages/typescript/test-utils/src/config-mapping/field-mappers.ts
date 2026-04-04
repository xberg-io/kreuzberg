import type { PlainRecord } from "./types.js";

/**
 * Assigns a boolean field from source to target, converting if necessary
 */
export function assignBooleanField(
	target: PlainRecord,
	source: PlainRecord,
	sourceKey: string,
	targetKey: string,
): void {
	if (sourceKey in source) {
		const value = source[sourceKey];
		if (typeof value === "boolean") {
			target[targetKey] = value;
		} else if (value != null) {
			target[targetKey] = Boolean(value);
		}
	}
}

/**
 * Assigns a number field from source to target, converting if necessary
 */
export function assignNumberField(
	target: PlainRecord,
	source: PlainRecord,
	sourceKey: string,
	targetKey: string,
): void {
	if (sourceKey in source) {
		const value = source[sourceKey];
		if (typeof value === "number") {
			target[targetKey] = value;
		} else if (typeof value === "string") {
			const parsed = Number(value);
			if (!Number.isNaN(parsed)) {
				target[targetKey] = parsed;
			}
		}
	}
}

/**
 * Assigns a string field from source to target
 */
export function assignStringField(
	target: PlainRecord,
	source: PlainRecord,
	sourceKey: string,
	targetKey: string,
): void {
	if (sourceKey in source) {
		const value = source[sourceKey];
		if (typeof value === "string") {
			target[targetKey] = value;
		}
	}
}

/**
 * Maps an array value to string array, filtering out non-string elements
 */
export function mapStringArray(value: unknown): string[] | undefined {
	if (!Array.isArray(value)) {
		return undefined;
	}
	return value.filter((item): item is string => typeof item === "string");
}

/**
 * Assigns a string array field from source to target
 */
export function assignStringArrayField(
	target: PlainRecord,
	source: PlainRecord,
	sourceKey: string,
	targetKey: string,
): void {
	if (sourceKey in source) {
		const mapped = mapStringArray(source[sourceKey]);
		if (mapped !== undefined) {
			target[targetKey] = mapped;
		}
	}
}

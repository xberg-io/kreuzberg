import type { AssertionAdapter } from "./types.js";

/**
 * Deno assertion adapter
 * Uses Deno's std/assert module
 */
export class DenoAdapter implements AssertionAdapter {
	assertTrue(value: boolean, message?: string): void {
		if (!value) {
			throw new Error(message || "Expected value to be true");
		}
	}

	assertEqual<T>(actual: T, expected: T, message?: string): void {
		const actualStr = JSON.stringify(actual);
		const expectedStr = JSON.stringify(expected);
		if (actualStr !== expectedStr) {
			throw new Error(message || `Expected ${actualStr} to equal ${expectedStr}`);
		}
	}

	assertDefined<T>(value: T | null | undefined, message?: string): void {
		if (value === null || value === undefined) {
			throw new Error(message || "Expected value to be defined");
		}
	}

	assertGreaterThanOrEqual(actual: number, minimum: number, message?: string): void {
		if (actual < minimum) {
			throw new Error(message || `Expected ${actual} to be greater than or equal to ${minimum}`);
		}
	}

	assertLessThanOrEqual(actual: number, maximum: number, message?: string): void {
		if (actual > maximum) {
			throw new Error(message || `Expected ${actual} to be less than or equal to ${maximum}`);
		}
	}

	fail(message: string): never {
		throw new Error(message);
	}
}

import type { AssertionAdapter } from "./types.js";

/**
 * Vitest assertion adapter
 * Assumes 'expect' is available from vitest
 */
export class VitestAdapter implements AssertionAdapter {
	private expect: typeof import("vitest")["expect"];

	constructor(expectFn?: typeof import("vitest")["expect"]) {
		// Allow injection for testing, but require it at runtime
		if (expectFn) {
			this.expect = expectFn;
		} else {
			// Dynamic import at runtime - this will fail gracefully if vitest is not available
			try {
				// @ts-expect-error - expect should be globally available in vitest context
				this.expect = expect;
			} catch {
				throw new Error(
					"VitestAdapter requires 'expect' to be available. Make sure you're running in a Vitest environment.",
				);
			}
		}
	}

	assertTrue(value: boolean, message?: string): void {
		this.expect(value, message).toBe(true);
	}

	assertEqual<T>(actual: T, expected: T, message?: string): void {
		this.expect(actual, message).toBe(expected);
	}

	assertDefined<T>(value: T | null | undefined, message?: string): void {
		this.expect(value, message).not.toBeNull();
		this.expect(value, message).not.toBeUndefined();
	}

	assertGreaterThanOrEqual(actual: number, minimum: number, message?: string): void {
		this.expect(actual, message).toBeGreaterThanOrEqual(minimum);
	}

	assertLessThanOrEqual(actual: number, maximum: number, message?: string): void {
		this.expect(actual, message).toBeLessThanOrEqual(maximum);
	}

	fail(message: string): never {
		throw new Error(message);
	}
}

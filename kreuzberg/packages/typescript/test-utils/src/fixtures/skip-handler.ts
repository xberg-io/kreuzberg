import type { FixtureSkipHandler } from "./types.js";

/**
 * Default implementation of fixture skip handler
 */
export class DefaultSkipHandler implements FixtureSkipHandler {
	shouldSkipFixture(error: unknown, fixtureId: string, requirements: string[], notes?: string | null): boolean {
		if (!(error instanceof Error)) {
			return false;
		}

		const message = `${error.name}: ${error.message}`;
		const lower = message.toLowerCase();

		const requirementHit = requirements.some((req) => lower.includes(req.toLowerCase()));
		const missingDependency = lower.includes("missingdependencyerror") || lower.includes("missing dependency");
		const unsupportedFormat = lower.includes("unsupported mime type") || lower.includes("unsupported format");

		if (missingDependency || unsupportedFormat || requirementHit) {
			const reason = missingDependency
				? "missing dependency"
				: unsupportedFormat
					? "unsupported format"
					: requirements.join(", ");
			console.warn(`Skipping ${fixtureId}: ${reason}. ${message}`);
			if (notes) {
				console.warn(`Notes: ${notes}`);
			}
			return true;
		}

		return false;
	}
}

/**
 * Convenience export of the default skip handler function
 */
export function shouldSkipFixture(
	error: unknown,
	fixtureId: string,
	requirements: string[],
	notes?: string | null,
): boolean {
	const handler = new DefaultSkipHandler();
	return handler.shouldSkipFixture(error, fixtureId, requirements, notes);
}

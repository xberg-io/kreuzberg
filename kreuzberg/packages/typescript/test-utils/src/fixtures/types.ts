/**
 * Interface for skipping fixtures based on errors
 */
export interface FixtureSkipHandler {
	shouldSkipFixture(error: unknown, fixtureId: string, requirements: string[], notes?: string | null): boolean;
}

// Auto-generated tests for xml fixtures.
// Designed for Cloudflare Workers with Vitest + Miniflare

import { describe, it, expect } from "vitest";
import { extractBytes } from "@kreuzberg/wasm";
import { assertions, buildConfig, getFixture, shouldSkipFixture } from "./helpers.js";
import type { ExtractionResult } from "@kreuzberg/wasm";

describe("xml", () => {
	it("xml_plant_catalog", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("xml/plant_catalog.xml");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "xml_plant_catalog", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/xml"]);
		assertions.assertMinContentLength(result, 100);
		assertions.assertMetadataExpectation(result, "element_count", { gte: 1 });
	});
});

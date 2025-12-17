// Auto-generated tests for pdf fixtures.
// Designed for Cloudflare Workers with Vitest + Miniflare

import { describe, it, expect } from "vitest";
import { extractBytes } from "@kreuzberg/wasm";
import { assertions, buildConfig, getFixture, shouldSkipFixture } from "./helpers.js";
import type { ExtractionResult } from "@kreuzberg/wasm";

describe("pdf", () => {
	it("pdf_assembly_technical", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs/assembly_language_for_beginners_al4_b_en.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_assembly_technical", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 5000);
		assertions.assertContentContainsAny(result, ["assembly", "register", "instruction"]);
		assertions.assertMetadataExpectation(result, "format_type", { eq: "pdf" });
	});

	it("pdf_bayesian_data_analysis", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs/bayesian_data_analysis_third_edition_13th_feb_2020.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_bayesian_data_analysis", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 10000);
		assertions.assertContentContainsAny(result, ["Bayesian", "probability", "distribution"]);
		assertions.assertMetadataExpectation(result, "format_type", { eq: "pdf" });
	});

	it("pdf_code_and_formula", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs/code_and_formula.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_code_and_formula", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 100);
	});

	it("pdf_deep_learning", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs/fundamentals_of_deep_learning_2014.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_deep_learning", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 1000);
		assertions.assertContentContainsAny(result, ["neural", "network", "deep learning"]);
		assertions.assertMetadataExpectation(result, "format_type", { eq: "pdf" });
	});

	it("pdf_embedded_images", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs/embedded_images_tables.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_embedded_images", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 50);
		assertions.assertTableCount(result, 0, null);
	});

	it("pdf_google_doc", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs/google_doc_document.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_google_doc", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 50);
		assertions.assertMetadataExpectation(result, "format_type", { eq: "pdf" });
	});

	it("pdf_large_ciml", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs/a_course_in_machine_learning_ciml_v0_9_all.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_large_ciml", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 10000);
		assertions.assertContentContainsAny(result, ["machine learning", "algorithm", "training"]);
		assertions.assertMetadataExpectation(result, "format_type", { eq: "pdf" });
	});

	it("pdf_non_english_german", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs/5_level_paging_and_5_level_ept_intel_revision_1_1_may_2017.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_non_english_german", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 100);
		assertions.assertContentContainsAny(result, ["Intel", "paging"]);
		assertions.assertMetadataExpectation(result, "format_type", { eq: "pdf" });
	});

	it("pdf_right_to_left", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs/right_to_left_01.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_right_to_left", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 50);
		assertions.assertMetadataExpectation(result, "format_type", { eq: "pdf" });
	});

	it("pdf_simple_text", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs/fake_memo.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_simple_text", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 50);
		assertions.assertContentContainsAny(result, ["May 5, 2023", "To Whom it May Concern", "Mallori"]);
	});

	it("pdf_tables_large", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs_with_tables/large.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_tables_large", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 500);
		assertions.assertTableCount(result, 1, null);
	});

	it("pdf_tables_medium", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs_with_tables/medium.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_tables_medium", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 100);
		assertions.assertTableCount(result, 1, null);
	});

	it("pdf_tables_small", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture("pdfs_with_tables/tiny.pdf");
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_tables_small", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 50);
		assertions.assertContentContainsAll(result, [
			"Table 1",
			"Selected Numbers",
			"Celsius",
			"Fahrenheit",
			"Water Freezing Point",
			"Water Boiling Point",
		]);
		assertions.assertTableCount(result, 1, null);
	});

	it("pdf_technical_stat_learning", async () => {
		let documentBytes: Uint8Array;
		let result: ExtractionResult | null = null;
		try {
			documentBytes = getFixture(
				"pdfs/an_introduction_to_statistical_learning_with_applications_in_r_islr_sixth_printing.pdf",
			);
			const config = buildConfig(undefined);
			result = await extractBytes(documentBytes, "application/pdf", config);
		} catch (error) {
			if (shouldSkipFixture(error, "pdf_technical_stat_learning", [], undefined)) {
				return;
			}
			throw error;
		}
		if (result === null) {
			return;
		}
		assertions.assertExpectedMime(result, ["application/pdf"]);
		assertions.assertMinContentLength(result, 10000);
		assertions.assertContentContainsAny(result, ["statistical", "regression", "learning"]);
		assertions.assertMetadataExpectation(result, "format_type", { eq: "pdf" });
	});
});

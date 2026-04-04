/**
 * Hand-written binding-specific edge case tests for PDF rendering.
 * Happy-path render tests are auto-generated from fixtures in e2e/.
 * These tests cover error handling, validation, and lifecycle patterns
 * that vary per language and can't be generated uniformly.
 */

import { existsSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { PdfPageIterator, renderPdfPageSync } from "../../dist/index.js";

const WORKSPACE_ROOT = resolve(__dirname, "../../../../..");
const TEST_PDF = resolve(WORKSPACE_ROOT, "test_documents/pdf/tiny.pdf");

describe("Render edge cases", () => {
	it("rendering functions exist", () => {
		expect(typeof renderPdfPageSync).toBe("function");
		expect(typeof PdfPageIterator).toBe("function");
	});

	it("renderPdfPageSync throws for nonexistent file", () => {
		expect(() => renderPdfPageSync("/nonexistent/path/to/document.pdf", 0)).toThrow(/No such file/);
	});

	it("renderPdfPageSync throws for out-of-bounds page index", () => {
		if (!existsSync(TEST_PDF)) return;
		expect(() => renderPdfPageSync(TEST_PDF, 9999)).toThrow(/not found/);
	});

	it("renderPdfPageSync throws for negative page index", () => {
		if (!existsSync(TEST_PDF)) return;
		// Negative index wraps to large unsigned — triggers page not found
		expect(() => renderPdfPageSync(TEST_PDF, -1)).toThrow();
	});

	it("PdfPageIterator throws for nonexistent file", () => {
		expect(() => new PdfPageIterator("/nonexistent/path/to/document.pdf")).toThrow(/No such file|error/i);
	});

	it("PdfPageIterator.close is safe to call multiple times", () => {
		if (!existsSync(TEST_PDF)) return;
		const iter = new PdfPageIterator(TEST_PDF);
		iter.close();
		iter.close(); // Should not throw
	});

	it("PdfPageIterator supports early termination", () => {
		if (!existsSync(TEST_PDF)) return;
		const iter = new PdfPageIterator(TEST_PDF);
		const result = iter.next();
		expect(result).not.toBeNull();
		if (result) {
			expect(result.pageIndex).toBe(0);
			expect(result.data).toBeInstanceOf(Buffer);
			expect(result.data.length).toBeGreaterThan(4);
		}
		iter.close(); // Close without exhausting
	});

	it("renderPdfPageSync throws for empty path", () => {
		expect(() => renderPdfPageSync("", 0)).toThrow();
	});
});

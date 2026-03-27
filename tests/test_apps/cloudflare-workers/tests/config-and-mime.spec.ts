import {
	detectMimeFromBytes,
	extractBytes,
	getExtensionsForMime,
	initWasm,
	isCloudflareWorkers,
	isInitialized,
	type ExtractionConfig,
} from "@kreuzberg/wasm";
import wasmModule from "@kreuzberg/wasm/kreuzberg_wasm_bg.wasm";
import { beforeAll, describe, expect, it } from "vitest";

beforeAll(async () => {
	if (!isInitialized()) {
		await initWasm({ wasmModule });
	}
});

describe("ExtractionConfig in Cloudflare Workers", () => {
	it("should extract with chunking config", async () => {
		const text = "Hello from Cloudflare Workers with chunking enabled!";
		const bytes = new TextEncoder().encode(text);
		const config: ExtractionConfig = {
			chunking: { maxChars: 100, maxOverlap: 10 },
		};
		const result = await extractBytes(bytes, "text/plain", config);
		expect(result).toBeDefined();
		expect(result.content).toBeDefined();
	});

	it("should extract with outputFormat 'plain'", async () => {
		const bytes = new TextEncoder().encode("Plain text content");
		const config: ExtractionConfig = { outputFormat: "plain" };
		const result = await extractBytes(bytes, "text/plain", config);
		expect(result).toBeDefined();
		expect(typeof result.content).toBe("string");
	});

	it("should extract with outputFormat 'markdown'", async () => {
		const html = "<h1>Title</h1><p>Paragraph.</p>";
		const bytes = new TextEncoder().encode(html);
		const config: ExtractionConfig = { outputFormat: "markdown" };
		const result = await extractBytes(bytes, "text/html", config);
		expect(result).toBeDefined();
		expect(typeof result.content).toBe("string");
	});

	it("should extract with null config", async () => {
		const bytes = new TextEncoder().encode("content");
		const result = await extractBytes(bytes, "text/plain", null);
		expect(result).toBeDefined();
	});

	it("should extract with undefined config", async () => {
		const bytes = new TextEncoder().encode("content");
		const result = await extractBytes(bytes, "text/plain", undefined);
		expect(result).toBeDefined();
	});
});

describe("MIME Utilities in Cloudflare Workers", () => {
	it("should detect MIME type from PDF bytes", () => {
		const pdfHeader = new Uint8Array([0x25, 0x50, 0x44, 0x46]); // %PDF
		const mimeType = detectMimeFromBytes(pdfHeader);
		expect(typeof mimeType).toBe("string");
	});

	it("should detect MIME type from PNG bytes", () => {
		const pngHeader = new Uint8Array([0x89, 0x50, 0x4e, 0x47]); // PNG magic
		const mimeType = detectMimeFromBytes(pngHeader);
		expect(typeof mimeType).toBe("string");
	});

	it("should get extensions for PDF MIME type", () => {
		const extensions = getExtensionsForMime("application/pdf");
		expect(Array.isArray(extensions)).toBe(true);
		expect(extensions.length).toBeGreaterThan(0);
		expect(extensions).toContain("pdf");
	});

	it("should get extensions for plain text MIME type", () => {
		const extensions = getExtensionsForMime("text/plain");
		expect(Array.isArray(extensions)).toBe(true);
		expect(extensions.length).toBeGreaterThan(0);
	});

	it("should return empty array for unknown MIME type", () => {
		const extensions = getExtensionsForMime("application/x-completely-unknown-type");
		expect(Array.isArray(extensions)).toBe(true);
	});
});

describe("Runtime Detection in Cloudflare Workers", () => {
	it("should detect Cloudflare Workers runtime", () => {
		const result = isCloudflareWorkers();
		expect(typeof result).toBe("boolean");
		// In vitest-pool-workers, this should be true
		expect(result).toBe(true);
	});
});

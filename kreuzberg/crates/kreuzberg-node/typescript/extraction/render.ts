/**
 * PDF page rendering functions.
 *
 * Render individual PDF pages or iterate over all pages as PNG images.
 */

import { getBinding } from "../core/binding.js";

/**
 * Render a single PDF page to a PNG buffer (synchronous).
 *
 * @param filePath - Path to the PDF file
 * @param pageIndex - Zero-based page index
 * @param options - Optional settings
 * @param options.dpi - DPI for rendering (default 150)
 * @returns Buffer containing PNG image data
 */
export function renderPdfPageSync(filePath: string, pageIndex: number, options?: { dpi?: number }): Buffer {
	return getBinding().renderPdfPageSync(filePath, pageIndex, options?.dpi ?? null);
}

/**
 * Render a single PDF page to a PNG buffer (asynchronous).
 *
 * @param filePath - Path to the PDF file
 * @param pageIndex - Zero-based page index
 * @param options - Optional settings
 * @param options.dpi - DPI for rendering (default 150)
 * @returns Promise resolving to a Buffer containing PNG image data
 */
export async function renderPdfPage(filePath: string, pageIndex: number, options?: { dpi?: number }): Promise<Buffer> {
	return getBinding().renderPdfPage(filePath, pageIndex, options?.dpi ?? null);
}

/** A rendered PDF page with its index and PNG data. */
export interface PdfPageResult {
	pageIndex: number;
	data: Buffer;
}

/**
 * Collect all PDF pages as PNG images (synchronous).
 *
 * @param filePath - Path to the PDF file
 * @param options - Optional settings
 * @param options.dpi - DPI for rendering (default 150)
 * @returns Array of PdfPageResult objects
 */
export function iteratePdfPagesSync(filePath: string, options?: { dpi?: number }): PdfPageResult[] {
	return getBinding().iteratePdfPagesSync(filePath, options?.dpi ?? null);
}

/**
 * Collect all PDF pages as PNG images (asynchronous).
 *
 * @param filePath - Path to the PDF file
 * @param options - Optional settings
 * @param options.dpi - DPI for rendering (default 150)
 * @returns Promise resolving to an array of PdfPageResult objects
 */
export async function iteratePdfPages(filePath: string, options?: { dpi?: number }): Promise<PdfPageResult[]> {
	return getBinding().iteratePdfPages(filePath, options?.dpi ?? null);
}

/**
 * Get the number of pages in a PDF file.
 *
 * @param filePath - Path to the PDF file
 * @returns Number of pages
 */
export function pdfPageCount(filePath: string): number {
	return getBinding().pdfPageCount(filePath);
}

/**
 * Lazy PDF page iterator. Renders one page at a time via `.next()`.
 * Call `.close()` when done to free native resources.
 *
 * @example
 * ```typescript
 * const iter = new PdfPageIterator("doc.pdf", { dpi: 150 });
 * let result;
 * while ((result = iter.next()) !== null) {
 *     const { pageIndex, data } = result;
 *     // process page...
 * }
 * iter.close();
 * ```
 */
export class PdfPageIterator {
	private inner: { next(): PdfPageResult | null; pageCount(): number; close(): void };

	constructor(filePath: string, options?: { dpi?: number }) {
		const Ctor = getBinding().JsPdfPageIterator;
		this.inner = new Ctor(filePath, options?.dpi ?? null);
	}

	/** Advance and return the next page, or null when exhausted. */
	next(): PdfPageResult | null {
		return this.inner.next();
	}

	/** Total number of pages in the PDF. */
	pageCount(): number {
		return this.inner.pageCount();
	}

	/** Free native resources. Safe to call multiple times. */
	close(): void {
		this.inner.close();
	}
}

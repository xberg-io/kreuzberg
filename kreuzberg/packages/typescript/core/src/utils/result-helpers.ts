/**
 * Result helper utilities for ExtractionResult manipulation.
 *
 * These functions provide standalone utilities for accessing and querying
 * extraction results, including page counts, chunk counts, language detection,
 * and metadata field access.
 */

import type { ExtractionResult } from "../types/results.js";

/**
 * Get the page count from an extraction result.
 *
 * Returns the total number of pages/slides/sheets detected
 * in the original document by checking metadata.page_count.
 *
 * @param result - The extraction result to query
 * @returns The page count (>= 0), or null if not available
 *
 * @example
 * ```typescript
 * import { resultGetPageCount } from '@kreuzberg/core';
 *
 * const result = await extractFile('document.pdf');
 * const pageCount = resultGetPageCount(result);
 * if (pageCount !== null) {
 *   console.log(`Document has ${pageCount} pages`);
 * }
 * ```
 */
export function resultGetPageCount(result: ExtractionResult): number | null {
	// Check metadata.page_count (PDF/PPTX format)
	if (result.metadata?.page_count !== undefined && result.metadata.page_count !== null) {
		return result.metadata.page_count;
	}

	// Check metadata.sheet_count (Excel format)
	if (result.metadata?.sheet_count !== undefined && result.metadata.sheet_count !== null) {
		return result.metadata.sheet_count;
	}

	return null;
}

/**
 * Get the chunk count from an extraction result.
 *
 * Returns the number of text chunks when chunking is enabled,
 * or null if chunking was not performed or information is unavailable.
 *
 * @param result - The extraction result to query
 * @returns The chunk count (>= 0), or null if not available
 *
 * @example
 * ```typescript
 * import { resultGetChunkCount } from '@kreuzberg/core';
 *
 * const result = await extractFile('document.pdf', {
 *   chunking: { enabled: true, maxChars: 1024 }
 * });
 * const chunkCount = resultGetChunkCount(result);
 * if (chunkCount !== null) {
 *   console.log(`Document has ${chunkCount} chunks`);
 * }
 * ```
 */
export function resultGetChunkCount(result: ExtractionResult): number | null {
	if (result.chunks && Array.isArray(result.chunks)) {
		return result.chunks.length;
	}
	return null;
}

/**
 * Get the detected language from an extraction result.
 *
 * Returns the primary detected language as an ISO 639 language code
 * (e.g., "en", "de", "fr"). If multiple languages were detected,
 * returns the primary one. Falls back to metadata.language if
 * detectedLanguages is not available.
 *
 * @param result - The extraction result to query
 * @returns The language code (e.g., "en"), or null if not detected
 *
 * @example
 * ```typescript
 * import { resultGetDetectedLanguage } from '@kreuzberg/core';
 *
 * const result = await extractFile('document.pdf');
 * const language = resultGetDetectedLanguage(result);
 * if (language) {
 *   console.log(`Detected language: ${language}`);
 * }
 * ```
 */
export function resultGetDetectedLanguage(result: ExtractionResult): string | null {
	// Check detectedLanguages array first (primary detection method)
	if (result.detectedLanguages && result.detectedLanguages.length > 0) {
		const firstLang = result.detectedLanguages[0];
		if (firstLang !== undefined) {
			return firstLang;
		}
	}

	// Fall back to metadata.language
	if (result.metadata?.language) {
		return result.metadata.language;
	}

	return null;
}

/**
 * Get a metadata field from an extraction result using dot notation.
 *
 * Retrieves a metadata field value. Supports nested fields with dot notation
 * (e.g., "page_count", "exif.DateTimeOriginal").
 *
 * @param result - The extraction result to query
 * @param fieldName - The metadata field name or path to retrieve
 * @returns The field value, or null if not found
 *
 * @example
 * ```typescript
 * import { resultGetMetadataField } from '@kreuzberg/core';
 *
 * const result = await extractFile('document.pdf');
 *
 * // Get simple field
 * const title = resultGetMetadataField(result, 'title') as string | null;
 * if (title) {
 *   console.log(`Title: ${title}`);
 * }
 *
 * // Get nested field (e.g., exif data for images)
 * const dateTime = resultGetMetadataField(result, 'exif.DateTimeOriginal') as string | null;
 * if (dateTime) {
 *   console.log(`Date taken: ${dateTime}`);
 * }
 *
 * // Get page count
 * const pageCount = resultGetMetadataField(result, 'page_count') as number | null;
 * if (pageCount !== null) {
 *   console.log(`Pages: ${pageCount}`);
 * }
 * ```
 */
export function resultGetMetadataField(result: ExtractionResult, fieldName: string): unknown {
	if (!result.metadata) {
		return null;
	}

	const parts = fieldName.split(".");
	let current: unknown = result.metadata;

	for (const part of parts) {
		if (current === null || current === undefined) {
			return null;
		}
		if (typeof current !== "object") {
			return null;
		}
		current = (current as Record<string, unknown>)[part];
	}

	return current ?? null;
}

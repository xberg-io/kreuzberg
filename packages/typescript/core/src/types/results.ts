/**
 * Result type definitions for Kreuzberg document extraction.
 *
 * These types represent the output of extraction operations,
 * including extracted content, metadata, tables, chunks, images, and keywords.
 */

import type { ExtractedKeyword } from "./config.js";
import type { Metadata } from "./metadata.js";

// ============================================================================

export interface Table {
	cells: string[][];
	markdown: string;
	pageNumber: number;
}

export interface ChunkMetadata {
	charStart: number;
	charEnd: number;
	tokenCount?: number | null;
	chunkIndex: number;
	totalChunks: number;
}

export interface Chunk {
	content: string;
	embedding?: number[] | null;
	metadata: ChunkMetadata;
}

export interface ExtractedImage {
	data: Uint8Array;
	format: string;
	imageIndex: number;
	pageNumber?: number | null;
	width?: number | null;
	height?: number | null;
	colorspace?: string | null;
	bitsPerComponent?: number | null;
	isMask: boolean;
	description?: string | null;
	ocrResult?: ExtractionResult | null;
}

// ============================================================================
// Element-based output types (compatible with Unstructured.io format)

/**
 * Semantic element type classification.
 *
 * Categorizes text content into semantic units for downstream processing.
 * Supports element types commonly found in document analysis.
 */
export type ElementType =
	| "title"
	| "narrative_text"
	| "heading"
	| "list_item"
	| "table"
	| "image"
	| "page_break"
	| "code_block"
	| "block_quote"
	| "footer"
	| "header";

/**
 * Bounding box coordinates for element positioning.
 *
 * Represents a rectangular region in a document with normalized coordinates.
 */
export interface BoundingBox {
	/** Left x-coordinate (0.0 to 1.0 or page-width normalized) */
	x0: number;
	/** Bottom y-coordinate (0.0 to 1.0 or page-height normalized) */
	y0: number;
	/** Right x-coordinate (0.0 to 1.0 or page-width normalized) */
	x1: number;
	/** Top y-coordinate (0.0 to 1.0 or page-height normalized) */
	y1: number;
}

/**
 * Metadata for a semantic element.
 *
 * Provides contextual information about an extracted element including
 * page location, document filename, spatial coordinates, and custom metadata.
 */
export interface ElementMetadata {
	/** Page number (1-indexed), or null if not available */
	page_number?: number | null;
	/** Source filename or document name, or null if not available */
	filename?: string | null;
	/** Bounding box coordinates if available, or null */
	coordinates?: BoundingBox | null;
	/** Position index in the element sequence, or null if not available */
	element_index?: number | null;
	/** Additional custom metadata fields */
	additional?: Record<string, string>;
}

/**
 * Semantic element extracted from document.
 *
 * Represents a logical unit of content with semantic classification,
 * unique identifier, and metadata for tracking origin and position.
 * Compatible with Unstructured.io element format.
 */
export interface Element {
	/** Unique element identifier (deterministic hash-based ID) */
	elementId: string;
	/** Semantic type of this element */
	elementType: ElementType;
	/** Text content of the element */
	text: string;
	/** Metadata about the element including page number, coordinates, etc. */
	metadata: ElementMetadata;
}

// ============================================================================
// Djot structured content types

/**
 * Structured Djot document representation.
 *
 * Provides rich, block-level document structure with formatting,
 * tables, images, links, and metadata extracted from source documents.
 * Available when Djot output format is enabled.
 */
export interface DjotContent {
	/** Plain text representation for backward compatibility */
	plainText: string;
	/** Structured block-level content (headings, paragraphs, lists, etc.) */
	blocks: FormattedBlock[];
	/** Metadata from YAML frontmatter */
	metadata: Metadata;
	/** Extracted tables as structured data */
	tables: Table[];
	/** Extracted images with metadata */
	images: DjotImage[];
	/** Extracted links with URLs and titles */
	links: DjotLink[];
	/** Footnote definitions */
	footnotes: Footnote[];
	/** Attributes mapped by element identifier */
	attributes?: Record<string, Attributes>;
}

/**
 * Block-level element in a Djot document.
 *
 * Represents structural elements like headings, paragraphs, lists, code blocks, etc.
 */
export interface FormattedBlock {
	/** Type of block element */
	blockType: BlockType;
	/** Heading level (1-6) for headings, or nesting level for lists */
	level?: number | null;
	/** Text content for inline elements */
	content?: string | null;
	/** Child blocks for list items and containers */
	children?: FormattedBlock[] | null;
	/** Attributes (id, class, etc.) */
	attributes?: Attributes | null;
}

/**
 * Block element type classification.
 */
export type BlockType =
	| "paragraph"
	| "heading"
	| "list_item"
	| "code_block"
	| "block_quote"
	| "thematic_break"
	| "table"
	| "image"
	| "footnote_definition"
	| "raw_block";

/**
 * HTML/CSS attributes for an element.
 */
export interface Attributes {
	[key: string]: string | number | boolean | string[] | null;
}

/**
 * Image with metadata in Djot document.
 */
export interface DjotImage {
	/** Image URL or reference */
	url: string;
	/** Alternative text */
	alt?: string | null;
	/** Image title/caption */
	title?: string | null;
	/** Image attributes */
	attributes?: Attributes | null;
}

/**
 * Link in Djot document.
 */
export interface DjotLink {
	/** Link URL */
	url: string;
	/** Link text */
	text: string;
	/** Link title */
	title?: string | null;
	/** Link type */
	linkType?: "internal" | "external" | "email" | "phone" | "footnote";
}

/**
 * Footnote definition.
 */
export interface Footnote {
	/** Footnote identifier */
	label: string;
	/** Footnote content */
	content: string;
}

export interface ExtractionResult {
	content: string;
	mimeType: string;
	metadata: Metadata;
	tables: Table[];
	detectedLanguages: string[] | null;
	chunks: Chunk[] | null;
	images: ExtractedImage[] | null;
	elements?: Element[] | null;
	keywords?: ExtractedKeyword[] | null;
	djotContent?: DjotContent | null;

	/**
	 * Get the page count from this extraction result.
	 *
	 * Returns the total number of pages/slides/sheets detected
	 * in the original document.
	 *
	 * @returns The page count (>= 0), or null if not available
	 *
	 * @example
	 * ```typescript
	 * const result = await extractFile('document.pdf');
	 * const pageCount = result.getPageCount();
	 * if (pageCount !== null) {
	 *   console.log(`Document has ${pageCount} pages`);
	 * }
	 * ```
	 */
	getPageCount(): number | null;

	/**
	 * Get the chunk count from this extraction result.
	 *
	 * Returns the number of text chunks when chunking is enabled,
	 * or null if chunking was not performed or information is unavailable.
	 *
	 * @returns The chunk count (>= 0), or null if not available
	 *
	 * @example
	 * ```typescript
	 * const result = await extractFile('document.pdf', {
	 *   chunking: { enabled: true, maxChars: 1024 }
	 * });
	 * const chunkCount = result.getChunkCount();
	 * if (chunkCount !== null) {
	 *   console.log(`Document has ${chunkCount} chunks`);
	 * }
	 * ```
	 */
	getChunkCount(): number | null;

	/**
	 * Get the detected language from this extraction result.
	 *
	 * Returns the primary detected language as an ISO 639 language code
	 * (e.g., "en", "de", "fr"). If multiple languages were detected,
	 * returns the primary one.
	 *
	 * @returns The language code (e.g., "en"), or null if not detected
	 *
	 * @example
	 * ```typescript
	 * const result = await extractFile('document.pdf');
	 * const language = result.getDetectedLanguage();
	 * if (language) {
	 *   console.log(`Detected language: ${language}`);
	 * }
	 * ```
	 */
	getDetectedLanguage(): string | null;

	/**
	 * Get a metadata field from this extraction result.
	 *
	 * Retrieves a metadata field value. Supports nested fields with dot notation
	 * (e.g., "format.pages", "author").
	 *
	 * @param fieldName - The metadata field name or path to retrieve
	 * @returns The field value (parsed from JSON), or null if not found
	 *
	 * @example
	 * ```typescript
	 * const result = await extractFile('document.pdf');
	 *
	 * // Get simple field
	 * const title = result.getMetadataField('title') as string | null;
	 * if (title) {
	 *   console.log(`Title: ${title}`);
	 * }
	 *
	 * // Get nested field
	 * const pageCount = result.getMetadataField('format.pages') as number | null;
	 * if (pageCount !== null) {
	 *   console.log(`Pages: ${pageCount}`);
	 * }
	 * ```
	 */
	getMetadataField(fieldName: string): unknown;
}

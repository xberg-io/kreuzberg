/**
 * Kreuzberg - Multi-language document intelligence framework.
 *
 * This is a TypeScript SDK around a high-performance Rust core.
 * All extraction logic, chunking, quality processing, and language detection
 * are implemented in Rust for maximum performance.
 *
 * ## Module Organization
 *
 * The SDK is organized into logical domains:
 * - **Extraction**: Single and batch document extraction with worker pool support
 * - **Types**: Core type definitions and interfaces
 * - **Errors**: Error classes and diagnostic utilities
 * - **Plugins**: Custom post-processors, validators, and OCR backends
 * - **Registry**: Plugin and document extractor management
 * - **Config**: Configuration loading and management
 * - **MIME**: MIME type detection and validation
 * - **Embeddings**: Embedding model presets
 *
 * ## API Usage Recommendations
 *
 * **For processing multiple documents**, prefer batch APIs:
 * - Use `batchExtractFiles()` / `batchExtractFilesSync()` for multiple files
 * - Use `batchExtractBytes()` / `batchExtractBytesSync()` for multiple byte arrays
 * - Use worker pool APIs for high-concurrency scenarios
 *
 * **Batch APIs provide**:
 * - Better performance (parallel processing in Rust)
 * - More reliable memory management
 * - Recommended for all multi-document workflows
 *
 * **Single extraction APIs** (`extractFile`, `extractBytes`) are suitable for:
 * - One-off document processing
 * - Interactive applications processing documents on-demand
 * - Avoid calling these in tight loops - use batch APIs instead
 *
 * ## Supported Formats
 *
 * - **Documents**: PDF, DOCX, PPTX, XLSX, DOC, PPT (with LibreOffice)
 * - **Text**: Markdown, Plain Text, XML
 * - **Web**: HTML (converted to Markdown)
 * - **Data**: JSON, YAML, TOML
 * - **Email**: EML, MSG
 * - **Images**: PNG, JPEG, TIFF (with OCR support)
 *
 * @example
 * ```typescript
 * import { extractFile, batchExtractFiles } from '@kreuzberg/node';
 *
 * // Single file extraction
 * const result = await extractFile('document.pdf');
 * console.log(result.content);
 *
 * // Multiple files (recommended approach)
 * const files = ['doc1.pdf', 'doc2.docx', 'doc3.xlsx'];
 * const results = await batchExtractFiles(files);
 * results.forEach(r => console.log(r.content));
 * ```
 *
 * @module @kreuzberg/node
 */

// ============================================================================
// Types
// ============================================================================

export type {
	Chunk,
	ChunkingConfig,
	ErrorClassification,
	ExtractedImage,
	ExtractionConfig,
	ExtractionResult,
	HtmlConversionOptions,
	HtmlPreprocessingOptions,
	ImageExtractionConfig,
	KeywordConfig,
	LanguageDetectionConfig,
	OcrBackendProtocol,
	OcrConfig,
	PageContent,
	PageExtractionConfig,
	PdfConfig,
	PostProcessorConfig,
	PostProcessorProtocol,
	Table,
	TesseractConfig,
	TokenReductionConfig,
	ValidatorProtocol,
	WorkerPool,
	WorkerPoolStats,
} from "./types.js";

// ============================================================================
// Errors and Error Handling
// ============================================================================

export { ErrorCode, KreuzbergError } from "./errors.js";

export {
	getLastErrorCode,
	getLastPanicContext,
	getErrorCodeName,
	getErrorCodeDescription,
	classifyError,
} from "./errors/diagnostics.js";

export type { PanicContext } from "./errors.js";

// ============================================================================
// Core Extraction APIs
// ============================================================================

export {
	extractFileSync,
	extractFile,
	extractBytesSync,
	extractBytes,
} from "./extraction/single.js";

export {
	batchExtractFilesSync,
	batchExtractFiles,
	batchExtractBytesSync,
	batchExtractBytes,
} from "./extraction/batch.js";

// ============================================================================
// Worker Pool APIs
// ============================================================================

export {
	createWorkerPool,
	getWorkerPoolStats,
	extractFileInWorker,
	batchExtractFilesInWorker,
	closeWorkerPool,
} from "./extraction/worker-pool.js";

// ============================================================================
// Plugin System: Post-Processors
// ============================================================================

export {
	registerPostProcessor,
	unregisterPostProcessor,
	clearPostProcessors,
	listPostProcessors,
} from "./plugins/post-processors.js";

// ============================================================================
// Plugin System: Validators
// ============================================================================

export {
	registerValidator,
	unregisterValidator,
	clearValidators,
	listValidators,
} from "./plugins/validators.js";

// ============================================================================
// Plugin System: OCR Backends
// ============================================================================

export {
	registerOcrBackend,
	unregisterOcrBackend,
	clearOcrBackends,
	listOcrBackends,
} from "./plugins/ocr-backends.js";

export { GutenOcrBackend } from "./ocr/guten-ocr.js";

// ============================================================================
// Registry: Document Extractors
// ============================================================================

export {
	listDocumentExtractors,
	unregisterDocumentExtractor,
	clearDocumentExtractors,
} from "./registry/document-extractors.js";

// ============================================================================
// Configuration
// ============================================================================

export * from "./config/loader.js";

// ============================================================================
// MIME Type Utilities
// ============================================================================

export {
	detectMimeType,
	detectMimeTypeFromPath,
	validateMimeType,
	getExtensionsForMime,
} from "./mime/utilities.js";

// ============================================================================
// Embeddings
// ============================================================================

export {
	listEmbeddingPresets,
	getEmbeddingPreset,
} from "./embeddings/presets.js";

export type { EmbeddingPreset } from "./embeddings/presets.js";

// ============================================================================
// Version
// ============================================================================

export const __version__ = "4.0.8";

extract_bytes <- function(content, mime_type, config) .Call("wrap__extract_bytes", content, mime_type, config, PACKAGE = "kreuzberg")

# nolint start


extract_file <- function(path, mime_type, config) .Call("wrap__extract_file", path, mime_type, config, PACKAGE = "kreuzberg")

extract_file_sync <- function(path, mime_type, config) .Call("wrap__extract_file_sync", path, mime_type, config, PACKAGE = "kreuzberg")

extract_bytes_sync <- function(content, mime_type, config) .Call("wrap__extract_bytes_sync", content, mime_type, config, PACKAGE = "kreuzberg")

batch_extract_files_sync <- function(items, config) .Call("wrap__batch_extract_files_sync", items, config, PACKAGE = "kreuzberg")

batch_extract_bytes_sync <- function(items, config) .Call("wrap__batch_extract_bytes_sync", items, config, PACKAGE = "kreuzberg")

batch_extract_files <- function(items, config) .Call("wrap__batch_extract_files", items, config, PACKAGE = "kreuzberg")

batch_extract_bytes <- function(items, config) .Call("wrap__batch_extract_bytes", items, config, PACKAGE = "kreuzberg")

detect_mime_type_from_bytes <- function(content) .Call("wrap__detect_mime_type_from_bytes", content, PACKAGE = "kreuzberg")

get_extensions_for_mime <- function(mime_type) .Call("wrap__get_extensions_for_mime", mime_type, PACKAGE = "kreuzberg")

list_document_extractors <- function() .Call("wrap__list_document_extractors", PACKAGE = "kreuzberg")

list_ocr_backends <- function() .Call("wrap__list_ocr_backends", PACKAGE = "kreuzberg")

clear_ocr_backends <- function() .Call("wrap__clear_ocr_backends", PACKAGE = "kreuzberg")

list_post_processors <- function() .Call("wrap__list_post_processors", PACKAGE = "kreuzberg")

clear_post_processors <- function() .Call("wrap__clear_post_processors", PACKAGE = "kreuzberg")

list_validators <- function() .Call("wrap__list_validators", PACKAGE = "kreuzberg")

clear_validators <- function() .Call("wrap__clear_validators", PACKAGE = "kreuzberg")

embed_texts_async <- function(texts, config) .Call("wrap__embed_texts_async", texts, config, PACKAGE = "kreuzberg")

render_pdf_page_to_png <- function(pdf_bytes, page_index, dpi, password) .Call("wrap__render_pdf_page_to_png", pdf_bytes, page_index, dpi, password, PACKAGE = "kreuzberg")

detect_mime_type <- function(path, check_exists) .Call("wrap__detect_mime_type", path, check_exists, PACKAGE = "kreuzberg")

embed_texts <- function(texts, config) .Call("wrap__embed_texts", texts, config, PACKAGE = "kreuzberg")

get_embedding_preset <- function(name) .Call("wrap__get_embedding_preset", name, PACKAGE = "kreuzberg")

list_embedding_presets <- function() .Call("wrap__list_embedding_presets", PACKAGE = "kreuzberg")

AccelerationConfig <- new.env(parent = emptyenv())

#' @export
`$.AccelerationConfig` <- function(self, name) {
  func <- AccelerationConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.AccelerationConfig` <- `$.AccelerationConfig`

ContentFilterConfig <- new.env(parent = emptyenv())

ContentFilterConfig$default <- function() .Call("wrap__ContentFilterConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.ContentFilterConfig` <- function(self, name) {
  func <- ContentFilterConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ContentFilterConfig` <- `$.ContentFilterConfig`

EmailConfig <- new.env(parent = emptyenv())

#' @export
`$.EmailConfig` <- function(self, name) {
  func <- EmailConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.EmailConfig` <- `$.EmailConfig`

ExtractionConfig <- new.env(parent = emptyenv())

ExtractionConfig$needs_image_processing <- function() .Call("wrap__ExtractionConfig__needs_image_processing", self, PACKAGE = "kreuzberg")

ExtractionConfig$default <- function() .Call("wrap__ExtractionConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.ExtractionConfig` <- function(self, name) {
  func <- ExtractionConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ExtractionConfig` <- `$.ExtractionConfig`

FileExtractionConfig <- new.env(parent = emptyenv())

#' @export
`$.FileExtractionConfig` <- function(self, name) {
  func <- FileExtractionConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.FileExtractionConfig` <- `$.FileExtractionConfig`

BatchBytesItem <- new.env(parent = emptyenv())

#' @export
`$.BatchBytesItem` <- function(self, name) {
  func <- BatchBytesItem[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.BatchBytesItem` <- `$.BatchBytesItem`

BatchFileItem <- new.env(parent = emptyenv())

#' @export
`$.BatchFileItem` <- function(self, name) {
  func <- BatchFileItem[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.BatchFileItem` <- `$.BatchFileItem`

ImageExtractionConfig <- new.env(parent = emptyenv())

ImageExtractionConfig$default <- function() .Call("wrap__ImageExtractionConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.ImageExtractionConfig` <- function(self, name) {
  func <- ImageExtractionConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ImageExtractionConfig` <- `$.ImageExtractionConfig`

TokenReductionOptions <- new.env(parent = emptyenv())

TokenReductionOptions$default <- function() .Call("wrap__TokenReductionOptions__default", PACKAGE = "kreuzberg")

#' @export
`$.TokenReductionOptions` <- function(self, name) {
  func <- TokenReductionOptions[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.TokenReductionOptions` <- `$.TokenReductionOptions`

LanguageDetectionConfig <- new.env(parent = emptyenv())

LanguageDetectionConfig$default <- function() .Call("wrap__LanguageDetectionConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.LanguageDetectionConfig` <- function(self, name) {
  func <- LanguageDetectionConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.LanguageDetectionConfig` <- `$.LanguageDetectionConfig`

HtmlOutputConfig <- new.env(parent = emptyenv())

HtmlOutputConfig$default <- function() .Call("wrap__HtmlOutputConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.HtmlOutputConfig` <- function(self, name) {
  func <- HtmlOutputConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.HtmlOutputConfig` <- `$.HtmlOutputConfig`

LayoutDetectionConfig <- new.env(parent = emptyenv())

LayoutDetectionConfig$default <- function() .Call("wrap__LayoutDetectionConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.LayoutDetectionConfig` <- function(self, name) {
  func <- LayoutDetectionConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.LayoutDetectionConfig` <- `$.LayoutDetectionConfig`

LlmConfig <- new.env(parent = emptyenv())

#' @export
`$.LlmConfig` <- function(self, name) {
  func <- LlmConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.LlmConfig` <- `$.LlmConfig`

StructuredExtractionConfig <- new.env(parent = emptyenv())

#' @export
`$.StructuredExtractionConfig` <- function(self, name) {
  func <- StructuredExtractionConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.StructuredExtractionConfig` <- `$.StructuredExtractionConfig`

OcrQualityThresholds <- new.env(parent = emptyenv())

OcrQualityThresholds$default <- function() .Call("wrap__OcrQualityThresholds__default", PACKAGE = "kreuzberg")

#' @export
`$.OcrQualityThresholds` <- function(self, name) {
  func <- OcrQualityThresholds[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OcrQualityThresholds` <- `$.OcrQualityThresholds`

OcrPipelineStage <- new.env(parent = emptyenv())

#' @export
`$.OcrPipelineStage` <- function(self, name) {
  func <- OcrPipelineStage[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OcrPipelineStage` <- `$.OcrPipelineStage`

OcrConfig <- new.env(parent = emptyenv())

OcrConfig$default <- function() .Call("wrap__OcrConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.OcrConfig` <- function(self, name) {
  func <- OcrConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OcrConfig` <- `$.OcrConfig`

PageConfig <- new.env(parent = emptyenv())

PageConfig$default <- function() .Call("wrap__PageConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.PageConfig` <- function(self, name) {
  func <- PageConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PageConfig` <- `$.PageConfig`

PdfConfig <- new.env(parent = emptyenv())

PdfConfig$default <- function() .Call("wrap__PdfConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.PdfConfig` <- function(self, name) {
  func <- PdfConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PdfConfig` <- `$.PdfConfig`

HierarchyConfig <- new.env(parent = emptyenv())

HierarchyConfig$default <- function() .Call("wrap__HierarchyConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.HierarchyConfig` <- function(self, name) {
  func <- HierarchyConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.HierarchyConfig` <- `$.HierarchyConfig`

PostProcessorConfig <- new.env(parent = emptyenv())

PostProcessorConfig$default <- function() .Call("wrap__PostProcessorConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.PostProcessorConfig` <- function(self, name) {
  func <- PostProcessorConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PostProcessorConfig` <- `$.PostProcessorConfig`

ChunkingConfig <- new.env(parent = emptyenv())

ChunkingConfig$default <- function() .Call("wrap__ChunkingConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.ChunkingConfig` <- function(self, name) {
  func <- ChunkingConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ChunkingConfig` <- `$.ChunkingConfig`

EmbeddingConfig <- new.env(parent = emptyenv())

EmbeddingConfig$default <- function() .Call("wrap__EmbeddingConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.EmbeddingConfig` <- function(self, name) {
  func <- EmbeddingConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.EmbeddingConfig` <- `$.EmbeddingConfig`

TreeSitterConfig <- new.env(parent = emptyenv())

TreeSitterConfig$default <- function() .Call("wrap__TreeSitterConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.TreeSitterConfig` <- function(self, name) {
  func <- TreeSitterConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.TreeSitterConfig` <- `$.TreeSitterConfig`

TreeSitterProcessConfig <- new.env(parent = emptyenv())

TreeSitterProcessConfig$default <- function() .Call("wrap__TreeSitterProcessConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.TreeSitterProcessConfig` <- function(self, name) {
  func <- TreeSitterProcessConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.TreeSitterProcessConfig` <- `$.TreeSitterProcessConfig`

SupportedFormat <- new.env(parent = emptyenv())

#' @export
`$.SupportedFormat` <- function(self, name) {
  func <- SupportedFormat[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.SupportedFormat` <- `$.SupportedFormat`

ServerConfig <- new.env(parent = emptyenv())

ServerConfig$listen_addr <- function() .Call("wrap__ServerConfig__listen_addr", self, PACKAGE = "kreuzberg")

ServerConfig$cors_allows_all <- function() .Call("wrap__ServerConfig__cors_allows_all", self, PACKAGE = "kreuzberg")

ServerConfig$is_origin_allowed <- function(origin) .Call("wrap__ServerConfig__is_origin_allowed", self, origin, PACKAGE = "kreuzberg")

ServerConfig$max_request_body_mb <- function() .Call("wrap__ServerConfig__max_request_body_mb", self, PACKAGE = "kreuzberg")

ServerConfig$max_multipart_field_mb <- function() .Call("wrap__ServerConfig__max_multipart_field_mb", self, PACKAGE = "kreuzberg")

ServerConfig$default <- function() .Call("wrap__ServerConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.ServerConfig` <- function(self, name) {
  func <- ServerConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ServerConfig` <- `$.ServerConfig`

StructuredDataResult <- new.env(parent = emptyenv())

#' @export
`$.StructuredDataResult` <- function(self, name) {
  func <- StructuredDataResult[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.StructuredDataResult` <- `$.StructuredDataResult`

StreamReader <- new.env(parent = emptyenv())

#' @export
`$.StreamReader` <- function(self, name) {
  func <- StreamReader[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.StreamReader` <- `$.StreamReader`

ExtractedInlineImage <- new.env(parent = emptyenv())

#' @export
`$.ExtractedInlineImage` <- function(self, name) {
  func <- ExtractedInlineImage[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ExtractedInlineImage` <- `$.ExtractedInlineImage`

Drawing <- new.env(parent = emptyenv())

#' @export
`$.Drawing` <- function(self, name) {
  func <- Drawing[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.Drawing` <- `$.Drawing`

AnchorProperties <- new.env(parent = emptyenv())

#' @export
`$.AnchorProperties` <- function(self, name) {
  func <- AnchorProperties[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.AnchorProperties` <- `$.AnchorProperties`

HeaderFooter <- new.env(parent = emptyenv())

#' @export
`$.HeaderFooter` <- function(self, name) {
  func <- HeaderFooter[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.HeaderFooter` <- `$.HeaderFooter`

Note <- new.env(parent = emptyenv())

#' @export
`$.Note` <- function(self, name) {
  func <- Note[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.Note` <- `$.Note`

PageMarginsPoints <- new.env(parent = emptyenv())

#' @export
`$.PageMarginsPoints` <- function(self, name) {
  func <- PageMarginsPoints[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PageMarginsPoints` <- `$.PageMarginsPoints`

StyleDefinition <- new.env(parent = emptyenv())

#' @export
`$.StyleDefinition` <- function(self, name) {
  func <- StyleDefinition[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.StyleDefinition` <- `$.StyleDefinition`

ResolvedStyle <- new.env(parent = emptyenv())

#' @export
`$.ResolvedStyle` <- function(self, name) {
  func <- ResolvedStyle[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ResolvedStyle` <- `$.ResolvedStyle`

TableProperties <- new.env(parent = emptyenv())

#' @export
`$.TableProperties` <- function(self, name) {
  func <- TableProperties[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.TableProperties` <- `$.TableProperties`

XlsxAppProperties <- new.env(parent = emptyenv())

#' @export
`$.XlsxAppProperties` <- function(self, name) {
  func <- XlsxAppProperties[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.XlsxAppProperties` <- `$.XlsxAppProperties`

PptxAppProperties <- new.env(parent = emptyenv())

#' @export
`$.PptxAppProperties` <- function(self, name) {
  func <- PptxAppProperties[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PptxAppProperties` <- `$.PptxAppProperties`

CustomProperties <- new.env(parent = emptyenv())

#' @export
`$.CustomProperties` <- function(self, name) {
  func <- CustomProperties[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.CustomProperties` <- `$.CustomProperties`

OdtProperties <- new.env(parent = emptyenv())

#' @export
`$.OdtProperties` <- function(self, name) {
  func <- OdtProperties[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OdtProperties` <- `$.OdtProperties`

TokenReductionConfig <- new.env(parent = emptyenv())

TokenReductionConfig$default <- function() .Call("wrap__TokenReductionConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.TokenReductionConfig` <- function(self, name) {
  func <- TokenReductionConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.TokenReductionConfig` <- `$.TokenReductionConfig`

PdfAnnotation <- new.env(parent = emptyenv())

#' @export
`$.PdfAnnotation` <- function(self, name) {
  func <- PdfAnnotation[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PdfAnnotation` <- `$.PdfAnnotation`

InlineElement <- new.env(parent = emptyenv())

#' @export
`$.InlineElement` <- function(self, name) {
  func <- InlineElement[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.InlineElement` <- `$.InlineElement`

DjotImage <- new.env(parent = emptyenv())

#' @export
`$.DjotImage` <- function(self, name) {
  func <- DjotImage[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.DjotImage` <- `$.DjotImage`

DjotLink <- new.env(parent = emptyenv())

#' @export
`$.DjotLink` <- function(self, name) {
  func <- DjotLink[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.DjotLink` <- `$.DjotLink`

DocumentRelationship <- new.env(parent = emptyenv())

#' @export
`$.DocumentRelationship` <- function(self, name) {
  func <- DocumentRelationship[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.DocumentRelationship` <- `$.DocumentRelationship`

GridCell <- new.env(parent = emptyenv())

#' @export
`$.GridCell` <- function(self, name) {
  func <- GridCell[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.GridCell` <- `$.GridCell`

TextAnnotation <- new.env(parent = emptyenv())

#' @export
`$.TextAnnotation` <- function(self, name) {
  func <- TextAnnotation[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.TextAnnotation` <- `$.TextAnnotation`

ArchiveEntry <- new.env(parent = emptyenv())

#' @export
`$.ArchiveEntry` <- function(self, name) {
  func <- ArchiveEntry[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ArchiveEntry` <- `$.ArchiveEntry`

ProcessingWarning <- new.env(parent = emptyenv())

#' @export
`$.ProcessingWarning` <- function(self, name) {
  func <- ProcessingWarning[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ProcessingWarning` <- `$.ProcessingWarning`

LlmUsage <- new.env(parent = emptyenv())

#' @export
`$.LlmUsage` <- function(self, name) {
  func <- LlmUsage[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.LlmUsage` <- `$.LlmUsage`

Chunk <- new.env(parent = emptyenv())

#' @export
`$.Chunk` <- function(self, name) {
  func <- Chunk[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.Chunk` <- `$.Chunk`

HeadingLevel <- new.env(parent = emptyenv())

#' @export
`$.HeadingLevel` <- function(self, name) {
  func <- HeadingLevel[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.HeadingLevel` <- `$.HeadingLevel`

ChunkMetadata <- new.env(parent = emptyenv())

#' @export
`$.ChunkMetadata` <- function(self, name) {
  func <- ChunkMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ChunkMetadata` <- `$.ChunkMetadata`

ExtractedImage <- new.env(parent = emptyenv())

#' @export
`$.ExtractedImage` <- function(self, name) {
  func <- ExtractedImage[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ExtractedImage` <- `$.ExtractedImage`

ElementMetadata <- new.env(parent = emptyenv())

#' @export
`$.ElementMetadata` <- function(self, name) {
  func <- ElementMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ElementMetadata` <- `$.ElementMetadata`

Element <- new.env(parent = emptyenv())

#' @export
`$.Element` <- function(self, name) {
  func <- Element[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.Element` <- `$.Element`

ExcelSheet <- new.env(parent = emptyenv())

#' @export
`$.ExcelSheet` <- function(self, name) {
  func <- ExcelSheet[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ExcelSheet` <- `$.ExcelSheet`

XmlExtractionResult <- new.env(parent = emptyenv())

#' @export
`$.XmlExtractionResult` <- function(self, name) {
  func <- XmlExtractionResult[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.XmlExtractionResult` <- `$.XmlExtractionResult`

TextExtractionResult <- new.env(parent = emptyenv())

#' @export
`$.TextExtractionResult` <- function(self, name) {
  func <- TextExtractionResult[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.TextExtractionResult` <- `$.TextExtractionResult`

EmailAttachment <- new.env(parent = emptyenv())

#' @export
`$.EmailAttachment` <- function(self, name) {
  func <- EmailAttachment[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.EmailAttachment` <- `$.EmailAttachment`

OcrTable <- new.env(parent = emptyenv())

#' @export
`$.OcrTable` <- function(self, name) {
  func <- OcrTable[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OcrTable` <- `$.OcrTable`

OcrTableBoundingBox <- new.env(parent = emptyenv())

#' @export
`$.OcrTableBoundingBox` <- function(self, name) {
  func <- OcrTableBoundingBox[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OcrTableBoundingBox` <- `$.OcrTableBoundingBox`

ImagePreprocessingConfig <- new.env(parent = emptyenv())

ImagePreprocessingConfig$default <- function() .Call("wrap__ImagePreprocessingConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.ImagePreprocessingConfig` <- function(self, name) {
  func <- ImagePreprocessingConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ImagePreprocessingConfig` <- `$.ImagePreprocessingConfig`

TesseractConfig <- new.env(parent = emptyenv())

TesseractConfig$default <- function() .Call("wrap__TesseractConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.TesseractConfig` <- function(self, name) {
  func <- TesseractConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.TesseractConfig` <- `$.TesseractConfig`

ImagePreprocessingMetadata <- new.env(parent = emptyenv())

#' @export
`$.ImagePreprocessingMetadata` <- function(self, name) {
  func <- ImagePreprocessingMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ImagePreprocessingMetadata` <- `$.ImagePreprocessingMetadata`

Metadata <- new.env(parent = emptyenv())

Metadata$is_empty <- function() .Call("wrap__Metadata__is_empty", self, PACKAGE = "kreuzberg")

#' @export
`$.Metadata` <- function(self, name) {
  func <- Metadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.Metadata` <- `$.Metadata`

ExcelMetadata <- new.env(parent = emptyenv())

#' @export
`$.ExcelMetadata` <- function(self, name) {
  func <- ExcelMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ExcelMetadata` <- `$.ExcelMetadata`

EmailMetadata <- new.env(parent = emptyenv())

#' @export
`$.EmailMetadata` <- function(self, name) {
  func <- EmailMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.EmailMetadata` <- `$.EmailMetadata`

ArchiveMetadata <- new.env(parent = emptyenv())

#' @export
`$.ArchiveMetadata` <- function(self, name) {
  func <- ArchiveMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ArchiveMetadata` <- `$.ArchiveMetadata`

XmlMetadata <- new.env(parent = emptyenv())

#' @export
`$.XmlMetadata` <- function(self, name) {
  func <- XmlMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.XmlMetadata` <- `$.XmlMetadata`

TextMetadata <- new.env(parent = emptyenv())

#' @export
`$.TextMetadata` <- function(self, name) {
  func <- TextMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.TextMetadata` <- `$.TextMetadata`

HeaderMetadata <- new.env(parent = emptyenv())

#' @export
`$.HeaderMetadata` <- function(self, name) {
  func <- HeaderMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.HeaderMetadata` <- `$.HeaderMetadata`

LinkMetadata <- new.env(parent = emptyenv())

#' @export
`$.LinkMetadata` <- function(self, name) {
  func <- LinkMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.LinkMetadata` <- `$.LinkMetadata`

ImageMetadataType <- new.env(parent = emptyenv())

#' @export
`$.ImageMetadataType` <- function(self, name) {
  func <- ImageMetadataType[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ImageMetadataType` <- `$.ImageMetadataType`

StructuredData <- new.env(parent = emptyenv())

#' @export
`$.StructuredData` <- function(self, name) {
  func <- StructuredData[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.StructuredData` <- `$.StructuredData`

OcrMetadata <- new.env(parent = emptyenv())

#' @export
`$.OcrMetadata` <- function(self, name) {
  func <- OcrMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OcrMetadata` <- `$.OcrMetadata`

ErrorMetadata <- new.env(parent = emptyenv())

#' @export
`$.ErrorMetadata` <- function(self, name) {
  func <- ErrorMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ErrorMetadata` <- `$.ErrorMetadata`

PptxMetadata <- new.env(parent = emptyenv())

#' @export
`$.PptxMetadata` <- function(self, name) {
  func <- PptxMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PptxMetadata` <- `$.PptxMetadata`

DocxMetadata <- new.env(parent = emptyenv())

#' @export
`$.DocxMetadata` <- function(self, name) {
  func <- DocxMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.DocxMetadata` <- `$.DocxMetadata`

CsvMetadata <- new.env(parent = emptyenv())

#' @export
`$.CsvMetadata` <- function(self, name) {
  func <- CsvMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.CsvMetadata` <- `$.CsvMetadata`

BibtexMetadata <- new.env(parent = emptyenv())

#' @export
`$.BibtexMetadata` <- function(self, name) {
  func <- BibtexMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.BibtexMetadata` <- `$.BibtexMetadata`

CitationMetadata <- new.env(parent = emptyenv())

#' @export
`$.CitationMetadata` <- function(self, name) {
  func <- CitationMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.CitationMetadata` <- `$.CitationMetadata`

YearRange <- new.env(parent = emptyenv())

#' @export
`$.YearRange` <- function(self, name) {
  func <- YearRange[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.YearRange` <- `$.YearRange`

FictionBookMetadata <- new.env(parent = emptyenv())

#' @export
`$.FictionBookMetadata` <- function(self, name) {
  func <- FictionBookMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.FictionBookMetadata` <- `$.FictionBookMetadata`

DbfFieldInfo <- new.env(parent = emptyenv())

#' @export
`$.DbfFieldInfo` <- function(self, name) {
  func <- DbfFieldInfo[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.DbfFieldInfo` <- `$.DbfFieldInfo`

ContributorRole <- new.env(parent = emptyenv())

#' @export
`$.ContributorRole` <- function(self, name) {
  func <- ContributorRole[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ContributorRole` <- `$.ContributorRole`

EpubMetadata <- new.env(parent = emptyenv())

#' @export
`$.EpubMetadata` <- function(self, name) {
  func <- EpubMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.EpubMetadata` <- `$.EpubMetadata`

PstMetadata <- new.env(parent = emptyenv())

#' @export
`$.PstMetadata` <- function(self, name) {
  func <- PstMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PstMetadata` <- `$.PstMetadata`

OcrConfidence <- new.env(parent = emptyenv())

#' @export
`$.OcrConfidence` <- function(self, name) {
  func <- OcrConfidence[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OcrConfidence` <- `$.OcrConfidence`

OcrRotation <- new.env(parent = emptyenv())

#' @export
`$.OcrRotation` <- function(self, name) {
  func <- OcrRotation[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OcrRotation` <- `$.OcrRotation`

OcrElement <- new.env(parent = emptyenv())

#' @export
`$.OcrElement` <- function(self, name) {
  func <- OcrElement[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OcrElement` <- `$.OcrElement`

OcrElementConfig <- new.env(parent = emptyenv())

#' @export
`$.OcrElementConfig` <- function(self, name) {
  func <- OcrElementConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OcrElementConfig` <- `$.OcrElementConfig`

PageBoundary <- new.env(parent = emptyenv())

#' @export
`$.PageBoundary` <- function(self, name) {
  func <- PageBoundary[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PageBoundary` <- `$.PageBoundary`

PageInfo <- new.env(parent = emptyenv())

#' @export
`$.PageInfo` <- function(self, name) {
  func <- PageInfo[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PageInfo` <- `$.PageInfo`

LayoutRegion <- new.env(parent = emptyenv())

#' @export
`$.LayoutRegion` <- function(self, name) {
  func <- LayoutRegion[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.LayoutRegion` <- `$.LayoutRegion`

HierarchicalBlock <- new.env(parent = emptyenv())

#' @export
`$.HierarchicalBlock` <- function(self, name) {
  func <- HierarchicalBlock[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.HierarchicalBlock` <- `$.HierarchicalBlock`

Uri <- new.env(parent = emptyenv())

#' @export
`$.Uri` <- function(self, name) {
  func <- Uri[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.Uri` <- `$.Uri`

StringBufferPool <- new.env(parent = emptyenv())

#' @export
`$.StringBufferPool` <- function(self, name) {
  func <- StringBufferPool[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.StringBufferPool` <- `$.StringBufferPool`

ByteBufferPool <- new.env(parent = emptyenv())

#' @export
`$.ByteBufferPool` <- function(self, name) {
  func <- ByteBufferPool[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ByteBufferPool` <- `$.ByteBufferPool`

InfoResponse <- new.env(parent = emptyenv())

#' @export
`$.InfoResponse` <- function(self, name) {
  func <- InfoResponse[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.InfoResponse` <- `$.InfoResponse`

EmbedRequest <- new.env(parent = emptyenv())

#' @export
`$.EmbedRequest` <- function(self, name) {
  func <- EmbedRequest[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.EmbedRequest` <- `$.EmbedRequest`

EmbedResponse <- new.env(parent = emptyenv())

#' @export
`$.EmbedResponse` <- function(self, name) {
  func <- EmbedResponse[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.EmbedResponse` <- `$.EmbedResponse`

ChunkRequest <- new.env(parent = emptyenv())

#' @export
`$.ChunkRequest` <- function(self, name) {
  func <- ChunkRequest[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ChunkRequest` <- `$.ChunkRequest`

ChunkResponse <- new.env(parent = emptyenv())

#' @export
`$.ChunkResponse` <- function(self, name) {
  func <- ChunkResponse[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ChunkResponse` <- `$.ChunkResponse`

DetectResponse <- new.env(parent = emptyenv())

#' @export
`$.DetectResponse` <- function(self, name) {
  func <- DetectResponse[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.DetectResponse` <- `$.DetectResponse`

ManifestEntryResponse <- new.env(parent = emptyenv())

#' @export
`$.ManifestEntryResponse` <- function(self, name) {
  func <- ManifestEntryResponse[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ManifestEntryResponse` <- `$.ManifestEntryResponse`

WarmResponse <- new.env(parent = emptyenv())

#' @export
`$.WarmResponse` <- function(self, name) {
  func <- WarmResponse[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.WarmResponse` <- `$.WarmResponse`

StructuredExtractionResponse <- new.env(parent = emptyenv())

#' @export
`$.StructuredExtractionResponse` <- function(self, name) {
  func <- StructuredExtractionResponse[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.StructuredExtractionResponse` <- `$.StructuredExtractionResponse`

OpenWebDocumentResponse <- new.env(parent = emptyenv())

#' @export
`$.OpenWebDocumentResponse` <- function(self, name) {
  func <- OpenWebDocumentResponse[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OpenWebDocumentResponse` <- `$.OpenWebDocumentResponse`

DoclingCompatResponse <- new.env(parent = emptyenv())

#' @export
`$.DoclingCompatResponse` <- function(self, name) {
  func <- DoclingCompatResponse[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.DoclingCompatResponse` <- `$.DoclingCompatResponse`

DetectMimeTypeParams <- new.env(parent = emptyenv())

#' @export
`$.DetectMimeTypeParams` <- function(self, name) {
  func <- DetectMimeTypeParams[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.DetectMimeTypeParams` <- `$.DetectMimeTypeParams`

CacheWarmParams <- new.env(parent = emptyenv())

#' @export
`$.CacheWarmParams` <- function(self, name) {
  func <- CacheWarmParams[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.CacheWarmParams` <- `$.CacheWarmParams`

EmbedTextParams <- new.env(parent = emptyenv())

#' @export
`$.EmbedTextParams` <- function(self, name) {
  func <- EmbedTextParams[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.EmbedTextParams` <- `$.EmbedTextParams`

ExtractStructuredParams <- new.env(parent = emptyenv())

#' @export
`$.ExtractStructuredParams` <- function(self, name) {
  func <- ExtractStructuredParams[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ExtractStructuredParams` <- `$.ExtractStructuredParams`

ChunkTextParams <- new.env(parent = emptyenv())

#' @export
`$.ChunkTextParams` <- function(self, name) {
  func <- ChunkTextParams[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ChunkTextParams` <- `$.ChunkTextParams`

DetectedBoundary <- new.env(parent = emptyenv())

#' @export
`$.DetectedBoundary` <- function(self, name) {
  func <- DetectedBoundary[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.DetectedBoundary` <- `$.DetectedBoundary`

MergedChunk <- new.env(parent = emptyenv())

#' @export
`$.MergedChunk` <- function(self, name) {
  func <- MergedChunk[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.MergedChunk` <- `$.MergedChunk`

EmbeddingPreset <- new.env(parent = emptyenv())

#' @export
`$.EmbeddingPreset` <- function(self, name) {
  func <- EmbeddingPreset[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.EmbeddingPreset` <- `$.EmbeddingPreset`

YakeParams <- new.env(parent = emptyenv())

YakeParams$default <- function() .Call("wrap__YakeParams__default", PACKAGE = "kreuzberg")

#' @export
`$.YakeParams` <- function(self, name) {
  func <- YakeParams[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.YakeParams` <- `$.YakeParams`

RakeParams <- new.env(parent = emptyenv())

RakeParams$default <- function() .Call("wrap__RakeParams__default", PACKAGE = "kreuzberg")

#' @export
`$.RakeParams` <- function(self, name) {
  func <- RakeParams[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.RakeParams` <- `$.RakeParams`

KeywordConfig <- new.env(parent = emptyenv())

KeywordConfig$default <- function() .Call("wrap__KeywordConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.KeywordConfig` <- function(self, name) {
  func <- KeywordConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.KeywordConfig` <- `$.KeywordConfig`

Keyword <- new.env(parent = emptyenv())

#' @export
`$.Keyword` <- function(self, name) {
  func <- Keyword[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.Keyword` <- `$.Keyword`

OcrCacheStats <- new.env(parent = emptyenv())

#' @export
`$.OcrCacheStats` <- function(self, name) {
  func <- OcrCacheStats[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OcrCacheStats` <- `$.OcrCacheStats`

RecognizedTable <- new.env(parent = emptyenv())

#' @export
`$.RecognizedTable` <- function(self, name) {
  func <- RecognizedTable[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.RecognizedTable` <- `$.RecognizedTable`

PaddleOcrConfig <- new.env(parent = emptyenv())

PaddleOcrConfig$with_cache_dir <- function(path) .Call("wrap__PaddleOcrConfig__with_cache_dir", self, path, PACKAGE = "kreuzberg")

PaddleOcrConfig$with_table_detection <- function(enable) .Call("wrap__PaddleOcrConfig__with_table_detection", self, enable, PACKAGE = "kreuzberg")

PaddleOcrConfig$with_angle_cls <- function(enable) .Call("wrap__PaddleOcrConfig__with_angle_cls", self, enable, PACKAGE = "kreuzberg")

PaddleOcrConfig$with_det_db_thresh <- function(threshold) .Call("wrap__PaddleOcrConfig__with_det_db_thresh", self, threshold, PACKAGE = "kreuzberg")

PaddleOcrConfig$with_det_db_box_thresh <- function(threshold) .Call("wrap__PaddleOcrConfig__with_det_db_box_thresh", self, threshold, PACKAGE = "kreuzberg")

PaddleOcrConfig$with_det_db_unclip_ratio <- function(ratio) .Call("wrap__PaddleOcrConfig__with_det_db_unclip_ratio", self, ratio, PACKAGE = "kreuzberg")

PaddleOcrConfig$with_det_limit_side_len <- function(length) .Call("wrap__PaddleOcrConfig__with_det_limit_side_len", self, length, PACKAGE = "kreuzberg")

PaddleOcrConfig$with_rec_batch_num <- function(batch_size) .Call("wrap__PaddleOcrConfig__with_rec_batch_num", self, batch_size, PACKAGE = "kreuzberg")

PaddleOcrConfig$with_drop_score <- function(score) .Call("wrap__PaddleOcrConfig__with_drop_score", self, score, PACKAGE = "kreuzberg")

PaddleOcrConfig$with_padding <- function(padding) .Call("wrap__PaddleOcrConfig__with_padding", self, padding, PACKAGE = "kreuzberg")

PaddleOcrConfig$with_model_tier <- function(tier) .Call("wrap__PaddleOcrConfig__with_model_tier", self, tier, PACKAGE = "kreuzberg")

PaddleOcrConfig$default <- function() .Call("wrap__PaddleOcrConfig__default", PACKAGE = "kreuzberg")

#' @export
`$.PaddleOcrConfig` <- function(self, name) {
  func <- PaddleOcrConfig[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PaddleOcrConfig` <- `$.PaddleOcrConfig`

ModelPaths <- new.env(parent = emptyenv())

#' @export
`$.ModelPaths` <- function(self, name) {
  func <- ModelPaths[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.ModelPaths` <- `$.ModelPaths`

OrientationResult <- new.env(parent = emptyenv())

#' @export
`$.OrientationResult` <- function(self, name) {
  func <- OrientationResult[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.OrientationResult` <- `$.OrientationResult`

BBox <- new.env(parent = emptyenv())

#' @export
`$.BBox` <- function(self, name) {
  func <- BBox[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.BBox` <- `$.BBox`

LayoutDetection <- new.env(parent = emptyenv())

#' @export
`$.LayoutDetection` <- function(self, name) {
  func <- LayoutDetection[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.LayoutDetection` <- `$.LayoutDetection`

EmbeddedFile <- new.env(parent = emptyenv())

#' @export
`$.EmbeddedFile` <- function(self, name) {
  func <- EmbeddedFile[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.EmbeddedFile` <- `$.EmbeddedFile`

PdfImage <- new.env(parent = emptyenv())

#' @export
`$.PdfImage` <- function(self, name) {
  func <- PdfImage[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PdfImage` <- `$.PdfImage`

PageLayoutResult <- new.env(parent = emptyenv())

#' @export
`$.PageLayoutResult` <- function(self, name) {
  func <- PageLayoutResult[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PageLayoutResult` <- `$.PageLayoutResult`

PageTiming <- new.env(parent = emptyenv())

#' @export
`$.PageTiming` <- function(self, name) {
  func <- PageTiming[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.PageTiming` <- `$.PageTiming`

CommonPdfMetadata <- new.env(parent = emptyenv())

#' @export
`$.CommonPdfMetadata` <- function(self, name) {
  func <- CommonPdfMetadata[[name]]
  environment(func) <- environment()
  func
}

#' @export
`[[.CommonPdfMetadata` <- `$.CommonPdfMetadata`


# nolint end

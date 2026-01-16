using System.Diagnostics.CodeAnalysis;
using System.Text.Json.Nodes;
using System.Text.Json.Serialization;

namespace Kreuzberg;

/// <summary>
/// The main result of document extraction containing extracted content, metadata, and structured data.
/// </summary>
public sealed class ExtractionResult
{
    /// <summary>
    /// The extracted text content from the document.
    /// </summary>
    [JsonPropertyName("content")]
    public string Content { get; set; } = string.Empty;

    /// <summary>
    /// The detected MIME type of the document (e.g., "application/pdf").
    /// </summary>
    [JsonPropertyName("mime_type")]
    public string MimeType { get; set; } = string.Empty;

    /// <summary>
    /// Document metadata including language, format-specific info, and other attributes.
    /// </summary>
    [JsonPropertyName("metadata")]
    public Metadata Metadata { get; set; } = new();

    /// <summary>
    /// Extracted tables from the document, if any.
    /// </summary>
    [JsonPropertyName("tables")]
    public List<Table> Tables { get; set; } = new();

    /// <summary>
    /// Detected languages in the document, if language detection was enabled.
    /// </summary>
    [JsonPropertyName("detected_languages")]
    public List<string>? DetectedLanguages { get; set; }

    /// <summary>
    /// Text chunks if chunking was enabled, each with metadata and optional embedding vector.
    /// </summary>
    [JsonPropertyName("chunks")]
    public List<Chunk>? Chunks { get; set; }

    /// <summary>
    /// Images extracted from the document, if image extraction was enabled.
    /// </summary>
    [JsonPropertyName("images")]
    public List<ExtractedImage>? Images { get; set; }

    /// <summary>
    /// Per-page extracted content when page extraction is enabled.
    /// </summary>
    [JsonPropertyName("pages")]
    public List<PageContent>? Pages { get; set; }

    /// <summary>
    /// Indicates whether extraction completed successfully.
    /// </summary>
    [JsonPropertyName("success")]
    public bool Success { get; set; }
}

/// <summary>
/// Extracted content for a single page when page extraction is enabled.
/// </summary>
public sealed class PageContent
{
    /// <summary>
    /// The page number (1-indexed).
    /// </summary>
    [JsonPropertyName("page_number")]
    public int PageNumber { get; set; }

    /// <summary>
    /// The extracted text content for this page.
    /// </summary>
    [JsonPropertyName("content")]
    public string Content { get; set; } = string.Empty;

    /// <summary>
    /// Tables extracted from this page, if any.
    /// </summary>
    [JsonPropertyName("tables")]
    public List<Table>? Tables { get; set; }

    /// <summary>
    /// Images extracted from this page, if any.
    /// </summary>
    [JsonPropertyName("images")]
    public List<ExtractedImage>? Images { get; set; }
}

/// <summary>
/// Represents a table extracted from a document.
/// </summary>
public sealed class Table
{
    /// <summary>
    /// Table cells arranged as rows (outer list) and columns (inner lists).
    /// </summary>
    [JsonPropertyName("cells")]
    public List<List<string>> Cells { get; set; } = new();

    /// <summary>
    /// Table representation in Markdown format.
    /// </summary>
    [JsonPropertyName("markdown")]
    public string Markdown { get; set; } = string.Empty;

    /// <summary>
    /// The page number (1-indexed) where this table appears in the document.
    /// </summary>
    [JsonPropertyName("page_number")]
    public int PageNumber { get; set; }
}

/// <summary>
/// A chunk of text from a document, used for splitting large documents into smaller pieces.
/// </summary>
public sealed class Chunk
{
    /// <summary>
    /// The text content of this chunk.
    /// </summary>
    [JsonPropertyName("content")]
    public string Content { get; set; } = string.Empty;

    /// <summary>
    /// Optional embedding vector for the chunk, if embedding was enabled.
    /// </summary>
    [JsonPropertyName("embedding")]
    public float[]? Embedding { get; set; }

    /// <summary>
    /// Metadata about the chunk including position and token count.
    /// </summary>
    [JsonPropertyName("metadata")]
    public ChunkMetadata Metadata { get; set; } = new();
}

/// <summary>
/// Metadata about a text chunk including position, token count, and page information.
/// </summary>
public sealed class ChunkMetadata
{
    /// <summary>
    /// Starting byte position of this chunk in the document.
    /// </summary>
    [JsonPropertyName("byte_start")]
    public long ByteStart { get; set; }

    /// <summary>
    /// Ending byte position of this chunk in the document.
    /// </summary>
    [JsonPropertyName("byte_end")]
    public long ByteEnd { get; set; }

    /// <summary>
    /// Token count for this chunk, if tokenization was enabled.
    /// </summary>
    [JsonPropertyName("token_count")]
    public int? TokenCount { get; set; }

    /// <summary>
    /// Zero-based index of this chunk among all chunks.
    /// </summary>
    [JsonPropertyName("chunk_index")]
    public int ChunkIndex { get; set; }

    /// <summary>
    /// Total number of chunks the document was split into.
    /// </summary>
    [JsonPropertyName("total_chunks")]
    public int TotalChunks { get; set; }

    /// <summary>
    /// Page number (1-indexed) of the first page this chunk starts on, if page tracking is enabled.
    /// </summary>
    [JsonPropertyName("first_page")]
    public int? FirstPage { get; set; }

    /// <summary>
    /// Page number (1-indexed) of the last page this chunk ends on, if page tracking is enabled.
    /// </summary>
    [JsonPropertyName("last_page")]
    public int? LastPage { get; set; }
}

/// <summary>
/// Represents an image extracted from a document with metadata and optional OCR results.
/// </summary>
public sealed class ExtractedImage
{
    /// <summary>
    /// Raw image data as bytes (PNG, JPEG, etc.).
    /// </summary>
    [JsonPropertyName("data")]
    public byte[] Data { get; set; } = Array.Empty<byte>();

    /// <summary>
    /// Image format (e.g., "PNG", "JPEG", "TIFF").
    /// </summary>
    [JsonPropertyName("format")]
    public string Format { get; set; } = string.Empty;

    /// <summary>
    /// Zero-based index of this image among all extracted images.
    /// </summary>
    [JsonPropertyName("image_index")]
    public int ImageIndex { get; set; }

    /// <summary>
    /// Page number (1-indexed) where this image appears, if page tracking is enabled.
    /// </summary>
    [JsonPropertyName("page_number")]
    public int? PageNumber { get; set; }

    /// <summary>
    /// Image width in pixels.
    /// </summary>
    [JsonPropertyName("width")]
    public uint? Width { get; set; }

    /// <summary>
    /// Image height in pixels.
    /// </summary>
    [JsonPropertyName("height")]
    public uint? Height { get; set; }

    /// <summary>
    /// Color space representation (e.g., "RGB", "CMYK", "DeviceGray").
    /// </summary>
    [JsonPropertyName("colorspace")]
    public string? Colorspace { get; set; }

    /// <summary>
    /// Bits per color component.
    /// </summary>
    [JsonPropertyName("bits_per_component")]
    public uint? BitsPerComponent { get; set; }

    /// <summary>
    /// Whether this image is a mask/transparency image.
    /// </summary>
    [JsonPropertyName("is_mask")]
    public bool IsMask { get; set; }

    /// <summary>
    /// Optional description or alternate text for the image.
    /// </summary>
    [JsonPropertyName("description")]
    public string? Description { get; set; }

    /// <summary>
    /// OCR extraction result if OCR was applied to this image.
    /// </summary>
    [JsonPropertyName("ocr_result")]
    public ExtractionResult? OcrResult { get; set; }
}

public enum FormatType
{
    Unknown,
    [JsonPropertyName("pdf")]
    Pdf,
    [JsonPropertyName("excel")]
    Excel,
    [JsonPropertyName("email")]
    Email,
    [JsonPropertyName("pptx")]
    Pptx,
    [JsonPropertyName("archive")]
    Archive,
    [JsonPropertyName("image")]
    Image,
    [JsonPropertyName("xml")]
    Xml,
    [JsonPropertyName("text")]
    Text,
    [JsonPropertyName("html")]
    Html,
    [JsonPropertyName("ocr")]
    Ocr,
}

/// <summary>
/// Container for format-specific metadata based on the document type.
/// </summary>
public sealed class FormatMetadata
{
    /// <summary>
    /// The detected document format type.
    /// </summary>
    public FormatType Type { get; set; } = FormatType.Unknown;

    /// <summary>
    /// PDF-specific metadata (if Type is Pdf).
    /// </summary>
    public PdfMetadata? Pdf { get; set; }

    /// <summary>
    /// Excel-specific metadata (if Type is Excel).
    /// </summary>
    public ExcelMetadata? Excel { get; set; }

    /// <summary>
    /// Email-specific metadata (if Type is Email).
    /// </summary>
    public EmailMetadata? Email { get; set; }

    /// <summary>
    /// PowerPoint-specific metadata (if Type is Pptx).
    /// </summary>
    public PptxMetadata? Pptx { get; set; }

    /// <summary>
    /// Archive-specific metadata (if Type is Archive).
    /// </summary>
    public ArchiveMetadata? Archive { get; set; }

    /// <summary>
    /// Image-specific metadata (if Type is Image).
    /// </summary>
    public ImageMetadata? Image { get; set; }

    /// <summary>
    /// XML-specific metadata (if Type is Xml).
    /// </summary>
    public XmlMetadata? Xml { get; set; }

    /// <summary>
    /// Plain text-specific metadata (if Type is Text).
    /// </summary>
    public TextMetadata? Text { get; set; }

    /// <summary>
    /// HTML-specific metadata (if Type is Html).
    /// </summary>
    public HtmlMetadata? Html { get; set; }

    /// <summary>
    /// OCR-specific metadata (if Type is Ocr).
    /// </summary>
    public OcrMetadata? Ocr { get; set; }
}

public sealed class Metadata
{
    [JsonPropertyName("language")]
    public string? Language { get; set; }

    [JsonPropertyName("date")]
    public string? Date { get; set; }

    [JsonPropertyName("subject")]
    public string? Subject { get; set; }

    [JsonPropertyName("format_type")]
    public FormatType FormatType { get; set; } = FormatType.Unknown;

    [JsonPropertyName("format")]
    public FormatMetadata Format { get; set; } = new();

    [JsonPropertyName("image_preprocessing")]
    public ImagePreprocessingMetadata? ImagePreprocessing { get; set; }

    [JsonPropertyName("json_schema")]
    public JsonNode? JsonSchema { get; set; }

    [JsonPropertyName("error")]
    public ErrorMetadata? Error { get; set; }

    [JsonPropertyName("pages")]
    public PageStructure? Pages { get; set; }

    /// <summary>
    /// Extracted keywords from keyword extraction algorithms (YAKE, RAKE).
    /// These are distinct from format-specific keywords (like HTML meta keywords).
    /// </summary>
    [JsonPropertyName("keywords")]
    public List<ExtractedKeyword>? Keywords { get; set; }

    [JsonExtensionData]
    public JsonObject? Additional { get; set; }
}

public sealed class ImagePreprocessingMetadata
{
    [JsonPropertyName("original_dimensions")]
    public int[]? OriginalDimensions { get; set; }

    [JsonPropertyName("original_dpi")]
    public double[]? OriginalDpi { get; set; }

    [JsonPropertyName("target_dpi")]
    public int TargetDpi { get; set; }

    [JsonPropertyName("scale_factor")]
    public double ScaleFactor { get; set; }

    [JsonPropertyName("auto_adjusted")]
    public bool AutoAdjusted { get; set; }

    [JsonPropertyName("final_dpi")]
    public int FinalDpi { get; set; }

    [JsonPropertyName("new_dimensions")]
    public int[]? NewDimensions { get; set; }

    [JsonPropertyName("resample_method")]
    public string? ResampleMethod { get; set; }

    [JsonPropertyName("dimension_clamped")]
    public bool DimensionClamped { get; set; }

    [JsonPropertyName("calculated_dpi")]
    public int? CalculatedDpi { get; set; }

    [JsonPropertyName("skipped_resize")]
    public bool SkippedResize { get; set; }

    [JsonPropertyName("resize_error")]
    public string? ResizeError { get; set; }
}

public sealed class ErrorMetadata
{
    [JsonPropertyName("error_type")]
    public string ErrorType { get; set; } = string.Empty;

    [JsonPropertyName("message")]
    public string Message { get; set; } = string.Empty;
}

/// <summary>
/// Represents an extracted keyword from keyword extraction algorithms (YAKE, RAKE).
/// </summary>
public sealed class ExtractedKeyword
{
    /// <summary>
    /// The keyword text.
    /// </summary>
    [JsonPropertyName("text")]
    public string Text { get; set; } = string.Empty;

    /// <summary>
    /// Relevance score (higher is better, algorithm-specific range).
    /// </summary>
    [JsonPropertyName("score")]
    public float Score { get; set; }

    /// <summary>
    /// Algorithm that extracted this keyword (e.g., "yake", "rake").
    /// </summary>
    [JsonPropertyName("algorithm")]
    public string Algorithm { get; set; } = string.Empty;

    /// <summary>
    /// Optional positions where keyword appears in text (character offsets).
    /// </summary>
    [JsonPropertyName("positions")]
    public List<int>? Positions { get; set; }
}

public sealed class PdfMetadata
{
    [JsonPropertyName("title")]
    public string? Title { get; set; }

    [JsonPropertyName("subject")]
    public string? Subject { get; set; }

    [JsonPropertyName("author")]
    public string? Author { get; set; }

    [JsonPropertyName("keywords")]
    public List<string>? Keywords { get; set; }

    [JsonPropertyName("creator")]
    public string? Creator { get; set; }

    [JsonPropertyName("producer")]
    public string? Producer { get; set; }

    [JsonPropertyName("creation_date")]
    public string? CreationDate { get; set; }

    [JsonPropertyName("modification_date")]
    public string? ModificationDate { get; set; }

    [JsonPropertyName("page_count")]
    public int? PageCount { get; set; }
}

public sealed class ExcelMetadata
{
    [JsonPropertyName("sheet_count")]
    public int? SheetCount { get; set; }

    [JsonPropertyName("sheet_names")]
    public List<string>? SheetNames { get; set; }
}

public sealed class EmailMetadata
{
    [JsonPropertyName("from_email")]
    public string? FromEmail { get; set; }

    [JsonPropertyName("from_name")]
    public string? FromName { get; set; }

    [JsonPropertyName("to_emails")]
    public List<string>? ToEmails { get; set; }

    [JsonPropertyName("cc_emails")]
    public List<string>? CcEmails { get; set; }

    [JsonPropertyName("bcc_emails")]
    public List<string>? BccEmails { get; set; }

    [JsonPropertyName("message_id")]
    public string? MessageId { get; set; }

    [JsonPropertyName("attachments")]
    public List<string>? Attachments { get; set; }
}

public sealed class ArchiveMetadata
{
    [JsonPropertyName("format")]
    public string? Format { get; set; }

    [JsonPropertyName("file_count")]
    public int? FileCount { get; set; }

    [JsonPropertyName("file_list")]
    public List<string>? FileList { get; set; }

    [JsonPropertyName("total_size")]
    public int? TotalSize { get; set; }

    [JsonPropertyName("compressed_size")]
    public int? CompressedSize { get; set; }
}

public sealed class ImageMetadata
{
    [JsonPropertyName("width")]
    public uint Width { get; set; }

    [JsonPropertyName("height")]
    public uint Height { get; set; }

    [JsonPropertyName("format")]
    public string Format { get; set; } = string.Empty;

    [JsonPropertyName("exif")]
    public Dictionary<string, string>? Exif { get; set; }
}

public sealed class XmlMetadata
{
    [JsonPropertyName("element_count")]
    public int? ElementCount { get; set; }

    [JsonPropertyName("unique_elements")]
    public List<string>? UniqueElements { get; set; }
}

public sealed class TextMetadata
{
    [JsonPropertyName("line_count")]
    public int? LineCount { get; set; }

    [JsonPropertyName("word_count")]
    public int? WordCount { get; set; }

    [JsonPropertyName("character_count")]
    public int? CharacterCount { get; set; }

    [JsonPropertyName("headers")]
    public List<string>? Headers { get; set; }

    [JsonPropertyName("links")]
    public List<List<string>>? Links { get; set; }

    [JsonPropertyName("code_blocks")]
    public List<List<string>>? CodeBlocks { get; set; }
}

public sealed class HtmlMetadata
{
    [JsonPropertyName("title")]
    public string? Title { get; set; }

    [JsonPropertyName("description")]
    public string? Description { get; set; }

    [JsonPropertyName("keywords")]
    public List<string> Keywords { get; set; } = new();

    [JsonPropertyName("author")]
    public string? Author { get; set; }

    [JsonPropertyName("canonical_url")]
    public string? CanonicalUrl { get; set; }

    [JsonPropertyName("base_href")]
    public string? BaseHref { get; set; }

    [JsonPropertyName("language")]
    public string? Language { get; set; }

    [JsonPropertyName("text_direction")]
    public string? TextDirection { get; set; }

    [JsonPropertyName("open_graph")]
    public Dictionary<string, string> OpenGraph { get; set; } = new();

    [JsonPropertyName("twitter_card")]
    public Dictionary<string, string> TwitterCard { get; set; } = new();

    [JsonPropertyName("meta_tags")]
    public Dictionary<string, string> MetaTags { get; set; } = new();

    [JsonPropertyName("headers")]
    public List<HeaderMetadata> Headers { get; set; } = new();

    [JsonPropertyName("links")]
    public List<LinkMetadata> Links { get; set; } = new();

    [JsonPropertyName("images")]
    public List<HtmlImageMetadata> Images { get; set; } = new();

    [JsonPropertyName("structured_data")]
    public List<StructuredData> StructuredData { get; set; } = new();
}

/// <summary>
/// Represents a header/heading in an HTML document.
/// </summary>
public sealed class HeaderMetadata
{
    [JsonPropertyName("level")]
    public byte Level { get; set; }

    [JsonPropertyName("text")]
    public string Text { get; set; } = string.Empty;

    [JsonPropertyName("id")]
    public string? Id { get; set; }

    [JsonPropertyName("depth")]
    public int Depth { get; set; }

    [JsonPropertyName("html_offset")]
    public int HtmlOffset { get; set; }
}

/// <summary>
/// Represents a link in an HTML document.
/// </summary>
public sealed class LinkMetadata
{
    [JsonPropertyName("href")]
    public string Href { get; set; } = string.Empty;

    [JsonPropertyName("text")]
    public string Text { get; set; } = string.Empty;

    [JsonPropertyName("title")]
    public string? Title { get; set; }

    [JsonPropertyName("link_type")]
    public string LinkType { get; set; } = "other";

    [JsonPropertyName("rel")]
    public List<string> Rel { get; set; } = new();

    [JsonPropertyName("attributes")]
    public Dictionary<string, string> Attributes { get; set; } = new();
}

/// <summary>
/// Represents an image in an HTML document.
/// </summary>
public sealed class HtmlImageMetadata
{
    [JsonPropertyName("src")]
    public string Src { get; set; } = string.Empty;

    [JsonPropertyName("alt")]
    public string? Alt { get; set; }

    [JsonPropertyName("title")]
    public string? Title { get; set; }

    [JsonPropertyName("dimensions")]
    public int[]? Dimensions { get; set; }

    [JsonPropertyName("image_type")]
    public string ImageType { get; set; } = "external";

    [JsonPropertyName("attributes")]
    public Dictionary<string, string> Attributes { get; set; } = new();
}

/// <summary>
/// Represents structured data (JSON-LD, etc.) in an HTML document.
/// </summary>
public sealed class StructuredData
{
    [JsonPropertyName("data_type")]
    public string DataType { get; set; } = "json_ld";

    [JsonPropertyName("raw_json")]
    public string RawJson { get; set; } = string.Empty;

    [JsonPropertyName("schema_type")]
    public string? SchemaType { get; set; }
}

public sealed class PptxMetadata
{
    [JsonPropertyName("title")]
    public string? Title { get; set; }

    [JsonPropertyName("author")]
    public string? Author { get; set; }

    [JsonPropertyName("description")]
    public string? Description { get; set; }

    [JsonPropertyName("summary")]
    public string? Summary { get; set; }

    [JsonPropertyName("fonts")]
    public List<string>? Fonts { get; set; }
}

public sealed class OcrMetadata
{
    [JsonPropertyName("language")]
    public string? Language { get; set; }

    [JsonPropertyName("psm")]
    public int? Psm { get; set; }

    [JsonPropertyName("output_format")]
    public string? OutputFormat { get; set; }

    [JsonPropertyName("table_count")]
    public int? TableCount { get; set; }

    [JsonPropertyName("table_rows")]
    public int? TableRows { get; set; }

    [JsonPropertyName("table_cols")]
    public int? TableCols { get; set; }
}

/// <summary>
/// Represents the page structure of a document with metadata about pagination.
/// </summary>
public sealed class PageStructure
{
    /// <summary>
    /// Total number of pages/slides/sheets in the document.
    /// </summary>
    [JsonPropertyName("total_count")]
    public int TotalCount { get; set; }

    /// <summary>
    /// Type of paginated unit (e.g., page, slide, sheet).
    /// </summary>
    [JsonPropertyName("unit_type")]
    public string UnitType { get; set; } = string.Empty;

    /// <summary>
    /// Character offset boundaries for each page. Maps character ranges in the extracted content to page numbers.
    /// </summary>
    [JsonPropertyName("boundaries")]
    public List<PageBoundary>? Boundaries { get; set; }

    /// <summary>
    /// Detailed per-page metadata (optional, only when needed).
    /// </summary>
    [JsonPropertyName("pages")]
    public List<PageInfo>? Pages { get; set; }
}

/// <summary>
/// Represents a character offset boundary for a page.
/// </summary>
public sealed class PageBoundary
{
    /// <summary>
    /// Page number (1-indexed).
    /// </summary>
    [JsonPropertyName("page_number")]
    public int PageNumber { get; set; }

    /// <summary>
    /// Starting character offset in the document.
    /// </summary>
    [JsonPropertyName("start")]
    public int Start { get; set; }

    /// <summary>
    /// Ending character offset in the document.
    /// </summary>
    [JsonPropertyName("end")]
    public int End { get; set; }
}

/// <summary>
/// Represents per-page metadata in a document.
/// </summary>
public sealed class PageInfo
{
    /// <summary>
    /// Page number (1-indexed).
    /// </summary>
    [JsonPropertyName("page_number")]
    public int PageNumber { get; set; }

    /// <summary>
    /// Page width (in points or pixels, depending on document type).
    /// </summary>
    [JsonPropertyName("width")]
    public double? Width { get; set; }

    /// <summary>
    /// Page height (in points or pixels, depending on document type).
    /// </summary>
    [JsonPropertyName("height")]
    public double? Height { get; set; }

    /// <summary>
    /// Optional text representation of the page.
    /// </summary>
    [JsonPropertyName("text")]
    public string? Text { get; set; }

    /// <summary>
    /// Optional labels or notes for the page.
    /// </summary>
    [JsonPropertyName("labels")]
    public List<string>? Labels { get; set; }
}

/// <summary>
/// Configuration for document extraction, controlling extraction behavior and features.
/// </summary>
public sealed class ExtractionConfig
{
    /// <summary>
    /// Whether to use caching for extraction results. Default is null (use server default).
    /// </summary>
    [JsonPropertyName("use_cache")]
    public bool? UseCache { get; init; }

    /// <summary>
    /// Whether to enable quality processing to improve extraction quality. Default is null.
    /// </summary>
    [JsonPropertyName("enable_quality_processing")]
    public bool? EnableQualityProcessing { get; init; }

    /// <summary>
    /// OCR configuration for handling scanned documents and images. If null, OCR is disabled.
    /// </summary>
    [JsonPropertyName("ocr")]
    public OcrConfig? Ocr { get; init; }

    /// <summary>
    /// Whether to force OCR processing even for documents with text. Default is false.
    /// </summary>
    [JsonPropertyName("force_ocr")]
    public bool? ForceOcr { get; init; }

    /// <summary>
    /// Text chunking configuration for splitting long documents. If null, chunking is disabled.
    /// </summary>
    [JsonPropertyName("chunking")]
    public ChunkingConfig? Chunking { get; init; }

    /// <summary>
    /// Image extraction configuration. If null, image extraction is disabled.
    /// </summary>
    [JsonPropertyName("images")]
    public ImageExtractionConfig? Images { get; init; }

    /// <summary>
    /// PDF-specific extraction options (password protection, metadata, etc.).
    /// </summary>
    [JsonPropertyName("pdf_options")]
    public PdfConfig? PdfOptions { get; init; }

    /// <summary>
    /// Token reduction configuration for reducing token counts in results.
    /// </summary>
    [JsonPropertyName("token_reduction")]
    public TokenReductionConfig? TokenReduction { get; init; }

    /// <summary>
    /// Language detection configuration. If null, language detection is disabled.
    /// </summary>
    [JsonPropertyName("language_detection")]
    public LanguageDetectionConfig? LanguageDetection { get; init; }

    /// <summary>
    /// Post-processor configuration for controlling which processors are enabled/disabled.
    /// </summary>
    [JsonPropertyName("postprocessor")]
    public PostProcessorConfig? Postprocessor { get; init; }

    /// <summary>
    /// HTML conversion options for HTML documents.
    /// </summary>
    [JsonPropertyName("html_options")]
    public HtmlConversionOptions? HtmlOptions { get; init; }

    /// <summary>
    /// Keyword extraction configuration.
    /// </summary>
    [JsonPropertyName("keywords")]
    public KeywordConfig? Keywords { get; init; }

    /// <summary>
    /// Page extraction and tracking configuration.
    /// </summary>
    [JsonPropertyName("pages")]
    public PageConfig? Pages { get; init; }

    /// <summary>
    /// Maximum number of concurrent extractions in batch operations. Default is null.
    /// </summary>
    [JsonPropertyName("max_concurrent_extractions")]
    public int? MaxConcurrentExtractions { get; init; }
}

/// <summary>
/// Configuration for OCR (Optical Character Recognition) processing.
/// </summary>
public sealed class OcrConfig
{
    /// <summary>
    /// OCR backend to use (e.g., "tesseract", "paddle").
    /// </summary>
    [JsonPropertyName("backend")]
    public string? Backend { get; init; }

    /// <summary>
    /// Language for OCR processing (e.g., "eng", "fra", "deu").
    /// </summary>
    [JsonPropertyName("language")]
    public string? Language { get; init; }

    /// <summary>
    /// Tesseract-specific configuration, if using Tesseract backend.
    /// </summary>
    [JsonPropertyName("tesseract_config")]
    public TesseractConfig? TesseractConfig { get; init; }
}

/// <summary>
/// Tesseract OCR engine-specific configuration options.
/// </summary>
public sealed class TesseractConfig
{
    /// <summary>
    /// OCR language (e.g., "eng", "fra", "deu").
    /// </summary>
    [JsonPropertyName("language")]
    public string? Language { get; init; }

    /// <summary>
    /// Page Segmentation Mode (PSM) for Tesseract (0-13).
    /// </summary>
    [JsonPropertyName("psm")]
    public int? Psm { get; init; }

    /// <summary>
    /// Output format specification.
    /// </summary>
    [JsonPropertyName("output_format")]
    public string? OutputFormat { get; init; }

    /// <summary>
    /// OCR Engine Mode (OEM) for Tesseract (0-3).
    /// </summary>
    [JsonPropertyName("oem")]
    public int? Oem { get; init; }

    /// <summary>
    /// Minimum confidence threshold for character recognition (0.0-1.0).
    /// </summary>
    [JsonPropertyName("min_confidence")]
    public double? MinConfidence { get; init; }

    /// <summary>
    /// Image preprocessing configuration for OCR preparation.
    /// </summary>
    [JsonPropertyName("preprocessing")]
    public ImagePreprocessingConfig? Preprocessing { get; init; }

    /// <summary>
    /// Whether to enable table detection and extraction.
    /// </summary>
    [JsonPropertyName("enable_table_detection")]
    public bool? EnableTableDetection { get; init; }

    /// <summary>
    /// Minimum confidence for table detection.
    /// </summary>
    [JsonPropertyName("table_min_confidence")]
    public double? TableMinConfidence { get; init; }

    /// <summary>
    /// Threshold for detecting table columns.
    /// </summary>
    [JsonPropertyName("table_column_threshold")]
    public int? TableColumnThreshold { get; init; }

    /// <summary>
    /// Ratio threshold for detecting table rows.
    /// </summary>
    [JsonPropertyName("table_row_threshold_ratio")]
    public double? TableRowThresholdRatio { get; init; }

    /// <summary>
    /// Whether to use caching for OCR results.
    /// </summary>
    [JsonPropertyName("use_cache")]
    public bool? UseCache { get; init; }

    /// <summary>
    /// Whether to classify using pre-adapted templates.
    /// </summary>
    [JsonPropertyName("classify_use_pre_adapted_templates")]
    public bool? ClassifyUsePreAdaptedTemplates { get; init; }

    /// <summary>
    /// Whether to use language model n-gram.
    /// </summary>
    [JsonPropertyName("language_model_ngram_on")]
    public bool? LanguageModelNgramOn { get; init; }

    /// <summary>
    /// Tesseract parameter: don't reject good words in blocks.
    /// </summary>
    [JsonPropertyName("tessedit_dont_blkrej_good_wds")]
    public bool? TesseditDontBlkrejGoodWds { get; init; }

    /// <summary>
    /// Tesseract parameter: don't reject good words in rows.
    /// </summary>
    [JsonPropertyName("tessedit_dont_rowrej_good_wds")]
    public bool? TesseditDontRowrejGoodWds { get; init; }

    /// <summary>
    /// Tesseract parameter: enable dictionary correction.
    /// </summary>
    [JsonPropertyName("tessedit_enable_dict_correction")]
    public bool? TesseditEnableDictCorrection { get; init; }

    /// <summary>
    /// Whitelist of characters for OCR recognition.
    /// </summary>
    [JsonPropertyName("tessedit_char_whitelist")]
    public string? TesseditCharWhitelist { get; init; }

    /// <summary>
    /// Blacklist of characters to exclude from OCR recognition.
    /// </summary>
    [JsonPropertyName("tessedit_char_blacklist")]
    public string? TesseditCharBlacklist { get; init; }

    /// <summary>
    /// Tesseract parameter: use primary params model.
    /// </summary>
    [JsonPropertyName("tessedit_use_primary_params_model")]
    public bool? TesseditUsePrimaryParamsModel { get; init; }

    /// <summary>
    /// Tesseract parameter: space size is variable.
    /// </summary>
    [JsonPropertyName("textord_space_size_is_variable")]
    public bool? TextordSpaceSizeIsVariable { get; init; }

    /// <summary>
    /// Thresholding method for image binarization.
    /// </summary>
    [JsonPropertyName("thresholding_method")]
    public bool? ThresholdingMethod { get; init; }
}

/// <summary>
/// Configuration for image preprocessing operations (rotation, deskew, denoise, etc.).
/// </summary>
public sealed class ImagePreprocessingConfig
{
    /// <summary>
    /// Target dots-per-inch (DPI) for image upscaling/downscaling.
    /// </summary>
    [JsonPropertyName("target_dpi")]
    public int? TargetDpi { get; init; }

    /// <summary>
    /// Whether to automatically rotate images to correct orientation.
    /// </summary>
    [JsonPropertyName("auto_rotate")]
    public bool? AutoRotate { get; init; }

    /// <summary>
    /// Whether to deskew (rotate to correct skew) the image.
    /// </summary>
    [JsonPropertyName("deskew")]
    public bool? Deskew { get; init; }

    /// <summary>
    /// Whether to apply denoising to reduce image noise.
    /// </summary>
    [JsonPropertyName("denoise")]
    public bool? Denoise { get; init; }

    /// <summary>
    /// Whether to enhance image contrast.
    /// </summary>
    [JsonPropertyName("contrast_enhance")]
    public bool? ContrastEnhance { get; init; }

    /// <summary>
    /// Binarization method for converting to black and white.
    /// </summary>
    [JsonPropertyName("binarization_method")]
    public string? BinarizationMode { get; init; }

    /// <summary>
    /// Whether to invert image colors.
    /// </summary>
    [JsonPropertyName("invert_colors")]
    public bool? InvertColors { get; init; }
}

/// <summary>
/// Configuration for embedding generation using ONNX Runtime models.
/// </summary>
public sealed class EmbeddingConfig
{
    /// <summary>
    /// Embedding model name or preset identifier (e.g., "default", "balanced", "compact").
    /// </summary>
    [JsonPropertyName("model")]
    public string? Model { get; init; }

    /// <summary>
    /// Number of texts to process simultaneously during embedding generation.
    /// Higher values use more memory but may be faster.
    /// </summary>
    [JsonPropertyName("batch_size")]
    public int? BatchSize { get; init; }

    /// <summary>
    /// Whether to normalize embedding vectors to unit length.
    /// Recommended for cosine similarity calculations.
    /// </summary>
    [JsonPropertyName("normalize")]
    public bool? Normalize { get; init; }

    /// <summary>
    /// Output dimensionality of the embedding vectors.
    /// Model-dependent; typically 384, 768, 1536, or higher.
    /// </summary>
    [JsonPropertyName("dimensions")]
    public int? Dimensions { get; init; }

    /// <summary>
    /// Whether to cache embeddings for identical text chunks.
    /// Improves performance when processing duplicate content.
    /// </summary>
    [JsonPropertyName("use_cache")]
    public bool? UseCache { get; init; }
}

/// <summary>
/// Configuration for text chunking (splitting long documents into smaller pieces).
/// </summary>
public sealed class ChunkingConfig
{
    /// <summary>
    /// Maximum number of characters per chunk.
    /// </summary>
    [JsonPropertyName("max_chars")]
    public int? MaxChars { get; init; }

    /// <summary>
    /// Maximum character overlap between consecutive chunks.
    /// </summary>
    [JsonPropertyName("max_overlap")]
    public int? MaxOverlap { get; init; }

    /// <summary>
    /// Chunk size (alternative to max_chars).
    /// </summary>
    [JsonPropertyName("chunk_size")]
    public int? ChunkSize { get; init; }

    /// <summary>
    /// Character overlap between chunks.
    /// </summary>
    [JsonPropertyName("chunk_overlap")]
    public int? ChunkOverlap { get; init; }

    /// <summary>
    /// Named preset for chunking strategy (e.g., "default", "semantic").
    /// </summary>
    [JsonPropertyName("preset")]
    public string? Preset { get; init; }

    /// <summary>
    /// Embedding configuration for vector generation.
    /// </summary>
    [JsonPropertyName("embedding")]
    public EmbeddingConfig? Embedding { get; init; }

    /// <summary>
    /// Whether chunking is enabled.
    /// </summary>
    [JsonPropertyName("enabled")]
    public bool? Enabled { get; init; }
}

/// <summary>
/// Configuration for image extraction from documents.
/// </summary>
public sealed class ImageExtractionConfig
{
    /// <summary>
    /// Whether to extract images from documents.
    /// </summary>
    [JsonPropertyName("extract_images")]
    public bool? ExtractImages { get; init; }

    /// <summary>
    /// Target DPI for extracted images.
    /// </summary>
    [JsonPropertyName("target_dpi")]
    public int? TargetDpi { get; init; }

    /// <summary>
    /// Maximum dimension (width or height) for extracted images.
    /// </summary>
    [JsonPropertyName("max_image_dimension")]
    public int? MaxImageDimension { get; init; }

    /// <summary>
    /// Whether to automatically adjust DPI based on image characteristics.
    /// </summary>
    [JsonPropertyName("auto_adjust_dpi")]
    public bool? AutoAdjustDpi { get; init; }

    /// <summary>
    /// Minimum allowed DPI for extracted images.
    /// </summary>
    [JsonPropertyName("min_dpi")]
    public int? MinDpi { get; init; }

    /// <summary>
    /// Maximum allowed DPI for extracted images.
    /// </summary>
    [JsonPropertyName("max_dpi")]
    public int? MaxDpi { get; init; }
}

/// <summary>
/// Font configuration for PDF processing.
/// </summary>
public sealed class FontConfig
{
    /// <summary>
    /// Whether font fallback is enabled for handling missing fonts.
    /// </summary>
    [JsonPropertyName("font_fallback_enabled")]
    public bool? FontFallbackEnabled { get; init; }

    /// <summary>
    /// Directory path for custom fonts.
    /// </summary>
    [JsonPropertyName("font_dir")]
    public string? FontDir { get; init; }
}

/// <summary>
/// Hierarchy detection configuration for PDF document structure.
/// </summary>
public sealed class HierarchyConfig
{
    /// <summary>
    /// Whether hierarchy detection is enabled.
    /// </summary>
    [JsonPropertyName("enabled")]
    public bool? Enabled { get; init; }

    /// <summary>
    /// Number of k clusters for hierarchy detection.
    /// </summary>
    [JsonPropertyName("k_clusters")]
    public int? KClusters { get; init; }

    /// <summary>
    /// Whether to include bounding box information in hierarchy output.
    /// </summary>
    [JsonPropertyName("include_bbox")]
    public bool? IncludeBbox { get; init; }

    /// <summary>
    /// OCR coverage threshold for hierarchy detection (0.0-1.0).
    /// </summary>
    [JsonPropertyName("ocr_coverage_threshold")]
    public float? OcrCoverageThreshold { get; init; }
}

/// <summary>
/// PDF-specific extraction configuration.
/// </summary>
public sealed class PdfConfig
{
    /// <summary>
    /// Whether to extract images from PDF documents.
    /// </summary>
    [JsonPropertyName("extract_images")]
    public bool? ExtractImages { get; init; }

    /// <summary>
    /// List of passwords to try for encrypted PDFs.
    /// </summary>
    [JsonPropertyName("passwords")]
    public List<string>? Passwords { get; init; }

    /// <summary>
    /// Whether to extract document metadata (title, author, etc.).
    /// </summary>
    [JsonPropertyName("extract_metadata")]
    public bool? ExtractMetadata { get; init; }

    /// <summary>
    /// Font configuration for PDF text extraction.
    /// </summary>
    [JsonPropertyName("font_config")]
    public FontConfig? FontConfig { get; init; }

    /// <summary>
    /// Hierarchy detection configuration for document structure analysis.
    /// </summary>
    [JsonPropertyName("hierarchy")]
    public HierarchyConfig? Hierarchy { get; init; }
}

/// <summary>
/// Configuration for token reduction to minimize token counts in results.
/// </summary>
public sealed class TokenReductionConfig
{
    /// <summary>
    /// Token reduction mode (e.g., "none", "aggressive", "balanced").
    /// </summary>
    [JsonPropertyName("mode")]
    public string? Mode { get; init; }

    /// <summary>
    /// Whether to preserve important words during token reduction.
    /// </summary>
    [JsonPropertyName("preserve_important_words")]
    public bool? PreserveImportantWords { get; init; }
}

/// <summary>
/// Configuration for language detection in extracted text.
/// </summary>
public sealed class LanguageDetectionConfig
{
    /// <summary>
    /// Whether language detection is enabled.
    /// </summary>
    [JsonPropertyName("enabled")]
    public bool? Enabled { get; init; }

    /// <summary>
    /// Minimum confidence threshold for language detection (0.0-1.0).
    /// </summary>
    [JsonPropertyName("min_confidence")]
    public double? MinConfidence { get; init; }

    /// <summary>
    /// Whether to detect multiple languages in the document.
    /// </summary>
    [JsonPropertyName("detect_multiple")]
    public bool? DetectMultiple { get; init; }
}

/// <summary>
/// Configuration for custom post-processor execution.
/// </summary>
public sealed class PostProcessorConfig
{
    /// <summary>
    /// Whether post-processing is enabled.
    /// </summary>
    [JsonPropertyName("enabled")]
    public bool? Enabled { get; init; }

    /// <summary>
    /// List of post-processor names to enable.
    /// </summary>
    [JsonPropertyName("enabled_processors")]
    public List<string>? EnabledProcessors { get; init; }

    /// <summary>
    /// List of post-processor names to disable.
    /// </summary>
    [JsonPropertyName("disabled_processors")]
    public List<string>? DisabledProcessors { get; init; }
}

/// <summary>
/// Configuration for HTML to text conversion with fine-grained formatting control.
/// </summary>
public sealed class HtmlConversionOptions
{
    /// <summary>
    /// Style for markdown headings (e.g., "setext", "atx").
    /// </summary>
    public string? HeadingStyle { get; init; }

    /// <summary>
    /// Type of indentation for lists (e.g., "space", "tab").
    /// </summary>
    public string? ListIndentType { get; init; }

    /// <summary>
    /// Width of list indentation.
    /// </summary>
    public int? ListIndentWidth { get; init; }

    /// <summary>
    /// Bullet style for unordered lists (e.g., "-", "*", "+").
    /// </summary>
    public string? Bullets { get; init; }

    /// <summary>
    /// Symbol for strong/emphasis text.
    /// </summary>
    public string? StrongEmSymbol { get; init; }

    /// <summary>
    /// Whether to escape asterisks in output.
    /// </summary>
    public bool? EscapeAsterisks { get; init; }

    /// <summary>
    /// Whether to escape underscores in output.
    /// </summary>
    public bool? EscapeUnderscores { get; init; }

    /// <summary>
    /// Whether to escape miscellaneous characters.
    /// </summary>
    public bool? EscapeMisc { get; init; }

    /// <summary>
    /// Whether to escape ASCII control characters.
    /// </summary>
    public bool? EscapeAscii { get; init; }

    /// <summary>
    /// Language for code blocks syntax highlighting.
    /// </summary>
    public string? CodeLanguage { get; init; }

    /// <summary>
    /// Whether to automatically convert URLs to hyperlinks.
    /// </summary>
    public bool? Autolinks { get; init; }

    /// <summary>
    /// Default title for documents without one.
    /// </summary>
    public string? DefaultTitle { get; init; }

    /// <summary>
    /// Whether to use HTML line breaks in tables.
    /// </summary>
    public bool? BrInTables { get; init; }

    /// <summary>
    /// Whether to use hOCR spatial tables.
    /// </summary>
    public bool? HocrSpatialTables { get; init; }

    /// <summary>
    /// Highlighting style for code blocks.
    /// </summary>
    public string? HighlightStyle { get; init; }

    /// <summary>
    /// Whether to extract and include document metadata.
    /// </summary>
    public bool? ExtractMetadata { get; init; }

    /// <summary>
    /// Whitespace handling mode (e.g., "preserve", "collapse").
    /// </summary>
    public string? WhitespaceMode { get; init; }

    /// <summary>
    /// Whether to strip newlines from output.
    /// </summary>
    public bool? StripNewlines { get; init; }

    /// <summary>
    /// Whether to wrap text output.
    /// </summary>
    public bool? Wrap { get; init; }

    /// <summary>
    /// Text wrapping width in characters.
    /// </summary>
    public int? WrapWidth { get; init; }

    /// <summary>
    /// Whether to convert HTML as inline content.
    /// </summary>
    public bool? ConvertAsInline { get; init; }

    /// <summary>
    /// Symbol for subscript text.
    /// </summary>
    public string? SubSymbol { get; init; }

    /// <summary>
    /// Symbol for superscript text.
    /// </summary>
    public string? SupSymbol { get; init; }

    /// <summary>
    /// Newline style for output (e.g., "lf", "crlf").
    /// </summary>
    public string? NewlineStyle { get; init; }

    /// <summary>
    /// Style for code blocks (e.g., "fenced", "indented").
    /// </summary>
    public string? CodeBlockStyle { get; init; }

    /// <summary>
    /// List of HTML elements to keep inline images in.
    /// </summary>
    public List<string>? KeepInlineImagesIn { get; init; }

    /// <summary>
    /// Character encoding for output.
    /// </summary>
    public string? Encoding { get; init; }

    /// <summary>
    /// Whether to include debug information in output.
    /// </summary>
    public bool? Debug { get; init; }

    /// <summary>
    /// HTML tags to strip from output.
    /// </summary>
    public List<string>? StripTags { get; init; }

    /// <summary>
    /// HTML tags to preserve in output.
    /// </summary>
    public List<string>? PreserveTags { get; init; }

    /// <summary>
    /// HTML preprocessing configuration.
    /// </summary>
    public HtmlPreprocessingOptions? Preprocessing { get; init; }
}

/// <summary>
/// Configuration for preprocessing HTML before conversion.
/// </summary>
public sealed class HtmlPreprocessingOptions
{
    /// <summary>
    /// Whether preprocessing is enabled.
    /// </summary>
    public bool? Enabled { get; init; }

    /// <summary>
    /// Named preset for preprocessing strategy.
    /// </summary>
    public string? Preset { get; init; }

    /// <summary>
    /// Whether to remove navigation elements.
    /// </summary>
    public bool? RemoveNavigation { get; init; }

    /// <summary>
    /// Whether to remove form elements.
    /// </summary>
    public bool? RemoveForms { get; init; }
}

/// <summary>
/// Keyword extraction algorithms supported by Kreuzberg.
/// </summary>
public static class KeywordAlgorithm
{
    /// <summary>
    /// YAKE (Yet Another Keyword Extractor) algorithm.
    /// </summary>
    public const string Yake = "yake";

    /// <summary>
    /// RAKE (Rapid Automatic Keyword Extraction) algorithm.
    /// </summary>
    public const string Rake = "rake";

    /// <summary>
    /// Alias for YAKE (uppercase version).
    /// </summary>
    public const string YAKE = Yake;

    /// <summary>
    /// Alias for RAKE (uppercase version).
    /// </summary>
    public const string RAKE = Rake;
}

/// <summary>
/// Configuration for keyword extraction from documents.
/// </summary>
public sealed class KeywordConfig
{
    /// <summary>
    /// Keyword extraction algorithm to use (e.g., "yake", "rake").
    /// </summary>
    [JsonPropertyName("algorithm")]
    public string? Algorithm { get; init; }

    /// <summary>
    /// Maximum number of keywords to extract.
    /// </summary>
    [JsonPropertyName("max_keywords")]
    public int? MaxKeywords { get; init; }

    /// <summary>
    /// Minimum relevance score threshold for keywords (0.0-1.0).
    /// </summary>
    [JsonPropertyName("min_score")]
    public double? MinScore { get; init; }

    /// <summary>
    /// N-gram range for keyword extraction [min, max].
    /// </summary>
    [JsonPropertyName("ngram_range")]
    public List<int>? NgramRange { get; init; }

    /// <summary>
    /// Language for keyword extraction (e.g., "en", "fr", "de").
    /// </summary>
    [JsonPropertyName("language")]
    public string? Language { get; init; }

    /// <summary>
    /// Algorithm-specific parameters for YAKE.
    /// </summary>
    [JsonPropertyName("yake_params")]
    public Dictionary<string, object?>? YakeParams { get; init; }

    /// <summary>
    /// Algorithm-specific parameters for RAKE.
    /// </summary>
    [JsonPropertyName("rake_params")]
    public Dictionary<string, object?>? RakeParams { get; init; }
}

/// <summary>
/// Configuration for page tracking and extraction during document processing.
/// </summary>
public sealed class PageConfig
{
    /// <summary>
    /// Whether to extract and track page information.
    /// </summary>
    [JsonPropertyName("extract_pages")]
    public bool? ExtractPages { get; init; }

    /// <summary>
    /// Whether to insert page markers in the extracted content.
    /// </summary>
    [JsonPropertyName("insert_page_markers")]
    public bool? InsertPageMarkers { get; init; }

    /// <summary>
    /// Format for page markers (e.g., "[PAGE_N]", "Page: N").
    /// </summary>
    [JsonPropertyName("marker_format")]
    public string? MarkerFormat { get; init; }
}

/// <summary>
/// Represents a document as bytes with its MIME type, used for batch extraction from in-memory data.
/// </summary>
public sealed class BytesWithMime
{
    /// <summary>
    /// The document bytes.
    /// </summary>
    public byte[] Data { get; }

    /// <summary>
    /// The MIME type of the document (e.g., "application/pdf").
    /// </summary>
    public string MimeType { get; }

    /// <summary>
    /// Initializes a new instance with document bytes and MIME type.
    /// </summary>
    /// <param name="data">Document bytes. Must not be null.</param>
    /// <param name="mimeType">MIME type string. Must not be null.</param>
    /// <exception cref="ArgumentNullException">If data or mimeType is null</exception>
    public BytesWithMime(byte[] data, string mimeType)
    {
        Data = data ?? throw new ArgumentNullException(nameof(data));
        MimeType = mimeType ?? throw new ArgumentNullException(nameof(mimeType));
    }
}

/// <summary>
/// Interface for custom post-processors that can modify extraction results.
/// </summary>
public interface IPostProcessor
{
    /// <summary>
    /// The unique name of this post-processor.
    /// </summary>
    string Name { get; }

    /// <summary>
    /// The priority order (higher values run first). Useful for ordering multiple processors.
    /// </summary>
    int Priority { get; }

    /// <summary>
    /// Processes an extraction result, potentially modifying it.
    /// </summary>
    /// <param name="result">The extraction result to process. Can be modified in-place.</param>
    /// <returns>The (potentially modified) extraction result.</returns>
    ExtractionResult Process(ExtractionResult result);
}

/// <summary>
/// Interface for custom validators that can validate extraction results.
/// </summary>
public interface IValidator
{
    /// <summary>
    /// The unique name of this validator.
    /// </summary>
    string Name { get; }

    /// <summary>
    /// The priority order (higher values run first). Useful for ordering multiple validators.
    /// </summary>
    int Priority { get; }

    /// <summary>
    /// Validates an extraction result. Should throw an exception if validation fails.
    /// </summary>
    /// <param name="result">The extraction result to validate.</param>
    /// <exception cref="Exception">Thrown if validation fails. The exception message will be used as the error reason.</exception>
    void Validate(ExtractionResult result);
}

/// <summary>
/// Interface for custom OCR backends for document extraction.
/// </summary>
public interface IOcrBackend
{
    /// <summary>
    /// The unique name of this OCR backend (e.g., "tesseract", "custom_ocr").
    /// </summary>
    string Name { get; }

    /// <summary>
    /// Processes image bytes and returns OCR results as JSON.
    /// </summary>
    /// <param name="imageBytes">Raw image data (PNG, JPEG, etc.).</param>
    /// <param name="config">OCR configuration, if any.</param>
    /// <returns>JSON string with OCR results (structure depends on implementation).</returns>
    string Process(ReadOnlySpan<byte> imageBytes, OcrConfig? config);
}

/// <summary>
/// Represents an embedding preset with configuration for text embedding generation.
/// </summary>
public sealed class EmbeddingPreset
{
    /// <summary>
    /// The name/identifier of this embedding preset (e.g., "default", "openai").
    /// </summary>
    [JsonPropertyName("name")]
    public string Name { get; set; } = string.Empty;

    /// <summary>
    /// The recommended chunk size (in tokens or characters) for this embedding model.
    /// </summary>
    [JsonPropertyName("chunk_size")]
    public int ChunkSize { get; set; }

    /// <summary>
    /// The recommended overlap between chunks when chunking text for this model.
    /// </summary>
    [JsonPropertyName("overlap")]
    public int Overlap { get; set; }

    /// <summary>
    /// The name of the embedding model (e.g., "text-embedding-ada-002").
    /// </summary>
    [JsonPropertyName("model_name")]
    public string ModelName { get; set; } = string.Empty;

    /// <summary>
    /// The output dimensionality of the embedding vectors from this model.
    /// </summary>
    [JsonPropertyName("dimensions")]
    public int Dimensions { get; set; }

    /// <summary>
    /// Human-readable description of this embedding preset.
    /// </summary>
    [JsonPropertyName("description")]
    public string Description { get; set; } = string.Empty;
}

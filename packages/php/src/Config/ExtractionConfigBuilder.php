<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * Builder class for constructing ExtractionConfig instances with a fluent interface.
 *
 * This builder pattern addresses the 12-parameter constructor issue in ExtractionConfig,
 * providing a clean, readable way to configure extraction behavior through method chaining.
 *
 * @example
 * ```php
 * use Kreuzberg\Config\ExtractionConfigBuilder;
 * use Kreuzberg\Config\OcrConfig;
 * use Kreuzberg\Config\ChunkingConfig;
 *
 * $config = ExtractionConfig::builder()
 *     ->withOcr(new OcrConfig(backend: 'tesseract', language: 'eng'))
 *     ->withChunking(new ChunkingConfig(maxChunkSize: 1000))
 *     ->withExtractImages(true)
 *     ->withExtractTables(true)
 *     ->build();
 * ```
 */
class ExtractionConfigBuilder
{
    private ?OcrConfig $ocr = null;
    private ?PdfConfig $pdf = null;
    private ?ChunkingConfig $chunking = null;
    private ?EmbeddingConfig $embedding = null;
    private ?ImageExtractionConfig $imageExtraction = null;
    private ?PageConfig $page = null;
    private ?LanguageDetectionConfig $languageDetection = null;
    private ?KeywordConfig $keyword = null;
    private bool $extractImages = false;
    private bool $extractTables = true;
    private bool $preserveFormatting = false;
    private ?string $outputFormat = null;

    /**
     * Set the OCR configuration.
     *
     * @param OcrConfig|null $ocr OCR backend configuration
     * @return self For method chaining
     */
    public function withOcr(?OcrConfig $ocr): self
    {
        $this->ocr = $ocr;
        return $this;
    }

    /**
     * Set the PDF configuration.
     *
     * @param PdfConfig|null $pdf PDF extraction settings
     * @return self For method chaining
     */
    public function withPdf(?PdfConfig $pdf): self
    {
        $this->pdf = $pdf;
        return $this;
    }

    /**
     * Set the chunking configuration.
     *
     * @param ChunkingConfig|null $chunking Text chunking settings
     * @return self For method chaining
     */
    public function withChunking(?ChunkingConfig $chunking): self
    {
        $this->chunking = $chunking;
        return $this;
    }

    /**
     * Set the embedding configuration.
     *
     * @param EmbeddingConfig|null $embedding Vector embedding settings
     * @return self For method chaining
     */
    public function withEmbedding(?EmbeddingConfig $embedding): self
    {
        $this->embedding = $embedding;
        return $this;
    }

    /**
     * Set the image extraction configuration.
     *
     * @param ImageExtractionConfig|null $imageExtraction Image extraction settings
     * @return self For method chaining
     */
    public function withImageExtraction(?ImageExtractionConfig $imageExtraction): self
    {
        $this->imageExtraction = $imageExtraction;
        return $this;
    }

    /**
     * Set the page configuration.
     *
     * @param PageConfig|null $page Page-specific settings
     * @return self For method chaining
     */
    public function withPage(?PageConfig $page): self
    {
        $this->page = $page;
        return $this;
    }

    /**
     * Set the language detection configuration.
     *
     * @param LanguageDetectionConfig|null $languageDetection Language detection settings
     * @return self For method chaining
     */
    public function withLanguageDetection(?LanguageDetectionConfig $languageDetection): self
    {
        $this->languageDetection = $languageDetection;
        return $this;
    }

    /**
     * Set the keyword extraction configuration.
     *
     * @param KeywordConfig|null $keyword Keyword extraction settings
     * @return self For method chaining
     */
    public function withKeyword(?KeywordConfig $keyword): self
    {
        $this->keyword = $keyword;
        return $this;
    }

    /**
     * Set whether to extract images from documents.
     *
     * @param bool $extractImages Whether to extract embedded images
     * @return self For method chaining
     */
    public function withExtractImages(bool $extractImages): self
    {
        $this->extractImages = $extractImages;
        return $this;
    }

    /**
     * Set whether to extract tables from documents.
     *
     * @param bool $extractTables Whether to extract document tables
     * @return self For method chaining
     */
    public function withExtractTables(bool $extractTables): self
    {
        $this->extractTables = $extractTables;
        return $this;
    }

    /**
     * Set whether to preserve document formatting.
     *
     * @param bool $preserveFormatting Whether to preserve text formatting
     * @return self For method chaining
     */
    public function withPreserveFormatting(bool $preserveFormatting): self
    {
        $this->preserveFormatting = $preserveFormatting;
        return $this;
    }

    /**
     * Set the output format for extracted content.
     *
     * @param string|null $outputFormat Desired output format (e.g., 'markdown', 'html', 'plain')
     * @return self For method chaining
     */
    public function withOutputFormat(?string $outputFormat): self
    {
        $this->outputFormat = $outputFormat;
        return $this;
    }

    /**
     * Build and return the configured ExtractionConfig instance.
     *
     * @return ExtractionConfig The constructed configuration object
     */
    public function build(): ExtractionConfig
    {
        return new ExtractionConfig(
            ocr: $this->ocr,
            pdf: $this->pdf,
            chunking: $this->chunking,
            embedding: $this->embedding,
            imageExtraction: $this->imageExtraction,
            page: $this->page,
            languageDetection: $this->languageDetection,
            keywords: $this->keyword,
            extractImages: $this->extractImages,
            extractTables: $this->extractTables,
            preserveFormatting: $this->preserveFormatting,
            outputFormat: $this->outputFormat,
        );
    }
}

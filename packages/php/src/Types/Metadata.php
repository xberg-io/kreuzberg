<?php

declare(strict_types=1);

namespace Kreuzberg\Types;

/**
 * Document metadata.
 *
 * All fields are optional and depend on the file format, extraction configuration,
 * and postprocessors enabled.
 *
 * @property-read string|null $language Document language (ISO 639-1 code)
 * @property-read string|null $subject Document subject
 * @property-read string|null $formatType Format discriminator ("pdf", "excel", "email", etc.)
 * @property-read string|null $title Document title
 * @property-read array<string>|null $authors Document authors
 * @property-read array<string>|null $keywords Document keywords
 * @property-read string|null $createdAt Creation date (ISO 8601)
 * @property-read string|null $modifiedAt Modification date (ISO 8601)
 * @property-read string|null $createdBy Creator/application name
 * @property-read string|null $modifiedBy Modifier name
 * @property-read int|null $pageCount Number of pages
 * @property-read int|null $sheetCount Number of sheets (for spreadsheets)
 * @property-read string|null $format Image format (e.g., "PNG", "JPEG") for image documents
 * @property-read string|null $category Document category
 * @property-read array<string>|null $tags Document tags
 * @property-read string|null $documentVersion Document version
 * @property-read string|null $abstractText Document abstract text
 * @property-read string|null $outputFormat Output format used for extraction
 * @property-read array<string, mixed> $custom Additional custom metadata from postprocessors
 */
readonly class Metadata
{
    /**
     * @param array<string>|null $authors
     * @param array<string>|null $keywords
     * @param array<string>|null $tags
     * @param array<string, mixed> $custom
     */
    public function __construct(
        public ?string $language = null,
        public ?string $subject = null,
        public ?string $formatType = null,
        public ?string $title = null,
        public ?array $authors = null,
        public ?array $keywords = null,
        public ?string $createdAt = null,
        public ?string $modifiedAt = null,
        public ?string $createdBy = null,
        public ?string $modifiedBy = null,
        public ?int $pageCount = null,
        public ?int $sheetCount = null,
        public ?string $format = null,
        public ?string $category = null,
        public ?array $tags = null,
        public ?string $documentVersion = null,
        public ?string $abstractText = null,
        public ?string $outputFormat = null,
        public array $custom = [],
    ) {
    }

    /**
     * Create Metadata from array returned by extension.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $knownFields = [
            'language',
            'subject',
            'format_type',
            'title',
            'authors',
            'keywords',
            'created_at',
            'modified_at',
            'created_by',
            'modified_by',
            'page_count',
            'pageCount',
            'sheet_count',
            'format',
            'category',
            'tags',
            'document_version',
            'abstract_text',
            'output_format',
        ];

        /** @var string|null $language */
        $language = $data['language'] ?? null;

        /** @var string|null $subject */
        $subject = $data['subject'] ?? null;

        /** @var string|null $formatType */
        $formatType = $data['format_type'] ?? null;

        /** @var string|null $title */
        $title = $data['title'] ?? null;

        /** @var array<string>|null $authors */
        $authors = $data['authors'] ?? null;

        /** @var array<string>|null $keywords */
        $keywords = $data['keywords'] ?? null;

        /** @var string|null $createdAt */
        $createdAt = $data['created_at'] ?? null;

        /** @var string|null $modifiedAt */
        $modifiedAt = $data['modified_at'] ?? null;

        /** @var string|null $createdBy */
        $createdBy = $data['created_by'] ?? null;

        /** @var string|null $modifiedBy */
        $modifiedBy = $data['modified_by'] ?? null;

        /** @var int|null $pageCount */
        $pageCount = $data['page_count'] ?? $data['pageCount'] ?? null;

        /** @var int|null $sheetCount */
        $sheetCount = $data['sheet_count'] ?? null;

        /** @var string|null $format */
        $format = $data['format'] ?? null;

        /** @var string|null $category */
        $category = $data['category'] ?? null;

        /** @var array<string>|null $tags */
        $tags = $data['tags'] ?? null;

        /** @var string|null $documentVersion */
        $documentVersion = $data['document_version'] ?? null;

        /** @var string|null $abstractText */
        $abstractText = $data['abstract_text'] ?? null;

        /** @var string|null $outputFormat */
        $outputFormat = $data['output_format'] ?? null;

        $custom = [];
        foreach ($data as $key => $value) {
            if (!in_array($key, $knownFields, true)) {
                $custom[$key] = $value;
            }
        }

        return new self(
            language: $language,
            subject: $subject,
            formatType: $formatType,
            title: $title,
            authors: $authors,
            keywords: $keywords,
            createdAt: $createdAt,
            modifiedAt: $modifiedAt,
            createdBy: $createdBy,
            modifiedBy: $modifiedBy,
            pageCount: $pageCount,
            sheetCount: $sheetCount,
            format: $format,
            category: $category,
            tags: $tags,
            documentVersion: $documentVersion,
            abstractText: $abstractText,
            outputFormat: $outputFormat,
            custom: $custom,
        );
    }

    /**
     * Get a custom metadata field.
     *
     * @param string $key
     * @return mixed
     */
    public function getCustom(string $key): mixed
    {
        return $this->custom[$key] ?? null;
    }

    /**
     * Check if a custom metadata field exists.
     */
    public function hasCustom(string $key): bool
    {
        return isset($this->custom[$key]);
    }
}

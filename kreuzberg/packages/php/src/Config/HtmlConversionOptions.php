<?php

declare(strict_types=1);

namespace Kreuzberg\Config;

/**
 * HTML to Markdown conversion options.
 *
 * Configures how HTML documents are converted to Markdown, including
 * heading styles, list formatting, code block styles, and preprocessing.
 *
 * @example
 * ```php
 * use Kreuzberg\Config\HtmlConversionOptions;
 *
 * $options = new HtmlConversionOptions(
 *     headingStyle: 'atx',
 *     bullets: '-',
 *     codeBlockStyle: 'fenced',
 * );
 * ```
 */
readonly class HtmlConversionOptions
{
    public function __construct(
        /**
         * Style for markdown headings (e.g., "setext", "atx").
         *
         * @var string|null
         */
        public ?string $headingStyle = null,

        /**
         * Type of indentation for lists (e.g., "space", "tab").
         *
         * @var string|null
         */
        public ?string $listIndentType = null,

        /**
         * Width of list indentation.
         *
         * @var int|null
         */
        public ?int $listIndentWidth = null,

        /**
         * Bullet style for unordered lists (e.g., "-", "*", "+").
         *
         * @var string|null
         */
        public ?string $bullets = null,

        /**
         * List style (e.g., "dash", "asterisk", "plus").
         *
         * @var string|null
         */
        public ?string $listStyle = null,

        /**
         * List format (e.g., "unordered", "ordered").
         *
         * @var string|null
         */
        public ?string $listFormat = null,

        /**
         * Symbol for strong/emphasis text.
         *
         * @var string|null
         */
        public ?string $strongEmSymbol = null,

        /**
         * Whether to escape asterisks in output.
         *
         * @var bool|null
         */
        public ?bool $escapeAsterisks = null,

        /**
         * Whether to escape underscores in output.
         *
         * @var bool|null
         */
        public ?bool $escapeUnderscores = null,

        /**
         * Whether to escape miscellaneous characters.
         *
         * @var bool|null
         */
        public ?bool $escapeMisc = null,

        /**
         * Whether to escape ASCII control characters.
         *
         * @var bool|null
         */
        public ?bool $escapeAscii = null,

        /**
         * Language for code blocks syntax highlighting.
         *
         * @var string|null
         */
        public ?string $codeLanguage = null,

        /**
         * Whether to automatically convert URLs to hyperlinks.
         *
         * @var bool|null
         */
        public ?bool $autolinks = null,

        /**
         * Default title for documents without one.
         *
         * @var string|null
         */
        public ?string $defaultTitle = null,

        /**
         * Whether to use HTML line breaks in tables.
         *
         * @var bool|null
         */
        public ?bool $brInTables = null,

        /**
         * Whether to use hOCR spatial tables.
         *
         * @var bool|null
         */
        public ?bool $hocrSpatialTables = null,

        /**
         * Highlighting style for code blocks.
         *
         * @var string|null
         */
        public ?string $highlightStyle = null,

        /**
         * Whether to extract and include document metadata.
         *
         * @var bool|null
         */
        public ?bool $extractMetadata = null,

        /**
         * Whitespace handling mode (e.g., "preserve", "collapse").
         *
         * @var string|null
         */
        public ?string $whitespaceMode = null,

        /**
         * Whether to strip newlines from output.
         *
         * @var bool|null
         */
        public ?bool $stripNewlines = null,

        /**
         * Whether to wrap text output.
         *
         * @var bool|null
         */
        public ?bool $wrap = null,

        /**
         * Text wrapping width in characters.
         *
         * @var int|null
         */
        public ?int $wrapWidth = null,

        /**
         * Whether to convert HTML as inline content.
         *
         * @var bool|null
         */
        public ?bool $convertAsInline = null,

        /**
         * Symbol for subscript text.
         *
         * @var string|null
         */
        public ?string $subSymbol = null,

        /**
         * Symbol for superscript text.
         *
         * @var string|null
         */
        public ?string $supSymbol = null,

        /**
         * Newline style for output (e.g., "lf", "crlf").
         *
         * @var string|null
         */
        public ?string $newlineStyle = null,

        /**
         * Style for code blocks (e.g., "fenced", "indented").
         *
         * @var string|null
         */
        public ?string $codeBlockStyle = null,

        /**
         * Character encoding for output.
         *
         * @var string|null
         */
        public ?string $encoding = null,

        /**
         * Whether to include debug information in output.
         *
         * @var bool|null
         */
        public ?bool $debug = null,
    ) {
    }

    /**
     * Create configuration from array data.
     *
     * @param array<string, mixed> $data
     */
    public static function fromArray(array $data): self
    {
        $headingStyle = $data['heading_style'] ?? null;
        $listIndentType = $data['list_indent_type'] ?? null;
        $listIndentWidth = $data['list_indent_width'] ?? null;
        $bullets = $data['bullets'] ?? null;
        $listStyle = $data['list_style'] ?? null;
        $listFormat = $data['list_format'] ?? null;
        $strongEmSymbol = $data['strong_em_symbol'] ?? null;
        $escapeAsterisks = $data['escape_asterisks'] ?? null;
        $escapeUnderscores = $data['escape_underscores'] ?? null;
        $escapeMisc = $data['escape_misc'] ?? null;
        $escapeAscii = $data['escape_ascii'] ?? null;
        $codeLanguage = $data['code_language'] ?? null;
        $autolinks = $data['autolinks'] ?? null;
        $defaultTitle = $data['default_title'] ?? null;
        $brInTables = $data['br_in_tables'] ?? null;
        $hocrSpatialTables = $data['hocr_spatial_tables'] ?? null;
        $highlightStyle = $data['highlight_style'] ?? null;
        $extractMetadata = $data['extract_metadata'] ?? null;
        $whitespaceMode = $data['whitespace_mode'] ?? null;
        $stripNewlines = $data['strip_newlines'] ?? null;
        $wrap = $data['wrap'] ?? null;
        $wrapWidth = $data['wrap_width'] ?? null;
        $convertAsInline = $data['convert_as_inline'] ?? null;
        $subSymbol = $data['sub_symbol'] ?? null;
        $supSymbol = $data['sup_symbol'] ?? null;
        $newlineStyle = $data['newline_style'] ?? null;
        $codeBlockStyle = $data['code_block_style'] ?? null;
        $encoding = $data['encoding'] ?? null;
        $debug = $data['debug'] ?? null;

        return new self(
            headingStyle: is_string($headingStyle) ? $headingStyle : null,
            listIndentType: is_string($listIndentType) ? $listIndentType : null,
            listIndentWidth: is_int($listIndentWidth) ? $listIndentWidth : null,
            bullets: is_string($bullets) ? $bullets : null,
            listStyle: is_string($listStyle) ? $listStyle : null,
            listFormat: is_string($listFormat) ? $listFormat : null,
            strongEmSymbol: is_string($strongEmSymbol) ? $strongEmSymbol : null,
            escapeAsterisks: is_bool($escapeAsterisks) ? $escapeAsterisks : null,
            escapeUnderscores: is_bool($escapeUnderscores) ? $escapeUnderscores : null,
            escapeMisc: is_bool($escapeMisc) ? $escapeMisc : null,
            escapeAscii: is_bool($escapeAscii) ? $escapeAscii : null,
            codeLanguage: is_string($codeLanguage) ? $codeLanguage : null,
            autolinks: is_bool($autolinks) ? $autolinks : null,
            defaultTitle: is_string($defaultTitle) ? $defaultTitle : null,
            brInTables: is_bool($brInTables) ? $brInTables : null,
            hocrSpatialTables: is_bool($hocrSpatialTables) ? $hocrSpatialTables : null,
            highlightStyle: is_string($highlightStyle) ? $highlightStyle : null,
            extractMetadata: is_bool($extractMetadata) ? $extractMetadata : null,
            whitespaceMode: is_string($whitespaceMode) ? $whitespaceMode : null,
            stripNewlines: is_bool($stripNewlines) ? $stripNewlines : null,
            wrap: is_bool($wrap) ? $wrap : null,
            wrapWidth: is_int($wrapWidth) ? $wrapWidth : null,
            convertAsInline: is_bool($convertAsInline) ? $convertAsInline : null,
            subSymbol: is_string($subSymbol) ? $subSymbol : null,
            supSymbol: is_string($supSymbol) ? $supSymbol : null,
            newlineStyle: is_string($newlineStyle) ? $newlineStyle : null,
            codeBlockStyle: is_string($codeBlockStyle) ? $codeBlockStyle : null,
            encoding: is_string($encoding) ? $encoding : null,
            debug: is_bool($debug) ? $debug : null,
        );
    }

    /**
     * Convert configuration to array for FFI.
     *
     * @return array<string, mixed>
     */
    public function toArray(): array
    {
        return array_filter([
            'heading_style' => $this->headingStyle,
            'list_indent_type' => $this->listIndentType,
            'list_indent_width' => $this->listIndentWidth,
            'bullets' => $this->bullets,
            'list_style' => $this->listStyle,
            'list_format' => $this->listFormat,
            'strong_em_symbol' => $this->strongEmSymbol,
            'escape_asterisks' => $this->escapeAsterisks,
            'escape_underscores' => $this->escapeUnderscores,
            'escape_misc' => $this->escapeMisc,
            'escape_ascii' => $this->escapeAscii,
            'code_language' => $this->codeLanguage,
            'autolinks' => $this->autolinks,
            'default_title' => $this->defaultTitle,
            'br_in_tables' => $this->brInTables,
            'hocr_spatial_tables' => $this->hocrSpatialTables,
            'highlight_style' => $this->highlightStyle,
            'extract_metadata' => $this->extractMetadata,
            'whitespace_mode' => $this->whitespaceMode,
            'strip_newlines' => $this->stripNewlines,
            'wrap' => $this->wrap,
            'wrap_width' => $this->wrapWidth,
            'convert_as_inline' => $this->convertAsInline,
            'sub_symbol' => $this->subSymbol,
            'sup_symbol' => $this->supSymbol,
            'newline_style' => $this->newlineStyle,
            'code_block_style' => $this->codeBlockStyle,
            'encoding' => $this->encoding,
            'debug' => $this->debug,
        ], static fn ($value): bool => $value !== null);
    }
}

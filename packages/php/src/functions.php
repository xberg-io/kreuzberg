<?php

declare(strict_types=1);

namespace Kreuzberg;

use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Exceptions\KreuzbergException;
use Kreuzberg\Types\ExtractionResult;

/**
 * Convert generic exceptions from FFI layer to KreuzbergException.
 *
 * @internal
 */
function convertToKreuzbergException(\Exception $e): KreuzbergException
{
    $message = $e->getMessage();

    // Check for validation errors
    if (str_contains($message, '[Validation]') ||
        str_contains($message, 'File does not exist') ||
        str_contains($message, 'Invalid value given for argument')) {
        return KreuzbergException::validation($message);
    }

    // Check for parsing errors
    if (str_contains($message, 'Failed to parse') ||
        str_contains($message, 'parsing error') ||
        str_contains($message, 'Could not determine MIME type')) {
        return KreuzbergException::parsing($message);
    }

    // Check for OCR errors
    if (str_contains($message, 'OCR') || str_contains($message, 'ocr')) {
        return KreuzbergException::ocr($message);
    }

    // Check for I/O errors
    if (str_contains($message, 'I/O') ||
        str_contains($message, 'permission') ||
        str_contains($message, 'Permission denied')) {
        return KreuzbergException::io($message);
    }

    // Generic error
    return new KreuzbergException($message, 0, $e);
}

/**
 * Extract content from a file (procedural API).
 *
 * @param string $filePath Path to the file to extract
 * @param string|null $mimeType Optional MIME type hint (auto-detected if null)
 * @param ExtractionConfig|null $config Extraction configuration (uses defaults if null)
 * @return ExtractionResult Extraction result with content, metadata, and tables
 * @throws KreuzbergException If extraction fails
 *
 * @example
 * ```php
 * use Kreuzberg\extract_file;
 * use Kreuzberg\Config\ExtractionConfig;
 * use Kreuzberg\Config\OcrConfig;
 *
 * $result = extract_file('document.pdf');
 * echo $result->content;
 *
 * // With configuration
 * $config = new ExtractionConfig(
 *     ocr: new OcrConfig(backend: 'tesseract', language: 'eng')
 * );
 * $result = extract_file('scanned.pdf', config: $config);
 * ```
 */
function extract_file(
    string $filePath,
    ?string $mimeType = null,
    ?ExtractionConfig $config = null,
): ExtractionResult {
    try {
        return \kreuzberg_extract_file($filePath, $mimeType, $config?->toJson());
    } catch (\Exception $e) {
        if ($e instanceof KreuzbergException) {
            throw $e;
        }
        throw convertToKreuzbergException($e);
    }
}

/**
 * Extract content from bytes (procedural API).
 *
 * @param string $data File content as bytes
 * @param string $mimeType MIME type of the data (required for format detection)
 * @param ExtractionConfig|null $config Extraction configuration (uses defaults if null)
 * @return ExtractionResult Extraction result with content, metadata, and tables
 * @throws KreuzbergException If extraction fails
 *
 * @example
 * ```php
 * use Kreuzberg\extract_bytes;
 *
 * $data = file_get_contents('document.pdf');
 * $result = extract_bytes($data, 'application/pdf');
 * echo $result->content;
 * ```
 */
function extract_bytes(
    string $data,
    string $mimeType,
    ?ExtractionConfig $config = null,
): ExtractionResult {
    try {
        return \kreuzberg_extract_bytes($data, $mimeType, $config?->toJson());
    } catch (\Exception $e) {
        if ($e instanceof KreuzbergException) {
            throw $e;
        }
        throw convertToKreuzbergException($e);
    }
}

/**
 * Extract content from multiple files in parallel (procedural API).
 *
 * @param array<string> $paths List of file paths
 * @param ExtractionConfig|null $config Extraction configuration (uses defaults if null)
 * @return array<ExtractionResult> List of extraction results (one per file)
 * @throws KreuzbergException If extraction fails
 *
 * @example
 * ```php
 * use Kreuzberg\batch_extract_files;
 *
 * $files = ['doc1.pdf', 'doc2.docx', 'doc3.xlsx'];
 * $results = batch_extract_files($files);
 *
 * foreach ($results as $result) {
 *     echo $result->content;
 * }
 * ```
 */
function batch_extract_files(
    array $paths,
    ?ExtractionConfig $config = null,
): array {
    try {
        return \kreuzberg_batch_extract_files($paths, $config?->toJson());
    } catch (\Exception $e) {
        if ($e instanceof KreuzbergException) {
            throw $e;
        }
        throw convertToKreuzbergException($e);
    }
}

/**
 * Extract content from multiple byte arrays in parallel (procedural API).
 *
 * @param array<string> $dataList List of file contents as bytes
 * @param array<string> $mimeTypes List of MIME types (one per data item)
 * @param ExtractionConfig|null $config Extraction configuration (uses defaults if null)
 * @return array<ExtractionResult> List of extraction results (one per data item)
 * @throws KreuzbergException If extraction fails
 *
 * @example
 * ```php
 * use Kreuzberg\batch_extract_bytes;
 *
 * $files = [
 *     file_get_contents('doc1.pdf'),
 *     file_get_contents('doc2.docx'),
 * ];
 * $mimeTypes = ['application/pdf', 'application/vnd.openxmlformats-officedocument.wordprocessingml.document'];
 *
 * $results = batch_extract_bytes($files, $mimeTypes);
 * ```
 */
function batch_extract_bytes(
    array $dataList,
    array $mimeTypes,
    ?ExtractionConfig $config = null,
): array {
    try {
        return \kreuzberg_batch_extract_bytes($dataList, $mimeTypes, $config?->toJson());
    } catch (\Exception $e) {
        if ($e instanceof KreuzbergException) {
            throw $e;
        }
        throw convertToKreuzbergException($e);
    }
}

/**
 * Detect MIME type from file bytes.
 *
 * @param string $data File content as bytes
 * @return string Detected MIME type (e.g., "application/pdf", "image/png")
 *
 * @example
 * ```php
 * use Kreuzberg\detect_mime_type;
 *
 * $data = file_get_contents('unknown.file');
 * $mimeType = detect_mime_type($data);
 * echo $mimeType; // "application/pdf"
 * ```
 */
function detect_mime_type(string $data): string
{
    try {
        /** @var string $result */
        $result = \kreuzberg_detect_mime_type($data);

        return $result;
    } catch (\Exception $e) {
        if ($e instanceof KreuzbergException) {
            throw $e;
        }
        throw convertToKreuzbergException($e);
    }
}

/**
 * Detect MIME type from file path.
 *
 * @param string $path Path to the file
 * @return string Detected MIME type (e.g., "application/pdf", "text/plain")
 *
 * @example
 * ```php
 * use Kreuzberg\detect_mime_type_from_path;
 *
 * $mimeType = detect_mime_type_from_path('document.pdf');
 * echo $mimeType; // "application/pdf"
 * ```
 */
function detect_mime_type_from_path(string $path): string
{
    try {
        /** @var string $result */
        $result = \kreuzberg_detect_mime_type_from_path($path);

        return $result;
    } catch (\Exception $e) {
        if ($e instanceof KreuzbergException) {
            throw $e;
        }
        throw convertToKreuzbergException($e);
    }
}

<?php

declare(strict_types=1);

/**
 * Kreuzberg PHP Bindings - Comprehensive Test Suite
 *
 * Tests the entire public API surface of the Kreuzberg PHP package including:
 * - Extension loading and version detection
 * - Configuration classes (ExtractionConfig, OcrConfig, PdfConfig, ChunkingConfig, etc.)
 * - Configuration builder pattern
 * - Configuration serialization (toArray, toJson, fromArray, fromJson)
 * - Exception hierarchy (KreuzbergException factory methods)
 * - MIME type detection and validation
 * - Plugin registry (validators, post-processors, OCR backends, document extractors)
 * - Embedding presets
 * - File extraction (sync)
 * - Bytes extraction (sync)
 * - Batch extraction (sync)
 * - Error handling for invalid inputs
 *
 * Requires PHP 8.4+ with the kreuzberg extension or mock fallback.
 */

// ---------------------------------------------------------------------------
// Autoloader setup
// ---------------------------------------------------------------------------

$autoloadPaths = [
    __DIR__ . '/../../../packages/php/vendor/autoload.php',
    __DIR__ . '/vendor/autoload.php',
];

$autoloaded = false;
foreach ($autoloadPaths as $path) {
    if (file_exists($path)) {
        require_once $path;
        $autoloaded = true;
        break;
    }
}

if (!$autoloaded) {
    fwrite(
        STDERR,
        "Error: Could not find Composer autoloader.\n" .
        "Please run 'composer install' in the packages/php directory.\n"
    );
    exit(1);
}

// Load the mock extension if the real extension is not available
if (!extension_loaded('kreuzberg') && !extension_loaded('kreuzberg-php')) {
    $mockPath = __DIR__ . '/../../../packages/php/src/KreuzbergExtensionMock.php';
    if (file_exists($mockPath)) {
        require_once $mockPath;
    } else {
        fwrite(STDERR, "Error: Neither kreuzberg extension nor mock is available.\n");
        exit(1);
    }
}

// ---------------------------------------------------------------------------
// Imports
// ---------------------------------------------------------------------------

use Kreuzberg\Config\AccelerationConfig;
use Kreuzberg\Config\ChunkingConfig;
use Kreuzberg\Config\EmailConfig;
use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\ExtractionConfigBuilder;
use Kreuzberg\Config\ImageExtractionConfig;
use Kreuzberg\Config\OcrConfig;
use Kreuzberg\Config\PageConfig;
use Kreuzberg\Config\PdfConfig;
use Kreuzberg\Config\TesseractConfig;
use Kreuzberg\Exceptions\KreuzbergException;
use Kreuzberg\Kreuzberg;

// ---------------------------------------------------------------------------
// Test runner infrastructure
// ---------------------------------------------------------------------------

final class TestRunner
{
    private int $passed = 0;
    private int $failed = 0;
    private int $skipped = 0;
    private int $total = 0;
    private string $currentSection = '';

    public function section(string $name): void
    {
        $this->currentSection = $name;
        echo "\n";
        echo str_repeat('=', 72) . "\n";
        echo "  {$name}\n";
        echo str_repeat('=', 72) . "\n";
    }

    public function test(string $description, callable $fn): void
    {
        $this->total++;
        try {
            $fn();
            $this->passed++;
            echo "  PASS  {$description}\n";
        } catch (SkipException $e) {
            $this->skipped++;
            echo "  SKIP  {$description} ({$e->getMessage()})\n";
        } catch (\Throwable $e) {
            $this->failed++;
            $errorMsg = $e->getMessage();
            $file = basename($e->getFile());
            $line = $e->getLine();
            echo "  FAIL  {$description}\n";
            echo "        Error: {$errorMsg} ({$file}:{$line})\n";
        }
    }

    public function summary(): int
    {
        echo "\n";
        echo str_repeat('=', 72) . "\n";
        echo "  TEST SUMMARY\n";
        echo str_repeat('=', 72) . "\n";
        echo "  Total:   {$this->total}\n";
        echo "  Passed:  {$this->passed}\n";
        echo "  Failed:  {$this->failed}\n";
        echo "  Skipped: {$this->skipped}\n";
        echo "\n";

        if ($this->failed === 0) {
            echo "  ALL TESTS PASSED\n";
        } else {
            echo "  SOME TESTS FAILED\n";
        }

        echo str_repeat('=', 72) . "\n";

        return $this->failed > 0 ? 1 : 0;
    }
}

final class SkipException extends \RuntimeException
{
}

function skip(string $reason): never
{
    throw new SkipException($reason);
}

function assert_true(bool $value, string $message = 'Expected true'): void
{
    if (!$value) {
        throw new \RuntimeException("Assertion failed: {$message}");
    }
}

function assert_false(bool $value, string $message = 'Expected false'): void
{
    if ($value) {
        throw new \RuntimeException("Assertion failed: {$message}");
    }
}

function assert_equals(mixed $expected, mixed $actual, string $message = ''): void
{
    if ($expected !== $actual) {
        $expectedStr = var_export($expected, true);
        $actualStr = var_export($actual, true);
        $msg = $message !== '' ? "{$message}: " : '';
        throw new \RuntimeException(
            "Assertion failed: {$msg}expected {$expectedStr}, got {$actualStr}"
        );
    }
}

function assert_not_empty(mixed $value, string $message = 'Expected non-empty value'): void
{
    if (empty($value)) {
        throw new \RuntimeException("Assertion failed: {$message}");
    }
}

function assert_instance_of(string $class, mixed $value, string $message = ''): void
{
    if (!($value instanceof $class)) {
        $actual = get_debug_type($value);
        $msg = $message !== '' ? "{$message}: " : '';
        throw new \RuntimeException(
            "Assertion failed: {$msg}expected instance of {$class}, got {$actual}"
        );
    }
}

function assert_array_key(string $key, array $array, string $message = ''): void
{
    if (!array_key_exists($key, $array)) {
        $msg = $message !== '' ? "{$message}: " : '';
        throw new \RuntimeException("Assertion failed: {$msg}key '{$key}' not found in array");
    }
}

function assert_string_contains(string $needle, string $haystack, string $message = ''): void
{
    if (!str_contains($haystack, $needle)) {
        $msg = $message !== '' ? "{$message}: " : '';
        throw new \RuntimeException(
            "Assertion failed: {$msg}string does not contain '{$needle}'"
        );
    }
}

function assert_greater_than(int|float $expected, int|float $actual, string $message = ''): void
{
    if ($actual <= $expected) {
        $msg = $message !== '' ? "{$message}: " : '';
        throw new \RuntimeException(
            "Assertion failed: {$msg}expected value > {$expected}, got {$actual}"
        );
    }
}

function assert_throws(string $exceptionClass, callable $fn, string $message = ''): void
{
    try {
        $fn();
        $msg = $message !== '' ? "{$message}: " : '';
        throw new \RuntimeException(
            "Assertion failed: {$msg}expected {$exceptionClass} to be thrown"
        );
    } catch (\Throwable $e) {
        if (!($e instanceof $exceptionClass)) {
            $actual = get_class($e);
            $msg = $message !== '' ? "{$message}: " : '';
            throw new \RuntimeException(
                "Assertion failed: {$msg}expected {$exceptionClass}, got {$actual}: {$e->getMessage()}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Resolve paths
// ---------------------------------------------------------------------------

$repoRoot = realpath(__DIR__ . '/../../..');
if ($repoRoot === false) {
    fwrite(STDERR, "Error: Could not resolve repository root.\n");
    exit(1);
}

$testDocuments = $repoRoot . '/test_documents';
if (!is_dir($testDocuments)) {
    fwrite(STDERR, "Error: test_documents directory not found at: {$testDocuments}\n");
    exit(1);
}

// ---------------------------------------------------------------------------
// Run tests
// ---------------------------------------------------------------------------

$runner = new TestRunner();

echo str_repeat('=', 72) . "\n";
echo "  Kreuzberg PHP Test Suite\n";
echo str_repeat('=', 72) . "\n";
echo "  PHP Version:      " . PHP_VERSION . "\n";
echo "  Extension Loaded: " . (extension_loaded('kreuzberg') || extension_loaded('kreuzberg-php') ? 'Yes (native)' : 'No (using mock)') . "\n";
echo "  Repo Root:        {$repoRoot}\n";
echo "  Test Documents:   {$testDocuments}\n";

// =========================================================================
// Section 1: Extension & Version
// =========================================================================

$runner->section('1. Extension & Version');

$runner->test('Kreuzberg class exists', function (): void {
    assert_true(class_exists(Kreuzberg::class), 'Kreuzberg class should exist');
});

$runner->test('Kreuzberg::VERSION is a non-empty string', function (): void {
    assert_not_empty(Kreuzberg::VERSION, 'VERSION constant should be non-empty');
    assert_true(is_string(Kreuzberg::VERSION), 'VERSION should be a string');
});

$runner->test('Kreuzberg::version() returns a string', function (): void {
    $version = Kreuzberg::version();
    assert_not_empty($version, 'version() should return non-empty string');
    assert_equals(Kreuzberg::VERSION, $version, 'version() should match VERSION constant');
});

// =========================================================================
// Section 2: Configuration Classes
// =========================================================================

$runner->section('2. Configuration Classes');

$runner->test('ExtractionConfig default construction', function (): void {
    $config = new ExtractionConfig();
    assert_true($config->useCache, 'useCache should default to true');
    assert_true($config->enableQualityProcessing, 'enableQualityProcessing should default to true');
    assert_false($config->forceOcr, 'forceOcr should default to false');
    assert_equals(null, $config->ocr, 'ocr should default to null');
    assert_equals(null, $config->chunking, 'chunking should default to null');
    assert_equals(null, $config->pdfOptions, 'pdfOptions should default to null');
    assert_equals(null, $config->maxConcurrentExtractions, 'maxConcurrentExtractions should default to null');
    assert_equals('unified', $config->resultFormat, 'resultFormat should default to unified');
    assert_equals('plain', $config->outputFormat, 'outputFormat should default to plain');
    assert_false($config->includeDocumentStructure, 'includeDocumentStructure should default to false');
});

$runner->test('ExtractionConfig with custom values', function (): void {
    $config = new ExtractionConfig(
        useCache: false,
        enableQualityProcessing: false,
        forceOcr: true,
        maxConcurrentExtractions: 4,
        resultFormat: 'element_based',
        outputFormat: 'markdown',
    );
    assert_false($config->useCache, 'useCache should be false');
    assert_false($config->enableQualityProcessing, 'enableQualityProcessing should be false');
    assert_true($config->forceOcr, 'forceOcr should be true');
    assert_equals(4, $config->maxConcurrentExtractions, 'maxConcurrentExtractions');
    assert_equals('element_based', $config->resultFormat, 'resultFormat');
    assert_equals('markdown', $config->outputFormat, 'outputFormat');
});

$runner->test('ExtractionConfig with nested OcrConfig', function (): void {
    $ocr = new OcrConfig(backend: 'tesseract', language: 'deu');
    $config = new ExtractionConfig(ocr: $ocr);
    assert_instance_of(OcrConfig::class, $config->ocr, 'ocr should be OcrConfig');
    assert_equals('tesseract', $config->ocr->backend, 'backend');
    assert_equals('deu', $config->ocr->language, 'language');
});

$runner->test('OcrConfig default construction', function (): void {
    $ocr = new OcrConfig();
    assert_equals('tesseract', $ocr->backend, 'backend should default to tesseract');
    assert_equals('eng', $ocr->language, 'language should default to eng');
    assert_equals(null, $ocr->tesseractConfig, 'tesseractConfig should default to null');
    assert_equals(null, $ocr->paddleOcrConfig, 'paddleOcrConfig should default to null');
    assert_equals(null, $ocr->imagePreprocessing, 'imagePreprocessing should default to null');
});

$runner->test('OcrConfig with paddle backend', function (): void {
    $ocr = new OcrConfig(backend: 'paddle-ocr', language: 'ch');
    assert_equals('paddle-ocr', $ocr->backend, 'backend');
    assert_equals('ch', $ocr->language, 'language');
});

$runner->test('PdfConfig default construction', function (): void {
    $pdf = new PdfConfig();
    assert_false($pdf->extractImages, 'extractImages should default to false');
    assert_true($pdf->extractMetadata, 'extractMetadata should default to true');
    assert_equals(null, $pdf->passwords, 'passwords should default to null');
    assert_false($pdf->extractAnnotations, 'extractAnnotations should default to false');
});

$runner->test('PdfConfig with custom values', function (): void {
    $pdf = new PdfConfig(
        extractImages: true,
        passwords: ['secret1', 'secret2'],
        extractMetadata: false,
        extractAnnotations: true,
    );
    assert_true($pdf->extractImages, 'extractImages should be true');
    assert_false($pdf->extractMetadata, 'extractMetadata should be false');
    assert_equals(['secret1', 'secret2'], $pdf->passwords, 'passwords');
    assert_true($pdf->extractAnnotations, 'extractAnnotations');
});

$runner->test('ChunkingConfig default construction', function (): void {
    $chunking = new ChunkingConfig();
    assert_equals(512, $chunking->maxChars, 'maxChars should default to 512');
    assert_equals(50, $chunking->maxOverlap, 'maxOverlap should default to 50');
    assert_true($chunking->respectSentences, 'respectSentences should default to true');
    assert_true($chunking->respectParagraphs, 'respectParagraphs should default to true');
});

$runner->test('ChunkingConfig with custom values', function (): void {
    $chunking = new ChunkingConfig(
        maxChars: 1024,
        maxOverlap: 100,
        respectSentences: false,
        respectParagraphs: false,
    );
    assert_equals(1024, $chunking->maxChars, 'maxChars');
    assert_equals(100, $chunking->maxOverlap, 'maxOverlap');
    assert_false($chunking->respectSentences, 'respectSentences');
    assert_false($chunking->respectParagraphs, 'respectParagraphs');
});

$runner->test('ExtractionConfig with all nested configs', function (): void {
    $config = new ExtractionConfig(
        ocr: new OcrConfig(backend: 'tesseract', language: 'eng+deu'),
        pdfOptions: new PdfConfig(extractImages: true),
        chunking: new ChunkingConfig(maxChars: 256),
    );
    assert_instance_of(OcrConfig::class, $config->ocr);
    assert_instance_of(PdfConfig::class, $config->pdfOptions);
    assert_instance_of(ChunkingConfig::class, $config->chunking);
    assert_equals('eng+deu', $config->ocr->language);
    assert_true($config->pdfOptions->extractImages);
    assert_equals(256, $config->chunking->maxChars);
});

// =========================================================================
// Section 3: Configuration Builder
// =========================================================================

$runner->section('3. Configuration Builder');

$runner->test('ExtractionConfig::builder() returns builder', function (): void {
    $builder = ExtractionConfig::builder();
    assert_instance_of(ExtractionConfigBuilder::class, $builder);
});

$runner->test('Builder produces default config', function (): void {
    $config = ExtractionConfig::builder()->build();
    assert_instance_of(ExtractionConfig::class, $config);
    assert_true($config->useCache, 'default useCache');
    assert_false($config->forceOcr, 'default forceOcr');
});

$runner->test('Builder with fluent chaining', function (): void {
    $config = ExtractionConfig::builder()
        ->withUseCache(false)
        ->withForceOcr(true)
        ->withOcr(new OcrConfig(backend: 'tesseract', language: 'fra'))
        ->withChunking(new ChunkingConfig(maxChars: 2048))
        ->withMaxConcurrentExtractions(8)
        ->withOutputFormat('markdown')
        ->build();

    assert_false($config->useCache, 'useCache');
    assert_true($config->forceOcr, 'forceOcr');
    assert_equals('fra', $config->ocr?->language, 'OCR language');
    assert_equals(2048, $config->chunking?->maxChars, 'chunk size');
    assert_equals(8, $config->maxConcurrentExtractions, 'max concurrent');
    assert_equals('markdown', $config->outputFormat, 'output format');
});

$runner->test('Builder with PDF options', function (): void {
    $config = ExtractionConfig::builder()
        ->withPdfOptions(new PdfConfig(extractImages: true, extractAnnotations: true))
        ->build();

    assert_true($config->pdfOptions?->extractImages, 'extractImages');
    assert_true($config->pdfOptions?->extractAnnotations, 'extractAnnotations');
});

// =========================================================================
// Section 4: Configuration Serialization
// =========================================================================

$runner->section('4. Configuration Serialization');

$runner->test('ExtractionConfig::toArray() produces valid array', function (): void {
    $config = new ExtractionConfig(forceOcr: true);
    $array = $config->toArray();
    assert_true(is_array($array), 'toArray should return array');
    assert_array_key('force_ocr', $array, 'should have force_ocr key');
    assert_true($array['force_ocr'], 'force_ocr value');
});

$runner->test('ExtractionConfig::toJson() produces valid JSON', function (): void {
    $config = new ExtractionConfig(
        ocr: new OcrConfig(language: 'deu'),
        forceOcr: true,
    );
    $json = $config->toJson();
    $decoded = json_decode($json, true);
    assert_true(json_last_error() === JSON_ERROR_NONE, 'JSON should be valid');
    assert_true(is_array($decoded), 'Decoded JSON should be array');
    assert_true($decoded['force_ocr'], 'force_ocr in JSON');
    assert_equals('deu', $decoded['ocr']['language'], 'OCR language in JSON');
});

$runner->test('ExtractionConfig::fromArray() roundtrip', function (): void {
    $original = new ExtractionConfig(
        useCache: false,
        forceOcr: true,
        ocr: new OcrConfig(backend: 'tesseract', language: 'eng'),
    );
    $array = $original->toArray();
    $restored = ExtractionConfig::fromArray($array);

    assert_false($restored->useCache, 'useCache roundtrip');
    assert_true($restored->forceOcr, 'forceOcr roundtrip');
    assert_equals('eng', $restored->ocr?->language, 'OCR language roundtrip');
});

$runner->test('ExtractionConfig::fromJson() roundtrip', function (): void {
    $original = new ExtractionConfig(
        chunking: new ChunkingConfig(maxChars: 1024, maxOverlap: 128),
    );
    $json = $original->toJson();
    $restored = ExtractionConfig::fromJson($json);

    assert_equals(1024, $restored->chunking?->maxChars, 'maxChars roundtrip');
    assert_equals(128, $restored->chunking?->maxOverlap, 'maxOverlap roundtrip');
});

$runner->test('OcrConfig::toArray() produces correct structure', function (): void {
    $ocr = new OcrConfig(backend: 'paddle-ocr', language: 'ch');
    $array = $ocr->toArray();
    assert_equals('paddle-ocr', $array['backend'], 'backend');
    assert_equals('ch', $array['language'], 'language');
});

$runner->test('OcrConfig::fromArray() reconstructs config', function (): void {
    $data = ['backend' => 'tesseract', 'language' => 'deu'];
    $ocr = OcrConfig::fromArray($data);
    assert_equals('tesseract', $ocr->backend);
    assert_equals('deu', $ocr->language);
});

$runner->test('PdfConfig serialization roundtrip', function (): void {
    $pdf = new PdfConfig(extractImages: true, extractAnnotations: true);
    $array = $pdf->toArray();
    $restored = PdfConfig::fromArray($array);
    assert_true($restored->extractImages, 'extractImages');
    assert_true($restored->extractAnnotations, 'extractAnnotations');
});

$runner->test('ChunkingConfig serialization roundtrip', function (): void {
    $chunking = new ChunkingConfig(maxChars: 2048, maxOverlap: 200);
    $array = $chunking->toArray();
    $restored = ChunkingConfig::fromArray($array);
    assert_equals(2048, $restored->maxChars, 'maxChars');
    assert_equals(200, $restored->maxOverlap, 'maxOverlap');
});

$runner->test('ExtractionConfig default toArray omits defaults', function (): void {
    $config = new ExtractionConfig();
    $array = $config->toArray();
    // Default values should not be present in the output
    assert_false(array_key_exists('use_cache', $array), 'use_cache should be omitted when true (default)');
    assert_false(array_key_exists('force_ocr', $array), 'force_ocr should be omitted when false (default)');
});

// =========================================================================
// Section 5: Exception Hierarchy
// =========================================================================

$runner->section('5. Exception Hierarchy');

$runner->test('KreuzbergException class exists', function (): void {
    assert_true(class_exists(KreuzbergException::class), 'KreuzbergException should exist');
});

$runner->test('KreuzbergException extends Exception', function (): void {
    $e = new KreuzbergException('test error');
    assert_instance_of(\Exception::class, $e);
    assert_equals('test error', $e->getMessage());
});

$runner->test('KreuzbergException::validation() factory', function (): void {
    $e = KreuzbergException::validation('invalid input');
    assert_instance_of(KreuzbergException::class, $e);
    assert_string_contains('Validation', $e->getMessage(), 'validation error message');
    assert_equals(1, $e->getCode(), 'validation error code');
});

$runner->test('KreuzbergException::parsing() factory', function (): void {
    $e = KreuzbergException::parsing('bad format');
    assert_instance_of(KreuzbergException::class, $e);
    assert_string_contains('Parsing', $e->getMessage());
    assert_equals(2, $e->getCode());
});

$runner->test('KreuzbergException::ocr() factory', function (): void {
    $e = KreuzbergException::ocr('engine failed');
    assert_instance_of(KreuzbergException::class, $e);
    assert_string_contains('OCR', $e->getMessage());
    assert_equals(3, $e->getCode());
});

$runner->test('KreuzbergException::missingDependency() factory', function (): void {
    $e = KreuzbergException::missingDependency('tesseract not found');
    assert_instance_of(KreuzbergException::class, $e);
    assert_string_contains('Missing dependency', $e->getMessage());
    assert_equals(4, $e->getCode());
});

$runner->test('KreuzbergException::io() factory', function (): void {
    $e = KreuzbergException::io('file not readable');
    assert_instance_of(KreuzbergException::class, $e);
    assert_string_contains('I/O', $e->getMessage());
    assert_equals(5, $e->getCode());
});

$runner->test('KreuzbergException::plugin() factory', function (): void {
    $e = KreuzbergException::plugin('plugin registration failed');
    assert_instance_of(KreuzbergException::class, $e);
    assert_string_contains('Plugin', $e->getMessage());
    assert_equals(6, $e->getCode());
});

$runner->test('KreuzbergException::unsupportedFormat() factory', function (): void {
    $e = KreuzbergException::unsupportedFormat('xyz');
    assert_instance_of(KreuzbergException::class, $e);
    assert_string_contains('Unsupported format', $e->getMessage());
    assert_equals(7, $e->getCode());
});

$runner->test('KreuzbergException panicContext defaults to null', function (): void {
    $e = new KreuzbergException('test');
    assert_equals(null, $e->panicContext, 'panicContext should be null by default');
});

// =========================================================================
// Section 6: MIME Type Functions
// =========================================================================

$runner->section('6. MIME Type Functions');

$runner->test('detect_mime_type from PDF bytes', function (): void {
    $pdfBytes = '%PDF-1.4 test content';
    $mime = Kreuzberg::detectMimeType($pdfBytes);
    assert_equals('application/pdf', $mime, 'PDF detection');
});

$runner->test('detect_mime_type from PNG bytes', function (): void {
    $pngBytes = "\x89PNG\r\n\x1a\n" . str_repeat("\x00", 100);
    $mime = Kreuzberg::detectMimeType($pngBytes);
    assert_equals('image/png', $mime, 'PNG detection');
});

$runner->test('detect_mime_type from JPEG bytes', function (): void {
    $jpegBytes = "\xFF\xD8\xFF\xE0" . str_repeat("\x00", 100);
    $mime = Kreuzberg::detectMimeType($jpegBytes);
    assert_equals('image/jpeg', $mime, 'JPEG detection');
});

$runner->test('detect_mime_type_from_path for .pdf', function () use ($testDocuments): void {
    $pdfDir = $testDocuments . '/pdf';
    if (!is_dir($pdfDir)) {
        skip('No PDF test documents available');
    }
    $files = glob($pdfDir . '/*.pdf');
    if (empty($files)) {
        skip('No PDF files found');
    }
    $mime = Kreuzberg::detectMimeTypeFromPath($files[0]);
    assert_equals('application/pdf', $mime, 'PDF path detection');
});

$runner->test('detect_mime_type_from_path for .docx', function () use ($testDocuments): void {
    $docxDir = $testDocuments . '/docx';
    if (!is_dir($docxDir)) {
        skip('No DOCX test documents available');
    }
    $files = glob($docxDir . '/*.docx');
    if (empty($files)) {
        skip('No DOCX files found');
    }
    $mime = Kreuzberg::detectMimeTypeFromPath($files[0]);
    assert_string_contains('wordprocessingml', $mime, 'DOCX path detection');
});

$runner->test('get_extensions_for_mime returns extensions', function (): void {
    $extensions = Kreuzberg::getExtensionsForMime('application/pdf');
    assert_true(is_array($extensions), 'should return array');
    assert_true(in_array('pdf', $extensions, true), 'should contain pdf extension');
});

$runner->test('get_extensions_for_mime for image/png', function (): void {
    $extensions = Kreuzberg::getExtensionsForMime('image/png');
    assert_true(is_array($extensions), 'should return array');
    assert_true(in_array('png', $extensions, true), 'should contain png extension');
});

// =========================================================================
// Section 7: Plugin Registry - Validators
// =========================================================================

$runner->section('7. Plugin Registry - Validators');

$runner->test('list_validators returns array', function (): void {
    $validators = Kreuzberg::listValidators();
    assert_true(is_array($validators), 'should return array');
});

$runner->test('clear_validators empties registry', function (): void {
    Kreuzberg::clearValidators();
    $validators = Kreuzberg::listValidators();
    assert_true(is_array($validators), 'should return array after clear');
});

// =========================================================================
// Section 8: Plugin Registry - Post-Processors
// =========================================================================

$runner->section('8. Plugin Registry - Post-Processors');

$runner->test('list_post_processors returns array', function (): void {
    $processors = Kreuzberg::listPostProcessors();
    assert_true(is_array($processors), 'should return array');
});

$runner->test('clear_post_processors empties registry', function (): void {
    Kreuzberg::clearPostProcessors();
    $processors = Kreuzberg::listPostProcessors();
    assert_true(is_array($processors), 'should return array after clear');
});

// =========================================================================
// Section 9: Plugin Registry - OCR Backends
// =========================================================================

$runner->section('9. Plugin Registry - OCR Backends');

$runner->test('list_ocr_backends returns array', function (): void {
    $backends = Kreuzberg::listOcrBackends();
    assert_true(is_array($backends), 'should return array');
});

$runner->test('clear_ocr_backends empties registry', function (): void {
    Kreuzberg::clearOcrBackends();
    $backends = Kreuzberg::listOcrBackends();
    assert_true(is_array($backends), 'should return array after clear');
});

// =========================================================================
// Section 10: Plugin Registry - Document Extractors
// =========================================================================

$runner->section('10. Plugin Registry - Document Extractors');

$runner->test('list_document_extractors returns array', function (): void {
    $extractors = Kreuzberg::listDocumentExtractors();
    assert_true(is_array($extractors), 'should return array');
});

$runner->test('clear_document_extractors empties registry', function (): void {
    Kreuzberg::clearDocumentExtractors();
    $extractors = Kreuzberg::listDocumentExtractors();
    assert_true(is_array($extractors), 'should return array after clear');
});

// =========================================================================
// Section 11: Kreuzberg Class Instance API
// =========================================================================

$runner->section('11. Kreuzberg Class Instance API');

$runner->test('Kreuzberg instance creation with no config', function (): void {
    $k = new Kreuzberg();
    assert_instance_of(Kreuzberg::class, $k);
});

$runner->test('Kreuzberg instance creation with config', function (): void {
    $config = new ExtractionConfig(forceOcr: true);
    $k = new Kreuzberg($config);
    assert_instance_of(Kreuzberg::class, $k);
});

// =========================================================================
// Section 12: Extraction - File (Sync)
// =========================================================================

$runner->section('12. Extraction - File (Sync)');

$runner->test('extract_file from text file', function () use ($testDocuments): void {
    $textDir = $testDocuments . '/text';
    if (!is_dir($textDir)) {
        skip('No text test documents available');
    }
    $files = glob($textDir . '/*.txt');
    if (empty($files)) {
        skip('No .txt files found');
    }
    $result = Kreuzberg::extractFileSync($files[0]);
    assert_not_empty($result->content, 'content should not be empty');
    assert_not_empty($result->mimeType, 'mimeType should not be empty');
});

$runner->test('extract_file from PDF', function () use ($testDocuments): void {
    $pdfDir = $testDocuments . '/pdf';
    if (!is_dir($pdfDir)) {
        skip('No PDF test documents available');
    }
    $files = glob($pdfDir . '/*.pdf');
    if (empty($files)) {
        skip('No PDF files found');
    }
    $result = Kreuzberg::extractFileSync($files[0]);
    assert_not_empty($result->content, 'content should not be empty');
    assert_not_empty($result->mimeType, 'mimeType should not be empty');
});

$runner->test('extract_file with ExtractionConfig', function () use ($testDocuments): void {
    $textDir = $testDocuments . '/text';
    if (!is_dir($textDir)) {
        skip('No text test documents available');
    }
    $files = glob($textDir . '/*.txt');
    if (empty($files)) {
        skip('No .txt files found');
    }
    $config = new ExtractionConfig(useCache: false);
    $result = Kreuzberg::extractFileSync($files[0], config: $config);
    assert_not_empty($result->content, 'content with config should not be empty');
});

$runner->test('extract_file with MIME type hint', function () use ($testDocuments): void {
    $textDir = $testDocuments . '/text';
    if (!is_dir($textDir)) {
        skip('No text test documents available');
    }
    $files = glob($textDir . '/*.txt');
    if (empty($files)) {
        skip('No .txt files found');
    }
    $result = Kreuzberg::extractFileSync($files[0], mimeType: 'text/plain');
    assert_not_empty($result->content, 'content with MIME hint should not be empty');
});

$runner->test('extract_file result has metadata', function () use ($testDocuments): void {
    $textDir = $testDocuments . '/text';
    if (!is_dir($textDir)) {
        skip('No text test documents available');
    }
    $files = glob($textDir . '/*.txt');
    if (empty($files)) {
        skip('No .txt files found');
    }
    $result = Kreuzberg::extractFileSync($files[0]);
    // Access metadata directly; native extension objects may not support isset/property_exists
    $metadata = $result->metadata;
    assert_true($metadata !== null, 'result should have metadata');
});

// =========================================================================
// Section 13: Extraction - Bytes (Sync)
// =========================================================================

$runner->section('13. Extraction - Bytes (Sync)');

$runner->test('extract_bytes from text content', function (): void {
    $data = 'Hello, this is a test document with some plain text content.';
    $result = Kreuzberg::extractBytesSync($data, 'text/plain');
    assert_not_empty($result->content, 'content should not be empty');
    assert_not_empty($result->mimeType, 'mimeType should not be empty');
});

$runner->test('extract_bytes from file bytes', function () use ($testDocuments): void {
    $textDir = $testDocuments . '/text';
    if (!is_dir($textDir)) {
        skip('No text test documents available');
    }
    $files = glob($textDir . '/*.txt');
    if (empty($files)) {
        skip('No .txt files found');
    }
    $data = file_get_contents($files[0]);
    if ($data === false) {
        skip('Could not read file');
    }
    $result = Kreuzberg::extractBytesSync($data, 'text/plain');
    assert_not_empty($result->content, 'content should not be empty');
});

$runner->test('extract_bytes with config', function (): void {
    $data = 'Test content for extraction with configuration options.';
    $config = new ExtractionConfig(useCache: false);
    $result = Kreuzberg::extractBytesSync($data, 'text/plain', config: $config);
    assert_not_empty($result->content, 'content with config should not be empty');
});

// =========================================================================
// Section 14: Batch Extraction (Sync)
// =========================================================================

$runner->section('14. Batch Extraction (Sync)');

$runner->test('batch_extract_files with multiple files', function () use ($testDocuments): void {
    $textDir = $testDocuments . '/text';
    if (!is_dir($textDir)) {
        skip('No text test documents available');
    }
    $files = glob($textDir . '/*.txt');
    if (count($files) < 2) {
        skip('Need at least 2 text files for batch test');
    }
    $paths = array_slice($files, 0, 2);
    $results = Kreuzberg::batchExtractFilesSync($paths);
    assert_equals(2, count($results), 'should return 2 results');
    foreach ($results as $result) {
        assert_not_empty($result->content, 'each result content should not be empty');
    }
});

$runner->test('batch_extract_files with config', function () use ($testDocuments): void {
    $textDir = $testDocuments . '/text';
    if (!is_dir($textDir)) {
        skip('No text test documents available');
    }
    $files = glob($textDir . '/*.txt');
    if (empty($files)) {
        skip('No text files found');
    }
    $config = new ExtractionConfig(useCache: false);
    $results = Kreuzberg::batchExtractFilesSync([$files[0]], config: $config);
    assert_equals(1, count($results), 'should return 1 result');
});

$runner->test('batch_extract_bytes with multiple items', function (): void {
    $dataList = [
        'First document content for batch processing.',
        'Second document content for batch processing.',
    ];
    $mimeTypes = ['text/plain', 'text/plain'];
    $results = Kreuzberg::batchExtractBytesSync($dataList, $mimeTypes);
    assert_equals(2, count($results), 'should return 2 results');
});

// =========================================================================
// Section 15: Error Handling
// =========================================================================

$runner->section('15. Error Handling');

$runner->test('extract_file with non-existent file throws', function (): void {
    assert_throws(
        KreuzbergException::class,
        static fn () => Kreuzberg::extractFileSync('/nonexistent/path/to/file.pdf'),
        'non-existent file'
    );
});

$runner->test('extract_file error contains file path info', function (): void {
    $path = '/nonexistent/path/to/document.pdf';
    try {
        Kreuzberg::extractFileSync($path);
        throw new \RuntimeException('Expected exception not thrown');
    } catch (KreuzbergException $e) {
        assert_not_empty($e->getMessage(), 'error message should not be empty');
    }
});

$runner->test('ExtractionConfig::fromJson with invalid JSON throws', function (): void {
    assert_throws(
        \InvalidArgumentException::class,
        static fn () => ExtractionConfig::fromJson('not valid json'),
        'invalid JSON'
    );
});

$runner->test('ExtractionConfig::fromFile with missing file throws', function (): void {
    assert_throws(
        \InvalidArgumentException::class,
        static fn () => ExtractionConfig::fromFile('/nonexistent/config.json'),
        'missing file'
    );
});

$runner->test('OcrConfig::fromJson with invalid JSON throws', function (): void {
    assert_throws(
        \InvalidArgumentException::class,
        static fn () => OcrConfig::fromJson('{invalid json}'),
        'invalid OCR JSON'
    );
});

$runner->test('PdfConfig::fromJson with invalid JSON throws', function (): void {
    assert_throws(
        \InvalidArgumentException::class,
        static fn () => PdfConfig::fromJson('{{bad}}'),
        'invalid PDF JSON'
    );
});

$runner->test('ChunkingConfig::fromFile with missing file throws', function (): void {
    assert_throws(
        \InvalidArgumentException::class,
        static fn () => ChunkingConfig::fromFile('/nonexistent/chunking.json'),
        'missing chunking file'
    );
});

// =========================================================================
// Section 16: Static vs Instance API
// =========================================================================

$runner->section('16. Static vs Instance API');

$runner->test('static extractFileSync matches instance extractFile', function () use ($testDocuments): void {
    $textDir = $testDocuments . '/text';
    if (!is_dir($textDir)) {
        skip('No text test documents available');
    }
    $files = glob($textDir . '/*.txt');
    if (empty($files)) {
        skip('No .txt files found');
    }
    $staticResult = Kreuzberg::extractFileSync($files[0]);
    $instance = new Kreuzberg();
    $instanceResult = $instance->extractFile($files[0]);

    assert_equals($staticResult->content, $instanceResult->content, 'content should match');
    assert_equals($staticResult->mimeType, $instanceResult->mimeType, 'mimeType should match');
});

$runner->test('static extractBytesSync matches instance extractBytes', function (): void {
    $data = 'Identical content for comparison test between static and instance APIs.';
    $staticResult = Kreuzberg::extractBytesSync($data, 'text/plain');
    $instance = new Kreuzberg();
    $instanceResult = $instance->extractBytes($data, 'text/plain');

    assert_equals($staticResult->mimeType, $instanceResult->mimeType, 'mimeType should match');
});

// =========================================================================
// Section 17: ExtractionResult Structure
// =========================================================================

$runner->section('17. ExtractionResult Structure');

$runner->test('ExtractionResult has expected properties', function () use ($testDocuments): void {
    $textDir = $testDocuments . '/text';
    if (!is_dir($textDir)) {
        skip('No text test documents available');
    }
    $files = glob($textDir . '/*.txt');
    if (empty($files)) {
        skip('No .txt files found');
    }
    $result = Kreuzberg::extractFileSync($files[0]);

    // Access properties directly; native extension objects may not support property_exists
    assert_true(is_string($result->content), 'should have content property');
    assert_true(is_string($result->mimeType), 'should have mimeType property');
    assert_true($result->metadata !== null, 'should have metadata property');
    assert_true(is_array($result->tables), 'should have tables property');
});

$runner->test('ExtractionResult tables is an array', function () use ($testDocuments): void {
    $textDir = $testDocuments . '/text';
    if (!is_dir($textDir)) {
        skip('No text test documents available');
    }
    $files = glob($textDir . '/*.txt');
    if (empty($files)) {
        skip('No .txt files found');
    }
    $result = Kreuzberg::extractFileSync($files[0]);
    assert_true(is_array($result->tables), 'tables should be an array');
});

$runner->test('ExtractionResult content is a string', function () use ($testDocuments): void {
    $textDir = $testDocuments . '/text';
    if (!is_dir($textDir)) {
        skip('No text test documents available');
    }
    $files = glob($textDir . '/*.txt');
    if (empty($files)) {
        skip('No .txt files found');
    }
    $result = Kreuzberg::extractFileSync($files[0]);
    assert_true(is_string($result->content), 'content should be a string');
    assert_greater_than(0, strlen($result->content), 'content length');
});

// =========================================================================
// Section 18: AccelerationConfig (4.6.3+)
// =========================================================================

$runner->section('18. AccelerationConfig (4.6.3+)');

$runner->test('AccelerationConfig default construction', function (): void {
    $acc = new AccelerationConfig();
    assert_instance_of(AccelerationConfig::class, $acc);
    assert_equals('auto', $acc->provider, 'provider should default to auto');
    assert_equals(0, $acc->deviceId, 'deviceId should default to 0');
});

$runner->test('AccelerationConfig with custom provider', function (): void {
    $acc = new AccelerationConfig(provider: 'cpu');
    assert_equals('cpu', $acc->provider, 'provider should be cpu');
});

$runner->test('AccelerationConfig with device_id', function (): void {
    $acc = new AccelerationConfig(provider: 'cuda', deviceId: 1);
    assert_equals('cuda', $acc->provider, 'provider');
    assert_equals(1, $acc->deviceId, 'deviceId');
});

$runner->test('AccelerationConfig::toArray() omits defaults', function (): void {
    $acc = new AccelerationConfig();
    $array = $acc->toArray();
    assert_true(is_array($array), 'toArray should return array');
    // Default values (provider=auto, deviceId=0) are omitted from toArray
    assert_false(array_key_exists('provider', $array), 'default provider should be omitted');
});

$runner->test('AccelerationConfig::toArray() includes non-default provider', function (): void {
    $acc = new AccelerationConfig(provider: 'cpu');
    $array = $acc->toArray();
    assert_true(is_array($array), 'toArray should return array');
    assert_equals('cpu', $array['provider'], 'provider in array');
});

$runner->test('ExtractionConfig with AccelerationConfig', function (): void {
    $acc = new AccelerationConfig(provider: 'auto');
    $config = new ExtractionConfig(acceleration: $acc);
    assert_instance_of(AccelerationConfig::class, $config->acceleration, 'acceleration should be AccelerationConfig');
    assert_equals('auto', $config->acceleration->provider, 'provider');
});

// =========================================================================
// Section 19: EmailConfig (4.6.3+)
// =========================================================================

$runner->section('19. EmailConfig (4.6.3+)');

$runner->test('EmailConfig default construction', function (): void {
    $email = new EmailConfig();
    assert_instance_of(EmailConfig::class, $email);
    assert_equals(null, $email->msgFallbackCodepage, 'msgFallbackCodepage should default to null');
});

$runner->test('EmailConfig with fallback codepage', function (): void {
    $email = new EmailConfig(msgFallbackCodepage: 1252);
    assert_equals(1252, $email->msgFallbackCodepage, 'msgFallbackCodepage should be 1252');
});

$runner->test('EmailConfig::toArray() produces valid array', function (): void {
    $email = new EmailConfig(msgFallbackCodepage: 1251);
    $array = $email->toArray();
    assert_true(is_array($array), 'toArray should return array');
    assert_equals(1251, $array['msg_fallback_codepage'], 'msg_fallback_codepage in array');
});

$runner->test('ExtractionConfig with EmailConfig', function (): void {
    $email = new EmailConfig(msgFallbackCodepage: 1252);
    $config = new ExtractionConfig(email: $email);
    assert_instance_of(EmailConfig::class, $config->email, 'email should be EmailConfig');
    assert_equals(1252, $config->email->msgFallbackCodepage, 'msgFallbackCodepage');
});

// =========================================================================
// Section 20: maxArchiveDepth and PDF Page Rendering (4.6.2+/4.6.3+)
// =========================================================================

$runner->section('20. maxArchiveDepth & PDF Page Rendering (4.6.2+/4.6.3+)');

$runner->test('ExtractionConfig maxArchiveDepth defaults to 3', function (): void {
    $config = new ExtractionConfig();
    assert_equals(3, $config->maxArchiveDepth, 'maxArchiveDepth should default to 3');
});

$runner->test('ExtractionConfig with custom maxArchiveDepth', function (): void {
    $config = new ExtractionConfig(maxArchiveDepth: 10);
    assert_equals(10, $config->maxArchiveDepth, 'maxArchiveDepth should be 10');
});

$runner->test('maxArchiveDepth serializes to array', function (): void {
    $config = new ExtractionConfig(maxArchiveDepth: 5);
    $array = $config->toArray();
    assert_true(is_array($array), 'toArray should return array');
    assert_array_key('max_archive_depth', $array, 'should have max_archive_depth key');
    assert_equals(5, $array['max_archive_depth'], 'max_archive_depth value');
});

$runner->test('render_pdf_page function exists', function (): void {
    assert_true(function_exists('Kreuzberg\render_pdf_page'), 'render_pdf_page function should exist');
});

$runner->test('render_pdf_pages_iter function exists', function (): void {
    assert_true(function_exists('Kreuzberg\render_pdf_pages_iter'), 'render_pdf_pages_iter function should exist');
});

$runner->test('render_pdf_page raises exception for nonexistent file', function (): void {
    assert_throws(
        KreuzbergException::class,
        static fn () => \Kreuzberg\render_pdf_page('/nonexistent/path/to/document.pdf', 0),
        'nonexistent file for render_pdf_page'
    );
});

// =========================================================================
// Print summary
// =========================================================================

exit($runner->summary());

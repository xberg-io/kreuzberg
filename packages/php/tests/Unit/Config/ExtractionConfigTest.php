<?php

declare(strict_types=1);

namespace Kreuzberg\Tests\Unit\Config;

use Kreuzberg\Config\ChunkingConfig;
use Kreuzberg\Config\EmbeddingConfig;
use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\ImageExtractionConfig;
use Kreuzberg\Config\KeywordConfig;
use Kreuzberg\Config\LanguageDetectionConfig;
use Kreuzberg\Config\OcrConfig;
use Kreuzberg\Config\PageConfig;
use Kreuzberg\Config\PdfConfig;
use PHPUnit\Framework\Attributes\CoversClass;
use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\TestCase;

/**
 * Unit tests for ExtractionConfig readonly class.
 *
 * Tests construction, serialization, factory methods, readonly enforcement,
 * and handling of complex nested configuration objects and boolean properties.
 * This is the main configuration class that aggregates all extraction settings.
 *
 * Test Coverage:
 * - Construction with default values
 * - Construction with custom values
 * - toArray() serialization with optional field inclusion
 * - fromArray() factory method with nested structures
 * - fromJson() factory method
 * - toJson() serialization
 * - Readonly enforcement
 * - Nested configuration handling
 * - Builder pattern
 * - Invalid JSON handling
 * - Round-trip serialization
 */
#[CoversClass(ExtractionConfig::class)]
#[Group('unit')]
#[Group('config')]
final class ExtractionConfigTest extends TestCase
{
    #[Test]
    public function it_creates_with_default_values(): void
    {
        $config = new ExtractionConfig();

        $this->assertNull($config->ocr);
        $this->assertNull($config->pdf);
        $this->assertNull($config->chunking);
        $this->assertNull($config->embedding);
        $this->assertNull($config->imageExtraction);
        $this->assertNull($config->page);
        $this->assertNull($config->languageDetection);
        $this->assertNull($config->keywords);
        $this->assertFalse($config->extractImages);
        $this->assertTrue($config->extractTables);
        $this->assertFalse($config->preserveFormatting);
        $this->assertNull($config->outputFormat);
    }

    #[Test]
    public function it_creates_with_custom_values(): void
    {
        $ocrConfig = new OcrConfig(backend: 'tesseract');
        $pdfConfig = new PdfConfig(extractImages: true);
        $chunkingConfig = new ChunkingConfig(maxChars: 1024);

        $config = new ExtractionConfig(
            ocr: $ocrConfig,
            pdf: $pdfConfig,
            chunking: $chunkingConfig,
            extractImages: true,
            extractTables: true,
            preserveFormatting: true,
            outputFormat: 'markdown',
        );

        $this->assertSame($ocrConfig, $config->ocr);
        $this->assertSame($pdfConfig, $config->pdf);
        $this->assertSame($chunkingConfig, $config->chunking);
        $this->assertTrue($config->extractImages);
        $this->assertTrue($config->extractTables);
        $this->assertTrue($config->preserveFormatting);
        $this->assertSame('markdown', $config->outputFormat);
    }

    #[Test]
    public function it_serializes_to_array_with_only_non_null_values(): void
    {
        $config = new ExtractionConfig(
            extractImages: true,
            extractTables: false,
        );
        $array = $config->toArray();

        $this->assertIsArray($array);
        $this->assertTrue($array['extract_images']);
        $this->assertFalse($array['extract_tables']);
        $this->assertFalse($array['preserve_formatting']);
        $this->assertArrayNotHasKey('ocr', $array);
        $this->assertArrayNotHasKey('pdf', $array);
    }

    #[Test]
    public function it_includes_nested_configs_in_array_when_set(): void
    {
        $ocr = new OcrConfig();
        $pdf = new PdfConfig();
        $chunking = new ChunkingConfig();

        $config = new ExtractionConfig(
            ocr: $ocr,
            pdf: $pdf,
            chunking: $chunking,
        );
        $array = $config->toArray();

        $this->assertArrayHasKey('ocr', $array);
        $this->assertArrayHasKey('pdf', $array);
        $this->assertArrayHasKey('chunking', $array);
        $this->assertIsArray($array['ocr']);
        $this->assertIsArray($array['pdf']);
        $this->assertIsArray($array['chunking']);
    }

    #[Test]
    public function it_creates_from_array_with_defaults(): void
    {
        $config = ExtractionConfig::fromArray([]);

        $this->assertNull($config->ocr);
        $this->assertFalse($config->extractImages);
        $this->assertTrue($config->extractTables);
    }

    #[Test]
    public function it_creates_from_array_with_all_fields(): void
    {
        $data = [
            'ocr' => ['backend' => 'tesseract', 'language' => 'eng'],
            'pdf' => ['extract_images' => true],
            'chunking' => ['max_chunk_size' => 512],
            'embedding' => ['model' => 'bert-base'],
            'image_extraction' => ['extract_images' => true],
            'page' => ['extract_pages' => true],
            'language_detection' => ['enabled' => true],
            'keywords' => ['max_keywords' => 10],
            'extract_images' => true,
            'extract_tables' => false,
            'preserve_formatting' => true,
            'output_format' => 'json',
        ];
        $config = ExtractionConfig::fromArray($data);

        $this->assertNotNull($config->ocr);
        $this->assertNotNull($config->pdf);
        $this->assertNotNull($config->chunking);
        $this->assertNotNull($config->embedding);
        $this->assertNotNull($config->imageExtraction);
        $this->assertNotNull($config->page);
        $this->assertNotNull($config->languageDetection);
        $this->assertNotNull($config->keywords);
        $this->assertTrue($config->extractImages);
        $this->assertFalse($config->extractTables);
        $this->assertTrue($config->preserveFormatting);
        $this->assertSame('json', $config->outputFormat);
    }

    #[Test]
    public function it_serializes_to_json(): void
    {
        $config = new ExtractionConfig(
            ocr: new OcrConfig(backend: 'tesseract'),
            extractImages: true,
            outputFormat: 'xml',
        );
        $json = $config->toJson();

        $this->assertJson($json);
        $decoded = json_decode($json, true);

        $this->assertArrayHasKey('ocr', $decoded);
        $this->assertTrue($decoded['extract_images']);
        $this->assertSame('xml', $decoded['output_format']);
    }

    #[Test]
    public function it_creates_from_json(): void
    {
        $json = json_encode([
            'ocr' => ['backend' => 'easyocr'],
            'extract_images' => true,
            'extract_tables' => true,
        ]);
        $config = ExtractionConfig::fromJson($json);

        $this->assertNotNull($config->ocr);
        $this->assertTrue($config->extractImages);
        $this->assertTrue($config->extractTables);
    }

    #[Test]
    public function it_round_trips_through_json(): void
    {
        $original = new ExtractionConfig(
            ocr: new OcrConfig(backend: 'tesseract', language: 'eng'),
            pdf: new PdfConfig(extractImages: true),
            chunking: new ChunkingConfig(maxChars: 1024),
            extractImages: true,
            extractTables: false,
            preserveFormatting: true,
            outputFormat: 'markdown',
        );

        $json = $original->toJson();
        $restored = ExtractionConfig::fromJson($json);

        $this->assertNotNull($restored->ocr);
        $this->assertNotNull($restored->pdf);
        $this->assertNotNull($restored->chunking);
        $this->assertSame($original->extractImages, $restored->extractImages);
        $this->assertSame($original->extractTables, $restored->extractTables);
        $this->assertSame($original->preserveFormatting, $restored->preserveFormatting);
        $this->assertSame($original->outputFormat, $restored->outputFormat);
    }

    #[Test]
    public function it_throws_on_invalid_json(): void
    {
        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage('Invalid JSON');

        ExtractionConfig::fromJson('{ invalid }');
    }

    #[Test]
    public function it_enforces_readonly_on_extract_images_property(): void
    {
        $this->expectException(\Error::class);

        $config = new ExtractionConfig(extractImages: true);
        $config->extractImages = false;
    }

    #[Test]
    public function it_enforces_readonly_on_output_format_property(): void
    {
        $this->expectException(\Error::class);

        $config = new ExtractionConfig(outputFormat: 'json');
        $config->outputFormat = 'xml';
    }

    #[Test]
    public function it_enforces_readonly_on_ocr_property(): void
    {
        $this->expectException(\Error::class);

        $config = new ExtractionConfig(ocr: new OcrConfig());
        $config->ocr = new OcrConfig(backend: 'easyocr');
    }

    #[Test]
    public function it_creates_from_file(): void
    {
        $tempFile = tempnam(sys_get_temp_dir(), 'extract_');
        if ($tempFile === false) {
            $this->markTestSkipped('Unable to create temporary file');
        }

        try {
            file_put_contents($tempFile, json_encode([
                'extract_images' => true,
                'extract_tables' => false,
                'ocr' => ['backend' => 'tesseract'],
            ]));

            $config = ExtractionConfig::fromFile($tempFile);

            $this->assertTrue($config->extractImages);
            $this->assertFalse($config->extractTables);
            $this->assertNotNull($config->ocr);
        } finally {
            if (file_exists($tempFile)) {
                unlink($tempFile);
            }
        }
    }

    #[Test]
    public function it_throws_when_file_not_found(): void
    {
        $this->expectException(\InvalidArgumentException::class);
        $this->expectExceptionMessage('File not found');

        ExtractionConfig::fromFile('/nonexistent/path/config.json');
    }

    #[Test]
    public function it_handles_type_coercion_for_extract_images(): void
    {
        $data = ['extract_images' => 1];
        $config = ExtractionConfig::fromArray($data);

        $this->assertIsBool($config->extractImages);
        $this->assertTrue($config->extractImages);
    }

    #[Test]
    public function it_handles_type_coercion_for_extract_tables(): void
    {
        $data = ['extract_tables' => 0];
        $config = ExtractionConfig::fromArray($data);

        $this->assertIsBool($config->extractTables);
        $this->assertFalse($config->extractTables);
    }

    #[Test]
    public function it_has_builder_method(): void
    {
        $this->assertTrue(method_exists(ExtractionConfig::class, 'builder'));
    }

    #[Test]
    public function it_supports_all_nested_configs_together(): void
    {
        $config = new ExtractionConfig(
            ocr: new OcrConfig(),
            pdf: new PdfConfig(),
            chunking: new ChunkingConfig(),
            embedding: new EmbeddingConfig(),
            imageExtraction: new ImageExtractionConfig(),
            page: new PageConfig(),
            languageDetection: new LanguageDetectionConfig(),
            keywords: new KeywordConfig(),
        );

        $array = $config->toArray();

        $this->assertArrayHasKey('ocr', $array);
        $this->assertArrayHasKey('pdf', $array);
        $this->assertArrayHasKey('chunking', $array);
        $this->assertArrayHasKey('embedding', $array);
        $this->assertArrayHasKey('image_extraction', $array);
        $this->assertArrayHasKey('page', $array);
        $this->assertArrayHasKey('language_detection', $array);
        $this->assertArrayHasKey('keywords', $array);
    }

    #[Test]
    public function it_json_output_is_prettified(): void
    {
        $config = new ExtractionConfig(
            ocr: new OcrConfig(),
            extractImages: true,
        );
        $json = $config->toJson();

        $this->assertStringContainsString("\n", $json);
        $this->assertStringContainsString('  ', $json);
    }
}

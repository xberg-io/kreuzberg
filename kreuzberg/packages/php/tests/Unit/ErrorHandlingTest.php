<?php

declare(strict_types=1);

namespace Kreuzberg\Tests\Unit;

use Kreuzberg\Exceptions\KreuzbergException;
use Kreuzberg\Kreuzberg;
use PHPUnit\Framework\Attributes\CoversClass;
use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\Attributes\RequiresPhpExtension;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\TestCase;

/**
 * Behavior-driven tests for error handling and edge cases.
 *
 * Tests exception behavior and boundary conditions.
 */
#[CoversClass(Kreuzberg::class)]
#[CoversClass(KreuzbergException::class)]
#[Group('unit')]
#[Group('errors')]
#[RequiresPhpExtension('kreuzberg-php')]
final class ErrorHandlingTest extends TestCase
{
    private string $testDocumentsPath;

    protected function setUp(): void
    {
        if (!extension_loaded('kreuzberg-php')) {
            $this->markTestSkipped('Kreuzberg extension is not loaded');
        }

        $this->testDocumentsPath = dirname(__DIR__, 4) . DIRECTORY_SEPARATOR . 'test_documents';
    }

    #[Test]
    public function it_throws_exception_for_nonexistent_file(): void
    {
        $this->expectException(KreuzbergException::class);

        $kreuzberg = new Kreuzberg();
        $kreuzberg->extractFile('/this/path/definitely/does/not/exist.pdf');
    }

    #[Test]
    public function it_throws_exception_for_empty_file_path(): void
    {
        $this->expectException(KreuzbergException::class);

        $kreuzberg = new Kreuzberg();
        $kreuzberg->extractFile('');
    }

    #[Test]
    public function it_throws_exception_for_directory_instead_of_file(): void
    {
        $this->expectException(KreuzbergException::class);

        $kreuzberg = new Kreuzberg();
        $kreuzberg->extractFile($this->testDocumentsPath);
    }

    #[Test]
    public function it_throws_exception_for_invalid_mime_type_in_extract_bytes(): void
    {
        $kreuzberg = new Kreuzberg();
        $tmpFile = tempnam(sys_get_temp_dir(), 'krz_test_');
        file_put_contents($tmpFile, 'test content');

        try {
            $bytes = file_get_contents($tmpFile);

            $this->expectException(KreuzbergException::class);
            $kreuzberg->extractBytes($bytes, 'invalid/mime-type');
        } finally {
            @unlink($tmpFile);
        }
    }

    #[Test]
    public function it_throws_exception_for_empty_bytes(): void
    {
        $this->expectException(KreuzbergException::class);

        $kreuzberg = new Kreuzberg();
        $kreuzberg->extractBytes('', 'application/pdf');
    }

    #[Test]
    public function it_throws_exception_for_corrupted_pdf_bytes(): void
    {
        $this->expectException(KreuzbergException::class);

        $kreuzberg = new Kreuzberg();
        $corruptedPdf = 'This is not a valid PDF content';
        $kreuzberg->extractBytes($corruptedPdf, 'application/pdf');
    }

    #[Test]
    public function it_throws_exception_for_mismatched_mime_type(): void
    {
        $filePath = $this->testDocumentsPath . '/pdf/code_and_formula.pdf';

        if (!file_exists($filePath)) {
            $this->markTestSkipped("Test file not found: {$filePath}");
        }

        $bytes = file_get_contents($filePath);

        $this->expectException(KreuzbergException::class);

        $kreuzberg = new Kreuzberg();
        $kreuzberg->extractBytes($bytes, 'application/vnd.ms-excel');
    }

    #[Test]
    public function it_handles_unreadable_file_gracefully(): void
    {
        $tmpFile = tempnam(sys_get_temp_dir(), 'krz_test_');
        file_put_contents($tmpFile, 'test');
        chmod($tmpFile, 0o000);

        try {
            $this->expectException(KreuzbergException::class);

            $kreuzberg = new Kreuzberg();
            $kreuzberg->extractFile($tmpFile);
        } finally {
            chmod($tmpFile, 0o644);
            @unlink($tmpFile);
        }
    }

    /**
     * Batch operations return error results in metadata for individual failures
     * rather than throwing exceptions. This allows the batch to continue.
     */
    #[Test]
    public function it_handles_batch_with_nonexistent_files_gracefully(): void
    {
        $files = [
            '/nonexistent/file1.pdf',
            '/nonexistent/file2.pdf',
        ];

        $kreuzberg = new Kreuzberg();
        $results = $kreuzberg->batchExtractFiles($files);

        $this->assertCount(2, $results, 'Should return results for all files');
        foreach ($results as $result) {
            $hasError = str_starts_with($result->content, 'Error:');
            $this->assertTrue($hasError, 'Invalid file result should indicate error');
        }
    }

    /**
     * Batch operations return error results in metadata for individual failures
     * rather than throwing exceptions. This allows the batch to continue.
     */
    #[Test]
    public function it_handles_batch_with_mixed_valid_invalid_files_gracefully(): void
    {
        $validFile = $this->testDocumentsPath . '/pdf/code_and_formula.pdf';
        if (!file_exists($validFile)) {
            $this->markTestSkipped("Test file not found: {$validFile}");
        }

        $files = [
            $validFile,
            '/nonexistent/file.pdf',
        ];

        $kreuzberg = new Kreuzberg();
        $results = $kreuzberg->batchExtractFiles($files);

        $this->assertCount(2, $results, 'Should return results for all files');
        $this->assertNotEmpty($results[0]->content, 'Valid file should have content');
        $hasError = str_starts_with($results[1]->content, 'Error:');
        $this->assertTrue($hasError, 'Invalid file result should indicate error');
    }

    #[Test]
    public function it_throws_exception_for_mismatched_batch_bytes_mime_types_count(): void
    {
        $filePath = $this->testDocumentsPath . '/pdf/code_and_formula.pdf';

        if (!file_exists($filePath)) {
            $this->markTestSkipped("Test file not found: {$filePath}");
        }

        $dataList = [
            file_get_contents($filePath),
            file_get_contents($filePath),
        ];

        $mimeTypes = ['application/pdf'];

        $this->expectException(KreuzbergException::class);

        $kreuzberg = new Kreuzberg();
        $kreuzberg->batchExtractBytes($dataList, $mimeTypes);
    }

    /**
     * Batch operations return error results in metadata for individual failures
     * rather than throwing exceptions. This allows the batch to continue.
     */
    #[Test]
    public function it_handles_batch_with_empty_byte_arrays_gracefully(): void
    {
        $dataList = ['', ''];
        $mimeTypes = ['application/pdf', 'application/pdf'];

        $kreuzberg = new Kreuzberg();
        $results = $kreuzberg->batchExtractBytes($dataList, $mimeTypes);

        $this->assertCount(2, $results, 'Should return results for all items');
        foreach ($results as $result) {
            $hasError = str_starts_with($result->content, 'Error:');
            $this->assertTrue($hasError, 'Empty byte array result should indicate error');
        }
    }

    #[Test]
    public function it_handles_very_large_file_path(): void
    {
        $longPath = str_repeat('/nonexistent', 1000) . '/file.pdf';

        $this->expectException(KreuzbergException::class);

        $kreuzberg = new Kreuzberg();
        $kreuzberg->extractFile($longPath);
    }

    #[Test]
    public function it_handles_special_characters_in_file_path(): void
    {
        $specialPath = '/tmp/file with spaces & special chars!@#$.pdf';

        $this->expectException(KreuzbergException::class);

        $kreuzberg = new Kreuzberg();
        $kreuzberg->extractFile($specialPath);
    }

    #[Test]
    public function it_validates_exception_has_meaningful_message(): void
    {
        try {
            $kreuzberg = new Kreuzberg();
            $kreuzberg->extractFile('/nonexistent/file.pdf');

            $this->fail('Expected KreuzbergException to be thrown');
        } catch (KreuzbergException $e) {
            $this->assertNotEmpty(
                $e->getMessage(),
                'Exception should have a meaningful error message',
            );
            $this->assertIsString($e->getMessage());
        }
    }

    #[Test]
    public function it_creates_validation_exception_with_proper_code(): void
    {
        $exception = KreuzbergException::validation('Test validation error');

        $this->assertStringContainsString('Validation error', $exception->getMessage());
        $this->assertSame(1, $exception->getCode());
    }

    #[Test]
    public function it_creates_parsing_exception_with_proper_code(): void
    {
        $exception = KreuzbergException::parsing('Test parsing error');

        $this->assertStringContainsString('Parsing error', $exception->getMessage());
        $this->assertSame(2, $exception->getCode());
    }

    #[Test]
    public function it_creates_ocr_exception_with_proper_code(): void
    {
        $exception = KreuzbergException::ocr('Test OCR error');

        $this->assertStringContainsString('OCR error', $exception->getMessage());
        $this->assertSame(3, $exception->getCode());
    }

    #[Test]
    public function it_creates_io_exception_with_proper_code(): void
    {
        $exception = KreuzbergException::io('Test I/O error');

        $this->assertStringContainsString('I/O error', $exception->getMessage());
        $this->assertSame(5, $exception->getCode());
    }

    #[Test]
    public function it_creates_unsupported_format_exception_with_proper_code(): void
    {
        $exception = KreuzbergException::unsupportedFormat('Test format error');

        $this->assertStringContainsString('Unsupported format', $exception->getMessage());
        $this->assertSame(7, $exception->getCode());
    }

    #[Test]
    public function it_handles_null_bytes_in_file_path(): void
    {
        $pathWithNull = "/tmp/file\0.pdf";

        $this->expectException(KreuzbergException::class);

        $kreuzberg = new Kreuzberg();
        $kreuzberg->extractFile($pathWithNull);
    }
}

<?php

declare(strict_types=1);

namespace Kreuzberg\Tests\Unit;

use Kreuzberg\Kreuzberg;
use Kreuzberg\Types\DeferredResult;
use PHPUnit\Framework\Attributes\CoversClass;
use PHPUnit\Framework\Attributes\Group;
use PHPUnit\Framework\Attributes\RequiresPhpExtension;
use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\TestCase;

/**
 * Unit tests for async extraction via DeferredResult.
 *
 * Tests verify the DeferredResult lifecycle: creation, polling, blocking wait,
 * and result retrieval. Also validates parity between sync and async results.
 */
#[CoversClass(Kreuzberg::class)]
#[Group('unit')]
#[RequiresPhpExtension('kreuzberg-php')]
final class AsyncExtractionTest extends TestCase
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
    public function it_returns_deferred_result_from_extract_file_async(): void
    {
        $deferred = \kreuzberg_extract_file_async(
            $this->testDocumentsPath . '/pdf/code_and_formula.pdf',
        );

        $this->assertInstanceOf(DeferredResult::class, $deferred);
    }

    #[Test]
    public function it_can_poll_deferred_result_readiness(): void
    {
        $deferred = \kreuzberg_extract_file_async(
            $this->testDocumentsPath . '/pdf/code_and_formula.pdf',
        );

        // isReady should return a boolean (may be true or false depending on timing)
        $this->assertIsBool($deferred->isReady());
    }

    #[Test]
    public function it_can_get_result_from_deferred_blocking(): void
    {
        $deferred = \kreuzberg_extract_file_async(
            $this->testDocumentsPath . '/pdf/code_and_formula.pdf',
        );

        // getResult blocks until the result is ready
        $result = $deferred->getResult();

        $this->assertNotEmpty($result->content);
        $this->assertIsString($result->mimeType);
    }

    #[Test]
    public function it_marks_deferred_as_ready_after_get_result(): void
    {
        $deferred = \kreuzberg_extract_file_async(
            $this->testDocumentsPath . '/pdf/code_and_formula.pdf',
        );

        $deferred->getResult();

        $this->assertTrue($deferred->isReady());
    }

    #[Test]
    public function it_can_try_get_result_returning_null_or_value(): void
    {
        $deferred = \kreuzberg_extract_file_async(
            $this->testDocumentsPath . '/pdf/code_and_formula.pdf',
        );

        // tryGetResult returns null if not ready, or the result if ready
        $result = $deferred->tryGetResult();

        if ($result === null) {
            // Not ready yet, wait and try again
            $result = $deferred->getResult();
        }

        $this->assertNotEmpty($result->content);
    }

    #[Test]
    public function it_can_wait_with_timeout(): void
    {
        $deferred = \kreuzberg_extract_file_async(
            $this->testDocumentsPath . '/pdf/code_and_formula.pdf',
        );

        // wait() with generous timeout should return the result
        $result = $deferred->wait(30000);

        $this->assertNotNull($result);
        $this->assertNotEmpty($result->content);
    }

    #[Test]
    public function async_result_matches_sync_result(): void
    {
        $filePath = $this->testDocumentsPath . '/pdf/code_and_formula.pdf';

        // Get sync result
        $kreuzberg = new Kreuzberg();
        $syncResult = $kreuzberg->extractFile($filePath);

        // Get async result
        $deferred = \kreuzberg_extract_file_async($filePath);
        $asyncResult = $deferred->getResult();

        $this->assertSame($syncResult->content, $asyncResult->content);
        $this->assertSame($syncResult->mimeType, $asyncResult->mimeType);
    }

    #[Test]
    public function it_propagates_errors_through_deferred(): void
    {
        $deferred = \kreuzberg_extract_file_async('/nonexistent/path/to/file.pdf');

        $this->expectException(\Exception::class);
        $deferred->getResult();
    }

    #[Test]
    public function it_supports_extract_bytes_async(): void
    {
        $data = file_get_contents($this->testDocumentsPath . '/pdf/code_and_formula.pdf');
        $this->assertNotFalse($data);

        $deferred = \kreuzberg_extract_bytes_async($data, 'application/pdf');

        $this->assertInstanceOf(DeferredResult::class, $deferred);

        $result = $deferred->getResult();
        $this->assertNotEmpty($result->content);
    }

    #[Test]
    public function it_supports_batch_extract_files_async(): void
    {
        $paths = [
            $this->testDocumentsPath . '/pdf/code_and_formula.pdf',
            $this->testDocumentsPath . '/text/plain.txt',
        ];

        $deferred = \kreuzberg_batch_extract_files_async($paths);

        $this->assertInstanceOf(DeferredResult::class, $deferred);
    }

    #[Test]
    public function it_supports_concurrent_async_extractions(): void
    {
        $filePath = $this->testDocumentsPath . '/pdf/code_and_formula.pdf';

        // Launch multiple async extractions concurrently
        $deferred1 = \kreuzberg_extract_file_async($filePath);
        $deferred2 = \kreuzberg_extract_file_async($filePath);
        $deferred3 = \kreuzberg_extract_file_async($filePath);

        // All should complete and return identical results
        $result1 = $deferred1->getResult();
        $result2 = $deferred2->getResult();
        $result3 = $deferred3->getResult();

        $this->assertSame($result1->content, $result2->content);
        $this->assertSame($result2->content, $result3->content);
    }

    #[Test]
    public function kreuzberg_class_has_extract_file_async(): void
    {
        $kreuzberg = new Kreuzberg();
        $deferred = $kreuzberg->extractFileAsync(
            $this->testDocumentsPath . '/pdf/code_and_formula.pdf',
        );

        $this->assertInstanceOf(DeferredResult::class, $deferred);

        $result = $deferred->getResult();
        $this->assertNotEmpty($result->content);
    }

    #[Test]
    public function kreuzberg_class_has_extract_bytes_async(): void
    {
        $data = file_get_contents($this->testDocumentsPath . '/pdf/code_and_formula.pdf');
        $this->assertNotFalse($data);

        $kreuzberg = new Kreuzberg();
        $deferred = $kreuzberg->extractBytesAsync($data, 'application/pdf');

        $this->assertInstanceOf(DeferredResult::class, $deferred);

        $result = $deferred->getResult();
        $this->assertNotEmpty($result->content);
    }

    #[Test]
    public function kreuzberg_class_has_static_extract_file_async(): void
    {
        $deferred = Kreuzberg::extractFileAsyncStatic(
            $this->testDocumentsPath . '/pdf/code_and_formula.pdf',
        );

        $this->assertInstanceOf(DeferredResult::class, $deferred);

        $result = $deferred->getResult();
        $this->assertNotEmpty($result->content);
    }
}

<?php

declare(strict_types=1);

/*
 * Hand-written binding-specific edge case tests for PDF rendering.
 * Happy-path render tests are auto-generated from fixtures in e2e/.
 * These tests cover error handling, validation, and lifecycle patterns
 * that vary per language and can't be generated uniformly.
 */

namespace Kreuzberg\Tests\Unit;

use Kreuzberg\Exceptions\KreuzbergException;

use function Kreuzberg\render_pdf_page;
use function Kreuzberg\render_pdf_pages_iter;

use PHPUnit\Framework\Attributes\Test;
use PHPUnit\Framework\TestCase;

final class RenderTest extends TestCase
{
    protected function setUp(): void
    {
        if (!extension_loaded('kreuzberg-php')) {
            $this->markTestSkipped('Kreuzberg extension is not loaded');
        }
    }

    private static function getTestPdfPath(): string
    {
        $dir = __DIR__;
        while ($dir !== '/' && $dir !== '') {
            $candidate = $dir . '/test_documents/pdf/tiny.pdf';
            if (file_exists($candidate)) {
                return $candidate;
            }
            $dir = dirname($dir);
        }

        return '';
    }

    #[Test]
    public function it_exposes_rendering_functions(): void
    {
        $this->assertTrue(function_exists('Kreuzberg\render_pdf_page'));
        $this->assertTrue(function_exists('Kreuzberg\render_pdf_pages_iter'));
    }

    #[Test]
    public function it_throws_for_nonexistent_file_page(): void
    {
        $this->expectException(KreuzbergException::class);
        render_pdf_page('/nonexistent/path/to/document.pdf', 0);
    }

    #[Test]
    public function it_throws_for_out_of_bounds_page_index(): void
    {
        $pdfPath = self::getTestPdfPath();
        if ($pdfPath === '' || !file_exists($pdfPath)) {
            $this->markTestSkipped('Test PDF not found');
        }

        $this->expectException(KreuzbergException::class);
        render_pdf_page($pdfPath, 9999);
    }

    #[Test]
    public function it_throws_for_negative_page_index(): void
    {
        $pdfPath = self::getTestPdfPath();
        if ($pdfPath === '' || !file_exists($pdfPath)) {
            $this->markTestSkipped('Test PDF not found');
        }

        $this->expectException(KreuzbergException::class);
        render_pdf_page($pdfPath, -1);
    }

    #[Test]
    public function it_throws_for_nonexistent_file_iter(): void
    {
        $this->expectException(KreuzbergException::class);
        foreach (render_pdf_pages_iter('/nonexistent/path/to/document.pdf') as $png) {
            // should not reach here
        }
    }

    #[Test]
    public function it_throws_for_empty_path_page(): void
    {
        $this->expectException(KreuzbergException::class);
        render_pdf_page('', 0);
    }

    #[Test]
    public function it_handles_iterator_cleanup_without_consuming(): void
    {
        $pdfPath = self::getTestPdfPath();
        if ($pdfPath === '' || !file_exists($pdfPath)) {
            $this->markTestSkipped('Test PDF not found');
        }

        $iter = render_pdf_pages_iter($pdfPath);
        // Create but don't consume — just let it go out of scope
        unset($iter);
        $this->assertTrue(true, 'Iterator cleanup without consuming should not crash');
    }

    #[Test]
    public function it_supports_early_termination(): void
    {
        $pdfPath = self::getTestPdfPath();
        if ($pdfPath === '' || !file_exists($pdfPath)) {
            $this->markTestSkipped('Test PDF not found');
        }

        foreach (render_pdf_pages_iter($pdfPath) as $pageIndex => $pngData) {
            $this->assertSame(0, $pageIndex);
            $this->assertIsString($pngData);
            $this->assertGreaterThan(8, strlen($pngData));
            // Verify PNG magic bytes
            $this->assertSame("\x89PNG", substr($pngData, 0, 4));
            break; // Stop after first page
        }
    }
}

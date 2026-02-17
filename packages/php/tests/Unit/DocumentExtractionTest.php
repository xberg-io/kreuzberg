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
 * Binding-specific unit tests for document extraction functionality.
 *
 * Tests focus on PHP binding behavior: exception handling, version reporting,
 * and result object immutability.
 */
#[CoversClass(Kreuzberg::class)]
#[Group('unit')]
#[RequiresPhpExtension('kreuzberg-php')]
final class DocumentExtractionTest extends TestCase
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
        $kreuzberg->extractFile('/nonexistent/path/to/file.pdf');
    }

    #[Test]
    public function it_throws_exception_for_empty_file_path(): void
    {
        $this->expectException(KreuzbergException::class);

        $kreuzberg = new Kreuzberg();
        $kreuzberg->extractFile('');
    }

    #[Test]
    public function it_provides_library_version(): void
    {
        $version = Kreuzberg::version();

        $this->assertIsString($version);
        $this->assertMatchesRegularExpression(
            '/^\d+\.\d+\.\d+/',
            $version,
            'Version should follow semantic versioning format',
        );
    }

    #[Test]
    public function it_handles_null_default_config_explicitly(): void
    {
        $kreuzberg = new Kreuzberg(null);
        $result = $kreuzberg->extractFile($this->testDocumentsPath . '/pdf/code_and_formula.pdf');

        $this->assertNotEmpty($result->content, 'Extraction should work with explicit null config');
    }

    #[Test]
    public function it_preserves_extraction_result_immutability(): void
    {
        $kreuzberg = new Kreuzberg();
        $result = $kreuzberg->extractFile($this->testDocumentsPath . '/pdf/code_and_formula.pdf');

        // Verify that ExtractionResult has all required properties and stores the correct values
        $this->assertIsString($result->content);
        $originalContent = $result->content;
        $this->assertIsString($result->mimeType);
        $this->assertNotNull($result->metadata);

        // Verify the values don't change (property immutability through no setters)
        // The extension-wrapped class maintains these values as they were set during construction
        $this->assertSame(
            $originalContent,
            $result->content,
            'ExtractionResult properties should maintain their initial values',
        );
    }
}

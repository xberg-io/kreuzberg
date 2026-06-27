```php title="multi_format.php"
<?php

declare(strict_types=1);

/**
 * Multi-Format Document Extraction
 *
 * Handle various document formats (PDF, DOCX, XLSX, PPTX, images, etc.)
 * with format-specific processing and unified output.
 */

require_once __DIR__ . '/vendor/autoload.php';

use Xberg\ExtractionConfig;

$formats = [
    'PDF' => 'document.pdf',
    'Word' => 'document.docx',
    'Excel' => 'spreadsheet.xlsx',
    'PowerPoint' => 'presentation.pptx',
    'Text' => 'readme.txt',
    'HTML' => 'page.html',
    'Markdown' => 'guide.md',
    'Image' => 'scan.png',
];

echo "Multi-Format Extraction:\n";
echo str_repeat('=', 60) . "\n\n";


foreach ($formats as $type => $file) {
    if (!file_exists($file)) {
        continue;
    }

    echo "Processing $type ($file):\n";

    $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($file), $config ?? \Xberg\ExtractionConfig::default());
$result = $output->results[0];

    echo "  Content length: " . strlen($result->content) . " chars\n";
    echo "  Tables: " . count($result->tables) . "\n";
    echo "  Images: " . count($result->images ?? []) . "\n";
    echo "  Pages: " . ($result->metadata?->pdf?->page_count ?? 'N/A') . "\n";
    echo "\n";
}

$mixedFiles = glob('documents/*.*');
$byFormat = [];

foreach ($mixedFiles as $file) {
    $extension = pathinfo($file, PATHINFO_EXTENSION);

    if (!isset($byFormat[$extension])) {
        $byFormat[$extension] = [];
    }

    $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($file), $config ?? \Xberg\ExtractionConfig::default());
$result = $output->results[0];
    $byFormat[$extension][] = [
        'file' => basename($file),
        'size' => strlen($result->content),
        'tables' => count($result->tables),
    ];
}

echo "Files by Format:\n";
echo str_repeat('=', 60) . "\n";
foreach ($byFormat as $ext => $files) {
    echo strtoupper($ext) . ": " . count($files) . " files\n";

    $totalSize = array_sum(array_column($files, 'size'));
    $totalTables = array_sum(array_column($files, 'tables'));

    echo "  Total content: " . number_format($totalSize) . " chars\n";
    echo "  Total tables: $totalTables\n\n";
}

$formatConfigs = [
    'pdf' => new ExtractionConfig(
        extractTables: true,
        extractImages: true,
        pdf: new \Xberg\PdfConfig(
            extractImages: true,
            imageQuality: 85
        )
    ),
    'docx' => new ExtractionConfig(
        extractTables: true,
        preserveFormatting: true
    ),
    'xlsx' => new ExtractionConfig(
        extractTables: true  
    ),
    'png' => new ExtractionConfig(
        ocr: new \Xberg\OcrConfig(
            backend: 'tesseract',
            language: 'eng'
        )
    ),
];

foreach ($mixedFiles as $file) {
    $ext = strtolower(pathinfo($file, PATHINFO_EXTENSION));

    if (!isset($formatConfigs[$ext])) {
        continue;
    }

    $config = $formatConfigs[$ext];
    $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($file), $config ?? \Xberg\ExtractionConfig::default());
$result = $output->results[0];

    echo "Processed " . basename($file) . " with $ext config\n";
}

function convertToMarkdown(string $inputFile): string
{
    $config = new ExtractionConfig(
        preserveFormatting: true,
        outputFormat: 'markdown',
        extractTables: true
    );

    $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($inputFile), $config ?? \Xberg\ExtractionConfig::default());
$result = $output->results[0];

    $markdown = "# " . ($result->metadata?->title ?? basename($inputFile)) . "\n\n";

    if (isset($result->metadata?->authors)) {
        $markdown .= "_Authors: " . implode(', ', $result->metadata?->authors) . "_\n\n";
    }

    $markdown .= $result->content . "\n\n";

    foreach ($result->tables as $index => $table) {
        $markdown .= "## Table " . ($index + 1) . "\n\n";
        $markdown .= $table->markdown . "\n\n";
    }

    return $markdown;
}

echo "\nConverting to Markdown:\n";
echo str_repeat('=', 60) . "\n";

foreach (['document.pdf', 'document.docx'] as $file) {
    if (!file_exists($file)) {
        continue;
    }

    $markdown = convertToMarkdown($file);
    $outputFile = pathinfo($file, PATHINFO_FILENAME) . '.md';

    file_put_contents($outputFile, $markdown);
    echo "Converted: $file -> $outputFile\n";
}

function extractFromArchive(string $archiveFile): array
{
    $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($archiveFile), $config ?? \Xberg\ExtractionConfig::default());
$result = $output->results[0];

    return [
        'archive' => basename($archiveFile),
        'listing' => $result->content,
        'mime' => $result->mimeType,
    ];
}

class UniversalExtractor
{
    private array $formatHandlers = [];

    public function __construct()
    {
        $this->formatHandlers = [
            'pdf' => [$this, 'handlePDF'],
            'docx' => [$this, 'handleDOCX'],
            'xlsx' => [$this, 'handleXLSX'],
            'png' => [$this, 'handleImage'],
            'jpg' => [$this, 'handleImage'],
            'jpeg' => [$this, 'handleImage'],
        ];
    }

    public function extract(string $file): array
    {
        $extension = strtolower(pathinfo($file, PATHINFO_EXTENSION));
        $handler = $this->formatHandlers[$extension] ?? [$this, 'handleGeneric'];

        return $handler($file, $extension);
    }

    private function handlePDF(string $file, string $ext): array
    {
        $config = new ExtractionConfig(extractTables: true, extractImages: true);
        $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($file), $config);
        $result = $output->results[0];

        return [
            'type' => 'PDF',
            'content' => $result->content,
            'tables' => count($result->tables),
            'images' => count($result->images ?? []),
            'pages' => $result->metadata?->pdf?->page_count,
        ];
    }

    private function handleDOCX(string $file, string $ext): array
    {
        $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($file), new ExtractionConfig());
        $result = $output->results[0];

        return [
            'type' => 'Word Document',
            'content' => $result->content,
            'tables' => count($result->tables),
            'authors' => $result->metadata?->authors,
        ];
    }

    private function handleXLSX(string $file, string $ext): array
    {
        $config = new ExtractionConfig(extractTables: true);
        $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($file), $config);
        $result = $output->results[0];

        return [
            'type' => 'Excel Spreadsheet',
            'content' => $result->content,
            'sheets' => count($result->tables),
        ];
    }

    private function handleImage(string $file, string $ext): array
    {
        $config = new ExtractionConfig(
            ocr: new \Xberg\OcrConfig(backend: 'tesseract', language: 'eng')
        );
        $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($file), $config);
        $result = $output->results[0];

        return [
            'type' => 'Image (OCR)',
            'content' => $result->content,
            'ocr_length' => strlen($result->content),
        ];
    }

    private function handleGeneric(string $file, string $ext): array
    {
        $output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri($file), new ExtractionConfig());
        $result = $output->results[0];

        return [
            'type' => 'Generic',
            'ext' => $ext,
            'content' => $result->content,
        ];
    }
}

$extractor = new UniversalExtractor();

echo "\nUniversal Extraction:\n";
echo str_repeat('=', 60) . "\n";

foreach ($mixedFiles as $file) {
    $data = $extractor->extract($file);
    echo basename($file) . " ({$data['type']}):\n";
    print_r(array_filter($data, fn($k) => $k !== 'content', ARRAY_FILTER_USE_KEY));
    echo "\n";
}
```

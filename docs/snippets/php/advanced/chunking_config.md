```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\ChunkingConfig;

// Basic chunking
$config = new ExtractionConfig(
    chunking: new ChunkingConfig(
        maxCharacters: 1000,
        overlap: 200
    )
);

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.pdf'), $config);

$result = $resultOutput->results[0];

echo "Number of chunks: " . count($result->chunks) . "\n";
foreach ($result->chunks as $chunk) {
    echo "Chunk size: " . strlen($chunk->content) . " characters\n";
}
?>
```

```php title="PHP - Semantic Chunking"
<?php
declare(strict_types=1);

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\ChunkingConfig;

$config = new ExtractionConfig(
    chunking: new ChunkingConfig(
        maxCharacters: 500,
        overlap: 50,
        chunkerType: 'semantic',
        topicThreshold: 0.75
    )
);

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.pdf'), $config);

$result = $resultOutput->results[0];

echo "Chunks with topic-based boundaries: " . count($result->chunks) . "\n";
?>
```

```php title="PHP - Prepend Heading Context"
<?php
declare(strict_types=1);

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\ChunkingConfig;

$config = new ExtractionConfig(
    chunking: new ChunkingConfig(
        maxCharacters: 500,
        overlap: 50,
        chunkerType: 'markdown',
        prependHeadingContext: true
    )
);

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.md'), $config);

$result = $resultOutput->results[0];

foreach ($result->chunks as $chunk) {
    $metadata = $chunk->metadata;
    if ($metadata && $metadata->getHeadingContext()) {
        $headings = $metadata->getHeadingContext()->getHeadings();
        foreach ($headings as $heading) {
            echo "Heading L" . $heading->getLevel() . ": " . $heading->getText() . "\n";
        }
    }
    echo "Content: " . substr($chunk->content, 0, 100) . "...\n";
}
?>
```

```php title="chunking.php"
<?php

declare(strict_types=1);

/**
 * Text Chunking Configuration
 *
 * Configure document chunking for processing long texts into manageable pieces.
 * Useful for RAG systems, embedding generation, and token limit management.
 */

require_once __DIR__ . '/vendor/autoload.php';

use Xberg\ExtractionConfig;
use Xberg\ChunkingConfig;
use Xberg\EmbeddingConfig;

$config = new ExtractionConfig(
    chunking: new ChunkingConfig(
        maxChars: 1500,
        maxOverlap: 200,
        embedding: new EmbeddingConfig(
            model: 'balanced'
        )
    )
);

$output = \Xberg\XbergApi::extract(\Xberg\ExtractInput::fromUri('document.pdf'), $config ?? \Xberg\ExtractionConfig::default());
$result = $output->results[0];

echo "Chunking Results:\n";
echo str_repeat('=', 60) . "\n";
echo "Total chunks created: " . count($result->chunks ?? []) . "\n\n";

foreach ($result->chunks ?? [] as $index => $chunk) {
    echo "Chunk " . ($index + 1) . ":\n";
    echo "  Length: " . strlen($chunk->content) . " characters\n";
    echo "  Preview: " . substr($chunk->content, 0, 100) . "...\n";

    if ($chunk->embedding !== null) {
        echo "  Embedding dimensions: " . count($chunk->embedding) . "\n";
    }

    echo "\n";
}
```

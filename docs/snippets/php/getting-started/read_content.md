```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\ChunkingConfig;

$config = ExtractionConfig::default();
$config->setChunking(new ChunkingConfig());
$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.pdf'), $config);
$result = $resultOutput->results[0];

echo "Total content length: " . strlen($result->content) . "\n";

if ($result->chunks !== null) {
    foreach ($result->chunks as $chunk) {
        echo "Chunk: " . $chunk->content . "\n";
    }
}

foreach ($result->tables as $table) {
    echo "Table with " . count($table->getRows()) . " rows\n";
}
```

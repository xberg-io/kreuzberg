```php title="PHP"
<?php
declare(strict_types=1);

use Xberg\XbergApi;
use Xberg\ExtractionConfig;
use Xberg\ChunkingConfig;
use Xberg\PageConfig;

$config = new ExtractionConfig(
    chunking: new ChunkingConfig(
        maxCharacters: 500,
        overlap: 50
    ),
    pages: new PageConfig(
        extractPages: true
    )
);

$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.pdf'), $config);

$result = $resultOutput->results[0];

if ($result->chunks) {
    foreach ($result->chunks as $chunk) {
        $metadata = $chunk->metadata;
        if ($metadata) {
            $firstPage = $metadata->getFirstPage();
            $lastPage = $metadata->getLastPage();

            if ($firstPage !== null && $lastPage !== null) {
                if ($firstPage === $lastPage) {
                    $pageRange = "Page " . $firstPage;
                } else {
                    $pageRange = "Pages " . $firstPage . "-" . $lastPage;
                }
                echo "Chunk: " . substr($chunk->content, 0, 50) . "... (" . $pageRange . ")\n";
            }
        }
    }
}
?>
```

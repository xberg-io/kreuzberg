```php title="Element-Based Output (PHP)"
<?php
use Xberg\ExtractionConfig;
use Xberg\XbergApi;

// Configure element-based output
$config = ExtractionConfig::default();
$config->setOutputFormat('element_based');

// Extract document
$resultOutput = Xberg::extract(\Xberg\ExtractInput::fromUri('document.pdf'), $config);
$result = $resultOutput->results[0];

// Access elements
foreach ($result->getElements() as $element) {
    echo "Type: " . $element->getElementType() . "\n";
    echo "Text: " . substr($element->getText(), 0, 100) . "\n";

    if ($element->metadata->pageNumber) {
        echo "Page: " . $element->metadata->pageNumber . "\n";
    }

    if ($element->metadata->coordinates) {
        $coords = $element->metadata->coordinates;
        echo sprintf("Coords: (%s, %s) - (%s, %s)\n",
            $coords->getLeft(), $coords->getTop(),
            $coords->getRight(), $coords->getBottom());
    }

    echo "---\n";
}

// Filter by element type
$titles = array_filter($result->getElements(), function($e) {
    return $e->getElementType() === 'title';
});

foreach ($titles as $title) {
    $level = $title->metadata->additional['level'] ?? 'unknown';
    echo "[{$level}] {$title->getText()}\n";
}
?>
```

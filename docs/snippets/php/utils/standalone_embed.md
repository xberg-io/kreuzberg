```php
<?php
use Xberg\Xberg;
use Xberg\Config\EmbeddingConfig;
use Xberg\Config\EmbeddingModelType;

$xberg = new Xberg();

// Embed with default config (balanced preset)
$embeddings = $xberg->embed(["Hello world", "How are you?"]);

// Embed with specific preset
$config = new EmbeddingConfig(model: EmbeddingModelType::preset("fast"));
$embeddings = $xberg->embed(["Hello world"], $config);

// Each embedding is a float array
foreach ($embeddings as $i => $vector) {
    echo "Text $i: " . count($vector) . " dimensions\n";
}
```

```php
<?php
use Xberg\XbergApi;
use Xberg\EmbeddingConfig;
use Xberg\EmbeddingModelType;


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

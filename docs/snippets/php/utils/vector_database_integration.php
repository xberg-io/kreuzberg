```php
<?php

declare(strict_types=1);

/**
 * Vector Database Integration
 *
 * Extract documents with chunking and embeddings for vector database storage.
 * Demonstrates preparing data for semantic search and RAG applications.
 */

require_once __DIR__ . '/vendor/autoload.php';

use Kreuzberg\Kreuzberg;
use Kreuzberg\Config\ExtractionConfig;
use Kreuzberg\Config\ChunkingConfig;
use Kreuzberg\Config\EmbeddingConfig;
use Kreuzberg\Enums\EmbeddingModelType;

// Configure for vector database integration
$config = new ExtractionConfig(
    chunking: new ChunkingConfig(
        maxChars: 512,
        maxOverlap: 50,
        embedding: new EmbeddingConfig(
            model: EmbeddingModelType::preset('balanced'),
            normalize: true
        )
    )
);

$kreuzberg = new Kreuzberg($config);
$result = $kreuzberg->extractFile('document.pdf');

echo "Vector Database Integration:\n";
echo str_repeat('=', 60) . "\n";
echo "Document: document.pdf\n";
echo "Total chunks: " . count($result->chunks ?? []) . "\n\n";

// Prepare records for vector database
$vectorRecords = [];

foreach ($result->chunks ?? [] as $index => $chunk) {
    if ($chunk->embedding === null) {
        continue;
    }

    // Generate unique ID for the chunk
    $chunkId = sprintf(
        'doc_%s_chunk_%d',
        md5('document.pdf'),
        $index
    );

    // Prepare vector record
    $vectorRecords[] = [
        'id' => $chunkId,
        'content' => $chunk->content,
        'embedding' => $chunk->embedding,
        'metadata' => [
            'source_file' => 'document.pdf',
            'chunk_index' => $index,
            'chunk_length' => strlen($chunk->content),
            'embedding_model' => 'balanced',
            'created_at' => date('c'),
        ],
    ];
}

echo "Prepared " . count($vectorRecords) . " records for vector database\n\n";

// Display sample record structure
if (!empty($vectorRecords)) {
    echo "Sample Vector Record Structure:\n";
    echo str_repeat('-', 40) . "\n";

    $sample = $vectorRecords[0];
    echo "ID: {$sample['id']}\n";
    echo "Content preview: " . substr($sample['content'], 0, 100) . "...\n";
    echo "Embedding dimensions: " . count($sample['embedding']) . "\n";
    echo "Metadata keys: " . implode(', ', array_keys($sample['metadata'])) . "\n\n";
}

// Example: Insert into Pinecone (pseudo-code)
function insertIntoPinecone(array $records, string $namespace = 'default'): void
{
    // This is a pseudo-code example
    // In practice, use Pinecone SDK: composer require pinecone-io/pinecone-php-client

    echo "Inserting into Pinecone:\n";
    echo str_repeat('-', 40) . "\n";

    $batches = array_chunk($records, 100); // Pinecone batch limit

    foreach ($batches as $batchIndex => $batch) {
        echo sprintf(
            "Batch %d: Upserting %d vectors to namespace '%s'...\n",
            $batchIndex + 1,
            count($batch),
            $namespace
        );

        // Pseudo-code for Pinecone upsert
        // $pinecone->upsert($namespace, $batch);
    }

    echo "Completed inserting " . count($records) . " vectors\n\n";
}

// Example: Insert into Weaviate (pseudo-code)
function insertIntoWeaviate(array $records, string $className = 'Document'): void
{
    // This is a pseudo-code example
    // In practice, use Weaviate SDK

    echo "Inserting into Weaviate:\n";
    echo str_repeat('-', 40) . "\n";

    foreach ($records as $index => $record) {
        $object = [
            'class' => $className,
            'properties' => [
                'content' => $record['content'],
                'sourceFile' => $record['metadata']['source_file'],
                'chunkIndex' => $record['metadata']['chunk_index'],
                'createdAt' => $record['metadata']['created_at'],
            ],
            'vector' => $record['embedding'],
        ];

        // Pseudo-code for Weaviate insert
        // $weaviate->data()->create($object);

        if (($index + 1) % 10 === 0) {
            echo sprintf("Inserted %d/%d objects\n", $index + 1, count($records));
        }
    }

    echo "Completed inserting " . count($records) . " objects\n\n";
}

// Example: Insert into Qdrant (pseudo-code)
function insertIntoQdrant(
    array $records,
    string $collectionName = 'documents'
): void {
    // This is a pseudo-code example
    // In practice, use Qdrant client

    echo "Inserting into Qdrant:\n";
    echo str_repeat('-', 40) . "\n";

    $points = [];

    foreach ($records as $record) {
        $points[] = [
            'id' => $record['id'],
            'vector' => $record['embedding'],
            'payload' => [
                'content' => $record['content'],
                'metadata' => $record['metadata'],
            ],
        ];
    }

    echo sprintf(
        "Upserting %d points to collection '%s'...\n",
        count($points),
        $collectionName
    );

    // Pseudo-code for Qdrant upsert
    // $qdrant->upsert($collectionName, $points);

    echo "Completed\n\n";
}

// Demonstrate vector database integration (pseudo-code)
echo "Vector Database Integration Examples:\n";
echo str_repeat('=', 60) . "\n\n";

// Pinecone integration
insertIntoPinecone($vectorRecords, 'documents');

// Weaviate integration
insertIntoWeaviate($vectorRecords, 'DocumentChunk');

// Qdrant integration
insertIntoQdrant($vectorRecords, 'document_chunks');

// Batch processing multiple documents
$documents = [
    'doc1.pdf',
    'doc2.pdf',
    'doc3.pdf',
];

echo "Batch Processing for Vector Database:\n";
echo str_repeat('=', 60) . "\n";

$allVectorRecords = [];

$vectorConfig = new ExtractionConfig(
    chunking: new ChunkingConfig(
        maxChars: 512,
        maxOverlap: 50,
        embedding: new EmbeddingConfig(
            model: EmbeddingModelType::preset('balanced'),
            normalize: true
        )
    )
);

$kreuzberg = new Kreuzberg($vectorConfig);

foreach ($documents as $document) {
    if (!file_exists($document)) {
        echo basename($document) . ": File not found\n";
        continue;
    }

    $result = $kreuzberg->extractFile($document);

    echo basename($document) . ":\n";
    echo "  Chunks: " . count($result->chunks ?? []) . "\n";

    foreach ($result->chunks ?? [] as $index => $chunk) {
        if ($chunk->embedding === null) {
            continue;
        }

        $chunkId = sprintf(
            'doc_%s_chunk_%d',
            md5($document),
            $index
        );

        $allVectorRecords[] = [
            'id' => $chunkId,
            'content' => $chunk->content,
            'embedding' => $chunk->embedding,
            'metadata' => [
                'source_file' => basename($document),
                'chunk_index' => $index,
                'chunk_length' => strlen($chunk->content),
                'embedding_model' => 'balanced',
                'created_at' => date('c'),
            ],
        ];
    }
}

echo "\nTotal records prepared: " . count($allVectorRecords) . "\n\n";

// Example: Semantic search simulation
function simulateSemanticSearch(string $query, array $records, int $topK = 5): array
{
    // This is a simplified simulation
    // In practice, the vector database handles similarity search

    echo "Simulating semantic search:\n";
    echo "  Query: \"$query\"\n";
    echo "  Searching " . count($records) . " vectors...\n";
    echo "  Top $topK results:\n\n";

    // In a real implementation, you would:
    // 1. Generate embedding for the query
    // 2. Compute similarity with all vectors
    // 3. Return top K results

    $results = array_slice($records, 0, $topK);

    foreach ($results as $index => $result) {
        echo sprintf(
            "  %d. %s (score: %.3f)\n",
            $index + 1,
            substr($result['content'], 0, 60) . '...',
            0.9 - ($index * 0.05) // Simulated score
        );
        echo sprintf("     Source: %s\n", $result['metadata']['source_file']);
        echo "\n";
    }

    return $results;
}

// Simulate search
if (!empty($allVectorRecords)) {
    echo "Semantic Search Example:\n";
    echo str_repeat('=', 60) . "\n";

    simulateSemanticSearch(
        "How to configure document extraction?",
        $allVectorRecords,
        3
    );
}

// Helper: Export vector records to JSON
function exportVectorRecordsToJson(array $records, string $filename): void
{
    $data = [
        'version' => '1.0',
        'count' => count($records),
        'generated_at' => date('c'),
        'records' => $records,
    ];

    $json = json_encode($data, JSON_PRETTY_PRINT | JSON_UNESCAPED_UNICODE);
    file_put_contents($filename, $json);

    echo "Exported " . count($records) . " vector records to: $filename\n";
}

// Export records
if (!empty($allVectorRecords)) {
    exportVectorRecordsToJson($allVectorRecords, 'vector_records.json');
}
```

---
name: chunking-embeddings
priority: critical
---

# Chunking & Embeddings

**Text splitting strategies, embedding generation with FastEmbed, RAG pipeline integration**

## Chunking Architecture Overview

**Location**: `crates/kreuzberg/src/chunking/`, `crates/kreuzberg/src/embeddings.rs`

```
Extracted Text
    |
[1. Normalization] -> Clean whitespace, remove control chars
    |
[2. Chunk Strategy Selection] -> Fixed-size, semantic, syntax-aware, recursive
    |
[3. Overlap Management] -> Control context window overlap
    |
[4. Optional Embedding] -> Generate vectors with FastEmbed
    |
Output: Vec<Chunk> with text, vectors, metadata
```

## Chunking Strategies

**Location**: `crates/kreuzberg/src/chunking/mod.rs`

| Strategy | Pattern | Best For |
|----------|---------|----------|
| **Fixed-Size** | Sliding window with configurable overlap | Uniform chunks for embedding models with fixed token limits |
| **Semantic** | Split by sentences, merge/split by similarity threshold | Smart context preservation for LLM consumption and semantic search |
| **Syntax-Aware** | Split by paragraph/section/heading/code-block structure | Preserving document structure (sections, code blocks) in RAG |
| **Recursive** (LangChain pattern) | Try separators in order: `\n\n`, `\n`, ` `, `` | Best general-purpose chunking; auto-finds optimal split points |

Key config fields per strategy (see struct definitions in `chunking/mod.rs`):
- Fixed-Size: `chunk_size`, `overlap`, `trim_whitespace`
- Semantic: `target_chunk_size`, `min/max_chunk_size`, `semantic_threshold`, `use_sentence_boundaries`
- Syntax-Aware: `chunk_by` (Paragraph/Section/Heading/Sentence/CodeBlock), `max_chunk_size`, `respect_code_blocks`
- Recursive: `separators[]`, `chunk_size`, `overlap`

## Chunking Configuration Presets

**Location**: `crates/kreuzberg/src/chunking/mod.rs`

| Preset | Chunk Size | Overlap | Strategy | Use Case |
|--------|-----------|---------|----------|----------|
| **Balanced** | 512 tokens | 50 | Semantic | RAG sweet spot |
| **Compact** | 256 tokens | 32 | Fixed-Size | Dense vectors |
| **Extended** | 1024 tokens | 100 | Recursive | Full context |
| **Minimal** | 128 tokens | 16 | (default) | Lightweight embeddings |

Usage: set `config.chunking.preset = Some("balanced")` in `ExtractionConfig`.

## Embedding Generation with FastEmbed

**Location**: `crates/kreuzberg/src/embeddings.rs`

### Model Selection

| Model | Dimensions | Notes |
|-------|-----------|-------|
| `BAAI/bge-small-en-v1.5` (default) | 384 | Fast, excellent for RAG |
| `BAAI/bge-small-zh-v1.5` | 384 | Chinese optimized |
| `BAAI/bge-base-en-v1.5` | 768 | Better quality, slower |
| `jinaai/jina-embeddings-v2-base-en` | 768 | Long context (up to 8192 tokens) |
| `Custom(path)` | varies | Custom ONNX model path |

### Embedding Pattern

`TextEmbeddingManager` provides singleton-cached models per config. Pattern:
1. `get_or_init_model()` -- lazy-loads ONNX model (downloads if needed), caches in `Arc<RwLock<HashMap>>`
2. `embed_chunks()` -- collects chunk texts, calls `model.embed(texts, batch_size)`, zips results back to `ChunkWithEmbedding`

Default config: `batch_size=256`, `device=CPU`, `parallel_requests=4`.

### ONNX Runtime Requirement

Embeddings require ONNX Runtime. Feature-gated via:
```toml
[features]
embeddings = ["dep:fastembed", "dep:ort"]
```

Install: `brew install onnxruntime` (macOS) / `apt install libonnxruntime libonnxruntime-dev` (Linux). Verify: `echo $ORT_DYLIB_PATH`.

## RAG Integration Pattern

The full extraction-to-RAG pipeline:
1. **Extract**: `extract_file(path, config)` -> `ExtractionResult`
2. **Chunk**: Apply preset strategy to `result.content` -> `Vec<Chunk>`
3. **Embed**: If embedding config present, `TextEmbeddingManager::embed_chunks()` -> `Vec<ChunkWithEmbedding>`
4. **Output**: `RagDocument { file_path, metadata, chunks }` ready for vector DB ingestion

See `ChunkWithEmbedding` struct in `types.rs`: contains `text`, `embedding: Vec<f32>`, `dimensions`, `norm`, `metadata`.

## Critical Rules

1. **Chunking is preprocessing** - Always apply before embedding to ensure consistent vector sizes
2. **Overlap prevents information loss** - Set overlap to 15-20% of chunk size
3. **Embedding models are stateful** - Lazy load and cache to avoid repeated initialization
4. **ONNX Runtime is required** - Gracefully degrade if not available (skip embeddings)
5. **Batch embedding for performance** - Never embed single chunks; batch 50-1000 chunks
6. **Normalize embeddings for search** - Use L2 norm for cosine similarity
7. **Cache embedding results** - Don't re-embed identical text chunks
8. **Model selection impacts quality** - bge-small (384) for speed, bge-base (768) for quality

## Related Skills

- **extraction-pipeline-patterns** - Text extraction preceding chunking
- **api-server-mcp** - Endpoint for chunking + embedding operations
- **ocr-backend-management** - OCR text quality affects chunking success

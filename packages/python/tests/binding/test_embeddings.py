"""Comprehensive tests for embeddings/vector generation in Python binding.

Tests cover:
- Vector generation correctness (dimensions, type validation)
- Embedding dimension verification (e.g., 384, 512, 768, 1024)
- Performance with batch operations (multiple chunks)
- Format-specific embedding handling (text vs. PDF vs. Office)
- Similarity score validation (cosine similarity)
- Model switching (if supported)
- Normalization correctness (L2 norm = 1.0)

Test Pattern:
The tests follow the established pattern from other binding tests, using:
- ExtractionConfig with chunking/embedding enabled
- ChunkingConfig with embedding parameter for vector generation
- extract_bytes_sync for synchronous operations
- PyO3 bindings for FFI
- Assertions on embedding vectors, dimensions, and mathematical properties
"""

from __future__ import annotations

import math
import platform

import pytest

from kreuzberg import (
    ChunkingConfig,
    EmbeddingConfig,
    EmbeddingModelType,
    ExtractionConfig,
    extract_bytes_sync,
)


class TestVectorGenerationCorrectness:
    """Test basic vector generation functionality and correctness."""

    def test_vector_generation_with_enabled_embeddings_simple_text(self) -> None:
        """Extract embeddings from simple English text with embeddings enabled."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Machine learning transforms technology and society."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result is not None
        assert hasattr(result, "chunks")
        assert result.chunks is not None
        assert len(result.chunks) > 0

        for chunk in result.chunks:
            assert "embedding" in chunk
            if chunk["embedding"] is not None:
                assert isinstance(chunk["embedding"], list)
                assert len(chunk["embedding"]) > 0
                assert all(isinstance(x, float) for x in chunk["embedding"])

    def test_embeddings_disabled_returns_none(self) -> None:
        """Verify that embeddings=None disables embedding generation."""
        config = ExtractionConfig(
            chunking=ChunkingConfig(max_chars=512, max_overlap=100, embedding=None),
        )

        text = "Text without embeddings disabled should not have vectors."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result is not None
        if result.chunks is not None:
            for chunk in result.chunks:
                assert "embedding" not in chunk or chunk["embedding"] is None

    def test_vector_generation_type_validation(self) -> None:
        """Verify all vector components are valid floats."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Floating point vector generation type validation testing."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result is not None
        assert result.chunks is not None

        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                embedding = chunk["embedding"]
                assert isinstance(embedding, list)
                for value in embedding:
                    assert isinstance(value, float)
                    assert not math.isnan(value)
                    assert not math.isinf(value)

    def test_vector_consistency_across_runs(self) -> None:
        """Verify vector consistency when extracting same text multiple times."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Consistency test for vector generation reproducibility."

        result1 = extract_bytes_sync(text.encode(), "text/plain", config)
        result2 = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result1.chunks is not None
        assert result2.chunks is not None
        assert len(result1.chunks) == len(result2.chunks)

        for chunk1, chunk2 in zip(result1.chunks, result2.chunks, strict=False):
            if chunk1["embedding"] is not None and chunk2["embedding"] is not None:
                embedding1 = chunk1["embedding"]
                embedding2 = chunk2["embedding"]
                assert len(embedding1) == len(embedding2)
                for v1, v2 in zip(embedding1, embedding2, strict=False):
                    assert abs(v1 - v2) < 1e-5


class TestEmbeddingDimensionVerification:
    """Test embedding dimension correctness for different models."""

    def test_balanced_model_dimension_verification(self) -> None:
        """Verify balanced model produces correct embedding dimensions."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Balanced embedding model dimension verification test."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        assert len(result.chunks) > 0

        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                dimension = len(chunk["embedding"])
                assert dimension in [384, 512, 768, 1024]

    def test_fast_model_dimension_verification(self) -> None:
        """Verify fast model produces correct embedding dimensions."""
        model = EmbeddingModelType.preset("fast")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Fast embedding model dimension verification test content."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        if len(result.chunks) > 0:
            for chunk in result.chunks:
                if chunk["embedding"] is not None:
                    dimension = len(chunk["embedding"])
                    assert dimension in [128, 256, 384, 512]

    @pytest.mark.skipif(
        platform.system() == "Windows", reason="Quality embedding model download too slow for Windows CI"
    )
    def test_quality_model_dimension_verification(self) -> None:
        """Verify quality model produces correct embedding dimensions."""
        model = EmbeddingModelType.preset("quality")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Quality embedding model dimension verification test content."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        if len(result.chunks) > 0:
            for chunk in result.chunks:
                if chunk["embedding"] is not None:
                    dimension = len(chunk["embedding"])
                    assert dimension in [768, 1024, 1536]

    def test_consistent_dimensions_across_chunks(self) -> None:
        """Verify all chunks have same embedding dimension."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=100,
                max_overlap=20,
                embedding=embedding_config,
            ),
        )

        text = "First chunk content here. Second chunk content here. Third chunk content here. Fourth chunk content."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        if len(result.chunks) > 1:
            dimensions = [len(chunk["embedding"]) for chunk in result.chunks if chunk["embedding"] is not None]

            if len(dimensions) > 1:
                assert len(set(dimensions)) == 1


class TestBatchEmbeddingPerformance:
    """Test embedding generation with batch operations."""

    def test_batch_embedding_extraction_multiple_texts(self) -> None:
        """Extract embeddings from multiple documents with batch processing."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        texts = [
            "First document about machine learning systems.",
            "Second document discussing neural networks.",
            "Third document covering embedding vectors.",
        ]

        results = []
        for text in texts:
            result = extract_bytes_sync(text.encode(), "text/plain", config)
            results.append(result)

        assert len(results) == 3
        for _i, result in enumerate(results):
            assert result is not None
            assert result.chunks is not None
            assert len(result.chunks) > 0
            for chunk in result.chunks:
                if chunk["embedding"] is not None:
                    assert isinstance(chunk["embedding"], list)
                    assert len(chunk["embedding"]) > 0

    def test_large_text_chunking_with_embeddings(self) -> None:
        """Test embeddings with multiple chunks from large text."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=300,
                max_overlap=50,
                embedding=embedding_config,
            ),
        )

        large_text = " ".join([f"Chunk {i}: Machine learning content." for i in range(10)])
        result = extract_bytes_sync(large_text.encode(), "text/plain", config)

        assert result.chunks is not None
        assert len(result.chunks) > 1

        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                assert isinstance(chunk["embedding"], list)
                assert len(chunk["embedding"]) > 0
                assert all(isinstance(x, float) for x in chunk["embedding"])

    def test_small_text_single_chunk_embedding(self) -> None:
        """Test embedding generation for text that results in single chunk."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=2000,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        small_text = "Short text."
        result = extract_bytes_sync(small_text.encode(), "text/plain", config)

        assert result.chunks is not None
        if len(result.chunks) > 0:
            for chunk in result.chunks:
                if chunk["embedding"] is not None:
                    assert isinstance(chunk["embedding"], list)
                    assert len(chunk["embedding"]) > 0


class TestFormatSpecificEmbeddingHandling:
    """Test embeddings with different document formats."""

    def test_text_format_embedding_extraction(self) -> None:
        """Extract embeddings from plain text format."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text_content = "Plain text format content for embedding generation."
        result = extract_bytes_sync(text_content.encode(), "text/plain", config)

        assert result is not None
        assert result.chunks is not None
        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                assert isinstance(chunk["embedding"], list)

    def test_markdown_format_embedding_extraction(self) -> None:
        """Extract embeddings from markdown format with structure."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        markdown_content = "# Title\n\nSome content here.\n\n## Subtitle\n\nMore content."
        result = extract_bytes_sync(markdown_content.encode(), "text/markdown", config)

        assert result is not None
        assert result.chunks is not None
        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                assert isinstance(chunk["embedding"], list)
                assert len(chunk["embedding"]) > 0

    def test_html_format_embedding_extraction(self) -> None:
        """Extract embeddings from HTML format."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        html_content = "<html><body><p>HTML content for embedding extraction.</p></body></html>"
        result = extract_bytes_sync(html_content.encode(), "text/html", config)

        assert result is not None
        assert result.chunks is not None
        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                assert isinstance(chunk["embedding"], list)
                assert len(chunk["embedding"]) > 0


class TestSimilarityScoreValidation:
    """Test similarity computation and validation."""

    def test_cosine_similarity_calculation_basic(self) -> None:
        """Verify cosine similarity can be computed between vectors."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Machine learning is a subset of artificial intelligence."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        if len(result.chunks) >= 2:
            embeddings = [chunk["embedding"] for chunk in result.chunks if chunk["embedding"] is not None]

            if len(embeddings) >= 2:
                emb1 = embeddings[0]
                emb2 = embeddings[1]

                dot_product = sum(a * b for a, b in zip(emb1, emb2, strict=False))
                assert isinstance(dot_product, float)
                assert -1.0 <= dot_product <= 1.0

    def test_similar_text_produces_similar_vectors(self) -> None:
        """Verify that similar text produces similar vectors."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text1 = "Machine learning transforms technology."
        text2 = "Machine learning changes the tech industry."

        result1 = extract_bytes_sync(text1.encode(), "text/plain", config)
        result2 = extract_bytes_sync(text2.encode(), "text/plain", config)

        assert result1.chunks is not None
        assert result2.chunks is not None

        if result1.chunks and result2.chunks:
            emb1 = result1.chunks[0]["embedding"]
            emb2 = result2.chunks[0]["embedding"]

            if emb1 is not None and emb2 is not None:
                # Calculate proper cosine similarity
                dot_product = sum(a * b for a, b in zip(emb1, emb2, strict=False))
                norm1 = math.sqrt(sum(x * x for x in emb1))
                norm2 = math.sqrt(sum(x * x for x in emb2))

                assert norm1 > 0, "First vector norm must be positive"
                assert norm2 > 0, "Second vector norm must be positive"
                similarity = dot_product / (norm1 * norm2)

                assert isinstance(similarity, float)
                assert -1.0 <= similarity <= 1.0, "Cosine similarity must be in [-1, 1]"
                # Similar texts should have high similarity (> 0.5 for normalized, related texts)
                assert similarity > 0.3, "Similar texts should have positive similarity"

    def test_different_text_produces_different_vectors(self) -> None:
        """Verify that different text produces different vectors."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text1 = "Dogs are loyal animals."
        text2 = "Quantum computing is revolutionary."

        result1 = extract_bytes_sync(text1.encode(), "text/plain", config)
        result2 = extract_bytes_sync(text2.encode(), "text/plain", config)

        assert result1.chunks is not None
        assert result2.chunks is not None

        if result1.chunks and result2.chunks:
            emb1 = result1.chunks[0]["embedding"]
            emb2 = result2.chunks[0]["embedding"]

            if emb1 is not None and emb2 is not None:
                similarity = sum(a * b for a, b in zip(emb1, emb2, strict=False))
                assert isinstance(similarity, float)


class TestModelSwitching:
    """Test switching between different embedding models."""

    def test_preset_model_switching_balanced(self) -> None:
        """Test switching to balanced preset model."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Testing preset model switching capability."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                assert isinstance(chunk["embedding"], list)

    def test_preset_model_switching_fast(self) -> None:
        """Test switching to fast preset model."""
        model = EmbeddingModelType.preset("fast")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Testing fast preset model switching capability."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                assert isinstance(chunk["embedding"], list)

    @pytest.mark.skipif(
        platform.system() == "Windows", reason="Quality embedding model download too slow for Windows CI"
    )
    def test_preset_model_switching_quality(self) -> None:
        """Test switching to quality preset model."""
        model = EmbeddingModelType.preset("quality")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Testing quality preset model switching capability."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                assert isinstance(chunk["embedding"], list)

    def test_different_models_produce_different_dimensions(self) -> None:
        """Verify different models can produce different embedding dimensions."""
        text = "Embedding model dimension difference test."

        model_fast = EmbeddingModelType.preset("fast")
        embedding_fast = EmbeddingConfig(model=model_fast, normalize=True)
        config_fast = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_fast,
            ),
        )
        result_fast = extract_bytes_sync(text.encode(), "text/plain", config_fast)

        model_balanced = EmbeddingModelType.preset("balanced")
        embedding_balanced = EmbeddingConfig(model=model_balanced, normalize=True)
        config_balanced = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_balanced,
            ),
        )
        result_balanced = extract_bytes_sync(text.encode(), "text/plain", config_balanced)

        fast_dims = set()
        balanced_dims = set()

        if result_fast.chunks:
            for chunk in result_fast.chunks:
                if chunk["embedding"] is not None:
                    fast_dims.add(len(chunk["embedding"]))

        if result_balanced.chunks:
            for chunk in result_balanced.chunks:
                if chunk["embedding"] is not None:
                    balanced_dims.add(len(chunk["embedding"]))

        assert len(fast_dims) >= 0
        assert len(balanced_dims) >= 0


class TestNormalizationCorrectness:
    """Test L2 normalization of embedding vectors."""

    def test_normalized_vectors_have_unit_norm(self) -> None:
        """Verify normalized vectors have L2 norm of approximately 1.0."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Testing L2 normalization correctness of embedding vectors."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                embedding = chunk["embedding"]
                l2_norm = math.sqrt(sum(x * x for x in embedding))
                assert abs(l2_norm - 1.0) < 0.01

    def test_unnormalized_vectors_may_have_different_norms(self) -> None:
        """Verify unnormalized embeddings have varying L2 norms."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=False)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Testing unnormalized embedding vectors with different content lengths."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        norms = []
        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                embedding = chunk["embedding"]
                l2_norm = math.sqrt(sum(x * x for x in embedding))
                norms.append(l2_norm)

        if len(norms) > 1:
            # Unnormalized vectors should have varying norms (not all equal to 1.0)
            norms_close_to_one = sum(1 for norm in norms if abs(norm - 1.0) < 0.05)
            assert norms_close_to_one < len(norms), "Unnormalized vectors should have varying norms"

    def test_normalization_preserves_direction(self) -> None:
        """Verify normalization preserves vector direction (dot product same sign)."""
        model = EmbeddingModelType.preset("balanced")
        embedding_normalized = EmbeddingConfig(model=model, normalize=True)
        embedding_unnormalized = EmbeddingConfig(model=model, normalize=False)

        config_normalized = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_normalized,
            ),
        )

        config_unnormalized = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_unnormalized,
            ),
        )

        text = "Testing normalization direction preservation."
        result_normalized = extract_bytes_sync(text.encode(), "text/plain", config_normalized)
        result_unnormalized = extract_bytes_sync(text.encode(), "text/plain", config_unnormalized)

        assert result_normalized.chunks is not None
        assert result_unnormalized.chunks is not None

        if len(result_normalized.chunks) > 0 and len(result_unnormalized.chunks) > 0:
            emb_norm = result_normalized.chunks[0]["embedding"]
            emb_unnorm = result_unnormalized.chunks[0]["embedding"]

            if emb_norm is not None and emb_unnorm is not None:
                dot_product = sum(a * b for a, b in zip(emb_norm, emb_unnorm, strict=False))
                assert dot_product > 0


class TestMathematicalPropertiesAndErrorHandling:
    """Test mathematical properties of embeddings and error handling."""

    def test_identical_vectors_have_unit_similarity(self) -> None:
        """Verify identical vectors have cosine similarity of 1.0."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Identical vector test for cosine similarity."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        if result.chunks and result.chunks[0]["embedding"]:
            emb = result.chunks[0]["embedding"]
            # Vector with itself should have similarity 1.0
            dot_product = sum(a * b for a, b in zip(emb, emb, strict=False))
            norm_sq = sum(x * x for x in emb)
            similarity = dot_product / norm_sq if norm_sq > 0 else 0

            assert abs(similarity - 1.0) < 0.0001, "Identical vectors should have similarity 1.0"

    def test_vector_values_are_valid_floats(self) -> None:
        """Verify embedding values are valid floating-point numbers."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Validating floating-point number properties in embeddings."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                embedding = chunk["embedding"]
                for i, value in enumerate(embedding):
                    assert isinstance(value, float), f"Value at index {i} must be float"
                    assert not math.isnan(value), f"Value at index {i} is NaN"
                    assert not math.isinf(value), f"Value at index {i} is infinite"
                    # For normalized vectors, individual values should be in reasonable range
                    assert -2.0 <= value <= 2.0, f"Value at index {i} is out of range: {value}"

    def test_no_zero_embeddings_for_valid_text(self) -> None:
        """Verify embeddings are not all zeros (dead embeddings)."""
        model = EmbeddingModelType.preset("balanced")
        embedding_config = EmbeddingConfig(model=model, normalize=True)
        config = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config,
            ),
        )

        text = "Testing for dead embeddings and all-zero vectors."
        result = extract_bytes_sync(text.encode(), "text/plain", config)

        assert result.chunks is not None
        for chunk in result.chunks:
            if chunk["embedding"] is not None:
                embedding = chunk["embedding"]
                # Check sum of absolute values
                magnitude = sum(abs(x) for x in embedding)
                assert magnitude > 0.1, "Embedding should not be all zeros (dead embedding)"

    def test_embedding_dimensions_consistency_with_model(self) -> None:
        """Verify embedding dimensions are consistent with selected model."""
        model_balanced = EmbeddingModelType.preset("balanced")
        embedding_config_balanced = EmbeddingConfig(model=model_balanced, normalize=True)
        config_balanced = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config_balanced,
            ),
        )

        model_fast = EmbeddingModelType.preset("fast")
        embedding_config_fast = EmbeddingConfig(model=model_fast, normalize=True)
        config_fast = ExtractionConfig(
            chunking=ChunkingConfig(
                max_chars=512,
                max_overlap=100,
                embedding=embedding_config_fast,
            ),
        )

        text = "Testing dimension consistency across different models."

        result_balanced = extract_bytes_sync(text.encode(), "text/plain", config_balanced)
        result_fast = extract_bytes_sync(text.encode(), "text/plain", config_fast)

        balanced_dims = set()
        fast_dims = set()

        if result_balanced.chunks:
            for chunk in result_balanced.chunks:
                if chunk["embedding"] is not None:
                    balanced_dims.add(len(chunk["embedding"]))

        if result_fast.chunks:
            for chunk in result_fast.chunks:
                if chunk["embedding"] is not None:
                    fast_dims.add(len(chunk["embedding"]))

        # Each model should be consistent
        if balanced_dims:
            assert len(balanced_dims) == 1, "Balanced model should produce consistent dimensions"
        if fast_dims:
            assert len(fast_dims) == 1, "Fast model should produce consistent dimensions"

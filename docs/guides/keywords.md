# Keyword Extraction User Guide

## Table of Contents

1. [Introduction](#introduction)
2. [When to Use Keyword Extraction](#when-to-use-keyword-extraction)
3. [Algorithm Deep-Dive](#algorithm-deep-dive)
4. [Configuration Guide](#configuration-guide)
5. [Practical Examples](#practical-examples)
6. [Best Practices and Tuning](#best-practices-and-tuning)
7. [Integration with Post-Processors](#integration-with-post-processors)
8. [Troubleshooting](#troubleshooting)

---

## Introduction

### What is Keyword Extraction?

Keyword extraction automatically identifies important terms and phrases from document text without manual annotation or training. Instead of reading through entire documents, you get a ranked list of the most relevant keywords that capture the document's essence.

**Example**:
```
Input Text:
"Machine learning is a subset of artificial intelligence that enables
systems to learn and improve from experience without explicit programming.
Deep learning uses neural networks to process data..."

Extracted Keywords:
1. "machine learning" (score: 0.92)
2. "neural networks" (score: 0.88)
3. "deep learning" (score: 0.85)
4. "artificial intelligence" (score: 0.82)
5. "experience" (score: 0.65)
```

### Why Keyword Extraction Matters

Keyword extraction is the foundation for several important use cases:

1. **Search Engine Optimization**: Automatically identify on-page keywords for SEO analysis
2. **Content Classification**: Tag documents automatically based on extracted keywords
3. **Information Retrieval**: Build efficient keyword indexes for fast document search
4. **Content Summarization**: Extract keywords to understand document themes at a glance
5. **Topic Modeling**: Group documents by common keywords to discover themes
6. **Metadata Generation**: Automatically generate document tags and descriptions
7. **Named Entity Recognition Preparation**: Pre-process documents to identify important entities

### How It Works

Kreuzberg provides two proven algorithms:

| Algorithm | Approach | Strength | Best For |
|-----------|----------|----------|----------|
| **YAKE** | Statistical scoring | General-purpose | Research papers, news, mixed content |
| **RAKE** | Co-occurrence graphs | Multi-word phrases | Technical docs, product descriptions |

---

## When to Use Keyword Extraction

### Use Cases Where Keyword Extraction Shines

**1. Research Paper Analysis**

Extract key concepts from scientific papers automatically:

```python title="Research Paper Keyword Extraction"
from kreuzberg import ExtractionConfig, KeywordConfig, KeywordAlgorithm

config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.YAKE,
        max_keywords=20,
        ngram_range=(1, 3),
        language="en"
    )
)

# Extract paper and get keywords for indexing
result = await extract_file("research_paper.pdf", config=config)
keywords = result.extracted_keywords or []

# Use for: indexing, citation networks, topic classification
```

**2. Product Catalog Analysis**

Extract product features and characteristics from descriptions:

```python title="Product Catalog Feature Extraction"
config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.RAKE,  # Better for features
        max_keywords=15,
        min_score=0.2,
        ngram_range=(1, 4),  # Multi-word feature names
        language="en"
    )
)

# Extract product descriptions
result = await extract_file("product_catalog.pdf", config=config)
# Use for: faceted search, recommendation systems, attribute extraction
```

**3. Legal Document Processing**

Extract relevant legal terms and concepts:

```python title="Legal Document Keyword Extraction"
config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.RAKE,
        max_keywords=25,
        language="en"
    )
)

# Process contracts and legal agreements
result = await extract_file("contract.pdf", config=config)
# Use for: contract analysis, risk identification, comparison
```

**4. Content Organization**

Automatically tag documents for knowledge management:

```python title="Content Tagging Configuration"
config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.YAKE,
        max_keywords=5,  # Top 5 for tagging
        min_score=0.4,
        language="en"
    )
)

# Tag documents for knowledge base
result = await extract_file("document.pdf", config=config)
top_keywords = [k for k in (result.extracted_keywords or []) if k["score"] > 0.4]
# Use for: taxonomy creation, knowledge graph building
```

### When Not to Use Keyword Extraction

**Skip keyword extraction when:**

- Your document is very short (< 100 words) - too little context for meaningful keywords
- You need human-authored tags - keyword extraction may miss important context
- The document is purely visual/decorative - no textual content to analyze
- You need semantic understanding beyond key terms - use NLP models instead
- Performance is critical and keywords aren't needed - extraction adds ~5-10% overhead

---

## Algorithm Deep-Dive

### YAKE: Yet Another Keyword Extractor

**Philosophy**: Statistical scoring based on term frequency, position, and co-occurrence statistics.

#### How YAKE Works

1. **Term Frequency Analysis**: Counts how often each term appears
2. **Position Weighting**: Terms appearing early in documents get higher relevance
3. **Co-occurrence Analysis**: Considers relationships between terms within a context window
4. **Statistical Scoring**: Combines these factors into a relevance score

#### YAKE Score Interpretation

**Important**: YAKE scores range from 0.0 to 1.0, where **lower scores indicate higher relevance**.

```
Score Range:
0.0 - 0.2  → Highly relevant (top keywords)
0.2 - 0.4  → Relevant keywords
0.4 - 0.6  → Moderately relevant
0.6 - 1.0  → Less relevant (probably noise)
```

#### YAKE Tuning: window_size Parameter

The `window_size` parameter controls the context window for co-occurrence analysis:

```python title="YAKE Window Size Configuration"
from kreuzberg import KeywordConfig, KeywordAlgorithm, YakeParams

# Small context window (more specific phrases)
config = KeywordConfig(
    algorithm=KeywordAlgorithm.YAKE,
    yake_params=YakeParams(window_size=1),
    # Extracts tightly-bound terms
)

# Default window (balanced)
config = KeywordConfig(
    algorithm=KeywordAlgorithm.YAKE,
    yake_params=YakeParams(window_size=2),  # default
)

# Large context window (broader relationships)
config = KeywordConfig(
    algorithm=KeywordAlgorithm.YAKE,
    yake_params=YakeParams(window_size=4),
    # Captures looser term relationships
)
```

**Practical Values**:
- `window_size=1-2`: Technical documents, narrow domains
- `window_size=2-3`: General-purpose documents (default works best)
- `window_size=3-4`: News articles, discussion-heavy documents

#### YAKE Strengths

- No training required - works out of the box
- Language-independent - same algorithm works across languages
- Handles rare and specialized terms well
- Balanced extraction across document types
- Minimal tuning needed - defaults work for most cases

#### YAKE Limitations

- May extract very common terms if not filtered
- Single-word focus - less effective for multi-word phrases
- Score interpretation requires understanding inverse scoring

---

### RAKE: Rapid Automatic Keyword Extraction

**Philosophy**: Co-occurrence graph analysis that separates keywords using stop words as delimiters.

#### How RAKE Works

1. **Stop Word Identification**: Identifies common stop words as phrase delimiters
2. **Candidate Extraction**: Splits text into candidate phrases using stop words
3. **Co-occurrence Graph**: Builds a graph of word relationships within candidates
4. **Frequency Scoring**: Scores phrases based on word frequencies and degrees in the graph

#### RAKE Score Interpretation

**Important**: RAKE scores are unbounded and higher scores indicate higher relevance.

```
Score Range:
0+ - Higher scores = more relevant
Typical Range:
0.0 - 1.0   → Less relevant phrases (common terms)
1.0 - 5.0   → Moderately relevant
5.0 - 20.0  → Highly relevant
20.0+       → Rare, very specific phrases
```

#### RAKE Tuning Parameters

```python title="RAKE Parameter Configuration"
from kreuzberg import KeywordConfig, KeywordAlgorithm, RakeParams

# Strict phrase extraction
config = KeywordConfig(
    algorithm=KeywordAlgorithm.RAKE,
    rake_params=RakeParams(
        min_word_length=3,           # Ignore short words
        max_words_per_phrase=3       # Shorter phrases only
    ),
)

# Balanced (default)
config = KeywordConfig(
    algorithm=KeywordAlgorithm.RAKE,
    rake_params=RakeParams(
        min_word_length=1,           # Include all words
        max_words_per_phrase=3       # default
    ),
)

# Comprehensive extraction
config = KeywordConfig(
    algorithm=KeywordAlgorithm.RAKE,
    rake_params=RakeParams(
        min_word_length=1,
        max_words_per_phrase=5       # Longer phrases
    ),
)
```

**Parameter Effects**:
- `min_word_length`: Higher values filter out short function words
- `max_words_per_phrase`: Controls phrase complexity

#### RAKE Strengths

- Excellent for multi-word phrases ("machine learning", "neural networks")
- Very fast extraction - suitable for large-scale processing
- Interpretable scoring - directly based on term frequency
- Domain-aware - good stopword handling captures domain specifics

#### RAKE Limitations

- Requires good stopword list - quality depends on language support
- Less effective on poorly structured text
- May over-segment text with many punctuation marks
- Not ideal for single-word indexing

---

## Configuration Guide

### Basic Configuration: Single-Word Keywords

Extract single important terms only:

```python title="Single-Word Keyword Configuration"
from kreuzberg import ExtractionConfig, KeywordConfig, KeywordAlgorithm

config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.YAKE,
        max_keywords=10,
        ngram_range=(1, 1),  # Only single words
        language="en"
    )
)

# Use case: hashtag generation, topic classification
```

### Balanced Configuration: Terms and Phrases

Extract both single terms and multi-word phrases:

```python title="Balanced Terms and Phrases Configuration"
config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.YAKE,
        max_keywords=15,
        ngram_range=(1, 2),  # Single words + bigrams
        language="en"
    )
)

# Use case: general-purpose keyword extraction, SEO analysis
```

### Comprehensive Configuration: Deep Phrase Extraction

Extract longer, more specific phrases:

```python title="Deep Phrase Extraction Configuration"
from kreuzberg import KeywordConfig, KeywordAlgorithm, RakeParams

config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.RAKE,
        max_keywords=20,
        ngram_range=(1, 4),  # Phrases up to 4 words
        min_score=0.1,       # Less selective for rare phrases
        language="en",
        rake_params=RakeParams(
            min_word_length=2,
            max_words_per_phrase=4
        )
    )
)

# Use case: feature extraction, detailed content analysis
```

### Language-Specific Configuration

Configure for different languages and stopword sets:

```python title="Multi-Language Keyword Configuration"
# German technical document
config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.RAKE,
        max_keywords=15,
        language="de",       # German stopwords
        ngram_range=(1, 3)
    )
)

# Japanese document (no stopword filtering)
config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.YAKE,
        max_keywords=10,
        language=None,       # No language-specific filtering
        ngram_range=(1, 2)
    )
)
```

### Selective Extraction: Quality Over Quantity

Extract only high-confidence keywords:

```python title="High-Quality Keyword Filtering"
config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.YAKE,
        max_keywords=50,         # Allow many candidates
        min_score=0.3,           # But filter to high-quality
        ngram_range=(1, 3),
        language="en"
    )
)

# Result: ~5-10 high-confidence keywords
# Use case: metadata generation, core concept extraction
```

---

## Practical Examples

### Example 1: Research Paper Analysis

Extract research concepts and methodologies:

```python title="Research Paper Analysis Function"
import asyncio
from kreuzberg import ExtractionConfig, KeywordConfig, KeywordAlgorithm

async def analyze_research_paper(pdf_path: str):
    config = ExtractionConfig(
        keywords=KeywordConfig(
            algorithm=KeywordAlgorithm.YAKE,
            max_keywords=20,
            min_score=0.2,
            ngram_range=(1, 3),
            language="en"
        )
    )

    result = await extract_file(pdf_path, config=config)

    # Process keywords for research indexing
    keywords = result.extracted_keywords or []

    # Sort by relevance (lower score = more relevant for YAKE)
    sorted_keywords = sorted(keywords, key=lambda k: k["score"])

    print("Top Research Keywords:")
    for keyword in sorted_keywords[:10]:
        print(f"  - {keyword["text"]} (relevance: {1-keyword["score"]:.2%})")

    return result

# Usage
result = asyncio.run(analyze_research_paper("neural_networks.pdf"))
```

### Example 2: Product Feature Extraction

Extract product features for search and filtering:

```python title="Product Feature Extraction with Tiered Scoring"
from kreuzberg import (
    ExtractionConfig,
    KeywordConfig,
    KeywordAlgorithm,
    RakeParams
)

async def extract_product_features(product_doc_path: str):
    config = ExtractionConfig(
        keywords=KeywordConfig(
            algorithm=KeywordAlgorithm.RAKE,  # Better for features
            max_keywords=15,
            min_score=0.5,                    # Higher threshold for relevance
            ngram_range=(1, 4),               # Feature names are often multi-word
            language="en",
            rake_params=RakeParams(
                min_word_length=2,
                max_words_per_phrase=4
            )
        )
    )

    result = await extract_file(product_doc_path, config=config)
    keywords = result.extracted_keywords or []

    # Group features by score tier
    tier_1 = [k for k in keywords if k["score"] > 10.0]  # Core features
    tier_2 = [k for k in keywords if 5.0 < k["score"] <= 10.0]  # Secondary
    tier_3 = [k for k in keywords if k["score"]<= 5.0]  # Tertiary

    print(f"Core Features ({len(tier_1)}):")
    for k in tier_1:
        print(f"  - {k['text']}")

    print(f"Secondary Features ({len(tier_2)}):")
    for k in tier_2:
        print(f"  - {k['text']}")

    return {
        "core": tier_1,
        "secondary": tier_2,
        "tertiary": tier_3
    }

# Usage
features = asyncio.run(extract_product_features("product_catalog.pdf"))
```

### Example 3: Content Tagging Pipeline

Automatically tag documents for knowledge management:

```python title="Document Tagging Pipeline Class"
import asyncio
from kreuzberg import ExtractionConfig, KeywordConfig, KeywordAlgorithm

class DocumentTagger:
    def __init__(self, num_tags: int = 5):
        self.config = ExtractionConfig(
            keywords=KeywordConfig(
                algorithm=KeywordAlgorithm.YAKE,
                max_keywords=num_tags,
                min_score=0.3,  # Filter to top quality
                ngram_range=(1, 2),
                language="en"
            )
        )

    async def tag_document(self, doc_path: str) -> list[str]:
        """Extract top N tags for a document."""
        result = await extract_file(doc_path, config=self.config)
        keywords = result.extracted_keywords or []

        # Return just the text, sorted by relevance
        sorted_keywords = sorted(keywords, key=lambda k: k["score"])
        tags = [k["text"] for k in sorted_keywords]

        return tags

    async def tag_batch(self, doc_paths: list[str]) -> dict[str, list[str]]:
        """Tag multiple documents."""
        results = {}
        for path in doc_paths:
            tags = await self.tag_document(path)
            results[path] = tags
        return results

# Usage
async def main():
    tagger = DocumentTagger(num_tags=5)

    documents = [
        "report1.pdf",
        "report2.pdf",
        "research.pdf"
    ]

    tags = await tagger.tag_batch(documents)

    for doc, doc_tags in tags.items():
        print(f"{doc}: {', '.join(doc_tags)}")

asyncio.run(main())
```

### Example 4: Keyword Comparison (YAKE vs RAKE)

See how algorithms differ on the same text:

```python title="YAKE vs RAKE Algorithm Comparison"
import asyncio
from kreuzberg import (
    ExtractionConfig,
    KeywordConfig,
    KeywordAlgorithm
)

async def compare_algorithms(doc_path: str):
    """Compare YAKE and RAKE on same document."""

    # Extract with YAKE
    yake_config = ExtractionConfig(
        keywords=KeywordConfig(
            algorithm=KeywordAlgorithm.YAKE,
            max_keywords=10,
            ngram_range=(1, 3),
            language="en"
        )
    )
    yake_result = await extract_file(doc_path, config=yake_config)
    yake_keywords = yake_result.extracted_keywords or []

    # Extract with RAKE
    rake_config = ExtractionConfig(
        keywords=KeywordConfig(
            algorithm=KeywordAlgorithm.RAKE,
            max_keywords=10,
            ngram_range=(1, 3),
            language="en"
        )
    )
    rake_result = await extract_file(doc_path, config=rake_config)
    rake_keywords = rake_result.extracted_keywords or []

    # Compare results
    yake_texts = {k["text"] for k in yake_keywords}
    rake_texts = {k["text"] for k in rake_keywords}

    common = yake_texts & rake_texts
    yake_only = yake_texts - rake_texts
    rake_only = rake_texts - yake_texts

    print("=== Algorithm Comparison ===\n")

    print(f"Common Keywords ({len(common)}):")
    for keyword in common:
        print(f"  - {keyword}")

    print(f"\nYAKE Only ({len(yake_only)}):")
    for keyword in yake_only:
        print(f"  - {keyword}")

    print(f"\nRAKE Only ({len(rake_only)}):")
    for keyword in rake_only:
        print(f"  - {keyword}")

# Usage
asyncio.run(compare_algorithms("document.pdf"))
```

---

## Best Practices and Tuning

### 1. Choosing the Right Algorithm

**Use YAKE when:**
- You need general-purpose keyword extraction
- Your documents are in various domains
- You want minimal tuning
- Single words are acceptable as keywords
- Language support is important

**Use RAKE when:**
- You need multi-word phrase extraction
- Documents follow consistent structure
- You can afford language-specific tuning
- Phrase quality is more important than coverage

### 2. Tuning max_keywords Parameter

The `max_keywords` parameter determines how many candidates to extract before filtering:

```python title="max_keywords Parameter Values"
# For high-precision tagging (most relevant only)
max_keywords = 5    # Extremely selective

# For general keyword extraction
max_keywords = 10   # Balanced (default)

# For comprehensive analysis
max_keywords = 20   # Broad coverage

# For keyword frequency analysis
max_keywords = 50+  # Capture all meaningful keywords
```

**Rule of Thumb**: Set `max_keywords` to 2-3x the number you actually want, then use `min_score` to filter.

### 3. Tuning min_score Parameter

Score thresholds depend on your algorithm and document type:

**For YAKE** (inverse scoring: lower = better):

```python title="YAKE min_score Thresholds"
# Very selective (top 1-2 keywords)
min_score = 0.5    # Only core concepts

# Selective (top 5-10 keywords)
min_score = 0.3    # Main topics

# Balanced (top 10-20 keywords)
min_score = 0.2    # Major and minor topics

# Comprehensive (most meaningful)
min_score = 0.1    # Include rare terms
```

**For RAKE** (higher = better):

```python title="RAKE min_score Thresholds"
# Very selective (top 1-3 keywords)
min_score = 20.0   # Only core features

# Selective (top 5-10 keywords)
min_score = 5.0    # Main features

# Balanced (top 10-20 keywords)
min_score = 1.0    # All important features

# Comprehensive (most meaningful)
min_score = 0.1    # Include minor features
```

### 4. Optimize for Document Length

**Short documents** (< 500 words):

```python title="Short Document Configuration"
# Less context means fewer meaningful keywords
config = KeywordConfig(
    algorithm=KeywordAlgorithm.YAKE,
    max_keywords=5,      # Reduce from default 10
    min_score=0.4,       # Higher threshold
    ngram_range=(1, 1)   # Single words only
)
```

**Medium documents** (500-5000 words):

```python title="Medium Document Configuration"
# Standard configuration works well
config = KeywordConfig(
    algorithm=KeywordAlgorithm.YAKE,
    max_keywords=10,     # Default
    min_score=0.2,
    ngram_range=(1, 3)
)
```

**Long documents** (5000+ words):

```python title="Long Document Configuration"
# More context allows comprehensive extraction
config = KeywordConfig(
    algorithm=KeywordAlgorithm.RAKE,
    max_keywords=30,     # Extract more
    min_score=0.5,       # Can afford higher threshold
    ngram_range=(1, 4)   # Longer phrases
)
```

### 5. Domain-Specific Optimization

**Technical Documentation**:

```python title="Technical Documentation Configuration"
# Technical docs benefit from phrase extraction
config = KeywordConfig(
    algorithm=KeywordAlgorithm.RAKE,
    max_keywords=20,
    ngram_range=(1, 4),  # Capture terms like "machine learning model"
    language="en",
    rake_params=RakeParams(
        min_word_length=2,
        max_words_per_phrase=4
    )
)
```

**News Articles**:

```python title="News Article Configuration"
# News needs balanced term/phrase mix
config = KeywordConfig(
    algorithm=KeywordAlgorithm.YAKE,
    max_keywords=15,
    ngram_range=(1, 2),  # Mix of terms and bigrams
    min_score=0.25
)
```

**Academic Papers**:

```python title="Academic Paper Configuration"
# Papers need comprehensive concept extraction
config = KeywordConfig(
    algorithm=KeywordAlgorithm.YAKE,
    max_keywords=25,
    ngram_range=(1, 3),
    min_score=0.15,
    language="en"
)
```

### 6. Benchmark Before Production

Always test your configuration on a sample of documents:

```python title="Configuration Benchmark Function"
import asyncio
from statistics import mean, stdev

async def benchmark_configuration(
    sample_docs: list[str],
    config: ExtractionConfig
) -> dict:
    """Test configuration on sample documents."""

    keyword_counts = []
    extraction_times = []

    for doc_path in sample_docs:
        import time
        start = time.time()
        result = await extract_file(doc_path, config=config)
        elapsed = time.time() - start

        keywords = result.extracted_keywords or []
        keyword_counts.append(len(keywords))
        extraction_times.append(elapsed)

    return {
        "avg_keywords": mean(keyword_counts),
        "keyword_stdev": stdev(keyword_counts) if len(keyword_counts) > 1 else 0,
        "avg_time_ms": mean(extraction_times) * 1000,
        "time_stdev_ms": stdev(extraction_times) * 1000 if len(extraction_times) > 1 else 0
    }

# Usage
async def main():
    config = ExtractionConfig(keywords=KeywordConfig(...))

    sample_docs = [
        "doc1.pdf", "doc2.pdf", "doc3.pdf", "doc4.pdf", "doc5.pdf"
    ]

    stats = await benchmark_configuration(sample_docs, config)

    print(f"Average keywords: {stats['avg_keywords']:.1f}")
    print(f"Average extraction time: {stats['avg_time_ms']:.0f}ms")

    # Adjust configuration based on results
    if stats['avg_keywords'] < 5:
        print("Consider lowering min_score for more keywords")
    elif stats['avg_keywords'] > 30:
        print("Consider raising min_score for fewer, higher-quality keywords")

asyncio.run(main())
```

---

## Integration with Post-Processors

### Deduplication with Keywords

Keywords extracted from the same document may contain duplicates or near-duplicates:

```python title="Keywords with Deduplication Post-Processing"
from kreuzberg import ExtractionConfig, KeywordConfig, KeywordAlgorithm

config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.RAKE,
        max_keywords=20,
        language="en"
    ),
    postprocessor=PostProcessorConfig(
        enabled=True,
        enabled_processors=["deduplication"]
    )
)

# Deduplication runs on extracted text before keyword extraction
# This improves keyword quality by removing duplicate content
```

### Keyword Extraction Pipeline

Typical workflow combining multiple features:

```python title="Full Keyword Extraction Pipeline"
async def keyword_pipeline(doc_path: str):
    config = ExtractionConfig(
        # Clean extraction with quality processing
        enable_quality_processing=True,

        # Detect document language
        language_detection=LanguageDetectionConfig(
            enabled=True,
            min_confidence=0.7
        ),

        # Extract keywords in detected language
        keywords=KeywordConfig(
            algorithm=KeywordAlgorithm.YAKE,
            max_keywords=15,
            ngram_range=(1, 3)
        ),

        # Clean up duplicates and normalize
        postprocessor=PostProcessorConfig(
            enabled=True,
            enabled_processors=[
                "deduplication",
                "whitespace_normalization"
            ]
        )
    )

    result = await extract_file(doc_path, config=config)

    # Detect language used for keyword extraction
    language = result.detected_languages[0] if result.detected_languages else "en"
    print(f"Document language: {language}")

    # Get extracted keywords
    keywords = result.extracted_keywords or []
    print(f"Extracted {len(keywords)} keywords")

    return result

# Usage
result = asyncio.run(keyword_pipeline("document.pdf"))
```

### Token Reduction with Keywords

For LLM processing, combine token reduction with keywords:

```python title="Keywords with Token Reduction for LLM"
from kreuzberg import TokenReductionConfig

config = ExtractionConfig(
    keywords=KeywordConfig(
        algorithm=KeywordAlgorithm.YAKE,
        max_keywords=10,
        language="en"
    ),

    # Reduce tokens while preserving keywords
    token_reduction=TokenReductionConfig(
        mode="moderate",
        preserve_important_words=True
    )
)

# Keywords remain even with aggressive token reduction
result = await extract_file("document.pdf", config=config)
```

---

## Troubleshooting

### Issue: Getting Too Few Keywords

**Symptoms**:
- Fewer keywords returned than `max_keywords`
- Only 1-2 keywords extracted from large documents

**Causes and Solutions**:

```python title="Fix: Too Few Keywords"
# 1. min_score is too high
# Solution: Lower the threshold
config.keywords.min_score = 0.1  # Was 0.5

# 2. Document has little text
# Solution: Check if extraction succeeded
if not result.content:
    print("No text extracted - may need OCR")

# 3. Document language mismatch
# Solution: Set correct language
config.keywords.language = "fr"  # Was "en"
```

### Issue: Getting Too Many Low-Quality Keywords

**Symptoms**:
- Too many irrelevant keywords returned
- Keywords are mostly stopwords or filler terms

**Causes and Solutions**:

```python title="Fix: Too Many Low-Quality Keywords"
# 1. min_score is too low
# Solution: Raise the threshold
config.keywords.min_score = 0.3  # Was 0.1

# 2. Language not set
# Solution: Specify language for stopword filtering
config.keywords.language = "en"  # Was None

# 3. ngram_range too broad
# Solution: Limit to shorter phrases
config.keywords.ngram_range = (1, 2)  # Was (1, 3)
```

### Issue: Wrong Algorithm for Document Type

**Symptoms**:
- YAKE returns mostly single words when you need phrases
- RAKE returns fewer results than YAKE

**Solution**:

```python title="Fix: Wrong Algorithm Selection"
# For multi-word phrases, use RAKE
config.keywords.algorithm = KeywordAlgorithm.RAKE
config.keywords.ngram_range = (1, 3)

# For single-word indexing, use YAKE
config.keywords.algorithm = KeywordAlgorithm.YAKE
config.keywords.ngram_range = (1, 1)
```

### Issue: Keyword Extraction Very Slow

**Symptoms**:
- Extraction time increases significantly with keywords enabled
- CPU usage spikes during keyword processing

**Causes and Solutions**:

```python title="Fix: Slow Keyword Extraction"
# 1. max_keywords too high
# Solution: Reduce candidate generation
config.keywords.max_keywords = 10  # Was 50

# 2. Large documents with complex phrases
# Solution: Disable features you don't need
config.keywords.ngram_range = (1, 2)  # Was (1, 4)

# 3. RAKE on very large documents
# Solution: Use YAKE instead (typically faster)
config.keywords.algorithm = KeywordAlgorithm.YAKE

# 4. Large batch processing
# Solution: Reduce max_concurrent_extractions
config.max_concurrent_extractions = 4  # Reduce parallelism
```

### Issue: Keywords Don't Match Document Content

**Symptoms**:
- Extracted keywords seem unrelated to document
- Different results on similar documents

**Debugging Steps**:

```python title="Debug Keyword Extraction Issues"
async def debug_keywords(doc_path: str):
    config = ExtractionConfig(
        keywords=KeywordConfig(
            algorithm=KeywordAlgorithm.YAKE,
            max_keywords=20,  # Get more for analysis
            language="en"
        )
    )

    result = await extract_file(doc_path, config=config)

    # Step 1: Verify extraction worked
    print(f"Extracted text length: {len(result.content)} chars")
    print(f"First 200 chars: {result.content[:200]}")

    # Step 2: Check keywords
    keywords = result.extracted_keywords or []
    print(f"\nTotal keywords: {len(keywords)}")

    # Step 3: Verify keywords appear in text
    text_lower = result.content.lower()
    for keyword in keywords[:5]:
        if keyword["text"].lower() in text_lower:
            print(f"✓ '{keyword["text]}' found in text")
        else:
            print(f"✗ '{keyword["text]}' NOT in text (check ngram_range)")

    # Step 4: Check algorithm choice
    print(f"\nAlgorithm: {config.keywords.algorithm}")
    print(f"Language: {config.keywords.language}")

# Usage
asyncio.run(debug_keywords("document.pdf"))
```

---

## Summary

Keyword extraction is a powerful tool for automatically understanding document content. Key takeaways:

1. **YAKE is best for general use**: Start with YAKE and default settings
2. **RAKE is best for phrases**: Use RAKE when you need multi-word expressions
3. **Always benchmark on your data**: Different document types need different configurations
4. **Lower scores for YAKE**: Remember YAKE uses inverse scoring
5. **Combine with other features**: Keywords work well with language detection and post-processing
6. **Monitor performance**: Keyword extraction adds ~5-10% overhead

For more details, see the [Configuration Reference](../reference/configuration.md#keywordconfig) and language-specific examples.

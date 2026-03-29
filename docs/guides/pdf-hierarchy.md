# PDF Hierarchy Detection User Guide

## Table of Contents

1. [Introduction](#introduction)
2. [How It Works](#how-it-works)
3. [Configuration Guide](#configuration-guide)
4. [Code Examples](#code-examples)
5. [Output Structure](#output-structure)
6. [Best Practices](#best-practices)
7. [Troubleshooting](#troubleshooting)

---

## Introduction

### What is PDF Hierarchy Detection?

PDF hierarchy detection is an advanced feature that automatically identifies and extracts document structure from PDFs. It analyzes text properties (primarily font size) to classify content into hierarchical levels corresponding to HTML heading tags (H1-H6) and body text.

Instead of treating all text equally, the system recognizes that:

- **Large text** typically represents main headings (H1)
- **Medium text** represents subheadings (H2, H3, etc.)
- **Small text** represents body content

This structure is extracted using machine learning techniques (K-means clustering) and spatial analysis to create a semantic understanding of your document.

### Why Does It Matter?

Document hierarchy is critical for several applications:

1. **Semantic Understanding**: Helps AI systems understand document structure without explicit markup
2. **Content Organization**: Enables better chunking and retrieval of related content
3. **Accessibility**: Improves compatibility with accessibility tools and screen readers
4. **Content Reuse**: Facilitates converting PDFs to structured formats (Markdown, HTML, etc.)
5. **Search Enhancement**: Improves search results by understanding content importance
6. **RAG Systems**: Enables better semantic chunking for Retrieval-Augmented Generation

### Common Use Cases

- **Knowledge Base Extraction**: Converting product documentation to structured knowledge bases
- **Academic Paper Analysis**: Organizing research papers by section hierarchy
- **Legal Document Processing**: Extracting sections from contracts and regulatory documents
- **Technical Manual Conversion**: Converting technical documentation to interactive formats
- **Content Migration**: Enabling PDF-to-Markdown or PDF-to-HTML conversion with structure preserved
- **Document Summarization**: Extracting key sections based on hierarchy levels
- **Intelligent Chunking**: Creating semantically meaningful chunks for LLM processing

---

## How It Works

### Overview of the Algorithm

PDF hierarchy detection combines three sophisticated techniques:

1. **Character Extraction with Font Metrics**: Extract individual characters with precise position and font size data
2. **Text Block Merging**: Group characters into coherent text blocks using spatial proximity analysis
3. **Font Size Clustering**: Use K-means clustering to identify distinct font size groups
4. **Hierarchy Assignment**: Map clusters to heading levels (H1-H6) and body text

### Step 1: Character Extraction

The system extracts every character from the PDF, preserving critical metadata:

```text
Character Data:
  Text: "A"
  Position: (100.0, 50.0) - x, y coordinates
  Font Size: 24.0 points
  Bounding Box: (100.0, 26.0, 110.0, 50.0)
```

**Why this matters**: Font size variations are the primary indicator of document hierarchy. By preserving exact font sizes, we can identify subtle differences between heading levels.

### Step 2: Text Block Merging

Individual characters are merged into text blocks using a greedy clustering algorithm with **weighted distance analysis**:

- **Horizontal Weight (X-axis)**: 5.0
- **Vertical Weight (Y-axis)**: 1.0

This weighting reflects how text is typically laid out: characters on the same line (small Y distance) should be grouped together, while characters at different X positions on different lines should be separate.

**Merging Thresholds**:

- Characters are merged if they satisfy distance criteria:
  - X-distance threshold: `font_size × 2.0` (allows up to 2 character widths apart)
  - Y-distance threshold: `font_size × 1.5` (allows for slight vertical alignment variations)
- Characters with intersection ratio > 0.05 are also merged (overlapping text)

**Example of block merging**:

```text
Characters: ['H', 'e', 'l', 'l', 'o']
After merging: TextBlock { text: "Hello", font_size: 12.0 }
```

### Step 3: K-Means Clustering

Once blocks are created, the system clusters them by font size using K-means algorithm:

```text
Font sizes: [24.0, 24.1, 18.0, 18.2, 12.0, 12.1, 12.0, 12.3]
K = 6 clusters (default)

Result:
  Cluster 0 (centroid 24.05): ["Title", "Heading"]
  Cluster 1 (centroid 18.1): ["Section A", "Section B"]
  Cluster 2 (centroid 12.1): ["Body text block 1", "Body text block 2", ...]
  Cluster 3-5: Empty or filled with similar-sized text
```

**Algorithm Details**:

- Initialization: Uses actual unique font sizes from the document, sorted in descending order
- Iteration: Up to 100 iterations with convergence threshold of 0.01 points
- Termination: Stops early when centroids stabilize or max iterations reached

### Step 4: Hierarchy Assignment

Clusters are mapped to heading levels based on font size ranking:

```text
Cluster 0 (largest font) → H1
Cluster 1 (2nd largest)  → H2
Cluster 2 (3rd largest)  → H3
Cluster 3 (4th largest)  → H4
Cluster 4 (5th largest)  → H5
Cluster 5 (6th largest)  → H6
Cluster 6+ (remaining)   → Body text
```

### Spatial Information Preservation

When `include_bbox` is enabled, each block retains its bounding box:

```json title="Bounding Box Output Format"
{
  "text": "Chapter 1: Introduction",
  "level": "h1",
  "font_size": 24.0,
  "bbox": [50.0, 100.0, 400.0, 125.0]
}
```

Bounding box format: `[left, top, right, bottom]` in PDF units (typically points).

---

## Configuration Guide

### HierarchyConfig Parameters

The `HierarchyConfig` structure provides fine-grained control over hierarchy extraction:

```rust title="HierarchyConfig Structure"
pub struct HierarchyConfig {
    pub enabled: bool,                              // Enable/disable hierarchy extraction
    pub k_clusters: usize,                          // Number of font size clusters (2-10)
    pub include_bbox: bool,                         // Include bounding box information
    pub ocr_coverage_threshold: Option<f32>,        // OCR trigger threshold (0.0-1.0)
}
```

#### 1. `enabled` (bool, default: true)

Controls whether hierarchy extraction is performed.

```text
Default: true
Type: boolean
Effect: When false, no PageHierarchy is included in output
```

**When to disable**:

- When you only need raw text extraction
- For performance-sensitive applications (enables ~5% performance improvement)
- When document has no clear hierarchy structure

#### 2. `k_clusters` (usize, default: 6, range: 2-10)

Specifies the number of font size clusters to create, directly mapping to heading levels.

```text
Default: 6
Valid Range: 2-10
Mapping: k_clusters = 6 provides H1-H6 (6 heading levels)
```

**Understanding k_clusters**:

| k_clusters | Heading Levels | Body Text | Use Case |
|------------|----------------|-----------|----------|
| 2 | H1 only | Remaining | Very simple documents (single heading) |
| 3 | H1, H2 | Remaining | Simple documents with 2 heading levels |
| 4 | H1-H3 | Remaining | Standard documents with up to 3 heading levels |
| 5 | H1-H4 | Remaining | Documents with up to 4 heading levels |
| 6 | H1-H5 (or H1-H6) | Remaining | Standard web documents (HTML compatibility) |
| 7 | H1-H6 | Remaining | Deeply nested documents |
| 8-10 | All levels | Remaining | Documents with very fine-grained hierarchy |

**Choosing the right value**:

- **Start with 6**: This is the default and works for 95% of documents
- **Reduce to 3-4**: If your document has simple structure (introduction, sections, conclusion)
- **Increase to 7-8**: If your document is deeply nested (book chapters, book sections, subsections, sub-subsections)
- **Analysis tip**: Run a test extraction and count distinct font sizes in your PDF

#### 3. `include_bbox` (bool, default: true)

Controls whether bounding box information is included in the output.

```text
Default: true
Type: boolean
Effect: When true, each block includes (left, top, right, bottom) coordinates
```

**When to disable**:

- Reducing output size (10-15% smaller JSON)
- When spatial information is not needed
- For performance optimization

**When to enable** (recommended):

- Building document visualization tools
- Implementing text selection/highlighting
- Creating PDF annotation systems
- Performing spatial analysis

#### 4. `ocr_coverage_threshold` (Option<f32>, default: None, range: 0.0-1.0)

Determines when Optical Character Recognition (OCR) should be triggered based on text block coverage.

```text
Default: None (OCR triggering disabled)
Valid Range: 0.0-1.0
Semantics: Trigger OCR if text_area / page_area < threshold
```

**Understanding the threshold**:

```text
ocr_coverage_threshold: 0.5
means: "If less than 50% of the page has text, run OCR"

Calculation:
  text_area = sum of all text block areas
  page_area = full page dimensions
  coverage = text_area / page_area

  If coverage < 0.5, OCR is triggered
```

**Practical values**:

| Threshold | Behavior | Use Case |
|-----------|----------|----------|
| None | OCR never triggered by coverage | Documents are fully digital PDF |
| 0.1 | OCR if <10% text coverage | Scanned documents with very little text |
| 0.3 | OCR if <30% text coverage | Mixed digital/scanned documents |
| 0.5 | OCR if <50% text coverage | Primarily digital with some scanned pages |
| 0.8 | OCR if <80% text coverage | Heavily scanned documents |
| 1.0 | OCR always triggered | Force OCR on all pages |

**Note**: OCR triggering requires OCR processor to be configured separately. This parameter only controls when the condition is met.

---

## Code Examples

This section provides complete, production-ready examples for configuring and using PDF hierarchy detection in multiple languages.

### Example 1: Python - Basic Configuration

```python title="Basic Hierarchy Configuration"
from kreuzberg import ExtractionConfig, PdfConfig, HierarchyConfig

# Create hierarchy configuration with default settings
hierarchy_config = HierarchyConfig(
    enabled=True,
    k_clusters=6,
    include_bbox=True,
    ocr_coverage_threshold=None
)

# Create PDF configuration with hierarchy enabled
pdf_config = PdfConfig(
    extract_images=False,
    extract_metadata=True,
    hierarchy=hierarchy_config
)

# Create main extraction configuration
config = ExtractionConfig(
    pdf_options=pdf_config
)

# Extract PDF with hierarchy
result = await extract_bytes(pdf_bytes, "application/pdf", config)

# Access hierarchy information
if result.pages:
    for page_num, page in enumerate(result.pages, 1):
        if page.hierarchy:
            print(f"Page {page_num}: {page.hierarchy.block_count} blocks")
            for block in page.hierarchy.blocks:
                print(f"  [{block.level}] {block.text[:50]}")
```

### Example 2: Python - Advanced Configuration for Academic Papers

```python title="Academic Paper Configuration"
from kreuzberg import ExtractionConfig, PdfConfig, HierarchyConfig

# Optimized for academic papers with deep hierarchies
config = ExtractionConfig(
    pdf_options=PdfConfig(
        extract_images=True,
        extract_metadata=True,
        hierarchy=HierarchyConfig(
            enabled=True,
            k_clusters=7,  # Support up to H6 headings
            include_bbox=True,
            ocr_coverage_threshold=0.3  # Trigger OCR if mostly scanned
        )
    ),
    pages=PageConfig(
        extract_pages=True,
        insert_page_markers=True
    )
)

result = await extract_bytes(pdf_bytes, "application/pdf", config)

# Process hierarchy for knowledge base
if result.pages:
    for page in result.pages:
        if page.hierarchy:
            for block in page.hierarchy.blocks:
                # Store with hierarchy context
                store_block(
                    text=block.text,
                    level=block.level,
                    font_size=block.font_size,
                    page_num=page.page_number,
                    bbox=block.bbox
                )
```

### Example 3: Python - Configuration for Scanned Documents

```python title="Scanned Document Configuration"
from kreuzberg import ExtractionConfig, PdfConfig, HierarchyConfig

# Optimized for scanned/mixed PDFs
config = ExtractionConfig(
    pdf_options=PdfConfig(
        extract_images=False,
        extract_metadata=True,
        hierarchy=HierarchyConfig(
            enabled=True,
            k_clusters=5,  # Simpler hierarchy for reliability
            include_bbox=True,
            ocr_coverage_threshold=0.5  # Trigger OCR aggressively
        )
    )
)

result = await extract_bytes(pdf_bytes, "application/pdf", config)
```

### Example 4: TypeScript - Basic Configuration

```typescript title="TypeScript Hierarchy Configuration"
import { ExtractionConfig, PdfConfig, HierarchyConfig } from 'kreuzberg';

const hierarchyConfig: HierarchyConfig = {
  enabled: true,
  k_clusters: 6,
  include_bbox: true,
  ocr_coverage_threshold: null,
};

const pdfConfig: PdfConfig = {
  extract_images: false,
  extract_metadata: true,
  hierarchy: hierarchyConfig,
};

const config: ExtractionConfig = {
  pdf_options: pdfConfig,
};

const result = await extractBytes(pdfBytes, 'application/pdf', config);

// Process hierarchy
if (result.pages) {
  for (const page of result.pages) {
    if (page.hierarchy) {
      for (const block of page.hierarchy.blocks) {
        console.log(`[${block.level}] ${block.text}`);
        if (block.bbox) {
          console.log(`  Position: ${block.bbox}`);
        }
      }
    }
  }
}
```

### Example 5: Go - Basic Configuration

```go title="Go Hierarchy Configuration"
package main

import (
    "github.com/yourusername/kreuzberg"
)

func main() {
    hierarchyConfig := &kreuzberg.HierarchyConfig{
        Enabled:              true,
        KClusters:           6,
        IncludeBbox:         true,
        OcrCoverageThreshold: nil,
    }

    pdfConfig := &kreuzberg.PdfConfig{
        ExtractImages:   false,
        ExtractMetadata: true,
        Hierarchy:       hierarchyConfig,
    }

    config := &kreuzberg.ExtractionConfig{
        PdfOptions: pdfConfig,
    }

    result, err := kreuzberg.ExtractBytes(pdfBytes, "application/pdf", config)
    if err != nil {
        panic(err)
    }

    // Process hierarchy
    if result.Pages != nil {
        for pageNum, page := range result.Pages {
            if page.Hierarchy != nil {
                println(pageNum, page.Hierarchy.BlockCount, "blocks")
                for _, block := range page.Hierarchy.Blocks {
                    println("[" + block.Level + "] " + block.Text)
                }
            }
        }
    }
}
```

### Example 6: Java - Basic Configuration

```java title="Java Hierarchy Configuration"
import com.kreuzberg.*;

public class HierarchyExample {
    public static void main(String[] args) throws Exception {
        // Create hierarchy configuration
        HierarchyConfig hierarchyConfig = new HierarchyConfig();
        hierarchyConfig.setEnabled(true);
        hierarchyConfig.setKClusters(6);
        hierarchyConfig.setIncludeBbox(true);
        hierarchyConfig.setOcrCoverageThreshold(null);

        // Create PDF configuration
        PdfConfig pdfConfig = new PdfConfig();
        pdfConfig.setExtractImages(false);
        pdfConfig.setExtractMetadata(true);
        pdfConfig.setHierarchy(hierarchyConfig);

        // Create extraction configuration
        ExtractionConfig config = new ExtractionConfig();
        config.setPdfOptions(pdfConfig);

        // Extract PDF
        ExtractionResult result = Kreuzberg.extractBytes(
            pdfBytes,
            "application/pdf",
            config
        );

        // Process hierarchy
        if (result.getPages() != null) {
            for (PageContent page : result.getPages()) {
                if (page.getHierarchy() != null) {
                    PageHierarchy hierarchy = page.getHierarchy();
                    System.out.println(hierarchy.getBlockCount() + " blocks");

                    for (HierarchicalBlock block : hierarchy.getBlocks()) {
                        System.out.println("[" + block.getLevel() + "] " +
                                         block.getText());
                        if (block.getBbox() != null) {
                            System.out.println("  Font: " +
                                             block.getFontSize());
                        }
                    }
                }
            }
        }
    }
}
```

---

## Output Structure

### PageHierarchy Structure

The complete hierarchy for a page is encapsulated in the `PageHierarchy` structure:

```rust title="PageHierarchy Structure"
pub struct PageHierarchy {
    // Number of hierarchy blocks on this page
    pub block_count: usize,

    // Hierarchical blocks with heading levels
    pub blocks: Vec<HierarchicalBlock>,
}
```

**JSON Representation**:

```json title="PageHierarchy JSON Output"
{
  "block_count": 5,
  "blocks": [
    {
      "text": "Chapter 1: Introduction",
      "level": "h1",
      "font_size": 24.0,
      "bbox": [50.0, 100.0, 400.0, 125.0]
    },
    {
      "text": "Background",
      "level": "h2",
      "font_size": 18.0,
      "bbox": [50.0, 150.0, 300.0, 168.0]
    },
    {
      "text": "This chapter provides...",
      "level": "body",
      "font_size": 12.0,
      "bbox": [50.0, 200.0, 550.0, 450.0]
    }
  ]
}
```

### HierarchicalBlock Structure

Individual blocks represent semantic units of text with hierarchy information:

```rust title="HierarchicalBlock Structure"
pub struct HierarchicalBlock {
    // The text content of this block
    pub text: String,

    // The font size of the text in this block
    pub font_size: f32,

    // The hierarchy level of this block (H1-H6 or Body)
    pub level: String,

    // Bounding box information for the block
    pub bbox: Option<(f32, f32, f32, f32)>,
}
```

**Field Descriptions**:

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| `text` | String | Text content of the block | "Introduction" |
| `font_size` | f32 | Font size in points | 24.0 |
| `level` | String | Hierarchy level | "h1", "h2", ..., "h6", "body" |
| `bbox` | Option<Tuple> | Bounding box as (left, top, right, bottom) | Some((50.0, 100.0, 400.0, 125.0)) |

### Level Values

The `level` field can have the following values:

```text
"h1"   - Top-level heading (largest font typically)
"h2"   - Secondary heading
"h3"   - Tertiary heading
"h4"   - Quaternary heading
"h5"   - Quinary heading
"h6"   - Senary heading (smallest heading level)
"body" - Body text (no heading classification)
```

### Complete Example Output

```json title="Complete Extraction Result"
{
  "pages": [
    {
      "page_number": 1,
      "text": "Chapter 1 Introduction This chapter covers the fundamentals...",
      "hierarchy": {
        "block_count": 7,
        "blocks": [
          {
            "text": "Chapter 1: Introduction",
            "level": "h1",
            "font_size": 28.0,
            "bbox": [50.0, 50.0, 500.0, 85.0]
          },
          {
            "text": "Overview",
            "level": "h2",
            "font_size": 20.0,
            "bbox": [50.0, 120.0, 300.0, 145.0]
          },
          {
            "text": "This chapter provides an introduction to the basic concepts and principles that form the foundation of the system.",
            "level": "body",
            "font_size": 12.0,
            "bbox": [50.0, 160.0, 550.0, 280.0]
          },
          {
            "text": "Key Concepts",
            "level": "h2",
            "font_size": 20.0,
            "bbox": [50.0, 310.0, 350.0, 335.0]
          },
          {
            "text": "Distributed Systems",
            "level": "h3",
            "font_size": 14.0,
            "bbox": [70.0, 360.0, 400.0, 378.0]
          },
          {
            "text": "A distributed system is a computing system whose components are located on different networked computers.",
            "level": "body",
            "font_size": 12.0,
            "bbox": [70.0, 390.0, 550.0, 450.0]
          },
          {
            "text": "See page 42 for advanced topics.",
            "level": "body",
            "font_size": 10.0,
            "bbox": [50.0, 520.0, 350.0, 535.0]
          }
        ]
      }
    }
  ]
}
```

---

## Best Practices

### 1. When to Use Hierarchy Detection

Hierarchy detection is most valuable when:

- Documents have clear visual structure with varying font sizes
- You need to understand document organization programmatically
- Building search or RAG systems that benefit from semantic structure
- Converting PDFs to structured formats (Markdown, XML, HTML)
- Creating document summaries or outlines
- Implementing intelligent chunking for LLMs

Hierarchy detection is less useful when:

- Documents have uniform font sizes throughout
- Visual layout is more important than semantic structure
- Performance is absolutely critical and structure is not needed
- Documents are purely decorative or art-focused

### 2. Choosing the Right k_clusters Value

**Start with analysis**:

```python title="Analyze Font Size Distribution"
# Analyze font sizes in your sample PDFs
from collections import Counter

font_sizes = [block.font_size for page in result.pages
              for block in page.hierarchy.blocks]
unique_sizes = sorted(set(font_sizes), reverse=True)

print(f"Found {len(unique_sizes)} unique font sizes")
print(f"Sizes: {unique_sizes[:10]}")  # Show top 10

# Rule of thumb: k_clusters = number of distinct font sizes + 1
recommended_k = min(len(unique_sizes) + 1, 8)
```

**Testing strategy**:

```python title="Test Multiple k_clusters Values"
# Test multiple k values
for k in [3, 4, 5, 6, 7]:
    config.pdf_options.hierarchy.k_clusters = k
    result = await extract(pdf_bytes, config)

    # Check distribution of levels
    level_distribution = Counter(
        block.level for page in result.pages
        for block in page.hierarchy.blocks
    )
    print(f"k={k}: {dict(level_distribution)}")
```

**Optimization decision tree**:

```text
Does your document have only 1-2 heading sizes?
  → Use k_clusters = 2-3

Does your document have 3-4 distinct heading levels?
  → Use k_clusters = 4-5

Does your document follow standard HTML patterns (H1-H6)?
  → Use k_clusters = 6 (recommended default)

Does your document have very deep nesting (book with parts, chapters, sections)?
  → Use k_clusters = 7-8

Are you unsure?
  → Start with k_clusters = 6 (works for 95% of documents)
```

### 3. Performance Optimization

**For maximum performance**:

```python title="Maximum Performance Configuration"
config = ExtractionConfig(
    pdf_options=PdfConfig(
        extract_images=False,          # Skip image extraction
        extract_metadata=False,        # Skip metadata
        hierarchy=HierarchyConfig(
            enabled=False              # Disable hierarchy if not needed
        )
    )
)
```

**For balanced performance**:

```python title="Balanced Performance Configuration"
config = ExtractionConfig(
    pdf_options=PdfConfig(
        extract_images=False,
        extract_metadata=True,
        hierarchy=HierarchyConfig(
            enabled=True,
            k_clusters=5,              # Reduce from 6
            include_bbox=False,        # Skip bounding boxes if not needed
            ocr_coverage_threshold=None
        )
    )
)
```

**Performance impact**:

- Hierarchy extraction: ~5-10% overhead
- Reducing k_clusters from 6 to 4: ~2-3% speedup
- Disabling bbox: ~10% size reduction
- OCR triggering: Minimal overhead (just comparison)

### 4. Handling Edge Cases

**Very short documents** (< 10 text blocks):

```python title="Short Document Configuration"
# Reduce k to number of blocks / 2
config.pdf_options.hierarchy.k_clusters = 3
```

**Documents with many similar font sizes**:

```python title="Similar Font Sizes Configuration"
# May result in many "body" classifications
# Adjust expectations or reduce k_clusters
config.pdf_options.hierarchy.k_clusters = 4
```

**Scanned/OCR documents**:

```python title="OCR Document Configuration"
# OCR may introduce font size variations
config.pdf_options.hierarchy.ocr_coverage_threshold = 0.5
# And use simpler hierarchy
config.pdf_options.hierarchy.k_clusters = 3
```

### 5. Data Quality Assurance

**Validation workflow**:

```python title="Hierarchy Validation Function"
def validate_hierarchy(result):
    issues = []

    for page_num, page in enumerate(result.pages, 1):
        if not page.hierarchy:
            continue

        hierarchy = page.hierarchy

        # Check for empty blocks
        for block in hierarchy.blocks:
            if not block.text.strip():
                issues.append(f"Page {page_num}: Empty text block")

            # Check font sizes are reasonable
            if block.font_size <= 0 or block.font_size > 100:
                issues.append(f"Page {page_num}: Unreasonable font size {block.font_size}")

            # Check hierarchy consistency
            if block.level not in ['h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'body']:
                issues.append(f"Page {page_num}: Invalid level {block.level}")

        # Check block ordering by font size
        levels = ['h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'body']
        prev_level_idx = -1
        for block in hierarchy.blocks:
            level_idx = levels.index(block.level) if block.level in levels else 6
            # Warn if hierarchy skips levels (e.g., H1 then H3)
            if level_idx > prev_level_idx + 1 and block.level.startswith('h'):
                issues.append(f"Page {page_num}: Possible level skip before '{block.text}'")
            prev_level_idx = level_idx

    return issues

# Usage
issues = validate_hierarchy(result)
for issue in issues:
    print(f"Warning: {issue}")
```

---

## Troubleshooting

### Issue: No hierarchy blocks returned

**Symptoms**:

- `page.hierarchy` is None
- `page.hierarchy.blocks` is empty

**Causes and Solutions**:

1. **Hierarchy extraction is disabled**:

   ```python
   # Check configuration
   if config.pdf_options.hierarchy.enabled:
       # Should be True
   ```

2. **PDF has no extractable text**:

   ```python
   # Check if raw text extraction works
   if not result.text:
       print("PDF may be image-only, needs OCR")
       # Enable OCR in your configuration
   ```

3. **Very small document**:

   ```python
   # K-means needs at least k blocks
   if len(all_blocks) < k_clusters:
       print("Document has fewer blocks than k_clusters")
       # Reduce k_clusters
   ```

**Solution workflow**:

```python title="Debugging Empty Hierarchy"
# Step 1: Verify extraction works
config.pdf_options.hierarchy.enabled = True
result = await extract(pdf_bytes, config)

# Step 2: Check raw text
if not result.text:
    print("ERROR: No text extracted - PDF may need OCR")
    return

# Step 3: Check if hierarchy is None or empty
if not result.pages:
    print("ERROR: No pages extracted")
    return

page = result.pages[0]
if not page.hierarchy:
    print("ERROR: Hierarchy is None - check enabled flag")
    return

if not page.hierarchy.blocks:
    print("WARNING: Hierarchy is empty - document may have no text blocks")
    print(f"Raw page text: {page.text[:100]}...")
    return
```

### Issue: Too many "body" level blocks

**Symptoms**:

- All or most blocks classified as "body" instead of headings
- No H1, H2, etc. blocks found

**Causes**:

1. **Document uses uniform font sizes**:
   - K-means cannot distinguish between conceptually different levels
   - This is the expected behavior

2. **k_clusters is too high**:
   - With more clusters, the smallest clusters become "body"
   - Solution: Reduce k_clusters

3. **Large number of similarly-sized text blocks**:
   - Many document types have most text in same size
   - Solution: Increase clustering specificity

**Solutions**:

```python title="Fixing Too Many Body Blocks"
# Solution 1: Reduce k_clusters
config.pdf_options.hierarchy.k_clusters = 4  # From 6

# Solution 2: Analyze and understand your document
result = await extract(pdf_bytes, config)
font_sizes = [block.font_size for page in result.pages
              for block in page.hierarchy.blocks]

from statistics import mean, stdev
if stdev(font_sizes) < 2.0:
    print("Document has very uniform font sizes")
    print("Hierarchy extraction may not be effective")

# Solution 3: Check if document actually has visual hierarchy
# Some documents simply don't have heading structure
```

### Issue: Unexpected hierarchy levels (e.g., H3 with small font, H1 with medium font)

**Symptoms**:

- Levels don't match what you expect based on visual inspection
- Font sizes don't correlate with heading levels

**Causes**:

1. **K-means assigns based on font size ranking, not absolute size**:
   - Largest fonts → H1 (always)
   - 2nd largest → H2 (always)
   - This is by design, not a bug

2. **Document has unusual font patterns**:
   - Title is small, body is large (unusual layouts)
   - Multiple font groups of similar size

**Understanding the behavior**:

```python title="Understanding Level Assignment"
# What's happening:
# - K-means identifies distinct font size clusters
# - Clusters are ranked by centroid (mean) size
# - Ranking is ALWAYS largest → smallest
# - This ensures H1 is always the "most prominent"

result = await extract(pdf_bytes, config)
for block in result.pages[0].hierarchy.blocks:
    print(f"{block.level:6} size={block.font_size:6.1f}  {block.text[:40]}")

# Output might show:
#  h1     size=  24.0  Chapter 1: Introduction
#  h2     size=  18.0  Section 1.1
#  h3     size=  14.0  Subsection 1.1.1
#  body   size=  12.0  This is body text with...
#  body   size=  12.0  More body text...

# This is correct - levels always rank by size
```

**Solution - If appearance doesn't match expectations**:

```python title="Custom Level Adjustment"
# The hierarchy is mathematically correct
# Adjust expectations or:

# Option 1: Use font_size directly instead of level
for block in hierarchy.blocks:
    if block.font_size >= 20:
        print(f"HEADING: {block.text}")
    else:
        print(f"BODY: {block.text}")

# Option 2: Post-process to adjust levels
def adjust_hierarchy(blocks):
    # E.g., merge H3 and H4 if sizes are very close
    for i in range(len(blocks)-1):
        if (blocks[i].level == 'h3' and
            blocks[i+1].level == 'h4' and
            abs(blocks[i].font_size - blocks[i+1].font_size) < 1.0):
            blocks[i+1].level = 'h3'
    return blocks
```

### Issue: Performance degradation with hierarchy enabled

**Symptoms**:

- Extraction is noticeably slower with hierarchy enabled
- Memory usage increases significantly

**Causes**:

1. **K-means algorithm overhead**:
   - ~50-100 iterations of clustering
   - Affects large documents more

2. **Spatial calculations**:
   - Bounding box merging requires distance calculations
   - bbox computation adds overhead

3. **Very large number of text blocks**:
   - Documents with 10,000+ blocks require significant memory

**Solutions**:

```python title="Performance Optimization Solutions"
# Solution 1: Disable bbox if not needed
config.pdf_options.hierarchy.include_bbox = False
# ~10-15% faster, ~10% smaller output

# Solution 2: Reduce k_clusters
config.pdf_options.hierarchy.k_clusters = 4  # From 6
# ~5-10% faster with simpler clustering

# Solution 3: Disable hierarchy if truly not needed
config.pdf_options.hierarchy.enabled = False
# ~5% faster overall

# Solution 4: Process large documents in batches
# Extract specific page ranges
config.pages.extract_pages = True
# Process page by page instead of all at once

# Solution 5: Monitor and optimize
import time
start = time.time()
result = await extract(pdf_bytes, config)
elapsed = time.time() - start
print(f"Extraction took {elapsed:.2f}s")

# Compare configurations
for k in [3, 4, 5, 6]:
    config.pdf_options.hierarchy.k_clusters = k
    start = time.time()
    result = await extract(pdf_bytes, config)
    print(f"k={k}: {time.time()-start:.2f}s")
```

### Issue: Inconsistent results across runs

**Symptoms**:

- Same PDF produces different hierarchy each time it's processed
- Blocks have different levels on different runs

**Likely cause**:

- K-means algorithm has random initialization in some implementations
- However, Kreuzberg uses deterministic initialization from actual font sizes

**Debugging**:

```python title="Debugging Inconsistent Results"
# This should NOT happen with Kreuzberg's implementation
# If you observe inconsistency, it might be due to:

# 1. Floating-point precision on the machine
# 2. Different PDF versions created on different systems
# 3. Concurrent modification of config

# Solution: Ensure config is not shared across async calls
config = ExtractionConfig(...)  # Create fresh for each call
result = await extract(pdf_bytes, config)
```

### Getting Help

When troubleshooting, provide:

1. **Configuration used**:

   ```python
   import json
   config_dict = {
       'k_clusters': config.pdf_options.hierarchy.k_clusters,
       'include_bbox': config.pdf_options.hierarchy.include_bbox,
       'enabled': config.pdf_options.hierarchy.enabled,
   }
   print(json.dumps(config_dict, indent=2))
   ```

2. **Sample of actual output**:

   ```python
   if result.pages:
       page = result.pages[0]
       if page.hierarchy:
           for i, block in enumerate(page.hierarchy.blocks[:5]):
               print(f"{i}: {block.level} (size={block.font_size}) {block.text[:50]}")
   ```

3. **Font size distribution**:

   ```python
   font_sizes = [b.font_size for p in result.pages
                 for b in p.hierarchy.blocks]
   print(f"Min: {min(font_sizes)}, Max: {max(font_sizes)}, Median: {sorted(font_sizes)[len(font_sizes)//2]}")
   ```

---

## Summary

PDF hierarchy detection is a powerful feature that brings semantic understanding to unstructured PDF documents. By leveraging font size analysis and K-means clustering, it automatically identifies document structure without requiring explicit markup.

Key takeaways:

- **Default settings work for 95% of cases**: Start with default k_clusters=6
- **Always validate your hierarchy**: Check a sample of blocks to ensure they make sense
- **Balance configuration for your use case**: Trade off between accuracy and performance
- **Monitor edge cases**: Scanned PDFs, unusual layouts, and simple documents may need adjustments

For best results, analyze your document corpus and adjust k_clusters accordingly. The hierarchy information enables powerful downstream applications like intelligent chunking, semantic search, and document structure preservation in format conversions.

# Layout Detection <span class="version-badge">v4.5.0</span>

Detect document layout regions (tables, figures, headers, text blocks, etc.) in PDFs using ONNX-based deep learning models.

## Overview

Layout detection analyzes document pages to identify and classify structural regions. This enables downstream tasks such as table extraction, figure isolation, and reading-order reconstruction.

Kreuzberg ships two model presets:

| Preset       | Model          | Classes | Speed   | Best For                                    |
| ------------ | -------------- | ------- | ------- | ------------------------------------------- |
| `"fast"`     | YOLO DocLayNet | 11      | Fastest | High-throughput pipelines, general documents |
| `"accurate"` | RT-DETR v2     | 17      | Fast    | Complex layouts, forms, mixed-content pages  |

!!! note "Feature gate"
    Layout detection requires the `layout-detection` Cargo feature. It is not included in the default feature set.

## Performance Impact

Layout detection improves extraction quality at the cost of processing time:

| Pipeline | SF1 (Structure) | TF1 (Text) | Avg Time/Doc |
|----------|-----------------|------------|--------------|
| Baseline | 33.9% | 87.4% | 447ms |
| Layout   | 41.1% | 90.1% | 1500ms |
| **Delta** | **+7.2%** | **+2.7%** | **3.4x slower** |

*Benchmarked on a 171-document PDF corpus (CPU only). GPU acceleration significantly reduces the time penalty.*

!!! tip "GPU Acceleration Recommended"
    Layout detection runs ONNX-optimized models that benefit significantly from GPU acceleration.
    On CPU, expect ~3.4x slower extraction. For layout-heavy workloads, we recommend:

    - **NVIDIA GPUs**: Enable CUDA or TensorRT via `AccelerationConfig(provider="cuda")`
    - **Apple Silicon**: CoreML acceleration is enabled automatically
    - **CPU fallback**: Works correctly but is slower for large document batches

    See [GPU Acceleration](#gpu-acceleration) below for configuration details.

## When to Enable Layout Detection

**Recommended for:**

- Complex PDFs with multi-column layouts, tables, and mixed content
- Scanned documents where structure recognition improves OCR targeting
- Academic papers with headings, formulas, figures, and references
- Business forms with checkboxes, key-value pairs, and tables
- Documents where table extraction quality matters (layout enables SLANet neural table recognition)

**Less beneficial for:**

- Simple single-column text documents (minimal quality improvement)
- High-throughput pipelines where 3.4x latency increase is unacceptable (consider GPU)
- Documents already well-handled by the PDF structure tree

## When to Use Layout Detection

- **Table extraction** -- Locate tables before running OCR or structure parsing.
- **Figure isolation** -- Identify pictures, charts, and diagrams for downstream processing.
- **Reading-order reconstruction** -- Use bounding boxes to determine logical reading order.
- **Selective OCR** -- Only OCR regions classified as text, skipping decorative elements.
- **Document understanding pipelines** -- Feed layout regions into LLMs for structured extraction.

## Model Presets

### Fast (YOLO DocLayNet)

The default preset. Uses a YOLO model trained on the DocLayNet dataset. Detects 11 layout classes:

`Caption`, `Footnote`, `Formula`, `ListItem`, `PageFooter`, `PageHeader`, `Picture`, `SectionHeader`, `Table`, `Text`, `Title`

### Accurate (RT-DETR v2)

Uses an RT-DETR v2 model with NMS-free detection. Detects all 17 layout classes including the 11 above plus:

`DocumentIndex`, `Code`, `CheckboxSelected`, `CheckboxUnselected`, `Form`, `KeyValueRegion`

## Table Structure Models <span class="version-badge">unreleased</span>

When layout detection identifies a table region, a **table structure model** analyzes its internal structure (rows, columns, headers, spanning cells) to produce accurate markdown tables.

### Model Comparison

| Model | Config Value | Size | Speed (CPU) | Best For |
|-------|-------------|------|-------------|----------|
| TATR | `"tatr"` (default) | 30 MB | Fast | General-purpose, consistent results |
| SLANeXT Wired | `"slanet_wired"` | 365 MB | Moderate | Bordered/gridlined tables |
| SLANeXT Wireless | `"slanet_wireless"` | 365 MB | Moderate | Borderless tables |
| SLANeXT Auto | `"slanet_auto"` | ~737 MB | Slower | Mixed documents (auto-classifies per page) |
| SLANet-plus | `"slanet_plus"` | 7.78 MB | Fastest | Resource-constrained environments |

### Choosing a Table Model

**TATR** (default) is a Microsoft DETR-based model that detects rows, columns, headers, and spanning cells from cropped table regions. It produces consistent results across document types and is the smallest full-featured option.

**SLANeXT** (PaddleOCR) takes a fundamentally different approach: it runs on the full page image and outputs HTML structure tokens with cell bounding boxes. Two specialized variants exist -- **wired** (optimized for bordered tables with visible gridlines) and **wireless** (optimized for borderless tables). The **auto** mode uses a PP-LCNet classifier to automatically select the appropriate variant per page.

**SLANet-plus** is a lightweight PaddleOCR model suitable for edge deployment or high-throughput pipelines where table quality is secondary to speed.

### Benchmark Results

On a 171-document PDF corpus (CPU, Apple Silicon):

| Model | Avg SF1 | Avg TF1 | Avg Time/Doc |
|-------|---------|---------|--------------|
| TATR (default) | 40.8% | 88.5% | ~1.7s |
| SLANeXT Wireless | 40.6% | 88.5% | ~1.7s |
| SLANeXT Auto | 40.6% | 88.2% | ~2.0s |

Aggregate scores are comparable, but per-document variance is significant. SLANeXT improves SF1 by 13-20% on some documents (embedded PDFs, academic papers) while TATR performs better on others (invoices, forms).

!!! note "Model Download"
    SLANeXT models are **not** downloaded by default. Use `cache warm --all-table-models` to pre-download them, or they will download automatically on first use.

## Configuration

### Programmatic Configuration

=== "Python"

    ```python
    from kreuzberg import ExtractionConfig, LayoutDetectionConfig, extract_file

    config = ExtractionConfig(
        layout=LayoutDetectionConfig(
            preset="accurate",
            confidence_threshold=0.5,
            apply_heuristics=True,
            table_model="tatr",  # or "slanet_wired", "slanet_wireless", "slanet_auto", "slanet_plus"
        )
    )

    result = await extract_file("document.pdf", config=config)
    ```

=== "TypeScript"

    ```typescript
    import { extract } from "kreuzberg";

    const result = await extract("document.pdf", {
      layout: {
        preset: "accurate",
        confidenceThreshold: 0.5,
        applyHeuristics: true,
        tableModel: "tatr", // or "slanet_wired", "slanet_wireless", "slanet_auto", "slanet_plus"
      },
    });
    ```

=== "Rust"

    ```rust
    use kreuzberg::core::{ExtractionConfig, LayoutDetectionConfig};

    let config = ExtractionConfig {
        layout: Some(LayoutDetectionConfig {
            preset: "accurate".to_string(),
            confidence_threshold: Some(0.5),
            apply_heuristics: true,
            table_model: Some("tatr".to_string()), // or "slanet_wired", "slanet_wireless", etc.
            ..Default::default()
        }),
        ..Default::default()
    };
    ```

### Configuration Files

=== "TOML"

    ```toml title="kreuzberg.toml"
    [layout]
    preset = "fast"
    # confidence_threshold = 0.4   # optional override
    apply_heuristics = true
    # table_model = "tatr"         # default; or "slanet_wired", "slanet_wireless", "slanet_auto", "slanet_plus"
    ```

=== "YAML"

    ```yaml title="kreuzberg.yaml"
    layout:
      preset: fast
      # confidence_threshold: 0.4
      apply_heuristics: true
      # table_model: tatr  # or slanet_wired, slanet_wireless, slanet_auto, slanet_plus
    ```

### Environment Variable

Set `KREUZBERG_LAYOUT_PRESET` to enable layout detection with a preset without modifying code or config files:

```bash
export KREUZBERG_LAYOUT_PRESET=accurate
```

Valid values: `fast`, `accurate` (aliases `yolo`, `rtdetr`, `rt-detr` are also accepted).

When this variable is set and no `layout` configuration exists, a default `LayoutDetectionConfig` is created with the specified preset.

## Model Download and Caching

Models are ONNX files downloaded automatically from HuggingFace on first use. Downloaded models are cached locally so subsequent runs start instantly.

**Default cache location**: `$HOME/.cache/kreuzberg/models/`

You can override the cache directory by setting the `cache_dir` field on `LayoutEngineConfig` when using the Rust API directly.

!!! tip "CI and Docker"
    In containerized environments, mount or pre-populate the model cache directory to avoid downloading models on every container start.

## GPU Acceleration

Layout detection uses ONNX Runtime (ORT) for inference. ORT supports multiple execution providers for hardware acceleration:

| Provider   | Platform      | Notes                                    |
| ---------- | ------------- | ---------------------------------------- |
| CPU        | All           | Default, no extra setup                  |
| CUDA       | Linux, Windows| Requires CUDA toolkit and cuDNN          |
| CoreML     | macOS         | Automatic on Apple Silicon               |
| TensorRT   | Linux         | Requires TensorRT installation           |

ORT automatically selects the best available execution provider at runtime. No configuration is needed -- if CUDA libraries are present, GPU inference is used automatically.

To explicitly control the execution provider, use `AccelerationConfig`:

```python
config = ExtractionConfig(
    layout=LayoutDetectionConfig(preset="accurate"),
    acceleration=AccelerationConfig(provider="cuda", device_id=0)
)
```

Set `provider` to `"cpu"` to disable GPU acceleration entirely. See [AccelerationConfig](../reference/configuration.md#accelerationconfig) for full details.

## Layout Classes Reference

All model backends map their native class IDs to a shared set of 17 canonical classes:

| Class                 | ID | Fast | Accurate | Description                            |
| --------------------- | -- | ---- | -------- | -------------------------------------- |
| `Caption`             | 0  | Yes  | Yes      | Figure or table caption                |
| `Footnote`            | 1  | Yes  | Yes      | Page footnote                          |
| `Formula`             | 2  | Yes  | Yes      | Mathematical formula                   |
| `ListItem`            | 3  | Yes  | Yes      | List item or bullet point              |
| `PageFooter`          | 4  | Yes  | Yes      | Running page footer                    |
| `PageHeader`          | 5  | Yes  | Yes      | Running page header                    |
| `Picture`             | 6  | Yes  | Yes      | Image, chart, or diagram               |
| `SectionHeader`       | 7  | Yes  | Yes      | Section or subsection heading          |
| `Table`               | 8  | Yes  | Yes      | Tabular data region                    |
| `Text`                | 9  | Yes  | Yes      | Body text paragraph                    |
| `Title`               | 10 | Yes  | Yes      | Document or page title                 |
| `DocumentIndex`       | 11 | --   | Yes      | Table of contents or index             |
| `Code`                | 12 | --   | Yes      | Code block or listing                  |
| `CheckboxSelected`    | 13 | --   | Yes      | Checked checkbox                       |
| `CheckboxUnselected`  | 14 | --   | Yes      | Unchecked checkbox                     |
| `Form`                | 15 | --   | Yes      | Form region                            |
| `KeyValueRegion`      | 16 | --   | Yes      | Key-value pair region                  |

## Acknowledgments

Layout detection in Kreuzberg builds on outstanding work from the open-source community:

- **[Docling](https://github.com/DS4SD/docling)** — We use the Docling Heron RT-DETR v2 model for document layout analysis. The Docling project's approach to document understanding, heuristics, and layout classification has been a significant influence on our pipeline design.
- **[TATR (Table Transformer)](https://github.com/microsoft/table-transformer)** — Table structure recognition uses a TATR ONNX model to detect rows, columns, headers, and spanning cells within layout-detected table regions, enabling accurate markdown table generation with colspan/rowspan support.
- **[PaddleOCR](https://github.com/PaddlePaddle/PaddleOCR)** — SLANeXT table structure recognition and PP-LCNet table classifier models provide alternative table analysis backends optimized for wired and wireless table layouts.

## Related Documentation

- [Configuration Reference](../reference/configuration.md#layoutdetectionconfig) -- Full field reference
- [Type Reference](../reference/types.md#layoutdetectionconfig) -- Type definitions across languages
- [Element-Based Output](element-based-output.md) -- Using layout-aware extraction results

# Layout Detection <span class="version-badge">v4.5.0</span>

Detect document layout regions (tables, figures, headers, text blocks, etc.) in PDFs using ONNX-based deep learning models. Enables table extraction, figure isolation, reading-order reconstruction, and selective OCR.

!!! Note "Feature gate"
    Requires the `layout-detection` Cargo feature. Not included in the default feature set.

## Model

Layout detection uses the **RT-DETR v2** model (17 layout classes), an ONNX-based deep learning model for accurate document layout analysis.

### When to Enable

**Recommended for:** complex multi-column PDFs, scanned documents, academic papers, business forms, documents where table extraction quality matters.

**Less beneficial for:** simple single-column text, high-throughput pipelines where latency is critical (consider GPU), documents already well-handled by the PDF structure tree.

### Performance Impact

| Pipeline | Structure F1 | Text F1 | Avg time/doc |
|----------|-------------|---------|--------------|
| Baseline | 33.9% | 87.4% | 447 ms |
| Layout | 41.1% | 90.1% | 1500 ms |

*171-document PDF corpus, CPU only. GPU acceleration significantly reduces the time penalty.*

!!! Warning "`preset` removed"
    The `preset` field (`"fast"` / `"accurate"`) was removed from `LayoutDetectionConfig`. If it appears in a config file it is silently ignored. Only the RT-DETR v2 model is used for layout detection.

## Configuration

=== "Python"

    ```python
    from kreuzberg import ExtractionConfig, LayoutDetectionConfig, extract_file

    config = ExtractionConfig(
        layout=LayoutDetectionConfig(
            confidence_threshold=0.5,
            apply_heuristics=True,
            table_model="tatr",
        )
    )
    result = await extract_file("document.pdf", config=config)
    ```

=== "TypeScript"

    ```typescript
    const result = await extract("document.pdf", {
      layout: {
        confidenceThreshold: 0.5,
        applyHeuristics: true,
        tableModel: "tatr",
      },
    });
    ```

=== "Rust"

    ```rust
    use kreuzberg::core::{ExtractionConfig, LayoutDetectionConfig};

    let config = ExtractionConfig {
        layout: Some(LayoutDetectionConfig {
            confidence_threshold: Some(0.5),
            apply_heuristics: true,
            table_model: Some("tatr".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };
    ```

=== "TOML"

    ```toml title="kreuzberg.toml"
    [layout]
    apply_heuristics = true
    # table_model = "tatr"
    ```

=== "CLI"

    ```bash title="Terminal"
    # Enable layout detection with default settings
    kreuzberg extract document.pdf --layout --content-format markdown

    # Custom confidence threshold
    kreuzberg extract document.pdf --layout-confidence 0.5 --content-format markdown

    # Specific table model
    kreuzberg extract document.pdf --layout --layout-table-model slanet_wired

    # Combined with GPU acceleration
    kreuzberg extract document.pdf --layout --acceleration coreml
    ```

## Table Structure Models <span class="version-badge">v4.5.3</span>

When layout detection identifies a table region, a table structure model analyzes rows, columns, headers, and spanning cells.

| Model | Config value | Size | Speed | Best for |
|-------|-------------|------|-------|----------|
| **TATR** | `"tatr"` (default) | 30 MB | Fast | General-purpose, consistent results |
| SLANeXT Wired | `"slanet_wired"` | 365 MB | Moderate | Bordered/gridlined tables |
| SLANeXT Wireless | `"slanet_wireless"` | 365 MB | Moderate | Borderless tables |
| SLANeXT Auto | `"slanet_auto"` | ~737 MB | Slower | Mixed documents (auto-classifies per page) |
| SLANet-plus | `"slanet_plus"` | 7.78 MB | Fastest | Resource-constrained environments |

!!! Note "Model Download"
    SLANeXT models are not downloaded by default. Use `cache warm --all-table-models` to pre-download, or they download automatically on first use.

## GPU Acceleration

Layout detection uses ONNX Runtime with automatic provider selection:

| Provider | Platform | Notes |
|----------|----------|-------|
| CPU | All | Default, no setup needed |
| CUDA | Linux, Windows | Requires CUDA toolkit + cuDNN |
| CoreML | macOS | Automatic on Apple Silicon |
| TensorRT | Linux | Requires TensorRT |

To override:

```python
config = ExtractionConfig(
    layout=LayoutDetectionConfig(),
    acceleration=AccelerationConfig(provider="cuda", device_id=0)
)
```

See [AccelerationConfig reference](../reference/configuration.md#accelerationconfig) for details.

## Layout Classes

The RT-DETR v2 model detects 17 layout classes:

| Class | Description |
|-------|-------------|
| `Caption` | Figure or table caption |
| `Footnote` | Page footnote |
| `Formula` | Mathematical formula |
| `ListItem` | List item or bullet point |
| `PageFooter` | Running page footer |
| `PageHeader` | Running page header |
| `Picture` | Image, chart, or diagram |
| `SectionHeader` | Section heading |
| `Table` | Tabular data region |
| `Text` | Body text paragraph |
| `Title` | Document or page title |
| `DocumentIndex` | Table of contents |
| `Code` | Code block |
| `CheckboxSelected` | Checked checkbox |
| `CheckboxUnselected` | Unchecked checkbox |
| `Form` | Form region |
| `KeyValueRegion` | Key-value pair region |

## Acknowledgments

- **[Docling](https://github.com/DS4SD/docling)** — RT-DETR v2 model and layout classification approach
- **[TATR](https://github.com/microsoft/table-transformer)** — Table structure recognition with ONNX
- **[PaddleOCR](https://github.com/PaddlePaddle/PaddleOCR)** — SLANeXT table structure and PP-LCNet classifier models

## Related

- [Configuration Reference](../reference/configuration.md#layoutdetectionconfig) — full field reference
- [Element-Based Output](element-based-output.md) — using layout-aware results

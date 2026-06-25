# Xberg vs MinerU

MinerU is an open-source tool from OpenDataLab designed for high-quality PDF extraction, especially for scientific and academic documents. Xberg is a Rust-based general-purpose extraction library covering 96 formats. They overlap on PDF extraction but differ significantly in scope, licensing, and architecture.

!!! Warning "License"

    MinerU is licensed under **AGPL-3.0**. This means any application that uses MinerU must also be released under AGPL, or you need a commercial license. Xberg is **Apache-2.0** -- no copyleft restrictions.

## At a Glance

|                  | Xberg                                                       | MinerU                                                    |
| ---------------- | --------------------------------------------------------------- | --------------------------------------------------------- |
| **Written in**   | Rust                                                            | Python                                                    |
| **File formats** | 96                                                              | PDF + PNG/JPG only                                        |
| **Use from**     | Python, TypeScript, Go, Ruby, Java, C#, PHP, Elixir, Rust, Wasm | Python CLI / library                                      |
| **License**      | Apache-2.0                                                      | **AGPL-3.0**                                              |
| **GPU**          | Optional (ONNX Runtime -- CUDA, CoreML, TensorRT)               | Recommended for best results                              |
| **Sweet spot**   | Broad format extraction, high throughput, polyglot stacks       | Scientific PDFs, academic papers, complex layout analysis |

---

## How They Differ

### Scope

The biggest difference is what each tool is designed to handle.

- **Xberg (96 formats)** -- PDFs, Office docs, spreadsheets, HTML, images (via OCR), email, archives, source code, structured data, LaTeX, Typst, Jupyter notebooks, EPUB, and more. A general-purpose extraction library.
- **MinerU (PDF + images)** -- Handles PDFs and PNG/JPG images. Nothing else. It's a specialist tool, not a general-purpose library.

If your pipeline processes only PDFs, both work. If you also need to handle Word docs, email files, or spreadsheets, Xberg is the only option.

### ML Approach

Different levels of ML investment.

- **Xberg** -- Offers **optional** ONNX-based layout detection (YOLO for speed, RT-DETR v2 for accuracy) covering 17 element classes. Also works without any ML models for pure extraction -- useful when speed matters more than layout understanding.
- **MinerU** -- **ML-first**. Uses heavy deep learning models for layout detection, formula recognition, and table structure extraction. This produces excellent results on complex scientific documents but comes with significant model loading time, memory usage, and a GPU recommendation.

If you're extracting from scientific papers with complex multi-column layouts, equations, and nested tables, MinerU's deep ML pipeline is purpose-built for that. For general document extraction where you don't need heavy layout analysis, Xberg is faster and lighter.

### Architecture and Performance

Different runtime characteristics.

- **Xberg** -- Rust core, runs on CPU by default. GPU acceleration available via ONNX Runtime when layout detection is enabled. Fast startup, low memory footprint for basic extraction.
- **MinerU** -- Python, with heavy PyTorch model loading on startup. GPU recommended for acceptable performance. First-run model downloads can be several gigabytes.

### Language Support

How you integrate each tool.

- **Xberg** -- Polyglot SDKs for Python, TypeScript, Rust, Go, Java/Kotlin JVM, Kotlin Android, C#, Ruby, PHP, Elixir, R, Dart, Swift, Zig, C, and Wasm in the browser.
- **MinerU** -- **Python CLI and library** only. No language bindings, no API server out of the box.

### Embeddings and Chunking

Downstream pipeline support.

- **Xberg** -- Built-in chunking (recursive, semantic, markdown-aware), local embeddings via ONNX models, token reduction, and keyword extraction. Ready for RAG pipelines out of the box.
- **MinerU** -- Outputs extracted content (Markdown, JSON). Chunking and embeddings are left to downstream tools.

---

## When to Use Xberg

- You need to process **more than just PDFs** -- Office docs, email, archives, code, structured data
- You're building a **commercial product** and need a permissive license (Apache-2.0)
- You want **native bindings** in Go, TypeScript, Ruby, Java, or other languages
- You need **fast extraction** without heavy ML model loading
- You want **built-in chunking and embeddings** for RAG pipelines

## When to Use MinerU

- You're working exclusively with **scientific papers and academic PDFs**
- You need **deep layout analysis** -- formulas, multi-column layouts, nested tables
- The **AGPL-3.0 license** is compatible with your project (open-source, research, or you'll purchase a commercial license)
- You have **GPU resources** available and can accept the startup cost of loading large models
- You need the highest possible extraction quality on **complex PDF layouts** and throughput is secondary

---

!!! Info "Benchmarks"

    For extraction speed and quality comparisons between Xberg and MinerU, see the [live benchmark dashboard](https://xberg.io/benchmarks).

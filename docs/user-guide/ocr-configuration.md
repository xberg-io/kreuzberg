# OCR Configuration

Kreuzberg offers simple configuration options for OCR to extract text from images and scanned documents.

## OCR Configuration

All extraction functions in Kreuzberg accept an [`ExtractionConfig`](../api-reference/types.md#extractionconfig) object that can contain OCR configuration:

### Language Configuration

The `language` parameter in a [`TesseractConfig`](../api-reference/ocr-configuration.md#tesseractconfig) object specifies which language model Tesseract should use for OCR:

```python
from kreuzberg import extract_file, ExtractionConfig, TesseractConfig

# Extract text from a German document
result = await extract_file("german_document.pdf", config=ExtractionConfig(ocr_config=TesseractConfig(language="deu")))
```

#### Supported Language Codes

| Language            | Code      | Language           | Code      |
| ------------------- | --------- | ------------------ | --------- |
| English             | `eng`     | German             | `deu`     |
| French              | `fra`     | Spanish            | `spa`     |
| Italian             | `ita`     | Japanese           | `jpn`     |
| Korean              | `kor`     | Simplified Chinese | `chi_sim` |
| Traditional Chinese | `chi_tra` | Russian            | `rus`     |
| Arabic              | `ara`     | Hindi              | `hin`     |

#### Multi-Language Support

You can specify multiple languages by joining codes with a plus sign:

```python
# Document contains both English and German text
result = await extract_file("multilingual.pdf", config=ExtractionConfig(ocr_config=TesseractConfig(language="eng+deu")))
```

!!! note

    The order of languages affects processing time and accuracy. The first language is treated as the primary language.

#### Language Installation

For Tesseract to recognize languages other than English, you need to install the corresponding language data:

- **Ubuntu/Debian**: `sudo apt-get install tesseract-ocr-<lang-code>`
- **macOS**: `brew install tesseract-lang` (installs all languages)
- **Windows**: Download language data from [GitHub](https://github.com/tesseract-ocr/tessdata)

### Page Segmentation Mode (PSM)

The `psm` parameter in a [`TesseractConfig`](../api-reference/ocr-configuration.md#tesseractconfig) object controls how Tesseract analyzes the layout of the page:

```python
from kreuzberg import extract_file, ExtractionConfig, TesseractConfig, PSMMode

# Extract text from a document with a simple layout
result = await extract_file("document.pdf", config=ExtractionConfig(ocr_config=TesseractConfig(psm=PSMMode.SINGLE_BLOCK)))
```

#### Available PSM Modes

| Mode          | Enum Value              | Description                                              | Best For                                       |
| ------------- | ----------------------- | -------------------------------------------------------- | ---------------------------------------------- |
| Auto Only     | `PSMMode.AUTO_ONLY`     | Automatic segmentation without orientation detection     | Modern documents (default - fastest)           |
| Automatic     | `PSMMode.AUTO`          | Automatic page segmentation with orientation detection   | Rotated/skewed documents                       |
| Single Block  | `PSMMode.SINGLE_BLOCK`  | Treat the image as a single text block                   | Simple layouts, preserving paragraph structure |
| Single Column | `PSMMode.SINGLE_COLUMN` | Assume a single column of text                           | Books, articles, single-column documents       |
| Single Line   | `PSMMode.SINGLE_LINE`   | Treat the image as a single text line                    | Receipts, labels, single-line text             |
| Single Word   | `PSMMode.SINGLE_WORD`   | Treat the image as a single word                         | Word recognition tasks                         |
| Sparse Text   | `PSMMode.SPARSE_TEXT`   | Find as much text as possible without assuming structure | Forms, tables, scattered text                  |

### Forcing OCR

By default, Kreuzberg will only use OCR for images and scanned PDFs. For searchable PDFs, it will extract text directly. You can override this behavior with the `force_ocr` parameter in the `ExtractionConfig` object:

```python
from kreuzberg import extract_file, ExtractionConfig

# Force OCR even for searchable PDFs
result = await extract_file("searchable.pdf", config=ExtractionConfig(force_ocr=True))
```

This is useful when:

- The PDF contains both searchable text and images with text
- The embedded text in the PDF has encoding or extraction issues
- You want consistent processing across all documents

## OCR Engine Selection

Kreuzberg supports multiple OCR engines:

### Tesseract (Default)

Tesseract is the default OCR engine and requires no additional installation beyond the system dependency.

### EasyOCR (Optional)

To use EasyOCR:

1. Install with the extra: `pip install "kreuzberg[easyocr]"`
1. Use the `ocr_backend` parameter in the `ExtractionConfig` object:

```python
from kreuzberg import extract_file, ExtractionConfig, EasyOCRConfig  # EasyOCRConfig is imported from kreuzberg

result = await extract_file(
    "document.jpg",
    config=ExtractionConfig(ocr_backend="easyocr", ocr_config=EasyOCRConfig(language_list=["en"])),
)
```

### PaddleOCR (Optional)

To use PaddleOCR:

1. Install with the extra: `pip install "kreuzberg[paddleocr]"`
1. Use the `ocr_backend` parameter in the `ExtractionConfig` object:

```python
from kreuzberg import extract_file, ExtractionConfig, PaddleOCRConfig  # PaddleOCRConfig is imported from kreuzberg

result = await extract_file(
    "document.jpg",
    config=ExtractionConfig(
        ocr_backend="paddleocr", ocr_config=PaddleOCRConfig(language="en")  # PaddleOCR uses different language codes
    ),
)
```

!!! note

    For PaddleOCR, the supported language codes are different: `ch` (Chinese), `en` (English), `french`, `german`, `japan`, and `korean`.

## Performance Optimization

### Default Configuration

Kreuzberg's defaults are optimized out-of-the-box for modern PDFs and standard documents:

- **PSM Mode**: `AUTO_ONLY` - Faster than `AUTO` without orientation detection overhead
- **Language Model**: Disabled by default for optimal performance on modern documents
- **Dictionary Correction**: Enabled for accuracy

The default configuration provides excellent extraction quality for:

- Modern PDFs with embedded text
- Scanned documents with clear printing
- Office documents (DOCX, PPTX, XLSX)
- Standard business documents

### Speed vs Quality Trade-offs

```python
from kreuzberg import ExtractionConfig, TesseractConfig, PSMMode

# Default configuration (optimized for modern documents)
default_config = ExtractionConfig()  # Already optimized for speed and quality

# Maximum speed configuration
speed_config = ExtractionConfig(
    ocr_backend="tesseract",
    ocr_config=TesseractConfig(
        psm=PSMMode.SINGLE_BLOCK,  # Assume simple layout
        tessedit_enable_dict_correction=False,  # Skip dictionary correction
    ),
)

# Maximum accuracy configuration (for degraded/historical documents)
accuracy_config = ExtractionConfig(
    ocr_backend="tesseract",
    ocr_config=TesseractConfig(
        psm=PSMMode.AUTO,  # Full analysis with orientation detection
        language_model_ngram_on=True,  # Enable for degraded/historical text
        tessedit_enable_dict_correction=True,  # Correct OCR errors
    ),
)
```

### Language Model N-gram Settings

The `language_model_ngram_on` parameter controls Tesseract's use of n-gram language models:

- **Default (False)**: Optimized for modern documents with clear text
- **When to enable**: Historical documents, degraded scans, handwritten text, or noisy images

```python
# For degraded or historical documents
historical_config = ExtractionConfig(
    ocr_backend="tesseract",
    ocr_config=TesseractConfig(
        language_model_ngram_on=True,  # Enable for better accuracy on poor quality text
    ),
)
```

### When to Disable OCR

For documents with text layers (searchable PDFs, Office docs), disable OCR entirely:

```python
# No OCR overhead for text documents
text_config = ExtractionConfig(ocr_backend=None)
```

This provides significant speedup (78% of PDFs have text layers and extract in \<0.01s)

## Best Practices

- **Language Selection**: Always specify the correct language for your documents to improve OCR accuracy
- **PSM Mode Selection**: Choose the appropriate PSM mode based on your document layout:
    - Use `PSMMode.AUTO_ONLY` (default) for modern, well-formatted documents
    - Use `PSMMode.SINGLE_BLOCK` for simple layouts with faster processing
    - Use `PSMMode.SPARSE_TEXT` for forms or documents with tables
    - Use `PSMMode.AUTO` only when orientation detection is needed
- **Performance Optimization**:
    - Disable OCR (`ocr_backend=None`) for documents with text layers
    - Disable language model for clean documents (`language_model_ngram_on=False`)
    - Disable dictionary correction for technical documents
- **Image Quality**: For best results, ensure images are:
    - High resolution (at least 300 DPI)
    - Well-lit with good contrast
    - Not skewed or rotated (unless using `PSMMode.AUTO`)

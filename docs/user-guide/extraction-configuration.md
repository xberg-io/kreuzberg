# Extraction Configuration

Kreuzberg provides extensive configuration options for the extraction process through the `ExtractionConfig` class. You can configure Kreuzberg either programmatically through code or via configuration files that are automatically discovered. This guide covers both approaches and common configuration scenarios.

## Configuration Files (Recommended)

Kreuzberg automatically discovers and loads configuration from files in your project directory. This is the recommended approach for consistent configuration across your project.

### Supported Configuration Files

Kreuzberg searches for configuration files in the following order:

1. `kreuzberg.toml` - Dedicated configuration file (recommended)
1. `pyproject.toml` with `[tool.kreuzberg]` section

The search starts from the current working directory and walks up the directory tree until a configuration file is found.

### kreuzberg.toml Example

Create a `kreuzberg.toml` file in your project root:

```toml
# Basic extraction settings
force_ocr = false
chunk_content = true
extract_tables = true
extract_entities = false
extract_keywords = true
keyword_count = 15
max_chars = 2000
max_overlap = 100
ocr_backend = "tesseract"
auto_detect_language = true
auto_detect_document_type = true
document_classification_mode = "text"  # or "vision"
type_confidence_threshold = 0.5

# Tesseract OCR configuration
[tesseract]
language = "eng+deu"  # English and German
psm = 6               # Uniform block of text

# EasyOCR configuration (if using easyocr backend)
[easyocr]
language_list = ["en", "de"]
gpu = false

# PaddleOCR configuration (if using paddleocr backend)
[paddleocr]
language = "en"
use_gpu = false

# Table extraction configuration (GMFT)
[gmft]
verbosity = 1
detector_base_threshold = 0.9
remove_null_rows = true
enable_multi_header = true

# Language detection configuration
[language_detection]
multilingual = true
top_k = 3
low_memory = false

# Entity extraction configuration (spaCy)
[spacy_entity_extraction]
language_models = { en = "en_core_web_sm", de = "de_core_news_sm" }
fallback_to_multilingual = true
```

### pyproject.toml Example

Alternatively, add configuration to your existing `pyproject.toml`:

```toml
[tool.kreuzberg]
force_ocr = false
chunk_content = true
extract_tables = true
auto_detect_language = true
auto_detect_document_type = true
document_classification_mode = "text"
type_confidence_threshold = 0.5

[tool.kreuzberg.tesseract]
language = "eng"
psm = 6

[tool.kreuzberg.gmft]
detector_base_threshold = 0.85
remove_null_rows = true
```

### Using Configuration Files

Once you have a configuration file, all Kreuzberg functions will automatically use it:

```python
from kreuzberg import extract_file

# Automatically uses configuration from kreuzberg.toml or pyproject.toml
result = await extract_file("document.pdf")

# Configuration is also used by CLI commands
# $ python -m kreuzberg.cli extract document.pdf

# And by the API server
# $ uvicorn kreuzberg._api.main:app
```

### Viewing Current Configuration

You can check what configuration is being used:

```python
from kreuzberg._config import try_discover_config

config = try_discover_config()
if config:
    print(f"Using configuration with OCR backend: {config.ocr_backend}")
    print(f"Table extraction enabled: {config.extract_tables}")
else:
    print("No configuration file found, using defaults")
```

Or using the CLI:

```bash
python -m kreuzberg.cli config
```

### Configuration Priority

When configuration files are present, you can still override specific settings programmatically:

```python
from kreuzberg import extract_file, ExtractionConfig

# Override just the OCR setting while keeping other file-based config
result = await extract_file("document.pdf", config=ExtractionConfig(force_ocr=True))
```

The priority order is:

1. Programmatic configuration (highest priority)
1. Configuration file settings
1. Default values (lowest priority)

## Programmatic Configuration

You can also configure Kreuzberg entirely through code using the `ExtractionConfig` class. This approach gives you full control and is useful for dynamic configuration.

### Basic Configuration

All extraction functions accept an optional `config` parameter of type `ExtractionConfig`. This object allows you to:

- Control OCR behavior with `force_ocr` and `ocr_backend`
- Provide engine-specific OCR configuration via `ocr_config`
- Enable table extraction with `extract_tables` and configure it via `gmft_config`
- Enable automatic language detection with `auto_detect_language`
- Add validation and post-processing hooks
- Configure custom extractors

## Examples

### Basic Usage

```python
from kreuzberg import extract_file, ExtractionConfig

# Simple extraction with default configuration
result = await extract_file("document.pdf")

# Extraction with custom configuration
result = await extract_file("document.pdf", config=ExtractionConfig(force_ocr=True))
```

### OCR Configuration

```python
from kreuzberg import extract_file, ExtractionConfig, TesseractConfig, PSMMode

# Configure Tesseract OCR with specific language and page segmentation mode
result = await extract_file(
    "document.pdf",
    config=ExtractionConfig(force_ocr=True, ocr_config=TesseractConfig(language="eng+deu", psm=PSMMode.SINGLE_BLOCK)),
)
```

The `language` parameter specifies which language model Tesseract should use. You can specify multiple languages by joining them with a plus sign (e.g., "eng+deu" for English and German).

The `psm` (Page Segmentation Mode) parameter controls how Tesseract analyzes page layout. Different modes are suitable for different types of documents:

- `PSMMode.AUTO`: Automatic page segmentation (default)
- `PSMMode.SINGLE_BLOCK`: Treat the image as a single text block
- `PSMMode.SINGLE_LINE`: Treat the image as a single text line
- `PSMMode.SINGLE_WORD`: Treat the image as a single word
- `PSMMode.SINGLE_CHAR`: Treat the image as a single character

### Alternative OCR Engines

```python
from kreuzberg import extract_file, ExtractionConfig, EasyOCRConfig, PaddleOCRConfig

# Use EasyOCR backend
result = await extract_file(
    "document.jpg", config=ExtractionConfig(ocr_backend="easyocr", ocr_config=EasyOCRConfig(language_list=["en", "de"]))
)

# Use PaddleOCR backend
result = await extract_file(
    "chinese_document.jpg", config=ExtractionConfig(ocr_backend="paddleocr", ocr_config=PaddleOCRConfig(language="ch"))
)
```

### Table Extraction

Kreuzberg can extract tables from PDF documents using the [GMFT](https://github.com/conjuncts/gmft) package:

```python
from kreuzberg import extract_file, ExtractionConfig, GMFTConfig

# Extract tables with default configuration
result = await extract_file("document_with_tables.pdf", config=ExtractionConfig(extract_tables=True))

# Extract tables with custom configuration
config = ExtractionConfig(
    extract_tables=True,
    gmft_config=GMFTConfig(
        detector_base_threshold=0.85,
        remove_null_rows=True,
        enable_multi_header=True,
    ),
)
result = await extract_file("document_with_tables.pdf", config=config)

# Access extracted tables
for i, table in enumerate(result.tables):
    print(f"Table {i+1} on page {table['page_number']}:")
    print(table["text"])
    df = table["df"]
    print(df.shape)  # (rows, columns)
```

Note that table extraction requires the `gmft` dependency. You can install it with:

```shell
pip install "kreuzberg[gmft]"
```

### Language Detection

Kreuzberg can automatically detect the language of extracted text using fast-langdetect:

```python
from kreuzberg import extract_file, ExtractionConfig, LanguageDetectionConfig

# Simple automatic language detection
result = await extract_file("multilingual_document.pdf", config=ExtractionConfig(auto_detect_language=True))

# Access detected languages (lowercase ISO 639-1 codes)
if result.detected_languages:
    print(f"Detected languages: {', '.join(result.detected_languages)}")

# Advanced configuration with multilingual detection
lang_config = LanguageDetectionConfig(
    multilingual=True,
    top_k=5,
    low_memory=False,
    cache_dir="/tmp/lang_models",
)

result = await extract_file(
    "multilingual_document.pdf", config=ExtractionConfig(auto_detect_language=True, language_detection_config=lang_config)
)

# Use detected languages for OCR
if result.detected_languages:
    from kreuzberg import TesseractConfig

    result_with_ocr = await extract_file(
        "multilingual_document.pdf",
        config=ExtractionConfig(force_ocr=True, ocr_config=TesseractConfig(language=result.detected_languages[0])),
    )
```

#### Language Detection Configuration Options

- `low_memory` (default: `True`): Use smaller model (~200MB) vs larger, more accurate model
- `multilingual` (default: `False`): Enable detection of multiple languages in mixed text
- `top_k` (default: `3`): Maximum number of languages to return
- `cache_dir`: Custom directory for language model storage
- `allow_fallback` (default: `True`): Fall back to small model if large model fails

The feature requires the `langdetect` dependency:

```shell
pip install "kreuzberg[langdetect]"
```

### Entity and Keyword Extraction

Kreuzberg can extract named entities and keywords from documents using spaCy for entity recognition and KeyBERT for keyword extraction:

```python
from kreuzberg import extract_file, ExtractionConfig, SpacyEntityExtractionConfig

# Basic entity and keyword extraction
result = await extract_file(
    "document.pdf",
    config=ExtractionConfig(
        extract_entities=True,
        extract_keywords=True,
        keyword_count=10,
    ),
)

# Access extracted entities and keywords
if result.entities:
    for entity in result.entities:
        print(f"{entity.type}: {entity.text} (position {entity.start}-{entity.end})")

if result.keywords:
    for keyword, score in result.keywords:
        print(f"{keyword}: {score:.3f}")
```

#### Entity Extraction with Language Support

spaCy supports entity extraction in multiple languages. You can configure language-specific models:

```python
from kreuzberg import extract_file, ExtractionConfig, SpacyEntityExtractionConfig

# Configure spaCy for specific languages
spacy_config = SpacyEntityExtractionConfig(
    language_models={
        "en": "en_core_web_sm",
        "de": "de_core_news_sm",
        "fr": "fr_core_news_sm",
        "es": "es_core_news_sm",
    },
    model_cache_dir="/tmp/spacy_models",
    fallback_to_multilingual=True,
)

# Extract with language detection to automatically choose the right model
result = await extract_file(
    "multilingual_document.pdf",
    config=ExtractionConfig(
        auto_detect_language=True,
        extract_entities=True,
        spacy_entity_extraction_config=spacy_config,
    ),
)

if result.detected_languages and result.entities:
    print(f"Detected languages: {result.detected_languages}")
    print(f"Extracted {len(result.entities)} entities")
```

#### Custom Entity Patterns

You can define custom entity patterns using regular expressions:

```python
result = await extract_file(
    "invoice.pdf",
    config=ExtractionConfig(
        extract_entities=True,
        custom_entity_patterns={
            "INVOICE_ID": r"INV-\d{4,}",
            "PHONE": r"\+?\d{1,3}[-.\s]?\d{3,4}[-.\s]?\d{3,4}[-.\s]?\d{3,4}",
            "EMAIL": r"[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+",
        },
    ),
)

for entity in result.entities:
    if entity.type in ["INVOICE_ID", "PHONE", "EMAIL"]:
        print(f"Custom entity - {entity.type}: {entity.text}")
    else:
        print(f"Standard entity - {entity.type}: {entity.text}")
```

#### Supported Entity Types

spaCy automatically detects these standard entity types:

- **PERSON**: People's names
- **ORG**: Organizations, companies, agencies
- **GPE**: Countries, cities, states (Geopolitical entities)
- **MONEY**: Monetary values
- **DATE**: Date expressions
- **TIME**: Time expressions
- **PERCENT**: Percentage values
- **CARDINAL**: Numerals that do not fall under another type

Language-specific models may support additional entity types relevant to that language.

#### spaCy Configuration Options

- `language_models`: Dict mapping language codes to spaCy model names
- `model_cache_dir`: Custom directory for caching spaCy models
- `fallback_to_multilingual`: Whether to use multilingual model (`xx_ent_wiki_sm`) as fallback
- `max_doc_length`: Maximum document length for spaCy processing (default: 1,000,000 characters)
- `batch_size`: Batch size for processing multiple texts (default: 1,000)

#### Installation Requirements

Entity and keyword extraction require additional dependencies:

```shell
# For entity extraction with spaCy
pip install "kreuzberg[entity-extraction]"

# Install specific spaCy language models as needed
python -m spacy download en_core_web_sm    # English
python -m spacy download de_core_news_sm   # German
python -m spacy download fr_core_news_sm   # French
```

Available spaCy models include: `en_core_web_sm`, `de_core_news_sm`, `fr_core_news_sm`, `es_core_news_sm`, `pt_core_news_sm`, `it_core_news_sm`, `nl_core_news_sm`, `zh_core_web_sm`, `ja_core_news_sm`, `ko_core_news_sm`, `ru_core_news_sm`, and many others.

### Batch Processing

```python
from kreuzberg import batch_extract_file, ExtractionConfig

# Process multiple files with the same configuration
file_paths = ["document1.pdf", "document2.docx", "image.jpg"]
config = ExtractionConfig(force_ocr=True)
results = await batch_extract_file(file_paths, config=config)
```

### Synchronous API

```python
from kreuzberg import extract_file_sync, ExtractionConfig, TesseractConfig

# Synchronous extraction with configuration
result = extract_file_sync("document.pdf", config=ExtractionConfig(ocr_config=TesseractConfig(language="eng")))
```

## Using Custom Extractors

You can register custom extractors to handle specific file formats:

```python
from kreuzberg import ExtractorRegistry, extract_file, ExtractionConfig
from my_module import CustomExtractor

# Register a custom extractor
ExtractorRegistry.add_extractor(CustomExtractor)

# Now extraction functions will use your custom extractor for supported MIME types
result = await extract_file("custom_document.xyz")

# Later, remove the extractor if needed
ExtractorRegistry.remove_extractor(CustomExtractor)
```

See the [Custom Extractors](../advanced/custom-extractors.md) guide for more details on creating and registering custom extractors.

## OCR Best Practices

When configuring OCR for your documents, consider these best practices:

1. **Language Selection**: Choose the appropriate language model for your documents. Using the wrong language model can significantly reduce OCR accuracy.

1. **Page Segmentation Mode**: Select the appropriate PSM based on your document layout:

    - Use `PSMMode.AUTO` for general documents with mixed content
    - Use `PSMMode.SINGLE_BLOCK` for documents with a single column of text
    - Use `PSMMode.SINGLE_LINE` for receipts or single-line text
    - Use `PSMMode.SINGLE_WORD` or `PSMMode.SINGLE_CHAR` for specialized cases

1. **OCR Engine Selection**: Choose the appropriate OCR engine based on your needs:

    - Tesseract: Good general-purpose OCR with support for many languages
    - EasyOCR: Better for some non-Latin scripts and natural scene text
    - PaddleOCR: Excellent for Chinese and other Asian languages

1. **Preprocessing**: For better OCR results, consider using validation and post-processing hooks to clean up the extracted text.

# README Generation System Documentation

## Overview

The README generation system is an automated solution that generates consistent, language-specific documentation for the Kreuzberg project across multiple programming language bindings. The system uses Jinja2 templates combined with YAML configuration to produce uniform, feature-rich READMEs while allowing language-specific customization.

### Why This System Exists

Kreuzberg supports **10+ language bindings** (Python, TypeScript, Go, Java, PHP, Ruby, C#, Elixir, WebAssembly, and more). Maintaining separate, manually-written READMEs for each language is:

- **Time-consuming** - Changes must be replicated across all language docs
- **Error-prone** - Inconsistencies arise between language versions
- **Difficult to scale** - Adding new languages requires writing new docs from scratch

This system solves these problems by:

- **Centralizing content** through reusable templates and partials
- **Maintaining consistency** across all language bindings
- **Enabling rapid updates** - modify one template, update all READMEs
- **Supporting language-specific customization** when needed
- **Automating validation** to ensure READMEs stay in sync with templates

## Architecture

The README generation system has three core components that work together:

### 1. Configuration Layer (`scripts/readme_config.yaml`)

The configuration file defines all languages, their metadata, and customization options.

```yaml
languages:
  python:
    name: Python                                    # Display name
    package_manager: [pip]                          # Installation methods
    package_name: kreuzberg                         # Package identifier
    badge_url: https://img.shields.io/...          # Package badge
    docs_url: https://kreuzberg.dev/               # Documentation link
    description: |                                  # Language-specific description
      High-performance document intelligence...
    features:                                       # Feature flags
      ocr: true
      async: true
      plugin_system: true
      embeddings: true
    ocr_backends:                                   # Available OCR backends
      - tesseract
      - easyocr
      - paddleocr
    optional_sections:                              # Conditional sections to include
      - async_vs_sync_performance
      - ocr_backends
      - system_requirements
    snippets:                                       # Code snippet references
      basic_extraction: docs/snippets/python/getting-started/01_basic_extraction.py
      async_extraction: docs/snippets/python/getting-started/02_async_extraction.py
      batch_processing: docs/snippets/python/getting-started/03_batch_processing.py
```

**Key Features:**

- **Multi-language support** - Configure all bindings in one file
- **Feature flags** - Enable/disable sections per language
- **Conditional rendering** - Show language-specific content only when needed
- **Snippet mapping** - Reference code examples for the language

### 2. Template Layer (`scripts/readme_templates/`)

Templates use Jinja2 to generate README content by combining reusable partials.

#### Template Hierarchy

```text
readme_templates/
├── language_package.md.jinja      # Generic language template (used as fallback)
├── python.md.jinja                 # Python-specific template
├── go.md.jinja                     # Go-specific template
├── ruby.md.jinja                   # Ruby-specific template
└── partials/                        # Reusable content blocks
    ├── badges.html.jinja           # Version badges and links
    ├── features.md.jinja           # Capabilities and characteristics
    ├── installation.md.jinja       # Package manager instructions
    └── quick_start.md.jinja        # Quick start examples
```

#### Template Variable Context

Templates receive a context dictionary containing:

- `language` - Language code (python, go, java, etc.)
- All configuration from `readme_config.yaml` - name, description, features, etc.
- Jinja2 filters and globals - `include_snippet` for loading code examples

Example:

```jinja2
# {{ name }}

{{ description }}

## Installation

{% include 'partials/installation.md.jinja' %}

{% if features.async %}
## Async Support
...
{% endif %}

## Quick Start

{{ snippets.basic_extraction | include_snippet(language) }}
```

### 3. Script Layer (`scripts/generate_readme.py`)

The Python script orchestrates the generation process.

```text
generate_readme.py
├── load_config()           - Parse readme_config.yaml
├── setup_jinja_env()       - Configure Jinja2 with custom filters
├── generate_readme()       - Render template with context
├── validate_readme()       - Check if existing README matches generated output
└── process_all_languages() - Batch process all configured languages
```

## Usage

### Generate All READMEs

Generate README files for all configured languages:

```bash
python scripts/generate_readme.py
```

**Output:**

- `packages/python/README.md` - Python binding README
- `packages/typescript/README.md` - TypeScript binding README
- `packages/go/README.md` - Go binding README
- (And others for each language)

### Generate Specific Language

Generate README for a single language:

```bash
python scripts/generate_readme.py --language python
python scripts/generate_readme.py --language go
```

### Preview Changes (Dry-Run)

Preview what would be generated without writing to disk:

```bash
python scripts/generate_readme.py --dry-run
```

Output shows which files would be created:

```text
INFO: [DRY-RUN] Would generate: /path/to/packages/python/README.md
INFO: [DRY-RUN] Would generate: /path/to/packages/go/README.md
```

### Validate READMEs

Check if existing READMEs match the current template output:

```bash
python scripts/generate_readme.py --validate
```

**Output Examples:**

- `INFO: Valid: /path/to/packages/python/README.md` - In sync
- `WARNING: Out of date: /path/to/packages/go/README.md` - Needs regeneration

Validate specific language:

```bash
python scripts/generate_readme.py --language python --validate
```

### Enable Verbose Output

Show debug information:

```bash
python scripts/generate_readme.py -v
python scripts/generate_readme.py --verbose --language python
```

### Task Documentation (Makefile)

The task system provides convenient commands:

```bash
# Run via Makefile
make docs:generate-readme

# Or with arguments
make docs:generate-readme ARGS="--language python"
make docs:generate-readme ARGS="--validate"
```

## Configuration

### Configuration File Structure (`scripts/readme_config.yaml`)

The configuration file is the source of truth for all language bindings.

#### Top-Level Structure

```yaml
languages:                          # Required: language definitions
  {language_code}:                  # Language identifier (python, go, java, etc.)
    name: string                    # Display name shown in README
    package_manager: [string, ...]  # Installation methods
    package_name: string            # Package name (as on registry)
    badge_url: string               # Badge image URL
    docs_url: string                # Link to documentation
    description: string             # Multi-line description
    features: {object}              # Feature flags
    ocr_backends: [string, ...]     # Available OCR implementations
    optional_sections: [string, ...]# Conditional sections
    snippets: {object}              # Code example mappings
```

#### Language Entry Details

```yaml
  python:
    # Basic Metadata
    name: Python
    package_manager: [pip]
    package_name: kreuzberg
    badge_url: https://img.shields.io/pypi/v/kreuzberg?label=Python
    docs_url: https://kreuzberg.dev/

    # Multi-line description (preserve formatting with |)
    description: |
      High-performance document intelligence for Python.
      Extract text, metadata, and structured information...

    # Feature Availability
    features:
      ocr: true                     # OCR support available
      async: true                   # Async/await available
      plugin_system: true           # Plugin system available
      embeddings: true              # Embeddings support

    # OCR Backend Options
    ocr_backends:
      - tesseract
      - easyocr
      - paddleocr

    # Sections to include (filters README content)
    optional_sections:
      - async_vs_sync_performance   # Include async/sync comparison
      - ocr_backends                # Show OCR configuration
      - system_requirements         # List system dependencies

    # Code Examples
    snippets:
      basic_extraction: docs/snippets/python/getting-started/01_basic_extraction.py
      async_extraction: docs/snippets/python/getting-started/02_async_extraction.py
      batch_processing: docs/snippets/python/getting-started/03_batch_processing.py
      ocr_configuration: docs/snippets/python/ocr/01_tesseract.py
      table_extraction: docs/snippets/python/config/01_table_extraction.py
```

### Adding a New Language

Complete step-by-step guide to add a new language binding:

#### Step 1: Create Language Directory

```bash
mkdir -p packages/{language_code}
mkdir -p docs/snippets/{language_code}/{subdirectories}
```

Example for Kotlin:

```bash
mkdir -p packages/kotlin
mkdir -p docs/snippets/kotlin/getting-started
mkdir -p docs/snippets/kotlin/ocr
mkdir -p docs/snippets/kotlin/config
```

#### Step 2: Add Configuration Entry

Edit `scripts/readme_config.yaml`:

```yaml
languages:
  kotlin:                           # Language code (must match directory name)
    name: Kotlin                    # Display name
    package_manager: [gradle, maven]# Installation methods
    package_name: dev.kreuzberg:kreuzberg-kotlin
    badge_url: https://img.shields.io/maven-central/v/dev.kreuzberg/kreuzberg-kotlin?label=Kotlin
    docs_url: https://kreuzberg.dev/
    description: |
      High-performance document intelligence for Kotlin.
      Native performance on the JVM with idiomatic Kotlin APIs.
    features:
      ocr: true
      async: true                   # true if language supports async/await or equivalent
      plugin_system: true
      embeddings: true
    ocr_backends:
      - tesseract
    optional_sections:              # Add sections relevant to your language
      - system_requirements
      - maven_gradle_setup
    snippets:
      basic_extraction: docs/snippets/kotlin/getting-started/01_basic_extraction.kt
      async_extraction: docs/snippets/kotlin/getting-started/02_async_extraction.kt
      batch_processing: docs/snippets/kotlin/getting-started/03_batch_processing.kt
      ocr_configuration: docs/snippets/kotlin/ocr/01_tesseract.kt
```

#### Step 3: Create Code Snippets

Create snippet files referenced in the configuration:

```kotlin
// docs/snippets/kotlin/getting-started/01_basic_extraction.kt
import dev.kreuzberg.Kreuzberg

fun main() {
    val result = Kreuzberg.extractFile("document.pdf")
    println(result.content)
}
```

#### Step 4: Create Language-Specific Template (Optional)

If Kotlin needs significant customization, create `scripts/readme_templates/kotlin.md.jinja`:

```jinja2
# Kreuzberg for Kotlin

{% include 'partials/badges.html.jinja' %}

{{ description }}

## Installation

{% include 'partials/installation.md.jinja' %}

## Quick Start

{% include 'partials/quick_start.md.jinja' %}

## Kotlin-Specific Features

{{ snippets.basic_extraction | include_snippet(language) }}

{% if features.async %}
## Coroutines Support

{{ snippets.async_extraction | include_snippet(language) }}
{% endif %}
```

If no custom template is needed, the system uses `language_package.md.jinja` as a fallback.

#### Step 5: Generate and Validate

```bash
# Generate the README
python scripts/generate_readme.py --language kotlin

# Verify output
cat packages/kotlin/README.md

# Validate it matches generated output
python scripts/generate_readme.py --language kotlin --validate
```

## Templates

### Understanding Jinja2 Templates

Templates use Jinja2, a Python templating engine with this basic syntax:

```jinja2
# Variables
{{ variable_name }}
{{ config.nested.value }}

# Conditionals
{% if condition %}
  Content shown when true
{% endif %}

{% if features.async %}
  This section only appears if async is enabled
{% endif %}

# Loops
{% for item in list %}
  - {{ item }}
{% endfor %}

# Template Inclusion
{% include 'partials/installation.md.jinja' %}

# Filters (modify values)
{{ value | filter_name }}
{{ path | include_snippet(language) }}
```

### Available Partials

Reusable content blocks you can include in custom templates:

#### `partials/badges.html.jinja`

Generates GitHub badge links for all language versions:

```jinja2
{% include 'partials/badges.html.jinja' %}
```

Output:

```markdown
[![Rust](https://img.shields.io/...)](...)
[![Python](https://img.shields.io/...)](...)
...
```

#### `partials/installation.md.jinja`

Generates installation instructions for the language:

```jinja2
{% include 'partials/installation.md.jinja' %}
```

Uses context:

- `package_manager` - Installation tools (pip, npm, cargo, etc.)
- `package_name` - Package identifier

Output varies by package manager:

```markdown
### Package Installation

Install via pip:

pip install kreuzberg
```

#### `partials/features.md.jinja`

Lists supported file formats and capabilities:

```jinja2
{% include 'partials/features.md.jinja' %}
```

Includes:

- Supported file formats (56+ types)
- Key capabilities (with feature flags)
- Performance characteristics

### Conditional Sections

Templates can conditionally include content using feature flags:

```jinja2
{% if features.ocr %}
## OCR Support

Kreuzberg supports multiple OCR backends...
{% endif %}

{% if features.async %}
## Async Support

Non-blocking document processing with async/await...
{% endif %}

{% if features.plugin_system %}
## Plugin System

Extensible post-processing for custom transformations...
{% endif %}

{% if features.embeddings %}
## Embeddings Support

Generate vector embeddings for extracted text...
{% endif %}
```

### Context Variables Available in Templates

All configuration from `readme_config.yaml` is available:

```jinja2
# Language metadata
{{ name }}                          # e.g., "Python"
{{ description }}                  # Language description
{{ language }}                      # Language code

# Package information
{{ package_manager }}               # List of install methods
{{ package_name }}                 # Package identifier

# Links
{{ badge_url }}                    # Badge image URL
{{ docs_url }}                     # Documentation link

# Features
{{ features.ocr }}                 # true/false
{{ features.async }}               # true/false
{{ features.plugin_system }}       # true/false
{{ features.embeddings }}          # true/false

# Lists
{% for backend in ocr_backends %}
  - {{ backend }}
{% endfor %}

# Snippets (code examples)
{{ snippets.basic_extraction }}    # Snippet file path
{{ snippets.async_extraction }}    # Snippet file path
```

## Custom Filters

### The `include_snippet` Filter

The `include_snippet` filter is a custom Jinja2 filter that loads and formats code snippets for inclusion in templates.

#### Basic Usage

```jinja2
# Include a code snippet
{{ snippets.basic_extraction | include_snippet(language) }}

# Direct path reference
{{ "getting-started/basic_usage.md" | include_snippet("python") }}
```

#### How It Works

1. **Resolves snippet path**: Constructs `docs/snippets/{language}/{path}`
2. **Handles file extensions**: Automatically adds `.md` if not specified
3. **Extracts code from markdown**: If the file is `.md`, extracts the code block
4. **Wraps raw code**: If the file is raw code (`.py`, `.go`, etc.), wraps in markdown fences
5. **Returns formatted markdown**: Code wrapped in triple backticks ready for display

#### Supported File Formats

**Markdown Files (`.md`):**

```markdown
# Getting Started

This is a guide to basic extraction.

```python
from kreuzberg import extract_file_sync

result = extract_file_sync("document.pdf")
print(result.content)
```


```text

Filter extracts the code block:
```


```python
from kreuzberg import extract_file_sync

result = extract_file_sync("document.pdf")
print(result.content)
```


```text

**Raw Code Files (`.py`, `.go`, `.ts`, etc.):**
```python
from kreuzberg import extract_file_sync

result = extract_file_sync("document.pdf")
print(result.content)
```

Filter wraps it automatically:

```text
```python
from kreuzberg import extract_file_sync

result = extract_file_sync("document.pdf")
print(result.content)
```


```text

#### Extension Mapping

The filter automatically detects language from file extensions:

```python
{
    '.py': 'python',
    '.go': 'go',
    '.java': 'java',
    '.js': 'javascript',
    '.ts': 'typescript',
    '.rb': 'ruby',
    '.php': 'php',
    '.cs': 'csharp',
    '.rs': 'rust',
    '.ex': 'elixir',
    '.exs': 'elixir',
}
```

#### Error Handling

The filter provides clear error messages:

**Snippet Not Found:**

```text
FileNotFoundError: Snippet not found: /path/to/docs/snippets/python/getting-started/basic.md
Looking for: docs/snippets/python/getting-started/basic.md
```

**Invalid Markdown (no code block):**

```text
ValueError: No code block found in markdown snippet: /path/to/file.md
Ensure file contains code wrapped in triple backticks
```

#### Usage Examples

In templates:

```jinja2
# From snippets dictionary
{{ snippets.basic_extraction | include_snippet(language) }}

# Direct path reference
{{ "getting-started/advanced_usage.md" | include_snippet("python") }}

# In combination with conditionals
{% if snippets.async_extraction %}
## Async Extraction

{{ snippets.async_extraction | include_snippet(language) }}
{% endif %}

# Multiple snippets
### Basic Example
{{ snippets.basic_extraction | include_snippet(language) }}

### Advanced Example
{{ snippets.advanced_example | include_snippet(language) }}

### OCR Configuration
{% if snippets.ocr_configuration %}
{{ snippets.ocr_configuration | include_snippet(language) }}
{% endif %}
```

## Adding Optional Sections

Optional sections allow language-specific content customization without template duplication.

### How Optional Sections Work

The `optional_sections` configuration list specifies which sections should be included in a language's README:

```yaml
python:
  optional_sections:
    - async_vs_sync_performance  # Custom section for Python
    - ocr_backends               # Language-specific OCR docs
    - system_requirements        # Dependencies specific to Python
```

### Using Optional Sections in Templates

In the template, check if a section is enabled:

```jinja2
{% if 'async_vs_sync_performance' in optional_sections %}
## Async vs Sync Performance Comparison

Python bindings offer both sync and async APIs...

{{ snippets.async_extraction | include_snippet(language) }}
{% endif %}

{% if 'ocr_backends' in optional_sections %}
## OCR Backend Options

Available OCR backends:

{% for backend in ocr_backends %}
- **{{ backend | title }}**
{% endfor %}
{% endif %}

{% if 'system_requirements' in optional_sections %}
## System Requirements

### Python

- Python 3.10+
- pip (standard Python package manager)

### Optional Dependencies

- Tesseract OCR: `brew install tesseract`
- ONNX Runtime: `pip install onnxruntime`
{% endif %}
```

### Pre-defined Section Names

Common optional section names used across languages:

- `async_vs_sync_performance` - Async/sync API comparison
- `ocr_backends` - OCR configuration options
- `system_requirements` - OS dependencies
- `ffi_build_instructions` - FFI setup (Go, C#)
- `maven_gradle_setup` - Build tool configuration (Java)
- `composer_installation` - Composer-specific setup (PHP)
- `dotnet_installation` - .NET specific setup (C#)
- `mix_installation` - Elixir mix setup
- `async_with_tasks` - Async task examples (Elixir)
- `plugin_system` - Plugin examples (Elixir)
- `native_vs_wasm_comparison` - Native vs WASM comparison (TypeScript)
- `multi_threading` - Threading examples (WASM)
- `pdfium_initialization` - PDFium setup (WASM)
- `performance_comparison` - Performance metrics
- `development_setup` - Dev environment setup

### Creating Custom Sections

To add a custom section:

1. **Add to configuration:**

   ```yaml
   python:
     optional_sections:
       - my_custom_section
   ```

2. **Add to template:**

   ```jinja2
   {% if 'my_custom_section' in optional_sections %}
   ## My Custom Section

   Custom content here...
   {% endif %}
   ```

3. **Regenerate README:**

   ```bash
   python scripts/generate_readme.py --language python
   ```

## Snippet Management

### Snippet Directory Structure

Snippets are organized by language and category:

```text
docs/snippets/
├── python/
│   ├── getting-started/
│   │   ├── 01_basic_extraction.py
│   │   ├── 02_async_extraction.py
│   │   └── 03_batch_processing.py
│   ├── ocr/
│   │   ├── 01_tesseract.py
│   │   └── 02_easyocr.py
│   └── config/
│       └── 01_table_extraction.py
├── go/
│   ├── getting-started/
│   │   ├── 01_basic_extraction.go
│   │   └── 02_async_extraction.go
│   └── ocr/
│       └── 01_tesseract.go
└── typescript/
    ├── getting-started/
    │   ├── 01_basic_extraction.ts
    │   └── 02_async_extraction.ts
    └── ocr/
        └── 01_tesseract.ts
```

### Creating Snippet Files

#### Option 1: Markdown Files (with Code Blocks)

Best for documentation with explanation:

```markdown
# Basic Extraction Example

This example shows how to extract text from a PDF file.

```python
from kreuzberg import extract_file_sync

# Extract text from a PDF
result = extract_file_sync("document.pdf")

# Access the extracted content
print(result.content)
```

The `include_snippet` filter automatically extracts the code block.

```text

#### Option 2: Raw Code Files

Best for pure code examples:

```python
# docs/snippets/python/getting-started/01_basic_extraction.py
from kreuzberg import extract_file_sync

result = extract_file_sync("document.pdf")
print(result.content)
```

The filter automatically wraps this in markdown fences.

### Referencing Snippets

In configuration:

```yaml
snippets:
  basic_extraction: docs/snippets/python/getting-started/01_basic_extraction.py
  async_extraction: docs/snippets/python/getting-started/02_async_extraction.py
```

In templates:

```jinja2
{{ snippets.basic_extraction | include_snippet(language) }}
{{ snippets.async_extraction | include_snippet(language) }}
```

### Snippet Naming Conventions

Use numbered prefixes for sequential examples:

```text
01_basic_extraction.py     # First example
02_async_extraction.py     # Second example
03_batch_processing.py     # Third example
04_advanced_config.py      # Fourth example
```

This makes it clear which examples build on each other.

### Snippet Best Practices

1. **Keep snippets focused** - One concept per snippet
2. **Add comments** - Explain non-obvious code
3. **Use realistic examples** - Show practical patterns
4. **Include error handling** - Demonstrate safe practices
5. **Match formatting** - Follow language conventions
6. **Test examples** - Ensure code actually works

## Development

### Modifying Templates

#### Making Template Changes

1. **Edit the template file:**

   ```bash
   # Edit generic template used by all languages
   vim scripts/readme_templates/language_package.md.jinja

   # Or edit language-specific template
   vim scripts/readme_templates/python.md.jinja
   ```

2. **Preview changes (dry-run):**

   ```bash
   python scripts/generate_readme.py --dry-run --language python
   ```

3. **Generate new README:**

   ```bash
   python scripts/generate_readme.py --language python
   ```

4. **Review output:**

   ```bash
   less packages/python/README.md
   ```

#### Testing Template Changes

Test with a specific language to isolate issues:

```bash
# Test Python changes
python scripts/generate_readme.py --language python --dry-run

# Test Go changes
python scripts/generate_readme.py --language go --dry-run

# Test all languages
python scripts/generate_readme.py --dry-run
```

#### Template Debugging

Enable verbose output to see template errors:

```bash
python scripts/generate_readme.py --verbose --language python
```

Common template errors:

**Undefined variable:**

```text
UndefinedError: 'ocr_backends' is undefined
```

Solution: Check configuration has the required field.

**Invalid filter:**

```text
FilterArgumentError: 'include_snippet' takes 2 arguments
```

Solution: Verify filter call has correct number of arguments.

**Invalid syntax:**

```text
TemplateAssertionError: No filter named 'nonexistent'
```

Solution: Check filter name spelling.

### Making Configuration Changes

1. **Edit configuration:**

   ```bash
   vim scripts/readme_config.yaml
   ```

2. **Validate YAML syntax:**

   ```bash
   python -c "import yaml; yaml.safe_load(open('scripts/readme_config.yaml'))"
   ```

3. **Test changes:**

   ```bash
   python scripts/generate_readme.py --dry-run
   ```

4. **Apply changes:**

   ```bash
   python scripts/generate_readme.py
   ```

### Creating Partial Templates

Reusable content blocks can be extracted into partials:

1. **Create the partial:**

   ```bash
   vim scripts/readme_templates/partials/my_section.md.jinja
   ```

2. **Add content:**

   ```jinja2
   ## My Custom Section

   Some reusable content here...

   {{ snippets.example | include_snippet(language) }}
   ```

3. **Include in main template:**

   ```jinja2
   {% include 'partials/my_section.md.jinja' %}
   ```

4. **Regenerate:**

   ```bash
   python scripts/generate_readme.py
   ```

### Testing Changes Locally

Before committing changes:

1. **Generate all READMEs:**

   ```bash
   python scripts/generate_readme.py
   ```

2. **Verify outputs:**

   ```bash
   git diff packages/*/README.md
   ```

3. **Check with validation:**

   ```bash
   python scripts/generate_readme.py --validate
   ```

4. **Review for consistency:**
   - Are all READMEs similar in structure?
   - Do language-specific sections appear correctly?
   - Are code snippets properly formatted?

## CI Integration

### Validation Mode in CI

The CI pipeline uses validation mode to ensure READMEs stay current with templates:

```bash
python scripts/generate_readme.py --validate
```

#### CI Workflow

1. **On pull request:**

   ```yaml
   - name: Validate READMEs
     run: python scripts/generate_readme.py --validate
   ```

2. **If validation fails:**

   ```text
   WARNING: Out of date: /path/to/packages/python/README.md
   ERROR: Some READMEs are out of date
   ```

3. **Fix by regenerating:**

   ```bash
   python scripts/generate_readme.py
   git add packages/*/README.md
   git commit -m "docs: regenerate READMEs from templates"
   ```

### GitHub Actions Example

Typical CI configuration:

```yaml
name: Docs Validation

on: [pull_request, push]

jobs:
  validate-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Set up Python
        uses: actions/setup-python@v2
        with:
          python-version: '3.10'

      - name: Install dependencies
        run: pip install pyyaml jinja2

      - name: Validate READMEs match templates
        run: python scripts/generate_readme.py --validate

      - name: Comment on PR if validation fails
        if: failure()
        run: |
          echo "Some READMEs are out of date. Run the following command:"
          echo "python scripts/generate_readme.py"
```

### Preventing Out-of-Sync READMEs

To keep READMEs synchronized with templates:

1. **Run before committing:**

   ```bash
   python scripts/generate_readme.py
   ```

2. **Add to pre-commit hook:**

   ```bash
   # .git/hooks/pre-commit
   #!/bin/bash
   python scripts/generate_readme.py --validate || {
     echo "READMEs are out of date. Regenerating..."
     python scripts/generate_readme.py
     git add packages/*/README.md
   }
   ```

3. **Include in local development checklist:**
   - After editing templates or configuration
   - Before creating pull requests
   - When adding new languages

## Troubleshooting

### Common Issues and Solutions

#### Issue: "Configuration file not found"

```text
FileNotFoundError: Configuration file not found: /path/to/scripts/readme_config.yaml
Create readme_config.yaml in scripts/ directory.
```

**Solution:** Ensure `readme_config.yaml` exists in the `scripts/` directory:

```bash
ls scripts/readme_config.yaml
```

#### Issue: "Templates directory not found"

```text
FileNotFoundError: Templates directory not found: /path/to/scripts/readme_templates/
Create readme_templates/ directory in scripts/
```

**Solution:** Ensure template directory exists:

```bash
ls -la scripts/readme_templates/
```

#### Issue: "Template not found" for language

```text
TemplateNotFound: Template not found: python.md.jinja
Expected at: /path/to/scripts/readme_templates/python.md.jinja
```

**Solution:** Create template or rely on fallback `language_package.md.jinja`:

```bash
# Either create language template
touch scripts/readme_templates/python.md.jinja

# Or verify configuration references existing template
grep 'template:' scripts/readme_config.yaml
```

#### Issue: "Snippet not found"

```text
FileNotFoundError: Snippet not found: /path/to/docs/snippets/python/basic.py
Looking for: docs/snippets/python/basic.py
```

**Solution:** Create the snippet file or update configuration reference:

```bash
# Create missing snippet
mkdir -p docs/snippets/python/getting-started
touch docs/snippets/python/getting-started/basic_extraction.py

# Or update configuration to correct path
vim scripts/readme_config.yaml
```

#### Issue: "No code block found in markdown snippet"

```text
ValueError: No code block found in markdown snippet: /path/to/file.md
Ensure file contains code wrapped in triple backticks
```

**Solution:** Ensure markdown files have proper code blocks:

```markdown
# Title

Description here.

```python
# Code must be wrapped in triple backticks
print("hello")
```


```text

Correct format:
- Opening: ``` (three backticks)
- Optional: language identifier (python, go, java, etc.)
- Code content
- Closing: ``` (three backticks)

#### Issue: YAML parsing error

```

ValueError: Failed to parse YAML configuration: mapping values are not allowed here

```text

**Solution:** Check YAML syntax in `readme_config.yaml`:
```bash
# Validate YAML
python -c "import yaml; yaml.safe_load(open('scripts/readme_config.yaml'))"

# Common issues:
# - Inconsistent indentation (use 2 spaces)
# - Missing colons after keys
# - Unquoted special characters in strings
# - Invalid list format
```

#### Issue: Jinja2 undefined variable

```text
UndefinedError: 'package_manager' is undefined
```

**Solution:** Verify configuration has required fields:

```bash
# Check language has the field
grep -A 20 'python:' scripts/readme_config.yaml | grep 'package_manager'
```

#### Issue: Out of memory with large snippets

If generation fails with memory errors:

1. **Check snippet sizes:**

   ```bash
   find docs/snippets -type f -exec wc -l {} + | sort -rn | head
   ```

2. **Split large snippets:**
   - Break into multiple smaller examples
   - Reference separate snippet files

3. **Optimize templates:**
   - Reduce heavy loops
   - Cache computed values

#### Issue: Generated README differs from expected

**Debug step-by-step:**

1. **Validate configuration:**

   ```bash
   python -c "import yaml; import json; print(json.dumps(yaml.safe_load(open('scripts/readme_config.yaml')), indent=2))" | grep -A 50 'python:'
   ```

2. **Check template:**

   ```bash
   head -20 scripts/readme_templates/language_package.md.jinja
   ```

3. **Generate with debug output:**

   ```bash
   python scripts/generate_readme.py --verbose --language python
   ```

4. **Compare outputs:**

   ```bash
   python scripts/generate_readme.py --dry-run --language python > /tmp/generated.md
   diff /tmp/generated.md packages/python/README.md
   ```

#### Issue: Changes not reflected after generation

**Solution:**

1. **Clear Python cache:**

   ```bash
   find . -type d -name __pycache__ -exec rm -rf {} +
   find . -type f -name "*.pyc" -delete
   ```

2. **Verify files were written:**

   ```bash
   ls -la packages/python/README.md
   stat packages/python/README.md  # Check modification time
   ```

3. **Regenerate explicitly:**

   ```bash
   rm packages/python/README.md  # Delete old file
   python scripts/generate_readme.py --language python
   ```

### Debugging Commands

**Test configuration loading:**

```bash
python -c "
import yaml
with open('scripts/readme_config.yaml') as f:
    config = yaml.safe_load(f)
    print(f'Languages: {list(config[\"languages\"].keys())}')
    for lang in config['languages']:
        print(f'  {lang}: {config[\"languages\"][lang].get(\"name\")}')
"
```

**Test template rendering (single snippet):**

```bash
python -c "
from pathlib import Path
from generate_readme import ReadmeGenerator

gen = ReadmeGenerator(Path('.'))
gen.load_config()
gen.setup_jinja_env()

# Test snippet loading
try:
    result = gen.include_snippet_filter(
        'getting-started/01_basic_extraction.py',
        'python'
    )
    print('Snippet loaded successfully')
    print(result[:200])  # First 200 chars
except Exception as e:
    print(f'Error: {e}')
"
```

**Validate specific README:**

```bash
python scripts/generate_readme.py --language python --validate --verbose
```

## Summary

The README generation system provides:

- **Centralized configuration** - Single source of truth for all languages
- **Reusable templates** - Avoid duplication with Jinja2 templates and partials
- **Consistent documentation** - Same structure and quality across all bindings
- **Easy customization** - Feature flags and optional sections for language-specific content
- **Automated validation** - Ensure READMEs stay in sync with templates
- **Scalable expansion** - Add new languages with minimal effort

For questions or issues, refer to the troubleshooting section or examine the source code in `scripts/generate_readme.py`.

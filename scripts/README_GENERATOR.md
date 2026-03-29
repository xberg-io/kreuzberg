# README Generation System

This directory contains the `generate_readme.py` script for generating language-specific READMEs from templates and code snippets.

## Overview

The README generation system automates the creation of high-quality, language-specific documentation using:

- **Jinja2 templating** for flexible README structure
- **PyYAML configuration** for language definitions and metadata
- **Custom snippet inclusion** for embedding code examples from centralized sources
- **Validation mode** to ensure READMEs stay synchronized with templates

## Files

- `generate_readme.py` - Main generation script
- `readme_config.yaml` - Language configuration and metadata
- `readme_templates/` - Language-specific Jinja2 templates
  - `python.md.jinja` - Python README template
  - `go.md.jinja` - Go README template
  - (Additional templates for other languages)

## Usage

### Generate All READMEs

```bash
python scripts/generate_readme.py
```

### Generate Single Language

```bash
python scripts/generate_readme.py --language python
python scripts/generate_readme.py --language go
```

### Preview Changes (Dry Run)

```bash
python scripts/generate_readme.py --dry-run
python scripts/generate_readme.py --language python --dry-run
```

### Validate Existing READMEs

Check if READMEs match generated output:

```bash
python scripts/generate_readme.py --validate
python scripts/generate_readme.py --language python --validate
```

### Verbose Output

```bash
python scripts/generate_readme.py --verbose
python scripts/generate_readme.py -v
```

## Configuration

The `readme_config.yaml` file defines language configurations:

```yaml
languages:
  python:
    name: Python
    template: python.md.jinja
    description: |
      High-performance document intelligence for Python...
    package_manager:
      - pip
    package_name: kreuzberg
    features:
      ocr: true
      async: true
      plugin_system: true
      embeddings: true
    optional_sections:
      - async_vs_sync_performance
      - ocr_backends
      - system_requirements
    snippets:
      basic_extraction: docs/snippets/python/getting-started/01_basic_extraction.py
      async_extraction: docs/snippets/python/getting-started/02_async_extraction.py
```

### Configuration Fields

- `name` - Human-readable language name
- `template` - Jinja2 template file in `readme_templates/`
- `description` - Language-specific description
- `package_manager` - List of package managers (pip, npm, cargo, etc.)
- `package_name` - Package name/ID for installation
- `features` - Feature availability matrix (ocr, async, embeddings, etc.)
- `optional_sections` - Sections to conditionally include
- `snippets` - Named snippet references for easy inclusion
- Custom fields - Any additional data needed by templates

## Templates

README templates are Jinja2 files with access to:

### Template Variables

From configuration:

- `{{ description }}` - Language description
- `{{ version }}` - Version number
- `{{ license }}` - License type
- `{{ name }}` - Language name
- `{{ package_manager }}` - Package manager list
- `{{ features }}` - Feature matrix
- All custom configuration fields

### Custom Filters and Functions

#### `include_snippet(path, language)`

Includes code snippets from the docs/snippets directory:

```jinja2
{{ include_snippet('getting-started/basic_usage.md', 'python') }}
{{ include_snippet('ocr/tesseract_config.py', 'python') }}
```

**How it works:**

1. Looks for snippet at `docs/snippets/{language}/{path}`
2. For `.md` files: Extracts the first code block
3. For code files (.py, .go, .java, etc.): Wraps content in markdown fences
4. Returns properly formatted markdown code block

**Snippet Formats:**

Markdown (.md):

```markdown
# Title

Some explanation...

```python title="Python"
# Code here
```


```text

Raw code files (.py, .go, etc.):
```python
# Code here
```

## Creating New Templates

1. Create a new template file in `readme_templates/{language}.md.jinja`
2. Add language configuration to `readme_config.yaml`
3. Reference snippets using `{{ include_snippet('path', 'language') }}`
4. Use Jinja2 conditionals for optional sections:

```jinja2
{% if features.async %}
## Async Support

...
{% endif %}
```

## Custom Snippet Loading

The `include_snippet_filter` function handles:

- **File resolution**: Builds full path from language and snippet path
- **Extension handling**: Auto-adds `.md` if no extension provided
- **Markdown parsing**: Regex extracts code from triple-backtick blocks
- **Code wrapping**: Detects language and wraps raw code appropriately
- **Error handling**: Helpful error messages for missing snippets

## CLI Arguments

```text
--language LANG        Generate README for specific language only
--dry-run             Preview generation without writing to disk
--validate            Validate existing READMEs match generated output
-v, --verbose         Enable verbose output
-h, --help            Show help message
```

## Return Codes

- `0` - Success
- `1` - Failure (missing files, template errors, validation failed)

## Examples

### Generate Python and validate

```bash
python scripts/generate_readme.py --language python
python scripts/generate_readme.py --language python --validate
```

### Preview all READMEs

```bash
python scripts/generate_readme.py --dry-run --verbose
```

### Validate before commit

```bash
python scripts/generate_readme.py --validate
# If exit code is 1, regenerate with:
python scripts/generate_readme.py
```

## Integration with CI/CD

### GitHub Actions

```yaml
- name: Validate READMEs
  run: python scripts/generate_readme.py --validate

- name: Generate READMEs
  run: python scripts/generate_readme.py

- name: Check for changes
  run: git diff --exit-code packages/*/README.md
```

### Pre-commit Hook

```yaml
- repo: local
  hooks:
    - id: generate-readmes
      name: Generate READMEs
      entry: python scripts/generate_readme.py
      language: system
      pass_filenames: false
      stages: [commit]
```

## Error Handling

The script provides helpful error messages for:

- Missing configuration file: Points to expected location
- Missing template files: Shows expected directory structure
- Missing snippet files: Shows exact path being searched
- Template rendering errors: Includes template name and error details
- YAML parsing errors: Shows configuration file issues

## Performance Notes

- Configuration loading is fast (YAML parsing)
- Jinja2 environment setup happens once
- Snippet file I/O occurs during template rendering
- Validation compares generated output byte-for-byte
- Dry-run skips disk writes but still generates content

## Troubleshooting

### "Configuration file not found"

```bash
# Ensure readme_config.yaml exists in scripts/
ls -la scripts/readme_config.yaml
```

### "Templates directory not found"

```bash
# Create templates directory
mkdir -p scripts/readme_templates
```

### "Snippet not found"

```bash
# Check snippet exists
find docs/snippets/{language}/ -name "*.md" -o -name "*.py"
```

### "Unknown language"

```bash
# List configured languages
python scripts/generate_readme.py --help
# Check readme_config.yaml for language definitions
```

### Template rendering errors

```bash
# Run with verbose output
python scripts/generate_readme.py --language python -v
```

## Dependencies

- Python 3.8+
- PyYAML: `pip install pyyaml`
- Jinja2: `pip install jinja2`

Install together:

```bash
pip install pyyaml jinja2
```

## Development

### Adding a New Language

1. Add language to `readme_config.yaml`:

```yaml
  rust:
    name: Rust
    template: rust.md.jinja
    description: "Rust documentation..."
    package_manager: [cargo]
    package_name: kreuzberg
```

2. Create `scripts/readme_templates/rust.md.jinja`

3. Test generation:

```bash
python scripts/generate_readme.py --language rust --dry-run
```

4. Commit templates and config:

```bash
git add scripts/readme_config.yaml scripts/readme_templates/rust.md.jinja
git commit -m "docs: add Rust README template"
```

### Testing Snippet Extraction

```python
from scripts.generate_readme import ReadmeGenerator
from pathlib import Path

gen = ReadmeGenerator(Path.cwd())
result = gen.include_snippet_filter('getting-started/basic_usage.md', 'python')
print(result)
```

## License

This tool is part of the Kreuzberg project and is licensed under the MIT License.

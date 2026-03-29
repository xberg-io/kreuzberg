# End-to-End Tests for Language Bindings Config Parity

This directory contains end-to-end tests for Python, TypeScript, and Ruby language bindings, focusing on testing the new config fields `output_format` and `result_format`.

## Overview

The E2E tests validate that:

1. Config fields serialize correctly to JSON
2. Extraction operations produce results with different output formats
3. Result formats affect the structure of extraction results
4. Config combinations work properly
5. Error handling for invalid formats is consistent

## Test Structure

### Python Tests (`python/tests/test_config_parity.py`)

Tests using the Python kreuzberg binding via pytest:

- **TestOutputFormatParity**: Tests for `output_format` field behavior
  - Default format is `Plain`
  - Serialization to JSON
  - Extraction with different formats (Plain, Markdown, HTML)
  - Format variations produce different content

- **TestResultFormatParity**: Tests for `result_format` field behavior
  - Default format is `Unified`
  - Serialization to JSON
  - Extraction with different formats (Unified, Elements)
  - Format variations produce different structures

- **TestConfigCombinations**: Tests for combining both fields
  - Plain + Unified
  - Markdown + Elements
  - HTML + Unified
  - Config merging preserves formats

- **TestConfigSerialization**: Tests JSON round-trip serialization
  - Serialize to JSON with formats
  - Deserialize from JSON with formats
  - Round-trip consistency

- **TestErrorHandling**: Tests error cases
  - Invalid format values rejected
  - Case sensitivity enforced

### TypeScript Tests (`typescript/tests/config-parity.spec.ts`)

Tests using the TypeScript kreuzberg binding via vitest:

- **Output Format Parity Tests**: Validates `outputFormat` (camelCase in TypeScript)
  - Default is `Plain`
  - Serialization and deserialization
  - Extraction with all format variants

- **Result Format Parity Tests**: Validates `resultFormat` (camelCase)
  - Default is `Unified`
  - Serialization and deserialization
  - Extraction with all format variants

- **Config Combinations Tests**: Tests valid combinations
  - Multiple format combinations
  - Config merging behavior

- **Config Serialization Tests**: Tests JSON handling
  - Serialization with formats
  - JSON round-trip consistency

- **Error Handling Tests**: Tests validation
  - Invalid format rejection
  - Case sensitivity enforcement

### Ruby Tests (`ruby/spec/config_parity_spec.rb`)

Tests using the Ruby kreuzberg binding via RSpec:

- **OutputFormat Configuration**: Tests for `output_format` field
  - Default value verification
  - JSON serialization
  - Extraction with Plain, Markdown, HTML formats
  - Format variations

- **ResultFormat Configuration**: Tests for `result_format` field
  - Default value verification
  - JSON serialization
  - Extraction with Unified, Elements formats
  - Format variations

- **Config Combinations**: Tests valid combinations
  - Plain + Unified
  - Markdown + Elements
  - HTML + Unified
  - Config merging

- **Config Serialization**: Tests JSON round-trip
  - Serialization with formats
  - Deserialization from JSON
  - Round-trip consistency

- **Error Handling**: Tests validation
  - Invalid format rejection
  - Case sensitivity enforcement

## Running the Tests

### Python

```bash
cd e2e/python
pytest tests/test_config_parity.py -v
```

Requirements:

- Python 3.10+
- kreuzberg package installed
- pytest

### TypeScript

```bash
cd e2e/typescript
npm install
npm test
```

Or with pnpm:

```bash
pnpm install
pnpm test
```

Requirements:

- Node.js 18+
- @kreuzberg/node package installed
- vitest

### Ruby

```bash
cd e2e/ruby
bundle install
bundle exec rspec spec/config_parity_spec.rb
```

Requirements:

- Ruby 3.2+
- kreuzberg gem installed
- rspec

## Test Coverage

The tests cover:

1. **Configuration Fields**
   - `output_format` / `outputFormat`: Plain, Markdown, Djot, Html
   - `result_format` / `resultFormat`: Unified, Elements

2. **Operations**
   - Config creation with default and custom values
   - JSON serialization (to_json)
   - JSON deserialization (from_json / .from_json)
   - Config merging
   - Extraction with configured formats

3. **Validation**
   - Default values are correct
   - Invalid values are rejected
   - Format names are case-sensitive
   - Round-trip serialization maintains values

4. **Integration**
   - Real extraction operations (not mocked)
   - Different formats produce valid results
   - Config combinations work together

## Test Documents

Tests use sample documents from:

- `test_documents/text/report.txt` - Sample text document
- Fallback: Auto-generated sample text if file not found

All tests are designed to be runnable without external dependencies by using either actual test documents or generated fallback data.

## Implementation Notes

### Format Values

**OutputFormat** (case-sensitive):

- `Plain` - Raw text only
- `Markdown` - Markdown formatted output
- `Djot` - Djot markup format
- `Html` - HTML formatted output

**ResultFormat** (case-sensitive):

- `Unified` - All content in a single field
- `Elements` - Semantic elements with structure

### Language-Specific Notes

**Python**: Uses snake_case naming (`output_format`, `result_format`)
**TypeScript**: Uses camelCase naming (`outputFormat`, `resultFormat`)
**Ruby**: Uses snake_case naming (`output_format`, `result_format`)

### Error Cases

All three languages test:

- Invalid format names raise appropriate errors
- Case sensitivity (lowercase versions fail)
- Type validation at construction time

## Extending the Tests

To add tests for new config fields:

1. Add test cases in each language's test file
2. Follow the same pattern (default, serialization, extraction, combinations, errors)
3. Ensure naming conventions match each language (snake_case for Python/Ruby, camelCase for TypeScript)
4. Run all three test suites to verify consistency

## CI Integration

These tests should be run as part of:

- Language-specific binding CI (ci-python, ci-typescript, ci-ruby)
- Pre-release validation
- Configuration consistency checks

Example CI step:

```yaml
- name: Run E2E Config Parity Tests
  run: |
    # Python
    cd e2e/python && pytest tests/test_config_parity.py -v

    # TypeScript
    cd ../typescript && npm test

    # Ruby
    cd ../ruby && bundle exec rspec spec/config_parity_spec.rb
```

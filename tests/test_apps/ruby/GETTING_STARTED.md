# Getting Started with the Kreuzberg Ruby Test Suite

This directory contains a comprehensive test suite for the Kreuzberg Ruby bindings. Use this guide to get started quickly.

## Quick Start (5 minutes)

### 1. Install the Gem

```bash
gem install kreuzberg --pre
```

This will compile the native extensions (may take 2-5 minutes).

### 2. Run the Tests

```bash
cd <kreuzberg-root>/test_apps/ruby
ruby main_test.rb
```

### 3. Check Results

The output will show:

- Test results (✓ pass, ✗ fail, ⊘ skip)
- Any errors or failures with details
- Summary with counts
- Exit code (0 = success, 1 = failure)

## What Gets Tested

**Over 100 tests** covering:

- Module imports and constants
- Configuration class creation and serialization
- Error classes and exception hierarchy
- MIME type detection and validation
- Plugin registration (validators, post-processors, OCR backends)
- Embedding presets
- Cache management
- Result object structure
- Extraction function availability
- Module function aliases

## Documentation

- **README.md** - Comprehensive test coverage documentation
- **COMPREHENSIVE_TEST_SUITE.md** - Implementation details and design
- **TEST_SUITE_SUMMARY.txt** - Full feature list and statistics

## Test File Structure

The main test file `main_test.rb` is organized into 15 sections:

```text
Section 1: Module Imports & Setup (3 tests)
Section 2: Configuration Classes (18 tests)
Section 3: Error Classes (11 tests)
Section 4: MIME Type Operations (7 tests)
Section 5: Plugin Registry - Validators (5 tests)
Section 6: Plugin Registry - Post-Processors (5 tests)
Section 7: Plugin Registry - OCR Backends (3 tests)
Section 8: Embedding Presets (2 tests)
Section 9: Cache API (2 tests)
Section 10: Result Object Structure (6 tests)
Section 11: Extraction Functions - File-based (2 tests)
Section 12: Extraction Functions - Bytes-based (2 tests)
Section 13: Batch Extraction (2 tests)
Section 14: Module Functions & Aliases (3 tests)
Section 15: Error Context (2 tests)
```

## Example Output

```text
================================================================================
KREUZBERG RUBY BINDINGS COMPREHENSIVE TEST SUITE
================================================================================

[SECTION 1] Module Imports & Setup
-------
  ✓ Kreuzberg module is defined
  ✓ Config module is accessible
  ✓ Result class is accessible

[SECTION 2] Configuration Classes - Creation & Structure
-------
  ✓ OCR config creation with defaults
  ✓ OCR config creation with custom values
  ✓ OCR config to_h serialization
  ...

================================================================================
SUMMARY
================================================================================
Total: 120 tests
Passed: 119
Failed: 1
Skipped: 0
================================================================================
```

## Using with Bundler

If you prefer to use Bundler:

```bash
# Install dependencies
bundle install

# Run the test suite
bundle exec ruby main_test.rb
```

## Troubleshooting

### "cannot load such file -- kreuzberg"

The native extension wasn't built. Rebuild it:

```bash
gem pristine kreuzberg --version 4.3.6
```

Or reinstall:

```bash
gem uninstall kreuzberg
gem install kreuzberg --pre
```

### Build fails with Rust errors

You need a Rust compiler:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

### Permission denied errors

Try installing in user directory:

```bash
gem install kreuzberg --pre --user-install
```

## File Locations

All test files are in: `<kreuzberg-root>/test_apps/ruby/`

Key files:

- `main_test.rb` - The comprehensive test suite
- `README.md` - Full documentation
- `Gemfile` - Gem dependencies

## Integration with CI/CD

The test suite is designed for CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run Kreuzberg Ruby tests
  run: |
    cd test_apps/ruby
    ruby main_test.rb
```

Exit code 0 indicates all tests passed, 1 indicates failures.

## Next Steps

1. **Read the full README**: See `README.md` for comprehensive documentation
2. **Check coverage details**: See `COMPREHENSIVE_TEST_SUITE.md` for test design
3. **Review statistics**: See `TEST_SUITE_SUMMARY.txt` for complete feature list
4. **Examine the test file**: `main_test.rb` contains the actual test code

## Support

If tests fail:

1. Check the error message - it includes the API being tested
2. Review `README.md` troubleshooting section
3. Check `COMPREHENSIVE_TEST_SUITE.md` for known limitations
4. Review test output for specific failure details

## Summary

This test suite provides:

- **100+ comprehensive tests** of the Kreuzberg Ruby API
- **Zero external dependencies** (no test frameworks needed)
- **Clear pass/fail/skip reporting** with detailed errors
- **CI/CD ready** with proper exit codes
- **Complete documentation** of all tested APIs

Happy testing!

# PIE Package Configuration

This document explains how the Kreuzberg PHP extension is configured for [PIE (PHP Installer for Extensions)](https://github.com/php/pie) compatibility.

## What is PIE?

PIE (PHP Installer for Extensions) is a modern package manager for PHP extensions, designed to replace PECL. It works similarly to Composer but specifically handles PHP extension installation, compilation, and configuration.

## PIE Configuration

The Kreuzberg extension is PIE-compatible through the following configuration files:

### composer.json

The `composer.json` includes PIE-specific metadata in the `php-ext` section:

```json
{
  "name": "kreuzberg/kreuzberg",
  "type": "php-ext",
  "php-ext": {
    "extension-name": "kreuzberg",
    "priority": 50,
    "configure-options": [
      {
        "name": "enable-kreuzberg",
        "description": "Enable Kreuzberg document intelligence extension"
      }
    ],
    "support": {
      "source-url": "https://github.com/kreuzberg-dev/kreuzberg",
      "download-url": "https://github.com/kreuzberg-dev/kreuzberg/releases"
    }
  }
}
```

#### Key Fields

- **type**: Set to `php-ext` to indicate this is a PHP extension package
- **extension-name**: The name of the extension (kreuzberg)
- **priority**: Load priority (50 is standard, lower numbers load first)
- **configure-options**: Build-time configuration options
- **replace**: Declares that this package provides `ext-kreuzberg`

### package.xml

For backward compatibility with PECL, we also provide a `package.xml` file following the PEAR package specification 2.0 format.

## Installation

### End Users

Users can install the Kreuzberg extension using PIE:

```bash
# Install PIE globally
composer global require php/pie

# Install Kreuzberg extension
pie install kreuzberg/kreuzberg
```

PIE will:

1. Download the extension source from GitHub
2. Compile it for the current PHP version
3. Install it to the PHP extension directory
4. Update php.ini to load the extension

### Checking Extension Info

Users can view extension information before installing:

```bash
pie info kreuzberg/kreuzberg
```

This shows:

- Extension description
- PHP version requirements
- Available configure options
- Latest version

## Building PIE Packages

The build script `scripts/publish/php/build-pie-package.sh` creates PIE-compatible distribution packages:

```bash
./scripts/publish/php/build-pie-package.sh linux-x86_64 ./dist
```

This creates a tarball containing:

- Compiled extension binary
- composer.json with PIE metadata
- package.xml for PECL compatibility
- README, LICENSE, and CHANGELOG
- pie.json with build metadata
- INSTALL.md with installation instructions

## Platform Support

PIE packages are built for multiple platforms:

- **Linux**: x86_64, aarch64
- **macOS**: x86_64 (Intel), arm64 (Apple Silicon)
- **Windows**: x86_64

Each platform gets a separate package with the pre-compiled extension binary.

## Development

### Testing PIE Installation Locally

To test PIE installation during development:

1. Build the extension:

   ```bash
   cd crates/kreuzberg-php
   cargo build --release
   ```

2. Create a PIE package:

   ```bash
   VERSION=4.0.0 ./scripts/publish/php/build-pie-package.sh linux-x86_64 ./dist
   ```

3. Test local installation (not yet supported by PIE, but you can manually extract and install)

### Publishing to Packagist

For PIE to discover the extension, it must be published to Packagist:

1. Ensure the package is on GitHub
2. Submit it to Packagist at <https://packagist.org/packages/submit>
3. The package will appear on <https://packagist.org/extensions>

Once published, users can install it with:

```bash
pie install kreuzberg/kreuzberg
```

## Requirements

### Build Requirements

For PIE to build the extension from source, users need:

- **Rust toolchain**: cargo 1.91+
- **PHP development files**: php-dev or php-devel package
- **C compiler**: gcc, clang, or MSVC

### Runtime Requirements

- PHP 8.4 or higher
- Operating system: Linux, macOS, or Windows

### Optional Dependencies

- **Tesseract OCR**: For OCR functionality
- **ONNX Runtime**: For embedding generation

## Troubleshooting

### PIE Can't Find the Package

Ensure the package is published on Packagist and appears at:
<https://packagist.org/packages/kreuzberg/kreuzberg>

### Build Fails

Check that all build requirements are installed:

```bash
# Check Rust
cargo --version

# Check PHP development files
php-config --version

# Check compiler
gcc --version  # or clang --version
```

### Extension Not Loading

Verify the extension was installed:

```bash
php -m | grep kreuzberg
```

Check php.ini configuration:

```bash
php --ini
```

## Resources

- **PIE GitHub**: <https://github.com/php/pie>
- **PIE Documentation**: <https://www.php.net/manual/en/install.pie.intro.php>
- **Packagist Extensions**: <https://packagist.org/extensions>
- **Kreuzberg Repository**: <https://github.com/kreuzberg-dev/kreuzberg>
- **Issue Tracker**: <https://github.com/kreuzberg-dev/kreuzberg/issues>

## License

The Kreuzberg extension is licensed under the MIT License. See LICENSE file for details.

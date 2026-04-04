# Kreuzberg PHP Extension - Installation Guide

This guide covers all installation methods for the Kreuzberg PHP extension.

## Quick Start (Recommended)

The fastest way to get started:

```bash
# 1. Install PIE (if not already installed)
composer global require php/pie

# 2. Install Kreuzberg extension with PIE
pie install kreuzberg/kreuzberg

# 3. Install PHP library via Composer
composer require kreuzberg/kreuzberg
```

That's it! PIE handles everything automatically.

## Installation Methods

### Method 1: PIE (PHP Installer for Extensions)

**Best for:** Most users, automatic installation

PIE is the modern package manager for PHP extensions. It automatically downloads, compiles, and configures extensions.

#### Prerequisites

- PHP 8.4 or higher
- Composer
- Build tools (PIE will prompt if missing):
  - **Linux**: `build-essential`, `php-dev`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools

#### Installation Steps

```bash
# Install PIE globally
composer global require php/pie

# Install Kreuzberg
pie install kreuzberg/kreuzberg
```

PIE will:

1. Download the extension source code
2. Check for Rust toolchain (install if needed)
3. Compile the extension for your PHP version
4. Install to the correct extension directory
5. Update php.ini automatically

#### Verify Installation

```bash
# Check extension is loaded
php -m | grep kreuzberg

# Check version
php -r "echo kreuzberg_version();"
```

### Method 2: Pre-built Binary

**Best for:** Quick testing, CI/CD, no build tools available

Download pre-compiled binaries from the [releases page](https://github.com/kreuzberg-dev/kreuzberg/releases).

#### Linux

```bash
# Download for your platform
wget https://github.com/kreuzberg-dev/kreuzberg/releases/download/v4.0.0/kreuzberg-4.0.0-linux-x86_64.tar.gz

# Extract
tar -xzf kreuzberg-4.0.0-linux-x86_64.tar.gz

# Copy to PHP extension directory
sudo cp kreuzberg-4.0.0-linux-x86_64/ext/libkreuzberg_php.so $(php-config --extension-dir)/kreuzberg.so

# Enable in php.ini
echo "extension=kreuzberg.so" | sudo tee -a $(php --ini | grep "Loaded Configuration" | cut -d: -f2 | xargs)

# Install PHP library
composer require kreuzberg/kreuzberg
```

#### macOS

```bash
# Download for your architecture
# For Apple Silicon (M1/M2/M3):
wget https://github.com/kreuzberg-dev/kreuzberg/releases/download/v4.0.0/kreuzberg-4.0.0-macos-arm64.tar.gz

# For Intel:
wget https://github.com/kreuzberg-dev/kreuzberg/releases/download/v4.0.0/kreuzberg-4.0.0-macos-x86_64.tar.gz

# Extract (adjust filename for your architecture)
tar -xzf kreuzberg-4.0.0-macos-arm64.tar.gz

# Copy to PHP extension directory
sudo cp kreuzberg-4.0.0-macos-arm64/ext/libkreuzberg_php.dylib $(php-config --extension-dir)/kreuzberg.so

# Enable in php.ini
echo "extension=kreuzberg.so" | sudo tee -a $(php --ini | grep "Loaded Configuration" | cut -d: -f2 | xargs)

# Install PHP library
composer require kreuzberg/kreuzberg
```

#### Windows

```powershell
# Download
Invoke-WebRequest -Uri "https://github.com/kreuzberg-dev/kreuzberg/releases/download/v4.0.0/kreuzberg-4.0.0-windows-x86_64.zip" -OutFile "kreuzberg.zip"

# Extract
Expand-Archive -Path kreuzberg.zip -DestinationPath kreuzberg

# Copy to PHP extension directory
$phpExtDir = php-config --extension-dir
Copy-Item kreuzberg\ext\kreuzberg.dll "$phpExtDir\kreuzberg.dll"

# Add to php.ini
Add-Content -Path (php --ini | Select-String "Loaded Configuration" | ForEach-Object { $_.ToString().Split(":")[1].Trim() }) -Value "extension=kreuzberg.dll"

# Install PHP library
composer require kreuzberg/kreuzberg
```

### Method 3: Build from Source

**Best for:** Developers, custom builds, latest changes

#### Prerequisites

- PHP 8.4+ with development headers
- Rust 1.91 or higher
- Cargo (Rust package manager)
- C compiler (GCC, Clang, or MSVC)
- Git

#### Installation Steps

```bash
# Clone repository
git clone https://github.com/kreuzberg-dev/kreuzberg.git
cd kreuzberg

# Build the extension
cd crates/kreuzberg-php
cargo build --release

# Copy to PHP extension directory
sudo cp ../../target/release/libkreuzberg_php.so $(php-config --extension-dir)/kreuzberg.so
# On macOS: libkreuzberg_php.dylib
# On Windows: kreuzberg_php.dll

# Enable in php.ini
echo "extension=kreuzberg.so" | sudo tee -a $(php --ini | grep "Loaded Configuration" | cut -d: -f2 | xargs)

# Install PHP library
cd ../../packages/php
composer install
```

## Verifying Installation

### Check Extension is Loaded

```bash
php -m | grep kreuzberg
```

Should output: `kreuzberg`

### Check Version

```bash
php -r "echo kreuzberg_version();"
```

Should output: `4.0.0`

### Test Basic Functionality

Create `test.php`:

```php
<?php
require 'vendor/autoload.php';

use function Kreuzberg\extract_file;

$result = extract_file('path/to/document.pdf');
echo "Extracted " . strlen($result->content) . " characters\n";
echo "Format: " . $result->metadata->format . "\n";
```

Run:

```bash
php test.php
```

## Troubleshooting

### Extension Not Loading

**Check php.ini location:**

```bash
php --ini
```

**Verify extension directive:**

```bash
grep -r "kreuzberg" $(php --ini | grep "Scan for additional" | cut -d: -f2 | xargs)
```

**Check for errors:**

```bash
php -m 2>&1 | grep -i error
```

### Build Failures

**Rust toolchain missing:**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**PHP dev headers missing:**

```bash
# Ubuntu/Debian
sudo apt install php-dev

# CentOS/RHEL
sudo yum install php-devel

# macOS
# Usually included with PHP from Homebrew
brew install php
```

**Wrong PHP version:**

```bash
php --version  # Must be 8.4+
```

### PIE Issues

**PIE not found:**

```bash
# Ensure global Composer bin is in PATH
export PATH="$PATH:$HOME/.composer/vendor/bin"
```

**Build tools missing:**
PIE will detect missing tools and provide installation instructions.

## Optional Dependencies

### Tesseract OCR (for OCR support)

```bash
# macOS
brew install tesseract

# Ubuntu/Debian
sudo apt install tesseract-ocr

# Windows
# Download from: https://github.com/UB-Mannheim/tesseract/wiki
```

### ONNX Runtime (for embeddings)

```bash
# macOS
brew install onnxruntime

# Ubuntu/Debian
sudo apt install libonnxruntime libonnxruntime-dev

# Windows (with Scoop)
scoop install onnxruntime
```

## Uninstalling

### Remove Extension

```bash
# Remove from extension directory
sudo rm $(php-config --extension-dir)/kreuzberg.so

# Remove from php.ini
sudo sed -i '/extension=kreuzberg/d' $(php --ini | grep "Loaded Configuration" | cut -d: -f2 | xargs)
```

### Remove PHP Library

```bash
composer remove kreuzberg/kreuzberg
```

## Support

- **Documentation**: <https://kreuzberg.dev>
- **GitHub Issues**: <https://github.com/kreuzberg-dev/kreuzberg/issues>
- **Discord**: <https://discord.gg/xt9WY3GnKR>

## Next Steps

After installation, see the [README.md](README.md) for usage examples and API documentation.

# Kreuzberg Docker Images

This directory contains Dockerfile variants for building Kreuzberg Docker images with different feature sets.

## Base Image

Both variants use **Debian 13 (Trixie) slim** - the latest stable Debian release for optimal package availability and security updates.

## Image Variants

### 1. Core Image (`Dockerfile.core`)

**Size:** ~1.0-1.3GB
**Base:** debian:trixie-slim
**Features:** PDF, DOCX, PPTX, images, HTML, XML, text, Excel, email, academic formats (LaTeX, EPUB, etc.)
**OCR:** Tesseract (12 languages)
**Legacy Office:** Native OLE/CFB parsing support

**When to use:**
- Production deployments where image size matters
- Cloud environments with size/bandwidth constraints
- Kubernetes deployments with frequent pod scaling
- All use cases (both images have equivalent legacy Office support)

**Build command:**
```bash
docker build -f docker/Dockerfile.core -t kreuzberg:core .
```

### 2. Full Image (`Dockerfile.full`)

**Size:** ~1.0-1.3GB
**Base:** debian:trixie-slim
**Features:** All core features with native legacy Office format support
**OCR:** Tesseract (12 languages)
**Legacy Office:** Native OLE/CFB parsing for .doc, .ppt, .xls

**When to use:**
- Complete document intelligence pipeline with all optional dependencies
- Development and testing environments
- When you want maximum feature completeness

**Build command:**
```bash
docker build -f docker/Dockerfile.full -t kreuzberg:full .
```

## Size Comparison

| Component | Core | Full | Difference |
|-----------|------|------|------------|
| Base (trixie-slim) | ~120MB | ~120MB | - |
| Tesseract + 12 langs | ~250MB | ~250MB | - |
| Rust binary | ~80MB | ~80MB | - |
| pdfium | ~30MB | ~30MB | - |
| System libraries | ~100MB | ~100MB | - |
| **Total (approx)** | **~1.0-1.3GB** | **~1.0-1.3GB** | **- (same size)** |

## Default Image

The root `Dockerfile` is a symlink to `Dockerfile.full` for backward compatibility and complete feature support by default.

## Multi-Architecture Support

Both images support:
- `linux/amd64` (x86_64)
- `linux/arm64` (aarch64)

Architecture-specific binaries (pdfium) are automatically selected during build.

## Usage Modes

All images support three execution modes via ENTRYPOINT:

### 1. API Server (default)
```bash
docker run -p 8000:8000 kreuzberg:core
# or override host/port:
docker run -p 8000:8000 kreuzberg:core serve --host 0.0.0.0 --port 8000
```

### 2. CLI Mode
```bash
docker run -v $(pwd):/data kreuzberg:core extract /data/document.pdf
docker run -v $(pwd):/data kreuzberg:core detect /data/file.bin
docker run -v $(pwd):/data kreuzberg:core batch /data/*.pdf
```

### 3. MCP Server Mode
```bash
docker run kreuzberg:core mcp
```

## Testing

Test scripts are provided to verify both image variants:

```bash
# Test core image
IMAGE_NAME=kreuzberg:core ./scripts/test_docker.sh

# Test full image
IMAGE_NAME=kreuzberg:full ./scripts/test_docker.sh
```

## GitHub Actions

The `.github/workflows/publish-docker.yaml` workflow builds and publishes both variants to GitHub Container Registry:
- `ghcr.io/kreuzberg-dev/kreuzberg:VERSION-core` - Core image (minimal runtime)
- `ghcr.io/kreuzberg-dev/kreuzberg:core` - Latest core image
- `ghcr.io/kreuzberg-dev/kreuzberg:VERSION` - Full image (all optional dependencies)
- `ghcr.io/kreuzberg-dev/kreuzberg:latest` - Latest full image

For local development, use the local tags shown in the build commands above.

## Recommendations

**Choose Core if:**
- ✅ Minimal runtime setup
- ✅ Standard document processing needs
- ✅ Cloud deployments with cost constraints
- ✅ Kubernetes or container orchestration

**Choose Full if:**
- ✅ Want maximum optional dependencies preinstalled
- ✅ Development and testing environments
- ✅ "Batteries included" experience preferred

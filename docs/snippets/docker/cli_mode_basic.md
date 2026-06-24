```bash title="Bash"
# Extract a single file
docker run -v $(pwd):/data ghcr.io/xberg-io/kreuzberg:latest \
  extract /data/document.pdf

# Batch process multiple files
docker run -v $(pwd):/data ghcr.io/xberg-io/kreuzberg:latest \
  batch /data/*.pdf --output-format json

# Detect MIME type
docker run -v $(pwd):/data ghcr.io/xberg-io/kreuzberg:latest \
  detect /data/unknown-file.bin
```

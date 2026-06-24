```bash title="Bash"
docker pull ghcr.io/xberg-io/kreuzberg-cli:latest
docker run -v $(pwd):/data ghcr.io/xberg-io/kreuzberg-cli:latest extract /data/document.pdf
```

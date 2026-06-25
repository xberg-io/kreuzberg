```bash title="Bash"
docker pull ghcr.io/xberg-io/xberg-cli:latest
docker run -v $(pwd):/data ghcr.io/xberg-io/xberg-cli:latest extract /data/document.pdf
```

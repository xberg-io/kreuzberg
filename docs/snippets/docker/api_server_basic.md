```bash title="Bash"
# Start API server (default mode)
docker run -p 8000:8000 ghcr.io/xberg-io/xberg:latest

# Test the API
curl -F "files=@document.pdf" http://localhost:8000/extract
```

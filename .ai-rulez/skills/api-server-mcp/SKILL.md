---
name: api-server-mcp
priority: critical
---

# API Server & MCP Protocol

**Axum server design for document extraction endpoints, middleware, async processing, and Model Context Protocol integration for AI agents**

## Kreuzberg API Architecture

**Location**: `crates/kreuzberg/src/api/`, `crates/kreuzberg-cli/`

Kreuzberg provides a dual REST API + MCP server built with Axum + Tokio.

```
Request Flow:
HTTP Client / AI Agent (Claude)
    |
[Transport Layer]
├── REST API (Axum HTTP)
└── MCP Protocol (HTTP or Stdio)
    |
[Middleware Layer]
├── CORS, Request Logging (TraceLayer)
├── Request/Response size limits
└── Rate limiting (optional)
    |
[Router]
├── REST Endpoints
│   ├── POST /extract - File upload extraction
│   ├── POST /extract-url - URL-based extraction
│   ├── GET /formats - List supported formats
│   ├── GET /health - Server health check
│   ├── POST /batch - Batch document processing
│   ├── GET /cache/stats - Cache statistics
│   └── DELETE /cache - Clear extraction cache
├── MCP Endpoints
│   ├── POST /mcp/tools - List available tools
│   ├── POST /mcp/tools/call - Call a tool
│   ├── GET /mcp/resources - List resources
│   ├── GET /mcp/resources/:uri - Read resource
│   ├── GET /mcp/prompts - List prompts
│   └── GET /mcp/prompts/:name - Get prompt
    |
[Handler / Tool Layer]
├── extract_handler / extract_file tool
├── batch_handler / batch_extract tool
├── health_handler / get_capabilities tool
└── format_handler
    |
[Extraction Core]
├── Format detection
├── Extraction pipeline
├── Post-processing (chunking, embeddings)
└── Result formatting
    |
JSON Response / MCP ToolResult
```

## Server Setup & Configuration

**Location**: `crates/kreuzberg/src/api/server.rs`

Server initialization pattern: Create `ApiState` (holds `ExtractionConfig` + `ExtractionCache`), build Axum `Router` with all REST + MCP routes, apply middleware layers (body limits, CORS, tracing), serve via `tokio::net::TcpListener`.

Key middleware layers applied in order:
- `DefaultBodyLimit::max(100MB)` + `RequestBodyLimitLayer` -- configurable via env vars
- `CorsLayer::permissive()` -- restrict in production via `CORS_ALLOWED_ORIGINS`
- `TraceLayer::new_for_http()` -- request/response logging

## Core REST Handlers

**Location**: `crates/kreuzberg/src/api/handlers.rs`

| Handler | Method | Description |
|---------|--------|-------------|
| `extract_handler` | POST /extract | Multipart upload: parse file + optional config JSON, check cache, call `extract_bytes()`, cache result |
| `extract_url_handler` | POST /extract-url | Fetch URL via reqwest, extract bytes |
| `batch_handler` | POST /batch | Parallel extraction with `Semaphore`-limited concurrency (default: CPU count) |
| `health_handler` | GET /health | Report status, version, uptime, feature availability (OCR, embeddings), cache stats |
| `formats_handler` | GET /formats | Return supported format categories (office, pdf, images, web, email, archives, academic) |
| `cache_stats_handler` | GET /cache/stats | Hit/miss counts and hit rate |
| `cache_clear_handler` | DELETE /cache | Clear LRU cache |

## Caching Strategy

**Location**: `crates/kreuzberg/src/cache/mod.rs`

LRU cache keyed by `SHA256(file_content)`, stores `Arc<ExtractionResult>`. Default 1000 entries. Thread-safe via `RwLock`. Tracks hit/miss counters with `AtomicU64` for stats endpoint.

## Error Handling

**Location**: `crates/kreuzberg/src/api/error.rs`

`ApiError` enum maps to HTTP status codes:
- `MissingFile` -> 400, `FileNotFound` -> 404
- `OnnxRuntimeMissing` / `TesseractMissing` -> 503 (with remediation message)
- `PayloadTooLarge` -> 413
- `ExtractionFailed` / `InvalidConfig` / `UnsupportedFormat` -> 500

## MCP Server Implementation

**Location**: `crates/kreuzberg/src/mcp/server.rs`

The MCP server allows Claude and other AI agents to call Kreuzberg extraction functions through the Model Context Protocol.

### MCP Tools (Callable Functions)

Three tools are registered:

| Tool | Purpose | Required Params |
|------|---------|----------------|
| `extract_file` | Extract text/tables/metadata from documents (56+ formats) | `file_path` |
| `batch_extract` | Extract from multiple documents in parallel | `file_paths[]` |
| `get_capabilities` | List supported formats, features, backends | (none) |

**Tool registration pattern** (example: `extract_file`):
```rust
// Define Tool with name, description, JSON Schema inputSchema
// Register with server.register_tool(tool, handler_fn)
// Handler: parse params -> build ExtractionConfig -> call extract_file() -> return ToolResult as JSON
```

`extract_file` optional params: `format`, `extract_tables`, `extract_images`, `ocr_enabled`, `extract_metadata`, `chunking_preset`, `generate_embeddings`.

### MCP Resources (Static Knowledge)

Three resources provide static information to agents:
- `kreuzberg://formats` -- Supported format list as JSON
- `kreuzberg://features` -- Cross-binding feature matrix (from `FEATURE_MATRIX.md`)
- `kreuzberg://api-reference` -- Generated API documentation

### MCP Prompts (Agent Templates)

Two prompts guide agent extraction workflows:
- `extract_for_rag` -- Document type-specific RAG extraction guidance (research paper, contract, report). Recommends chunking preset and embedding config.
- `batch_document_processing` -- Optimal concurrency, grouping, and error handling for batch workflows.

### MCP Transport Protocols

- **HTTP/REST**: MCP routes mounted alongside REST API on separate `/mcp/` prefix
- **Stdio**: JSON-RPC 2.0 over stdin/stdout for local CLI integration (e.g., Claude Desktop)

### Integration with Claude Desktop

```json
{
  "mcpServers": {
    "kreuzberg": {
      "command": "kreuzberg-mcp",
      "env": {
        "KREUZBERG_API_BASE": "http://localhost:8000",
        "KREUZBERG_MCP_TRANSPORT": "stdio"
      }
    }
  }
}
```

### MCP Error Handling

`ToolError` variants: `FileNotFound`, `UnsupportedFormat`, `ExtractionFailed`, `OnnxRuntimeMissing`, `TesseractMissing`, `Timeout`. Each maps to an MCP `ToolResultError` with descriptive code and message.

## Environment Configuration

See `.env.example` for all configurable variables. Key categories:
- **Server**: `KREUZBERG_HOST`, `KREUZBERG_PORT`
- **Size limits**: `KREUZBERG_MAX_REQUEST_BODY_BYTES` (default 100MB), `KREUZBERG_MAX_MULTIPART_FIELD_BYTES`
- **Features**: `KREUZBERG_ENABLE_OCR`, `KREUZBERG_ENABLE_EMBEDDINGS`, `KREUZBERG_ENABLE_KEYWORDS`
- **Cache**: `KREUZBERG_CACHE_ENABLED`, `KREUZBERG_CACHE_SIZE`
- **CORS**: `CORS_ALLOWED_ORIGINS` (comma-separated)
- **MCP**: `KREUZBERG_MCP_HOST`, `KREUZBERG_MCP_PORT`, `KREUZBERG_MCP_TRANSPORT` (stdio/http)
- **Logging**: `RUST_LOG=kreuzberg=info,tower_http=debug`

## Critical Rules

### REST API Rules
1. **Always validate multipart file uploads** - Check MIME type, size, magic bytes
2. **Timeout long-running extractions** - Set per-handler timeout (5 min default)
3. **Stream large files** - Never buffer entire multi-GB file in memory
4. **Cache aggressively** - Identical files should return from cache in <1ms
5. **Parallel extraction is CPU-bound** - Limit workers to CPU count + 1
6. **Error responses must be actionable** - Include error code and remediation suggestion
7. **Health checks must verify features** - Report missing dependencies (ONNX, Tesseract)
8. **Size limits are configurable** - Allow override via env var for large deployments
9. **CORS is permissive by default** - Restrict in production via env var
10. **Logging all requests** - Track extraction metrics for observability

### MCP Rules
1. **All tools must have timeout** - Prevent hanging on large files (default 5 min)
2. **Error responses must be detailed** - Include suggestions for missing dependencies
3. **Feature gates must be checked** - Return helpful message if feature unavailable (embeddings, OCR)
4. **Resources should be static** - Don't query external services in resource handlers
5. **Prompts guide agents** - Provide clear examples and best practices
6. **Batch tools must support cancellation** - Allow agent to stop long-running batch operations
7. **Logging all tool calls** - Track usage for analytics and debugging

## Related Skills

- **extraction-pipeline-patterns** - Core extraction called by handlers and MCP tools
- **chunking-embeddings** - Optional chunking/embedding parameters in extraction
- **ocr-backend-management** - OCR engine selection and image preprocessing

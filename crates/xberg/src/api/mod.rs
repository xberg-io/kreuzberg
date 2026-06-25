//! REST API server for Xberg document extraction.
//!
//! This module provides an Axum-based HTTP server for document extraction
//! with endpoints for single and batch extraction operations.
//!
//! # Endpoints
//!
//! - `POST /extract` - Extract text from uploaded files (multipart form data)
//! - `POST /extract-structured` - Extract structured data via LLM with JSON schema (multipart form data)
//! - `POST /detect` - Detect MIME type of an uploaded file (multipart form data)
//! - `POST /embed` - Generate embeddings for text (JSON body with texts array)
//! - `POST /rerank` - Rerank documents by relevance to a query (JSON body with query and documents)
//! - `POST /chunk` - Chunk text into smaller pieces (JSON body with text and config)
//! - `GET /health` - Health check endpoint
//! - `GET /info` - Server information
//! - `GET /version` - Version information
//! - `GET /cache/stats` - Get cache statistics
//! - `GET /cache/manifest` - Get model manifest with checksums and sizes
//! - `POST /cache/warm` - Pre-download models to cache
//! - `DELETE /cache/clear` - Clear all cached files
//! - `PUT /process` - OpenWebUI "External" engine compatibility
//! - `POST /v1/convert/file` - OpenWebUI "Docling" engine compatibility (docling-serve drop-in)
//!
//! # Examples
//!
//! ## Starting the server
//!
//! ```no_run
//! use xberg::api::serve;
//!
//! #[tokio::main]
//! async fn main() -> xberg::Result<()> {
//!     // Local development
//!     serve("127.0.0.1", 8000).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Embedding the router in your app
//!
//! ```no_run
//! use xberg::{ExtractionConfig, api::create_router};
//! use axum::Router;
//!
//! #[tokio::main]
//! async fn main() -> xberg::Result<()> {
//!     // Load config (from file or use default)
//!     let config = ExtractionConfig::default();
//!     let xberg_router = create_router(config);
//!
//!     // Nest under /api prefix
//!     let app = Router::new().nest("/api", xberg_router);
//!
//!     // Add your own routes
//!     // ...
//!
//!     Ok(())
//! }
//! ```
//!
//! # cURL Examples
//!
//! ```bash
//! # Single file extraction
//! curl -F "files=@document.pdf" http://localhost:8000/extract
//!
//! # Multiple files with OCR config
//! curl -F "files=@doc1.pdf" -F "files=@doc2.jpg" \
//!      -F 'config={"ocr":{"language":"eng"}}' \
//!      http://localhost:8000/extract
//!
//! # Health check
//! curl http://localhost:8000/health
//!
//! # Server info
//! curl http://localhost:8000/info
//!
//! # Cache statistics
//! curl http://localhost:8000/cache/stats
//!
//! # Clear cache
//! curl -X DELETE http://localhost:8000/cache/clear
//!
//! # Generate embeddings
//! curl -X POST http://localhost:8000/embed \
//!      -H "Content-Type: application/json" \
//!      -d '{"texts":["Hello world","Second text"]}'
//!
//! # Rerank documents by relevance to a query
//! curl -X POST http://localhost:8000/rerank \
//!      -H "Content-Type: application/json" \
//!      -d '{"query":"What is Rust?","documents":["Rust is a systems language","Python is dynamic","Rust has ownership"]}'
//!
//! # Chunk text
//! curl -X POST http://localhost:8000/chunk \
//!      -H "Content-Type: application/json" \
//!      -d '{"text":"Long text to chunk...","chunker_type":"text"}'
//! ```

mod config;
mod error;
mod handlers;
#[cfg(feature = "api")]
pub(crate) mod jobs;
#[cfg(feature = "api")]
pub mod openapi;
mod openweb;
mod router;
mod startup;
mod types;

pub use error::ApiError;
#[allow(unused_imports)]
pub use router::{create_router, create_router_with_limits};
#[allow(unused_imports)]
pub use startup::{serve, serve_default, serve_with_config, serve_with_config_and_limits, serve_with_server_config};
pub use types::{
    ApiSizeLimits, ApiState, CacheClearResponse, CacheStatsResponse, ChunkRequest, ChunkResponse, DetectResponse,
    DoclingCompatDocument, DoclingCompatResponse, EmbedRequest, EmbedResponse, ErrorResponse, ExtractResponse,
    HealthResponse, InfoResponse, ManifestEntryResponse, ManifestResponse, OpenWebDocumentMetadata,
    OpenWebDocumentResponse, RerankRequest, RerankResponse, StructuredExtractionResponse, VersionResponse, WarmRequest,
    WarmResponse,
};

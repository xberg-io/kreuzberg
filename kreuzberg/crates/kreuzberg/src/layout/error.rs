use thiserror::Error;

#[derive(Error, Debug)]
pub enum LayoutError {
    #[error("ORT error: {0}")]
    Ort(#[from] ort::Error),
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Session not initialized")]
    SessionNotInitialized,
    #[error("Invalid model output: {0}")]
    InvalidOutput(String),
    #[error("Model download failed: {0}")]
    ModelDownload(String),
}

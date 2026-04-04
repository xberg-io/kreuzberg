//! Type conversions and marshaling between Rust and Python
//!
//! Provides From/Into implementations for converting between Python wrapper types
//! and their corresponding Kreuzberg Rust types.

use crate::config::*;

/// Convert ExtractionConfig to its inner Rust type
impl From<ExtractionConfig> for kreuzberg::ExtractionConfig {
    fn from(config: ExtractionConfig) -> Self {
        config.inner
    }
}

/// Convert Rust ExtractionConfig to Python wrapper
impl From<kreuzberg::ExtractionConfig> for ExtractionConfig {
    fn from(config: kreuzberg::ExtractionConfig) -> Self {
        Self {
            inner: config,
            html_options_dict: None,
        }
    }
}

/// Convert OcrConfig to its inner Rust type
impl From<OcrConfig> for kreuzberg::OcrConfig {
    fn from(config: OcrConfig) -> Self {
        config.inner
    }
}

/// Convert Rust OcrConfig to Python wrapper
impl From<kreuzberg::OcrConfig> for OcrConfig {
    fn from(config: kreuzberg::OcrConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert EmbeddingModelType to its inner Rust type
impl From<EmbeddingModelType> for kreuzberg::EmbeddingModelType {
    fn from(model: EmbeddingModelType) -> Self {
        model.inner
    }
}

/// Convert Rust EmbeddingModelType to Python wrapper
impl From<kreuzberg::EmbeddingModelType> for EmbeddingModelType {
    fn from(model: kreuzberg::EmbeddingModelType) -> Self {
        Self { inner: model }
    }
}

/// Convert EmbeddingConfig to its inner Rust type
impl From<EmbeddingConfig> for kreuzberg::EmbeddingConfig {
    fn from(config: EmbeddingConfig) -> Self {
        config.inner
    }
}

/// Convert Rust EmbeddingConfig to Python wrapper
impl From<kreuzberg::EmbeddingConfig> for EmbeddingConfig {
    fn from(config: kreuzberg::EmbeddingConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert ChunkingConfig to its inner Rust type
impl From<ChunkingConfig> for kreuzberg::ChunkingConfig {
    fn from(config: ChunkingConfig) -> Self {
        config.inner
    }
}

/// Convert Rust ChunkingConfig to Python wrapper
impl From<kreuzberg::ChunkingConfig> for ChunkingConfig {
    fn from(config: kreuzberg::ChunkingConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert ImageExtractionConfig to its inner Rust type
impl From<ImageExtractionConfig> for kreuzberg::ImageExtractionConfig {
    fn from(config: ImageExtractionConfig) -> Self {
        config.inner
    }
}

/// Convert Rust ImageExtractionConfig to Python wrapper
impl From<kreuzberg::ImageExtractionConfig> for ImageExtractionConfig {
    fn from(config: kreuzberg::ImageExtractionConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert PdfConfig to its inner Rust type
impl From<PdfConfig> for kreuzberg::PdfConfig {
    fn from(config: PdfConfig) -> Self {
        config.inner
    }
}

/// Convert Rust PdfConfig to Python wrapper
impl From<kreuzberg::PdfConfig> for PdfConfig {
    fn from(config: kreuzberg::PdfConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert TokenReductionConfig to its inner Rust type
impl From<TokenReductionConfig> for kreuzberg::TokenReductionConfig {
    fn from(config: TokenReductionConfig) -> Self {
        config.inner
    }
}

/// Convert Rust TokenReductionConfig to Python wrapper
impl From<kreuzberg::TokenReductionConfig> for TokenReductionConfig {
    fn from(config: kreuzberg::TokenReductionConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert LanguageDetectionConfig to its inner Rust type
impl From<LanguageDetectionConfig> for kreuzberg::LanguageDetectionConfig {
    fn from(config: LanguageDetectionConfig) -> Self {
        config.inner
    }
}

/// Convert Rust LanguageDetectionConfig to Python wrapper
impl From<kreuzberg::LanguageDetectionConfig> for LanguageDetectionConfig {
    fn from(config: kreuzberg::LanguageDetectionConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert PostProcessorConfig to its inner Rust type
impl From<PostProcessorConfig> for kreuzberg::PostProcessorConfig {
    fn from(config: PostProcessorConfig) -> Self {
        config.inner
    }
}

/// Convert Rust PostProcessorConfig to Python wrapper
impl From<kreuzberg::PostProcessorConfig> for PostProcessorConfig {
    fn from(config: kreuzberg::PostProcessorConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert ImagePreprocessingConfig to its inner Rust type
impl From<ImagePreprocessingConfig> for kreuzberg::types::ImagePreprocessingConfig {
    fn from(config: ImagePreprocessingConfig) -> Self {
        config.inner
    }
}

/// Convert Rust ImagePreprocessingConfig to Python wrapper
impl From<kreuzberg::types::ImagePreprocessingConfig> for ImagePreprocessingConfig {
    fn from(config: kreuzberg::types::ImagePreprocessingConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert TesseractConfig to its inner Rust type
impl From<TesseractConfig> for kreuzberg::types::TesseractConfig {
    fn from(config: TesseractConfig) -> Self {
        config.inner
    }
}

/// Convert Rust TesseractConfig to Python wrapper
impl From<kreuzberg::types::TesseractConfig> for TesseractConfig {
    fn from(config: kreuzberg::types::TesseractConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert PageConfig to its inner Rust type
impl From<PageConfig> for kreuzberg::core::config::PageConfig {
    fn from(config: PageConfig) -> Self {
        config.inner
    }
}

/// Convert Rust PageConfig to Python wrapper
impl From<kreuzberg::core::config::PageConfig> for PageConfig {
    fn from(config: kreuzberg::core::config::PageConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert HierarchyConfig to its inner Rust type
impl From<HierarchyConfig> for kreuzberg::core::config::HierarchyConfig {
    fn from(config: HierarchyConfig) -> Self {
        config.inner
    }
}

/// Convert Rust HierarchyConfig to Python wrapper
impl From<kreuzberg::core::config::HierarchyConfig> for HierarchyConfig {
    fn from(config: kreuzberg::core::config::HierarchyConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert EmailConfig to its inner Rust type
impl From<EmailConfig> for kreuzberg::EmailConfig {
    fn from(config: EmailConfig) -> Self {
        config.inner
    }
}

/// Convert Rust EmailConfig to Python wrapper
impl From<kreuzberg::EmailConfig> for EmailConfig {
    fn from(config: kreuzberg::EmailConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert AccelerationConfig to its inner Rust type
impl From<AccelerationConfig> for kreuzberg::AccelerationConfig {
    fn from(config: AccelerationConfig) -> Self {
        config.inner
    }
}

/// Convert Rust AccelerationConfig to Python wrapper
impl From<kreuzberg::AccelerationConfig> for AccelerationConfig {
    fn from(config: kreuzberg::AccelerationConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert TreeSitterProcessConfig to its inner Rust type
impl From<TreeSitterProcessConfig> for kreuzberg::core::config::TreeSitterProcessConfig {
    fn from(config: TreeSitterProcessConfig) -> Self {
        config.inner
    }
}

/// Convert Rust TreeSitterProcessConfig to Python wrapper
impl From<kreuzberg::core::config::TreeSitterProcessConfig> for TreeSitterProcessConfig {
    fn from(config: kreuzberg::core::config::TreeSitterProcessConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert TreeSitterConfig to its inner Rust type
impl From<TreeSitterConfig> for kreuzberg::core::config::TreeSitterConfig {
    fn from(config: TreeSitterConfig) -> Self {
        config.inner
    }
}

/// Convert Rust TreeSitterConfig to Python wrapper
impl From<kreuzberg::core::config::TreeSitterConfig> for TreeSitterConfig {
    fn from(config: kreuzberg::core::config::TreeSitterConfig) -> Self {
        Self { inner: config }
    }
}

/// Convert FileExtractionConfig to its inner Rust type
impl From<FileExtractionConfig> for kreuzberg::FileExtractionConfig {
    fn from(config: FileExtractionConfig) -> Self {
        config.inner
    }
}

/// Convert Rust FileExtractionConfig to Python wrapper
impl From<kreuzberg::FileExtractionConfig> for FileExtractionConfig {
    fn from(config: kreuzberg::FileExtractionConfig) -> Self {
        Self {
            inner: config,
            html_options_dict: None,
        }
    }
}

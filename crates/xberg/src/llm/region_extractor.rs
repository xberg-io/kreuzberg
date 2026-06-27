//! Per-region VLM extraction for diagrams, dense tables, and complex layouts.
//!
//! When layout detection identifies a region as a figure, dense table, or
//! complex layout, this module crops the region's bounding box from the page
//! image and sends it to a VLM for precise extraction. The result is spliced
//! back into the markdown at the region's anchor position.
//!
//! This module is only compiled when `liter-llm` is available (non-Windows).

use super::vlm_ocr::vlm_ocr;
use crate::core::config::LlmConfig;
use crate::types::LlmUsage;

/// Classification of a detected layout region that warrants VLM extraction.
///
/// Each variant maps to a specific prompt optimised for that content type.
/// The mapping is intentionally narrow — only region kinds for which VLM
/// extraction provides a clear quality benefit over classical suppression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RegionKind {
    /// A figure, diagram, chart, or image region.
    ///
    /// VLM prompt: describe the diagram / chart, including axis labels,
    /// legend entries, and any embedded text.
    Figure,

    /// A densely formatted or complex table that classical extraction garbles.
    ///
    /// VLM prompt: extract the table as GitHub-Flavoured Markdown.
    DenseTable,

    /// A region whose layout the classical pipeline cannot handle (multi-column
    /// insets, heavily annotated forms, mixed text+diagram).
    ///
    /// VLM prompt: extract all text and structure as markdown, preserving
    /// reading order.
    ComplexLayout,

    /// A standalone image to be captioned (not extracted as figure markdown).
    ///
    /// VLM prompt: produce a single-sentence alt-text-style caption suitable
    /// for accessibility tooling and downstream indexing. Used by the
    /// captioning post-processor to populate
    /// [`ExtractedImage::caption`](crate::types::ExtractedImage::caption).
    Caption,
}

impl RegionKind {
    /// Returns the default Jinja2 prompt template for this region kind.
    ///
    /// Templates are plain strings (no variables required by default).
    /// Callers may override by passing a custom template to
    /// [`extract_region_with_vlm`].
    pub fn default_prompt(self) -> &'static str {
        match self {
            Self::Figure => REGION_FIGURE_TEMPLATE,
            Self::DenseTable => REGION_DENSE_TABLE_TEMPLATE,
            Self::ComplexLayout => REGION_COMPLEX_LAYOUT_TEMPLATE,
            Self::Caption => REGION_CAPTION_TEMPLATE,
        }
    }
}

impl From<String> for RegionKind {
    /// Converts a string variant name to a [`RegionKind`].
    ///
    /// Accepted values (case-sensitive): `"Figure"`, `"DenseTable"`,
    /// `"ComplexLayout"`, `"Caption"`. Any other value maps to
    /// [`RegionKind::Figure`] as the safe default.
    fn from(s: String) -> Self {
        match s.as_str() {
            "Figure" => Self::Figure,
            "DenseTable" => Self::DenseTable,
            "ComplexLayout" => Self::ComplexLayout,
            "Caption" => Self::Caption,
            _ => Self::Figure,
        }
    }
}

/// Default prompt for figure / diagram regions.
const REGION_FIGURE_TEMPLATE: &str = "\
Describe this figure or diagram in detail. Include:
- The type of figure (chart, graph, diagram, photo, illustration, etc.)
- All text visible in the figure (labels, titles, legends, axis names, annotations)
- The key data or relationships the figure conveys
- Any embedded numeric values, percentages, or measurements

Return the description as concise markdown. Do not add headings — return only \
a paragraph or a short bulleted list if appropriate. If the figure contains no \
meaningful content, return an empty string.";

/// Default prompt for dense / complex table regions.
const REGION_DENSE_TABLE_TEMPLATE: &str = "\
Extract the table from this image as GitHub-Flavoured Markdown.
- Preserve all columns and rows exactly as they appear.
- Use `|` column separators and a `---` separator row after the header.
- If the table has no visible header, create a row of empty header cells.
- Do not add explanatory text — return only the Markdown table.
- If the image does not contain a table, return an empty string.";

/// Default prompt for complex / mixed-layout regions.
const REGION_COMPLEX_LAYOUT_TEMPLATE: &str = "\
Extract all text and structured content from this image region as Markdown.
- Preserve the original reading order (top to bottom, left to right).
- Use appropriate Markdown elements: paragraphs, lists, code blocks, tables.
- Do not add commentary or explanations beyond what the image contains.
- If the region contains no meaningful text, return an empty string.";

/// Default prompt for image captioning (per-image alt-text style).
const REGION_CAPTION_TEMPLATE: &str = "\
Write a concise, factual caption for this image suitable for use as alt text \
or a search-index entry.
- One or two sentences at most.
- Describe what is visible: subject, action, setting, notable text.
- Do not speculate about intent, mood, or context that is not visible.
- Do not start the caption with phrases like \"This image shows\" or \
\"A picture of\" — lead with the subject.
- If the image has no recognisable content, return an empty string.";

/// Extract content from a pre-cropped image region using a VLM.
///
/// The caller is responsible for cropping the page image to the region's bounding
/// box before calling this function. The `image_bytes` parameter must contain the
/// raw bytes of the **cropped** region image (JPEG, PNG, WebP, etc.).
///
/// # Arguments
///
/// * `image_bytes` — Raw bytes of the **pre-cropped** region image.
/// * `image_mime` — MIME type of the image (`"image/png"`, `"image/jpeg"`, etc.).
/// * `region_kind` — Content type of the region, used to select the default prompt.
/// * `llm_config` — LLM provider and model configuration.
/// * `custom_prompt` — Optional override for the default per-region prompt template.
///
/// # Returns
///
/// Extracted Markdown text from the VLM, or an error if the VLM call fails.
///
/// # Errors
///
/// - [`crate::XbergError::Ocr`] if the VLM call fails or returns no content.
/// - [`crate::XbergError::MissingDependency`] if the liter-llm client cannot
///   be initialised.
///
/// # Example
///
/// ```rust,no_run
/// use xberg::llm::region_extractor::{RegionKind, extract_region_with_vlm};
/// use xberg::LlmConfig;
///
/// # async fn example() -> xberg::Result<()> {
/// let image_bytes: Vec<u8> = std::fs::read("cropped_figure.png")?;
/// let config = LlmConfig {
///     model: "openai/gpt-4o-mini".to_string(),
///     base_url: Some("http://localhost:9999".to_string()),
///     ..Default::default()
/// };
/// let markdown = extract_region_with_vlm(
///     &image_bytes,
///     "image/png",
///     RegionKind::Figure,
///     &config,
///     None,
/// )
/// .await?;
/// println!("Extracted: {markdown}");
/// # Ok(())
/// # }
/// ```
pub async fn extract_region_with_vlm(
    image_bytes: &[u8],
    image_mime: &str,
    region_kind: RegionKind,
    llm_config: &LlmConfig,
    custom_prompt: Option<&str>,
) -> crate::Result<String> {
    let (text, _usage) =
        extract_region_with_vlm_usage(image_bytes, image_mime, region_kind, llm_config, custom_prompt).await?;
    Ok(text)
}

/// Same as [`extract_region_with_vlm`], but also returns the [`LlmUsage`] data captured
/// from the underlying VLM call.
///
/// Callers that need to track token / cost data per call (for example the captioning
/// post-processor, which appends every call's usage to
/// [`ExtractedDocument::llm_usage`](crate::types::ExtractedDocument::llm_usage)) should
/// prefer this variant. The plain [`extract_region_with_vlm`] is kept for callers that
/// only care about the markdown output (PDF region splicing).
///
/// # Errors
///
/// Same as [`extract_region_with_vlm`].
#[cfg_attr(alef, alef(skip))]
pub async fn extract_region_with_vlm_usage(
    image_bytes: &[u8],
    image_mime: &str,
    region_kind: RegionKind,
    llm_config: &LlmConfig,
    custom_prompt: Option<&str>,
) -> crate::Result<(String, Option<LlmUsage>)> {
    let prompt = custom_prompt.unwrap_or_else(|| region_kind.default_prompt());

    // vlm_ocr re-uses the language=eng path (no language hint for region extraction).
    // The prompt is passed as `vlm_prompt`; `language` is set to a neutral value that
    // suppresses the language-hint suffix in the VLM OCR template.
    vlm_ocr(
        image_bytes,
        image_mime,
        "eng", // language hint unused — prompt is self-contained
        llm_config,
        Some(prompt),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_kind_default_prompt_figure() {
        let prompt = RegionKind::Figure.default_prompt();
        assert!(
            prompt.contains("diagram") || prompt.contains("figure"),
            "figure prompt must mention figures/diagrams; got: {prompt}"
        );
    }

    #[test]
    fn test_region_kind_default_prompt_dense_table() {
        let prompt = RegionKind::DenseTable.default_prompt();
        assert!(
            prompt.contains("Markdown") || prompt.contains("table"),
            "dense table prompt must mention Markdown/table; got: {prompt}"
        );
    }

    #[test]
    fn test_region_kind_default_prompt_complex_layout() {
        let prompt = RegionKind::ComplexLayout.default_prompt();
        assert!(
            prompt.contains("Markdown") || prompt.contains("reading order"),
            "complex layout prompt must mention Markdown; got: {prompt}"
        );
    }

    #[test]
    fn test_region_kind_prompts_are_non_empty() {
        for kind in [
            RegionKind::Figure,
            RegionKind::DenseTable,
            RegionKind::ComplexLayout,
            RegionKind::Caption,
        ] {
            assert!(
                !kind.default_prompt().is_empty(),
                "{kind:?} default prompt must not be empty"
            );
        }
    }

    #[test]
    fn test_region_kind_default_prompt_caption() {
        let prompt = RegionKind::Caption.default_prompt();
        assert!(
            prompt.contains("caption") || prompt.contains("alt text"),
            "caption prompt must mention captions/alt text; got: {prompt}"
        );
    }

    #[test]
    fn test_region_kind_equality() {
        assert_eq!(RegionKind::Figure, RegionKind::Figure);
        assert_ne!(RegionKind::Figure, RegionKind::DenseTable);
        assert_ne!(RegionKind::DenseTable, RegionKind::ComplexLayout);
    }
}

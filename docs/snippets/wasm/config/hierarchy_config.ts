import {
  extract,
  initWasm,
  type ExtractionConfig,
  type HierarchyConfig,
  type PdfConfig,
} from "@xberg-io/xberg-wasm";

// Example 1: Basic hierarchy extraction
// Enabled with default kClusters=6 for standard H1-H6 heading hierarchy.
// Extract bounding box information for spatial layout awareness.
const hierarchyConfigBasic: HierarchyConfig = {
  enabled: true,
  kClusters: 6, // Default: creates 6 font size clusters (H1-H6 structure)
  includeBbox: true, // Include bounding box coordinates
  ocrCoverageThreshold: undefined, // No OCR coverage threshold
};

const pdfConfigBasic: PdfConfig = {
  hierarchy: hierarchyConfigBasic,
};

const extractionConfigBasic: ExtractionConfig = {
  pdfOptions: pdfConfigBasic,
};

// Example 2: Custom kClusters for minimal structure
// Use 3 clusters for simpler hierarchy with minimal structure.
// Useful when you only need major section divisions (Main, Subsection, Detail).
const hierarchyConfigMinimal: HierarchyConfig = {
  enabled: true,
  kClusters: 3, // Minimal clustering: just 3 levels
  includeBbox: true,
  ocrCoverageThreshold: undefined,
};

const pdfConfigMinimal: PdfConfig = {
  hierarchy: hierarchyConfigMinimal,
};

const _extractionConfigMinimal: ExtractionConfig = {
  pdfOptions: pdfConfigMinimal,
};

// Example 3: With OCR coverage threshold
// Trigger OCR if less than 50% of text has font data.
// Useful for documents with mixed digital and scanned content.
const hierarchyConfigOcr: HierarchyConfig = {
  enabled: true,
  kClusters: 6,
  includeBbox: true,
  ocrCoverageThreshold: 0.5, // Trigger OCR if text coverage < 50%
};

const pdfConfigOcr: PdfConfig = {
  hierarchy: hierarchyConfigOcr,
};

const _extractionConfigOcr: ExtractionConfig = {
  pdfOptions: pdfConfigOcr,
};

// Integration with Xberg WASM instance
async function _extractWithHierarchy(): Promise<void> {
  await initWasm();

  try {
    // Extract from file bytes with hierarchy configuration
    const pdfBytes = new Uint8Array([/* PDF file data */]);
    const output = await extract(
      { kind: "bytes", bytes: pdfBytes, mimeType: "application/pdf" },
      extractionConfigBasic,
    );
    console.log("Extraction complete:", output.results[0].content);
  } catch (error) {
    console.error("Extraction failed:", error);
  }
}

// Field descriptions:
//
// enabled: boolean (default: true)
//   - Enable or disable hierarchy extraction
//   - When false, hierarchy structure is not analyzed
//
// kClusters: number (default: 6, valid: 1-7)
//   - Number of font size clusters for hierarchy levels
//   - 6 provides H1-H6 heading levels with body text
//   - Higher values create more fine-grained hierarchy
//   - Lower values create simpler structure
//
// includeBbox: boolean (default: true)
//   - Include bounding box coordinates in hierarchy blocks
//   - Required for spatial layout awareness and document structure
//   - Set to false only if space optimization is critical
//
// ocrCoverageThreshold: number | undefined (default: undefined)
//   - Range: 0.0 to 1.0
//   - Triggers OCR when text block coverage falls below this fraction
//   - Example: 0.5 means "run OCR if less than 50% of page has text data"
//   - undefined means no OCR coverage-based triggering
//

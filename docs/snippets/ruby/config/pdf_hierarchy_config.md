```ruby title="Ruby"
require 'xberg'

# Using keyword arguments with defaults
config = Xberg::ExtractionConfig.new(
  pdf_options: Xberg::PdfConfig.new(
    extract_images: true,
    hierarchy: Xberg::HierarchyConfig.new(
      enabled: true,
      k_clusters: 6,
      include_bbox: true,
      ocr_coverage_threshold: 0.8
    )
  )
)

# Using hash syntax alternative
config = Xberg::ExtractionConfig.new(
  pdf_options: Xberg::PdfConfig.new(
    extract_images: true,
    hierarchy: {
      enabled: true,
      k_clusters: 6,
      include_bbox: true,
      ocr_coverage_threshold: 0.8
    }
  )
)
```

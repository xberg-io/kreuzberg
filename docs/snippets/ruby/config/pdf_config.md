```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  pdf_options: Xberg::PdfConfig.new(
    extract_images: true,
    extract_metadata: true,
    passwords: ['password1', 'password2'],
    hierarchy: Xberg::HierarchyConfig.new(
      enabled: true,
      k_clusters: 6,
      include_bbox: true
    )
  )
)
```

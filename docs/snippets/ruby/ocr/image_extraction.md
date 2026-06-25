```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  images: Xberg::ImageExtractionConfig.new(
    extract_images: true,
    target_dpi: 200,
    max_image_dimension: 2048,
    inject_placeholders: true, # set to false to extract images without markdown references
    auto_adjust_dpi: true
  )
)
```

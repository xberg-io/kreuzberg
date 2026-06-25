```ruby title="Ruby"
require 'xberg'

config = Xberg::ExtractionConfig.new(
  ocr: Xberg::OcrConfig.new(
    tesseract_config: Xberg::TesseractConfig.new(
      preprocessing: Xberg::ImagePreprocessingConfig.new(
        target_dpi: 300,
        denoise: true,
        deskew: true,
        contrast_enhance: true,
        binarization_method: 'otsu'
      )
    )
  )
)
```

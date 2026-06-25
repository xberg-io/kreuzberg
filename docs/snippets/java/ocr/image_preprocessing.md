```java title="Java"
import dev.xberg.ExtractionConfig;
import dev.xberg.ImagePreprocessingConfig;
import dev.xberg.OcrConfig;
import dev.xberg.TesseractConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .ocr(OcrConfig.builder()
        .tesseractConfig(TesseractConfig.builder()
            .preprocessing(ImagePreprocessingConfig.builder()
                .targetDpi(300)
                .denoise(true)
                .deskew(true)
                .contrastEnhance(true)
                .binarizationMethod("otsu")
                .build())
            .build())
        .build())
    .build();
```

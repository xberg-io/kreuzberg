```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.ExtractionConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .enableQualityProcessing(true)
    .build();

ExtractionResult result = Xberg.extractFile("scanned_document.pdf", config);

double qualityScore = result.getQualityScore() != null ? result.getQualityScore() : 0.0;

if (qualityScore < 0.5) {
    System.out.printf("Warning: Low quality extraction (%.2f)%n", qualityScore);
    System.out.println("Consider re-scanning or adjusting OCR settings");
} else {
    System.out.printf("Quality score: %.2f%n", qualityScore);
}
```

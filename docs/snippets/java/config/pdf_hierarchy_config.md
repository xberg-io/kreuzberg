```java title="Java"
import dev.xberg.ExtractionConfig;
import dev.xberg.PdfConfig;
import dev.xberg.HierarchyConfig;

ExtractionConfig config = ExtractionConfig.builder()
    .pdfOptions(PdfConfig.builder()
        .hierarchyConfig(HierarchyConfig.builder()
            .enabled(true)
            .detectionThreshold(0.75)
            .ocrCoverageThreshold(0.8)
            .minLevel(1)
            .maxLevel(5)
            .build())
        .build())
    .build();
```

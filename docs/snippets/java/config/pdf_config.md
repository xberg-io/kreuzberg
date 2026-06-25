```java title="Java"
import dev.xberg.ExtractionConfig;
import dev.xberg.PdfConfig;
import dev.xberg.HierarchyConfig;
import java.util.Arrays;

ExtractionConfig config = ExtractionConfig.builder()
    .pdfOptions(PdfConfig.builder()
        .extractImages(true)
        .extractMetadata(true)
        .passwords(Arrays.asList("password1", "password2"))
        .hierarchyConfig(HierarchyConfig.builder().build())
        .build())
    .build();
```

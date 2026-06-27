```java title="Java"
import io.xberg.Xberg;
import io.xberg.ExtractInputKind;
import io.xberg.ExtractionResult;
import io.xberg.ExtractedDocument;
import io.xberg.ExtractInput;
import io.xberg.ExtractionConfig;
import io.xberg.PostProcessor;
import io.xberg.XbergException;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.logging.Logger;

public class PdfMetadataExtractorExample {
    private static final Logger logger = Logger.getLogger(
        PdfMetadataExtractorExample.class.getName()
    );
    public static void main(String[] args) {
        AtomicInteger processedCount = new AtomicInteger(0);
        PostProcessor pdfMetadata = result -> {
            if (!result.mimeType().equals("application/pdf")) {
                return result;
            }
            processedCount.incrementAndGet();
            Map<String, Object> metadata = new HashMap<>(result.metadata());
            metadata.put("pdf_processed", true);
            metadata.put("processing_timestamp", System.currentTimeMillis());
            logger.info("Processed PDF: " + processedCount.get());
            return result;
        };
        try {
            Xberg.registerPostProcessor("pdf-metadata-extractor", pdfMetadata, 50);
            logger.info("PDF metadata extractor initialized");
            ExtractionResult output = Xberg.extract(
                ExtractInput.builder().withKind(ExtractInputKind.Uri).withUri("document.pdf").build(),
                ExtractionConfig.builder().build()
            );
            ExtractedDocument result = output.results().get(0);
            System.out.println("PDF processed: " + result.metadata().get("pdf_processed"));
            logger.info("Processed " + processedCount.get() + " PDFs");
        } catch (IOException | XbergException e) {
            e.printStackTrace();
        }
    }
}
```

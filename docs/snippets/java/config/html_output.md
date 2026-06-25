```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionConfig;
import dev.xberg.ExtractionResult;
import dev.xberg.HtmlOutputConfig;
import dev.xberg.HtmlTheme;
import dev.xberg.OutputFormat;
import java.nio.file.Path;
import java.util.Optional;

public class HtmlOutput {
    public static void main(String[] args) throws Exception {
        HtmlOutputConfig htmlOutput = HtmlOutputConfig.builder()
            .withTheme(HtmlTheme.GitHub)
            .withEmbedCss(true)
            .build();

        ExtractionConfig config = ExtractionConfig.builder()
            .withOutputFormat(OutputFormat.Html)
            .withHtmlOutput(Optional.of(htmlOutput))
            .build();

        ExtractionResult result = Xberg.extractFileSync(Path.of("document.pdf"), config);
        System.out.println(result.content()); // HTML with kb-* classes
    }
}
```

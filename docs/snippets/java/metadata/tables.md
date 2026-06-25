```java title="Java"
import dev.xberg.Xberg;
import dev.xberg.ExtractionResult;
import dev.xberg.XbergException;
import dev.xberg.Table;
import java.io.IOException;
import java.util.List;

public class Main {
    public static void main(String[] args) {
        try {
            ExtractionResult result = Xberg.extractFile("document.pdf");

            for (Table table : result.getTables()) {
                System.out.println("Table with " + table.cells().size() + " rows");
                System.out.println(table.markdown());

                for (List<String> row : table.cells()) {
                    System.out.println(row);
                }
            }
        } catch (IOException | XbergException e) {
            System.err.println("Extraction failed: " + e.getMessage());
        }
    }
}
```

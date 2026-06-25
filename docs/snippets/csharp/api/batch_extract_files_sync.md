```csharp title="C#"
using Xberg;

var items = new List<BatchFileItem>
{
    new() { Path = "document1.pdf", Config = null },
    new()
    {
        Path = "document2.pdf",
        Config = new FileExtractionConfig { ForceOcr = true }
    }
};

var config = new ExtractionConfig { OutputFormat = OutputFormat.Text };
var results = XbergLib.BatchExtractFilesSync(items, config);

foreach (var result in results)
{
    Console.WriteLine($"Content length: {result.Content.Length}");
}
```

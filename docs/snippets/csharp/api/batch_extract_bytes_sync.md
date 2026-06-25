```csharp title="C#"
using Xberg;

var items = new List<BatchBytesItem>
{
    new() { Content = await File.ReadAllBytesAsync("doc1.pdf"), MimeType = "application/pdf", Config = null },
    new() { Content = await File.ReadAllBytesAsync("doc2.txt"), MimeType = "text/plain", Config = null }
};

var config = new ExtractionConfig { OutputFormat = OutputFormat.Text };
var results = XbergLib.BatchExtractBytesSync(items, config);

foreach (var result in results)
{
    Console.WriteLine($"Content length: {result.Content.Length}");
}
```

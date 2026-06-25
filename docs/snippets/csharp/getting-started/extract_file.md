```csharp title="C#"
using Xberg;

var result = XbergLib.ExtractFileSync("document.pdf", new ExtractionConfig());

Console.WriteLine(result.Content);
Console.WriteLine($"MIME Type: {result.Metadata.FormatType}");
Console.WriteLine($"Tables: {result.Tables.Count}");
```

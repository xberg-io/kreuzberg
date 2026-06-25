```csharp title="C#"
using Xberg;

var config = new ExtractionConfig { OutputFormat = OutputFormat.Text };
var result = XbergLib.ExtractFileSync("document.pdf", null, config);

Console.WriteLine(result.Content);
Console.WriteLine($"MIME Type: {result.MimeType}");
```

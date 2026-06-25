```csharp title="C#"
using Xberg;

var result = XbergLib.ExtractFileSync("document.pdf", new ExtractionConfig());
Console.WriteLine(result.Content);
```

```csharp title="C#"
using Xberg;

var config = ExtractionConfig.Discover() ?? new ExtractionConfig();

var result = await XbergLib.ExtractFile("document.pdf", null, config);
Console.WriteLine(result.Content);
```

```csharp title="C#"
using Xberg;

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), new ExtractionConfig())).Results[0];
Console.WriteLine(result.Content);
```

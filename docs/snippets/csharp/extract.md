```csharp title="C#"
using Xberg;

var output = await XbergConverter.ExtractAsync(
    ExtractInput.FromUri("document.pdf"),
    ExtractionConfig.Default()
);

Console.WriteLine(output.Results[0].Content);
```

```csharp title="C#"
using Xberg;

var config = new ExtractionConfig();
var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];

Console.WriteLine(result.Content[..Math.Min(100, result.Content.Length)]);
Console.WriteLine($"Total length: {result.Content.Length}");
```

```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    EnableQualityProcessing = true,
    UseCache = true
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];
Console.WriteLine($"Quality score: {result.QualityScore}");
Console.WriteLine($"Content length: {result.Content.Length}");
```

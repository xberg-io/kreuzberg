```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    TokenReduction = new TokenReductionOptions
    {
        Mode = "moderate",
        PreserveImportantWords = true
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];
Console.WriteLine($"Reduced content length: {result.Content.Length}");
Console.WriteLine($"Content: {result.Content.Substring(0, Math.Min(100, result.Content.Length))}");
```

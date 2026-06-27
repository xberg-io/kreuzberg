```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    Postprocessor = new PostProcessorConfig
    {
        Enabled = true,
        EnabledProcessors = new List<string>
        {
            "whitespace_normalizer",
            "unicode_normalizer"
        },
        DisabledProcessors = null
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];
Console.WriteLine($"Processed content: {result.Content.Substring(0, Math.Min(100, result.Content.Length))}");
```

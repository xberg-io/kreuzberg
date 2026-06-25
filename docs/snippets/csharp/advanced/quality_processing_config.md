```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    EnableQualityProcessing = true
};

var result = await XbergLib.ExtractFileAsync(
    "document.pdf",
    config
);

var qualityScore = result.QualityScore;

Console.WriteLine($"Quality score: {qualityScore:F2}");
```

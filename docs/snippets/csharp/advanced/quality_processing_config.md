```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    EnableQualityProcessing = true
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri(
    "document.pdf"), config
)).Results[0];

var qualityScore = result.QualityScore;

Console.WriteLine($"Quality score: {qualityScore:F2}");
```

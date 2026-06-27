```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    LanguageDetection = new LanguageDetectionConfig
    {
        Enabled = true,
        MinConfidence = 0.9,
        DetectMultiple = false
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];

if (result.DetectedLanguages != null && result.DetectedLanguages.Count > 0)
{
    Console.WriteLine($"Primary language: {result.DetectedLanguages[0]}");
}
```

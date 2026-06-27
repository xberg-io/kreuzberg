```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    UseCache = true,
    EnableQualityProcessing = true,
    Ocr = new OcrConfig
    {
        Backend = "tesseract",
        Language = "eng+deu"
    },
    Chunking = new ChunkingConfig
    {
        MaxCharacters = 1000,
        Overlap = 200
    },
    LanguageDetection = new LanguageDetectionConfig
    {
        Enabled = true,
        DetectMultiple = true
    },
    TokenReduction = new TokenReductionOptions
    {
        Mode = "moderate"
    },
    Keywords = new KeywordConfig
    {
        MaxKeywords = 10,
        MinScore = 0.1f
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];
Console.WriteLine(result.Content);

if (result.DetectedLanguages?.Count > 0)
{
    Console.WriteLine($"Languages: {string.Join(", ", result.DetectedLanguages)}");
}
```

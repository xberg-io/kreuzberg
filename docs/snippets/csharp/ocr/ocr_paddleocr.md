```csharp title="C#"
using Kreuzberg;

var config = new ExtractionConfig
{
    Ocr = new OcrConfig
    {
        Backend = "paddle-ocr",
        Language = "en",
        PaddleOcrConfig = new PaddleOcrConfig
        {
            ModelTier = "mobile"
        }
    }
};

var result = KreuzbergClient.ExtractFileSync("scanned.pdf", config);
Console.WriteLine(result.Content);
```

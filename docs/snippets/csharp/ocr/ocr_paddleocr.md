```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    Ocr = new OcrConfig
    {
        Backend = "paddle-ocr",
        Language = "en",
        // PaddleOcrConfig = new PaddleOcrConfig { ModelTier = "server" } // for max accuracy
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("scanned.pdf"), config)).Results[0];
Console.WriteLine(result.Content);
```

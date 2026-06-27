```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    Ocr = new OcrConfig
    {
        Backend = "tesseract",
        Language = "eng"
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("scanned.pdf"), config)).Results[0];
Console.WriteLine(result.Content);
```

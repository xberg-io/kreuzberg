```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    Ocr = new OcrConfig
    {
        TesseractConfig = new TesseractConfig
        {
            Preprocessing = new ImagePreprocessingConfig
            {
                TargetDpi = 300,
                Denoise = true,
                Deskew = true,
                ContrastEnhance = true,
                BinarizationMethod = "otsu"
            }
        }
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("scanned.pdf"), config)).Results[0];
Console.WriteLine($"Content: {result.Content[..Math.Min(100, result.Content.Length)]}");
```

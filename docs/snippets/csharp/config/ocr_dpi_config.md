```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    Images = new ImageExtractionConfig
    {
        ExtractImages = true,
        TargetDpi = 300,
        MaxImageDimension = 4096,
        AutoAdjustDpi = true,
        MinDpi = 150,
        MaxDpi = 600
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];
if (result.Images != null)
{
    Console.WriteLine($"Extracted images: {result.Images.Count}");
}
```

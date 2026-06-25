```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    UseCache = true,
    EnableQualityProcessing = true
};

var result = await XbergLib.ExtractFileAsync("document.pdf", config);
Console.WriteLine(result.Content);
```

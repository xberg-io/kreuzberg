```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    UseCache = true,
    EnableQualityProcessing = true
};

var result = await XbergLib.ExtractFile("document.pdf", null, config);
Console.WriteLine(result.Content);
```

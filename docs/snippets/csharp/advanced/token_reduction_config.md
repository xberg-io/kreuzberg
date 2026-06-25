```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    TokenReduction = new TokenReductionConfig
    {
        Mode = "moderate",              // "off", "moderate", or "aggressive"
        PreserveMarkdown = true,
        PreserveCode = true,
        LanguageHint = "eng"
    }
};
```

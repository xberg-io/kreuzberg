```csharp title="C#"
using Xberg;

var config = new ExtractionConfig
{
    OutputFormat = OutputFormat.Html,
    HtmlOutput = new HtmlOutputConfig
    {
        Theme = HtmlTheme.GitHub,
        EmbedCss = true,
        ClassPrefix = "kb-"
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];
Console.WriteLine(result.Content);
```

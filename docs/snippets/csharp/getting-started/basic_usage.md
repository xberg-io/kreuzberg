```csharp title="C#"
using Xberg;

var config = new ExtractionConfig();
var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];
Console.WriteLine(result.Content);
```

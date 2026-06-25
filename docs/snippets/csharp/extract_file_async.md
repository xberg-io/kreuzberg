```csharp title="C#"
using Xberg;

var result = await XbergLib.ExtractFileAsync("document.pdf");

Console.WriteLine(result.Content);
Console.WriteLine(result.MimeType);
```

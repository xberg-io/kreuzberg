```csharp title="C#"
using Xberg;

var data = await File.ReadAllBytesAsync("document.pdf");
var result = await XbergLib.ExtractBytesAsync(data, "application/pdf");

Console.WriteLine(result.Content);
Console.WriteLine(result.MimeType);
```

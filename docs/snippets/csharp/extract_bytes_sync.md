```csharp title="C#"
using Xberg;

var data = await File.ReadAllBytesAsync("document.pdf");
var result = XbergLib.ExtractBytesSync(data, "application/pdf");

Console.WriteLine(result.Content);
Console.WriteLine(result.MimeType);
```

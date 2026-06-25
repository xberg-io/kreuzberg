```csharp title="C#"
using Xberg;

var files = new[] { "doc1.pdf", "doc2.docx", "doc3.pptx" };
var results = XbergLib.BatchExtractFilesSync(files, new ExtractionConfig());

foreach (var result in results)
{
    Console.WriteLine($"Content length: {result.Content.Length}");
}
```

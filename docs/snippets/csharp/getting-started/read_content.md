```csharp title="C#"
using Xberg;

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), new ExtractionConfig())).Results[0];

foreach (var table in result.Tables)
{
    Console.WriteLine($"Table with {table.Rows.Count} rows");
}

foreach (var chunk in result.Chunks)
{
    Console.WriteLine(chunk.Content);
}
```

```csharp title="C#"
using Xberg;

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), new ExtractionConfig())).Results[0];

foreach (var table in result.Tables)
{
    Console.WriteLine($"Table with {table.Cells.Count} rows");
    Console.WriteLine(table.Markdown);

    foreach (var row in table.Cells)
    {
        Console.WriteLine(string.Join(" | ", row));
    }
}
```

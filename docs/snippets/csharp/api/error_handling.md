```csharp title="C#"
using Xberg;

try
{
    var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("nonexistent.pdf"), null)).Results[0];
    Console.WriteLine(result.Content);
}
catch (XbergException ex)
{
    Console.WriteLine($"Error Code: {ex.Code}");
    Console.WriteLine($"Error Message: {ex.Message}");
}
catch (Exception ex)
{
    Console.WriteLine($"Unexpected error: {ex.Message}");
}
```

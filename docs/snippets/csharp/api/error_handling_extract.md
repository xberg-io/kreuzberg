```csharp title="C#"
using Xberg;

try
{
    var data = File.ReadAllBytes("document.unsupported");
    var input = ExtractInput.FromBytes(data, "application/x-custom", "document.unsupported");
    var output = await XbergConverter.ExtractAsync(input, ExtractionConfig.Default());
    var result = output.Results[0];
    Console.WriteLine(result.Content);
}
catch (XbergException ex) when (ex.Code == 1)
{
    Console.WriteLine("Validation error: Invalid MIME type");
}
catch (XbergException ex) when (ex.Code == 2)
{
    Console.WriteLine("Format error: MIME type not supported");
}
catch (XbergException ex)
{
    Console.WriteLine($"Extraction failed with error {ex.Code}: {ex.Message}");
}
```

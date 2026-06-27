using Xberg;

var data = File.ReadAllBytes("document.pdf");
var output = await XbergConverter.ExtractAsync(
    ExtractInput.FromBytes(data, "application/pdf", "document.pdf"),
    ExtractionConfig.Default()
);
var result = output.Results[0];

Console.WriteLine(result.Content);
Console.WriteLine($"Success: {result.Success}");
Console.WriteLine($"Content Length: {result.Content.Length}");

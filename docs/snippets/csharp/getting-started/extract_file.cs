using Xberg;

var result = await XbergClient.ExtractFileAsync("document.pdf");

Console.WriteLine(result.Content);
Console.WriteLine($"Tables: {result.Tables.Count}");
Console.WriteLine($"Metadata: {result.Metadata.FormatType}");

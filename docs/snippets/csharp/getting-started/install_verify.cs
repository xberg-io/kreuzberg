using Xberg;

var version = XbergClient.GetVersion();
Console.WriteLine($"Xberg version: {version}");

var result = XbergClient.ExtractFileSync("document.pdf");
Console.WriteLine($"Extraction successful: {result.Success}");

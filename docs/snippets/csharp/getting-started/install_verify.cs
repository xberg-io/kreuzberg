using Xberg;

var version = XbergClient.GetVersion();
Console.WriteLine($"Xberg version: {version}");

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), ExtractionConfig.Default())).Results[0];
Console.WriteLine($"Extraction successful: {result.Success}");

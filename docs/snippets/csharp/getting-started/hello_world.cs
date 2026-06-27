using Xberg;

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), ExtractionConfig.Default())).Results[0];

Console.WriteLine(result.Content);

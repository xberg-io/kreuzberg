using Xberg;

var config = new ExtractionConfig
{
    UseCache = true,
    EnableQualityProcessing = true
};

var result = XbergClient.ExtractFileSync("document.pdf", config);

Console.WriteLine(result.Content);
Console.WriteLine($"MIME Type: {result.MimeType}");

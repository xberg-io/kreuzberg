using Xberg;

var config = new ExtractionConfig
{
    ForceOcr = true,
    Ocr = new OcrConfig
    {
        Backend = "tesseract",
        Language = "eng"
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("scanned.pdf"), config)).Results[0];

Console.WriteLine(result.Content);
Console.WriteLine($"Detected Languages: {string.Join(", ", result.DetectedLanguages ?? new List<string>())}");

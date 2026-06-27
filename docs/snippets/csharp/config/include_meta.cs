using Xberg;

var config = new ExtractionConfig
{
    UseCache = true,
    Ocr = new OcrConfig
    {
        Backend = "tesseract",
        Language = "eng"
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];

if (result.Metadata != null)
{
    var language = result.Metadata.Language;
    var format = result.Metadata.FormatType;
}

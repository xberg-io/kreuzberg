using Xberg;

var config = new ExtractionConfig
{
    UseCache = true,
    EnableQualityProcessing = true
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri(
    new BytesWithMime(fileBytes), "application/pdf")).Results[0],
    config
);

var mimeType = result.MimeType;

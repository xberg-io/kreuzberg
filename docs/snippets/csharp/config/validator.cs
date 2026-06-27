using Xberg;

var config = new ExtractionConfig
{
    UseCache = true,
    EnableQualityProcessing = true
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];

if (!result.Success)
{
    if (result.Metadata?.Error != null)
    {
        var errorType = result.Metadata.Error.ErrorType;
        var errorMessage = result.Metadata.Error.Message;
    }
}

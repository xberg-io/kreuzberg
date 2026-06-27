using Xberg;

var config = new ExtractionConfig
{
    UseCache = true,
    EnableQualityProcessing = true,
    ExtractionTimeoutSecs = 30
};

var output = await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config);
var result = output.Results[0];

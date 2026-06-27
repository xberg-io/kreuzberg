using Xberg;

var config = new ExtractionConfig
{
    UseCache = false
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];

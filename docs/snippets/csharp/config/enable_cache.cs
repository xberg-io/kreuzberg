using Xberg;

var config = new ExtractionConfig
{
    UseCache = true
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];

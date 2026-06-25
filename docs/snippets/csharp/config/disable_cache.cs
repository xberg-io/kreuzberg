using Xberg;

var config = new ExtractionConfig
{
    UseCache = false
};

var result = XbergLib.ExtractFileSync("document.pdf", config);

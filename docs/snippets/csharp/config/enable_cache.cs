using Xberg;

var config = new ExtractionConfig
{
    UseCache = true
};

var result = XbergLib.ExtractFileSync("document.pdf", config);

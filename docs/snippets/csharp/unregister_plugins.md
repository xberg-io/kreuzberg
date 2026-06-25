```csharp title="C#"
using Xberg;

var names = new List<string>
{
    "custom-json-extractor",
    "word_count",
    "cloud-ocr",
    "min_length_validator"
};

XbergLib.UnregisterDocumentExtractor(names[0]);
XbergLib.UnregisterPostProcessor(names[1]);
XbergLib.UnregisterOcrBackend(names[2]);
XbergLib.UnregisterValidator(names[3]);
```

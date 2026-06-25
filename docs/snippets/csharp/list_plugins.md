```csharp title="C#"
using Xberg;

var extractors = XbergLib.ListDocumentExtractors();
var processors = XbergLib.ListPostProcessors();
var ocrBackends = XbergLib.ListOcrBackends();
var validators = XbergLib.ListValidators();

Console.WriteLine($"Extractors: {string.Join(", ", extractors)}");
Console.WriteLine($"Processors: {string.Join(", ", processors)}");
Console.WriteLine($"OCR backends: {string.Join(", ", ocrBackends)}");
Console.WriteLine($"Validators: {string.Join(", ", validators)}");
```

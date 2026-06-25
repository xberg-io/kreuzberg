```csharp title="C#"
using Xberg;

var extractors = XbergLib.ListDocumentExtractors();
Console.WriteLine("Registered extractors: " + string.Join(", ", extractors));

var ocrBackends = XbergLib.ListOcrBackends();
Console.WriteLine("Registered OCR backends: " + string.Join(", ", ocrBackends));

var processors = XbergLib.ListPostProcessors();
Console.WriteLine("Registered post-processors: " + string.Join(", ", processors));

var validators = XbergLib.ListValidators();
Console.WriteLine("Registered validators: " + string.Join(", ", validators));
```

```csharp title="C#"
using Xberg;

// Basic hierarchy configuration with properties
var config = new ExtractionConfig
{
    PdfOptions = new PdfConfig
    {
        ExtractImages = true,
        Hierarchy = new HierarchyConfig
        {
            Enabled = true,
            KClusters = 6,
            IncludeBbox = true,
            OcrCoverageThreshold = 0.8f
        }
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];
Console.WriteLine($"Content length: {result.Content.Length}");

// Advanced hierarchy detection with custom parameters
var advancedConfig = new ExtractionConfig
{
    PdfOptions = new PdfConfig
    {
        ExtractImages = true,
        Hierarchy = new HierarchyConfig
        {
            Enabled = true,
            KClusters = 12,           // More clusters for detailed hierarchy
            IncludeBbox = true,       // Include bounding box coordinates
            OcrCoverageThreshold = 0.7f  // Higher OCR threshold for stricter detection
        }
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("complex_document.pdf"), advancedConfig)).Results[0];
Console.WriteLine($"Advanced hierarchy detection completed: {result.Content.Length} chars");

// Minimal configuration with only enabled flag
var minimalConfig = new ExtractionConfig
{
    PdfOptions = new PdfConfig
    {
        Hierarchy = new HierarchyConfig
        {
            Enabled = true,
            // Other properties use defaults:
            // KClusters = 6
            // IncludeBbox = true
        }
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), minimalConfig)).Results[0];
Console.WriteLine("Extraction with default hierarchy settings complete");

// Disabling hierarchy detection
var noHierarchyConfig = new ExtractionConfig
{
    PdfOptions = new PdfConfig
    {
        Hierarchy = new HierarchyConfig
        {
            Enabled = false
        }
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), noHierarchyConfig)).Results[0];
Console.WriteLine("Extraction without hierarchy detection complete");
```

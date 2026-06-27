using Xberg;

class Program
{
    static async Task Main()
    {
        try
        {
            var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), ExtractionConfig.Default())).Results[0];
            Console.WriteLine($"Extracted {result.Content.Length} characters");
        }
        catch (XbergParsingException ex)
        {
            Console.WriteLine($"Failed to parse document: {ex.Message}");
        }
        catch (XbergOcrException ex)
        {
            Console.WriteLine($"OCR processing failed: {ex.Message}");
        }
        catch (XbergMissingDependencyException ex)
        {
            Console.WriteLine($"Missing dependency: {ex.Message}");
        }
        catch (XbergException ex)
        {
            Console.WriteLine($"Extraction error: {ex.Message}");
        }

        try
        {
            var config = new ExtractionConfig();
            var pdfBytes = new byte[] { 0x25, 0x50, 0x44, 0x46 }; 

            var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri(
                pdfBytes), "application/pdf",
                config
            )).Results[0];

            var preview = result.Content.Length > 100
                ? result.Content[..100] + "..."
                : result.Content;

            Console.WriteLine($"Extracted: {preview}");
        }
        catch (XbergValidationException ex)
        {
            Console.WriteLine($"Invalid configuration: {ex.Message}");
        }
        catch (XbergOcrException ex)
        {
            Console.WriteLine($"OCR failed: {ex.Message}");
        }
        catch (XbergException ex)
        {
            Console.WriteLine($"Extraction failed: {ex.Message}");
        }

        try
        {
            var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("nonexistent.pdf"), ExtractionConfig.Default())).Results[0];
        }
        catch (XbergIOException)
        {
            Console.WriteLine("File not found");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Unexpected error: {ex.Message}");
        }
    }
}

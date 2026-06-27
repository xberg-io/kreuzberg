using Xberg;

class Program
{
    static async Task Main()
    {
        try
        {
            var pdfBytes = await File.ReadAllBytesAsync("document.pdf");

            var output = await XbergConverter.ExtractAsync(
                ExtractInput.FromBytes(pdfBytes, "application/pdf", "document.pdf"),
                ExtractionConfig.Default()
            );
            var result = output.Results[0];

            Console.WriteLine($"Content: {result.Content}");
            Console.WriteLine($"MIME type: {result.MimeType}");

            var config = new ExtractionConfig
            {
                UseCache = true,
                EnableQualityProcessing = true
            };

            var output2 = await XbergConverter.ExtractAsync(
                ExtractInput.FromBytes(pdfBytes, "application/pdf", "document.pdf"),
                config
            );
            var result2 = output2.Results[0];

            Console.WriteLine($"Configured extraction: {result2.Content.Length} chars");

            var imageBytes = new byte[] {  };

            var imageOutput = await XbergConverter.ExtractAsync(
                ExtractInput.FromBytes(imageBytes, "image/jpeg", "image.jpg"),
                ExtractionConfig.Default()
            );
            var imageResult = imageOutput.Results[0];

            Console.WriteLine($"Image text: {imageResult.Content}");

            var multipleFiles = new Dictionary<string, (byte[], string)>
            {
                { "file1", (await File.ReadAllBytesAsync("file1.pdf"), "application/pdf") },
                { "file2", (await File.ReadAllBytesAsync("file2.pdf"), "application/pdf") }
            };

            foreach (var (name, (bytes, mimeType)) in multipleFiles)
            {
                var extractOutput = await XbergConverter.ExtractAsync(
                    ExtractInput.FromBytes(bytes, mimeType, name),
                    ExtractionConfig.Default()
                );
                var extractResult = extractOutput.Results[0];
                Console.WriteLine($"{name}: {extractResult.Content.Length} chars");
            }
        }
        catch (XbergException ex)
        {
            Console.WriteLine($"Extraction error: {ex.Message}");
        }
        catch (IOException ex)
        {
            Console.WriteLine($"File I/O error: {ex.Message}");
        }
    }
}

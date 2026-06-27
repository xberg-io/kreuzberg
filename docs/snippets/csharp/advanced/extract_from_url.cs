using Xberg;
using System.Net.Http;

class Program
{
    static async Task Main()
    {
        using var httpClient = new HttpClient();

        try
        {
            var url = "https://example.com/document.pdf";
            var documentBytes = await httpClient.GetByteArrayAsync(url);

            var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri(
                documentBytes), "application/pdf"
            )).Results[0];

            Console.WriteLine($"Extracted from URL: {result.Content.Length} chars");

            var config = new ExtractionConfig
            {
                EnableQualityProcessing = true
            };

            var result2 = (await XbergConverter.ExtractAsync(ExtractInput.FromUri(
                documentBytes), "application/pdf",
                config
            )).Results[0];

            Console.WriteLine($"Quality score: {result2.QualityScore}");

            var urls = new[]
            {
                "https://example.com/doc1.pdf",
                "https://example.com/doc2.pdf",
                "https://example.com/doc3.pdf"
            };

            var downloadTasks = urls.Select(async u =>
            {
                try
                {
                    var bytes = await httpClient.GetByteArrayAsync(u);
                    return (await XbergConverter.ExtractAsync(ExtractInput.FromUri(
                        bytes), "application/pdf"
                    )).Results[0];
                }
                catch (HttpRequestException ex)
                {
                    Console.WriteLine($"Download failed for {u}: {ex.Message}");
                    return null;
                }
            });

            var results = await Task.WhenAll(downloadTasks);

            var successCount = results.Count(r => r != null);
            Console.WriteLine($"Successfully processed {successCount} documents");
        }
        catch (HttpRequestException ex)
        {
            Console.WriteLine($"HTTP error: {ex.Message}");
        }
        catch (XbergException ex)
        {
            Console.WriteLine($"Extraction error: {ex.Message}");
        }
    }
}

using Xberg;

class Program
{
    static async Task Main()
    {
        var config = new ExtractionConfig
        {
            LanguageDetection = new LanguageDetectionConfig
            {
                Enabled = true,
                MinConfidence = 0.8m,
                DetectMultiple = true
            }
        };

        try
        {
            var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("multilingual_document.pdf"), config)).Results[0];

            var languages = result.DetectedLanguages ?? new List<string>();

            if (languages.Count > 0)
            {
                Console.WriteLine($"Detected {languages.Count} language(s): {string.Join(", ", languages)}");
            }
            else
            {
                Console.WriteLine("No languages detected");
            }

            Console.WriteLine($"Total content: {result.Content.Length} characters");
            Console.WriteLine($"MIME type: {result.MimeType}");
        }
        catch (XbergException ex)
        {
            Console.WriteLine($"Processing failed: {ex.Message}");
        }
    }
}

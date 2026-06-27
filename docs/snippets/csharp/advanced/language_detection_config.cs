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
                DetectMultiple = false
            }
        };

        try
        {
            var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];

            if (result.DetectedLanguages?.Count > 0)
            {
                Console.WriteLine($"Detected Language: {result.DetectedLanguages[0]}");
            }
            else
            {
                Console.WriteLine("No language detected");
            }

            Console.WriteLine($"Content length: {result.Content.Length} characters");
        }
        catch (XbergException ex)
        {
            Console.WriteLine($"Extraction failed: {ex.Message}");
        }
    }
}

using Xberg;

var config = new ExtractionConfig
{
    PdfOptions = new PdfConfig
    {
        ExtractMetadata = true
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];

if (result.Metadata?.Format.Pdf != null)
{
    var author = result.Metadata.Format.Pdf.Author;
    Console.WriteLine($"Author: {author}");
}

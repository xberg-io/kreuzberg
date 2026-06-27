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
    var title = result.Metadata.Format.Pdf.Title;
    Console.WriteLine($"Title: {title}");
}

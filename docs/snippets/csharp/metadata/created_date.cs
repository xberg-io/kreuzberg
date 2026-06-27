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
    var createdDate = result.Metadata.Format.Pdf.CreatedDate;
    if (createdDate.HasValue)
    {
        Console.WriteLine($"Created: {createdDate.Value:O}");
    }
}

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
    var pdfMeta = result.Metadata.Format.Pdf;
    Console.WriteLine($"Pages: {pdfMeta.PageCount}");
    Console.WriteLine($"Author: {pdfMeta.Author}");
    Console.WriteLine($"Title: {pdfMeta.Title}");
    Console.WriteLine($"Subject: {pdfMeta.Subject}");
    Console.WriteLine($"Created: {pdfMeta.CreatedDate:O}");
}

var htmlResult = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("page.html"), config)).Results[0];
if (htmlResult.Metadata?.Format.Html != null)
{
    var htmlMeta = htmlResult.Metadata.Format.Html;
    Console.WriteLine($"Title: {htmlMeta.Title}");
    Console.WriteLine($"Description: {htmlMeta.Description}");
    if (htmlMeta.OpenGraph != null && htmlMeta.OpenGraph.ContainsKey("image"))
        Console.WriteLine($"Open Graph Image: {htmlMeta.OpenGraph["image"]}");
}

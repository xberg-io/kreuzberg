using Xberg;

var config = new ExtractionConfig
{
    Pages = new PageConfig
    {
        ExtractPages = true
    }
};

var result = (await XbergConverter.ExtractAsync(ExtractInput.FromUri("document.pdf"), config)).Results[0];

if (result.Pages != null)
{
    foreach (var page in result.Pages)
    {
        Console.WriteLine($"Page {page.PageNumber}:");
        Console.WriteLine($"  Content: {page.Content.Length} chars");
        Console.WriteLine($"  Tables: {page.Tables.Count}");
        Console.WriteLine($"  Images: {page.Images.Count}");
    }
}

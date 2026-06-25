using Xberg;

var result = XbergClient.ExtractFileSync("document.pdf");

Console.WriteLine(result.Content);

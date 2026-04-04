using System;
using System.Text.Json;
using Kreuzberg;

var jsonTest = @"{
    ""format_type"": ""html"",
    ""title"": ""Test Page"",
    ""description"": ""Test Description"",
    ""keywords"": [""test"", ""keywords""],
    ""author"": ""Test Author"",
    ""canonical_url"": ""https://example.com"",
    ""open_graph"": {
        ""og:title"": ""Test"",
        ""og:description"": ""Test Description""
    },
    ""twitter_card"": {
        ""twitter:card"": ""summary""
    },
    ""headers"": [],
    ""links"": [],
    ""images"": [],
    ""structured_data"": []
}";

var metadata = JsonSerializer.Deserialize<Metadata>(jsonTest, new JsonSerializerOptions { PropertyNameCaseInsensitive = true });
Console.WriteLine($"HTML Metadata: {metadata?.Format?.Html}");

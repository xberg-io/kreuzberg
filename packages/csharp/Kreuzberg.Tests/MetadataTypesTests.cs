using System;
using System.Collections.Generic;
using System.Text.Json;
using Kreuzberg;
using Xunit;

namespace Kreuzberg.Tests;

/// <summary>
/// Comprehensive tests for C# metadata types, including HtmlMetadata, HeaderMetadata, LinkMetadata,
/// HtmlImageMetadata, and StructuredData. Tests verify type structure, JSON serialization, and
/// default values.
/// </summary>
public class MetadataTypesTests
{
    public MetadataTypesTests()
    {
        NativeTestHelper.EnsureNativeLibraryLoaded();

        // Clean up any registered callbacks from previous tests to prevent GCHandle accumulation
        try { KreuzbergClient.ClearPostProcessors(); } catch { }
        try { KreuzbergClient.ClearValidators(); } catch { }
        try { KreuzbergClient.ClearOcrBackends(); } catch { }
    }

    #region Type Structure Tests

    [Fact]
    public void HtmlMetadata_HasCorrectProperties()
    {
        var metadata = new HtmlMetadata();

        Assert.NotNull(metadata.Keywords);
        Assert.IsType<List<string>>(metadata.Keywords);
        Assert.Empty(metadata.Keywords);

        Assert.NotNull(metadata.OpenGraph);
        Assert.IsType<Dictionary<string, string>>(metadata.OpenGraph);
        Assert.Empty(metadata.OpenGraph);

        Assert.NotNull(metadata.TwitterCard);
        Assert.IsType<Dictionary<string, string>>(metadata.TwitterCard);
        Assert.Empty(metadata.TwitterCard);

        Assert.NotNull(metadata.MetaTags);
        Assert.IsType<Dictionary<string, string>>(metadata.MetaTags);
        Assert.Empty(metadata.MetaTags);

        Assert.NotNull(metadata.Headers);
        Assert.IsType<List<HeaderMetadata>>(metadata.Headers);
        Assert.Empty(metadata.Headers);

        Assert.NotNull(metadata.Links);
        Assert.IsType<List<LinkMetadata>>(metadata.Links);
        Assert.Empty(metadata.Links);

        Assert.NotNull(metadata.Images);
        Assert.IsType<List<HtmlImageMetadata>>(metadata.Images);
        Assert.Empty(metadata.Images);

        Assert.NotNull(metadata.StructuredData);
        Assert.IsType<List<StructuredData>>(metadata.StructuredData);
        Assert.Empty(metadata.StructuredData);

        Assert.Null(metadata.Title);
        Assert.Null(metadata.Description);
        Assert.Null(metadata.Author);
        Assert.Null(metadata.CanonicalUrl);
        Assert.Null(metadata.BaseHref);
        Assert.Null(metadata.Language);
        Assert.Null(metadata.TextDirection);
    }

    [Fact]
    public void Keywords_IsList_NotString()
    {
        var metadata = new HtmlMetadata();

        metadata.Keywords.Add("test");
        metadata.Keywords.Add("keywords");

        Assert.IsType<List<string>>(metadata.Keywords);
        Assert.Equal(2, metadata.Keywords.Count);
        Assert.Contains("test", metadata.Keywords);
        Assert.Contains("keywords", metadata.Keywords);
    }

    [Fact]
    public void CanonicalUrl_Renamed_PropertyExists()
    {
        var metadata = new HtmlMetadata();

        metadata.CanonicalUrl = "https://example.com/canonical";

        Assert.Equal("https://example.com/canonical", metadata.CanonicalUrl);
        Assert.NotNull(metadata.CanonicalUrl);
    }

    [Fact]
    public void OpenGraph_IsDictionary_StringToString()
    {
        var metadata = new HtmlMetadata();

        metadata.OpenGraph["og:title"] = "Test Title";
        metadata.OpenGraph["og:description"] = "Test Description";
        metadata.OpenGraph["og:image"] = "https://example.com/image.jpg";

        Assert.IsType<Dictionary<string, string>>(metadata.OpenGraph);
        Assert.Equal(3, metadata.OpenGraph.Count);
        Assert.Equal("Test Title", metadata.OpenGraph["og:title"]);
        Assert.Equal("Test Description", metadata.OpenGraph["og:description"]);
        Assert.Equal("https://example.com/image.jpg", metadata.OpenGraph["og:image"]);
    }

    [Fact]
    public void TwitterCard_IsDictionary_StringToString()
    {
        var metadata = new HtmlMetadata();

        metadata.TwitterCard["twitter:card"] = "summary_large_image";
        metadata.TwitterCard["twitter:title"] = "Test Title";
        metadata.TwitterCard["twitter:description"] = "Test Description";

        Assert.IsType<Dictionary<string, string>>(metadata.TwitterCard);
        Assert.Equal(3, metadata.TwitterCard.Count);
        Assert.Equal("summary_large_image", metadata.TwitterCard["twitter:card"]);
        Assert.Equal("Test Title", metadata.TwitterCard["twitter:title"]);
    }

    [Fact]
    public void HeaderMetadata_HasCorrectProperties()
    {
        var header = new HeaderMetadata
        {
            Level = 1,
            Text = "Main Title",
            Id = "main-title",
            Depth = 0,
            HtmlOffset = 100
        };

        Assert.Equal(1, header.Level);
        Assert.Equal("Main Title", header.Text);
        Assert.Equal("main-title", header.Id);
        Assert.Equal(0, header.Depth);
        Assert.Equal(100, header.HtmlOffset);
    }

    [Fact]
    public void LinkMetadata_HasCorrectProperties()
    {
        var link = new LinkMetadata
        {
            Href = "https://example.com",
            Text = "Example Link",
            Title = "Example Website",
            LinkType = "external",
            Rel = new List<string> { "nofollow", "external" }
        };

        Assert.Equal("https://example.com", link.Href);
        Assert.Equal("Example Link", link.Text);
        Assert.Equal("Example Website", link.Title);
        Assert.Equal("external", link.LinkType);
        Assert.Equal(2, link.Rel.Count);
        Assert.Contains("nofollow", link.Rel);
    }

    [Fact]
    public void LinkMetadata_Attributes_IsDictionary()
    {
        var link = new LinkMetadata { Href = "https://example.com" };

        link.Attributes["class"] = "external-link";
        link.Attributes["data-tracking"] = "123";

        Assert.IsType<Dictionary<string, string>>(link.Attributes);
        Assert.Equal(2, link.Attributes.Count);
        Assert.True(link.Attributes.ContainsKey("class") && link.Attributes["class"] == "external-link");
    }

    [Fact]
    public void HtmlImageMetadata_HasCorrectProperties()
    {
        var image = new HtmlImageMetadata
        {
            Src = "https://example.com/image.jpg",
            Alt = "Example image",
            Title = "Example",
            Dimensions = new[] { 800, 600 },
            ImageType = "external"
        };

        Assert.Equal("https://example.com/image.jpg", image.Src);
        Assert.Equal("Example image", image.Alt);
        Assert.Equal("Example", image.Title);
        Assert.NotNull(image.Dimensions);
        Assert.Equal(2, image.Dimensions.Length);
        Assert.Equal(800, image.Dimensions[0]);
        Assert.Equal(600, image.Dimensions[1]);
        Assert.Equal("external", image.ImageType);
    }

    [Fact]
    public void HtmlImageMetadata_Attributes_IsDictionary()
    {
        var image = new HtmlImageMetadata { Src = "image.jpg" };

        image.Attributes["loading"] = "lazy";
        image.Attributes["data-src"] = "image-hd.jpg";

        Assert.IsType<Dictionary<string, string>>(image.Attributes);
        Assert.Equal(2, image.Attributes.Count);
        Assert.True(image.Attributes.ContainsKey("loading") && image.Attributes["loading"] == "lazy");
    }

    [Fact]
    public void StructuredData_HasCorrectProperties()
    {
        var structuredData = new StructuredData
        {
            DataType = "json_ld",
            RawJson = @"{""@context"": ""https://schema.org"", ""@type"": ""Article""}",
            SchemaType = "Article"
        };

        Assert.Equal("json_ld", structuredData.DataType);
        Assert.NotEmpty(structuredData.RawJson);
        Assert.Equal("Article", structuredData.SchemaType);
    }

    #endregion

    #region JSON Serialization Tests

    [Fact]
    public void HtmlMetadata_SerializesCorrectly_WithJsonPropertyNames()
    {
        var metadata = new HtmlMetadata
        {
            Title = "Test Page",
            Description = "Test Description",
            Keywords = new List<string> { "test", "keywords" },
            Author = "Test Author",
            CanonicalUrl = "https://example.com",
            BaseHref = "https://example.com/",
            Language = "en",
            TextDirection = "ltr",
            OpenGraph = new Dictionary<string, string>
            {
                { "og:title", "Test" },
                { "og:description", "Test Description" }
            },
            TwitterCard = new Dictionary<string, string>
            {
                { "twitter:card", "summary" }
            }
        };

        var json = JsonSerializer.Serialize(metadata, new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower });

        Assert.NotEmpty(json);
        Assert.Contains("\"title\"", json);
        Assert.Contains("\"description\"", json);
        Assert.Contains("\"keywords\"", json);
        Assert.Contains("\"author\"", json);
        Assert.Contains("\"canonical_url\"", json);
        Assert.Contains("\"open_graph\"", json);
        Assert.Contains("\"twitter_card\"", json);
    }

    [Fact]
    public void HtmlMetadata_DeserializesCorrectly_FromJson()
    {
        var json = @"{
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

        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var metadata = JsonSerializer.Deserialize<HtmlMetadata>(json, options);

        Assert.NotNull(metadata);
        Assert.Equal("Test Page", metadata.Title);
        Assert.Equal("Test Description", metadata.Description);
        Assert.Equal(2, metadata.Keywords.Count);
        Assert.Contains("test", metadata.Keywords);
        Assert.Equal("Test Author", metadata.Author);
        Assert.Equal("https://example.com", metadata.CanonicalUrl);
        Assert.Equal(2, metadata.OpenGraph.Count);
        Assert.Equal("Test", metadata.OpenGraph["og:title"]);
        Assert.Single(metadata.TwitterCard);
        Assert.Equal("summary", metadata.TwitterCard["twitter:card"]);
    }

    [Fact]
    public void HeaderMetadata_JsonSerialization_RoundTrip()
    {
        var header = new HeaderMetadata
        {
            Level = 2,
            Text = "Subheading",
            Id = "subheading",
            Depth = 1,
            HtmlOffset = 250
        };

        var json = JsonSerializer.Serialize(header, new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower });
        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var deserialized = JsonSerializer.Deserialize<HeaderMetadata>(json, options);

        Assert.NotNull(deserialized);
        Assert.Equal(header.Level, deserialized.Level);
        Assert.Equal(header.Text, deserialized.Text);
        Assert.Equal(header.Id, deserialized.Id);
        Assert.Equal(header.Depth, deserialized.Depth);
        Assert.Equal(header.HtmlOffset, deserialized.HtmlOffset);
    }

    [Fact]
    public void LinkMetadata_JsonSerialization_RoundTrip()
    {
        var link = new LinkMetadata
        {
            Href = "https://example.com/page",
            Text = "Test Link",
            Title = "Test Page",
            LinkType = "internal",
            Rel = new List<string> { "canonical" },
            Attributes = new Dictionary<string, string> { { "class", "nav-link" } }
        };

        var json = JsonSerializer.Serialize(link, new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower });
        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var deserialized = JsonSerializer.Deserialize<LinkMetadata>(json, options);

        Assert.NotNull(deserialized);
        Assert.Equal(link.Href, deserialized.Href);
        Assert.Equal(link.Text, deserialized.Text);
        Assert.Equal(link.Title, deserialized.Title);
        Assert.Equal(link.LinkType, deserialized.LinkType);
        Assert.Single(deserialized.Rel);
        Assert.Equal("canonical", deserialized.Rel[0]);
        Assert.Single(deserialized.Attributes);
        Assert.True(deserialized.Attributes.ContainsKey("class") && deserialized.Attributes["class"] == "nav-link");
    }

    [Fact]
    public void HtmlImageMetadata_JsonSerialization_RoundTrip()
    {
        var image = new HtmlImageMetadata
        {
            Src = "images/photo.jpg",
            Alt = "Photo of example",
            Title = "Example Photo",
            Dimensions = new[] { 1920, 1080 },
            ImageType = "embedded",
            Attributes = new Dictionary<string, string> { { "srcset", "photo-small.jpg 800w" } }
        };

        var json = JsonSerializer.Serialize(image, new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower });
        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var deserialized = JsonSerializer.Deserialize<HtmlImageMetadata>(json, options);

        Assert.NotNull(deserialized);
        Assert.Equal(image.Src, deserialized.Src);
        Assert.Equal(image.Alt, deserialized.Alt);
        Assert.Equal(image.Title, deserialized.Title);
        Assert.NotNull(deserialized.Dimensions);
        Assert.Equal(1920, deserialized.Dimensions[0]);
        Assert.Equal(1080, deserialized.Dimensions[1]);
        Assert.Equal(image.ImageType, deserialized.ImageType);
        Assert.Single(deserialized.Attributes);
        Assert.True(deserialized.Attributes.ContainsKey("srcset") && deserialized.Attributes["srcset"] == "photo-small.jpg 800w");
    }

    [Fact]
    public void StructuredData_JsonSerialization_RoundTrip()
    {
        var structuredData = new StructuredData
        {
            DataType = "json_ld",
            RawJson = @"{""@context"":""https://schema.org"",""@type"":""NewsArticle"",""headline"":""Test""}",
            SchemaType = "NewsArticle"
        };

        var json = JsonSerializer.Serialize(structuredData, new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower });
        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var deserialized = JsonSerializer.Deserialize<StructuredData>(json, options);

        Assert.NotNull(deserialized);
        Assert.Equal(structuredData.DataType, deserialized.DataType);
        Assert.Equal(structuredData.RawJson, deserialized.RawJson);
        Assert.Equal(structuredData.SchemaType, deserialized.SchemaType);
    }

    [Fact]
    public void MetaTags_SerializeCorrectly()
    {
        var metadata = new HtmlMetadata();
        metadata.MetaTags["viewport"] = "width=device-width, initial-scale=1";
        metadata.MetaTags["charset"] = "utf-8";

        var json = JsonSerializer.Serialize(metadata, new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower });
        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var deserialized = JsonSerializer.Deserialize<HtmlMetadata>(json, options);

        Assert.NotNull(deserialized);
        Assert.NotNull(deserialized.MetaTags);
        Assert.Equal(2, deserialized.MetaTags.Count);
        Assert.Equal("width=device-width, initial-scale=1", deserialized.MetaTags["viewport"]);
        Assert.Equal("utf-8", deserialized.MetaTags["charset"]);
    }

    #endregion

    #region Default Values Tests

    [Fact]
    public void HtmlMetadata_DefaultConstructor_InitializesCollections()
    {
        var metadata = new HtmlMetadata();

        Assert.NotNull(metadata.Keywords);
        Assert.IsType<List<string>>(metadata.Keywords);
        Assert.Empty(metadata.Keywords);

        Assert.NotNull(metadata.OpenGraph);
        Assert.IsType<Dictionary<string, string>>(metadata.OpenGraph);
        Assert.Empty(metadata.OpenGraph);

        Assert.NotNull(metadata.TwitterCard);
        Assert.IsType<Dictionary<string, string>>(metadata.TwitterCard);
        Assert.Empty(metadata.TwitterCard);

        Assert.NotNull(metadata.MetaTags);
        Assert.IsType<Dictionary<string, string>>(metadata.MetaTags);
        Assert.Empty(metadata.MetaTags);

        Assert.NotNull(metadata.Headers);
        Assert.IsType<List<HeaderMetadata>>(metadata.Headers);
        Assert.Empty(metadata.Headers);

        Assert.NotNull(metadata.Links);
        Assert.IsType<List<LinkMetadata>>(metadata.Links);
        Assert.Empty(metadata.Links);

        Assert.NotNull(metadata.Images);
        Assert.IsType<List<HtmlImageMetadata>>(metadata.Images);
        Assert.Empty(metadata.Images);

        Assert.NotNull(metadata.StructuredData);
        Assert.IsType<List<StructuredData>>(metadata.StructuredData);
        Assert.Empty(metadata.StructuredData);
    }

    [Fact]
    public void HeaderMetadata_DefaultConstructor_InitializesDefaults()
    {
        var header = new HeaderMetadata();

        Assert.Equal(0, header.Level);
        Assert.Equal(string.Empty, header.Text);
        Assert.Null(header.Id);
        Assert.Equal(0, header.Depth);
        Assert.Equal(0, header.HtmlOffset);
    }

    [Fact]
    public void LinkMetadata_DefaultConstructor_InitializesDefaults()
    {
        var link = new LinkMetadata();

        Assert.Equal(string.Empty, link.Href);
        Assert.Equal(string.Empty, link.Text);
        Assert.Null(link.Title);
        Assert.Equal("other", link.LinkType);
        Assert.NotNull(link.Rel);
        Assert.Empty(link.Rel);
        Assert.NotNull(link.Attributes);
        Assert.Empty(link.Attributes);
    }

    [Fact]
    public void HtmlImageMetadata_DefaultConstructor_InitializesDefaults()
    {
        var image = new HtmlImageMetadata();

        Assert.Equal(string.Empty, image.Src);
        Assert.Null(image.Alt);
        Assert.Null(image.Title);
        Assert.Null(image.Dimensions);
        Assert.Equal("external", image.ImageType);
        Assert.NotNull(image.Attributes);
        Assert.Empty(image.Attributes);
    }

    [Fact]
    public void StructuredData_DefaultConstructor_InitializesDefaults()
    {
        var data = new StructuredData();

        Assert.Equal("json_ld", data.DataType);
        Assert.Equal(string.Empty, data.RawJson);
        Assert.Null(data.SchemaType);
    }

    [Fact]
    public void HtmlMetadata_OptionalFields_AreNullWhenMissing()
    {
        var json = @"{
            ""headers"": [],
            ""links"": [],
            ""images"": [],
            ""structured_data"": []
        }";

        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var metadata = JsonSerializer.Deserialize<HtmlMetadata>(json, options);

        Assert.NotNull(metadata);
        Assert.Null(metadata.Title);
        Assert.Null(metadata.Description);
        Assert.Null(metadata.Author);
        Assert.Null(metadata.CanonicalUrl);
        Assert.Null(metadata.BaseHref);
        Assert.Null(metadata.Language);
        Assert.Null(metadata.TextDirection);
        Assert.NotNull(metadata.Keywords);
        Assert.NotNull(metadata.OpenGraph);
        Assert.NotNull(metadata.TwitterCard);
        Assert.NotNull(metadata.Headers);
        Assert.NotNull(metadata.Links);
        Assert.NotNull(metadata.Images);
        Assert.NotNull(metadata.StructuredData);
    }

    [Fact]
    public void HtmlMetadata_EmptyCollections_AreNotNull()
    {
        var metadata = new HtmlMetadata();

        Assert.NotNull(metadata.Keywords);
        Assert.Empty(metadata.Keywords);

        Assert.NotNull(metadata.OpenGraph);
        Assert.Empty(metadata.OpenGraph);

        Assert.NotNull(metadata.TwitterCard);
        Assert.Empty(metadata.TwitterCard);

        Assert.NotNull(metadata.MetaTags);
        Assert.Empty(metadata.MetaTags);

        Assert.NotNull(metadata.Headers);
        Assert.Empty(metadata.Headers);

        Assert.NotNull(metadata.Links);
        Assert.Empty(metadata.Links);

        Assert.NotNull(metadata.Images);
        Assert.Empty(metadata.Images);

        Assert.NotNull(metadata.StructuredData);
        Assert.Empty(metadata.StructuredData);
    }

    #endregion

    #region Complex Type Tests

    [Fact]
    public void LinkMetadata_With_MultipleRelValues_PreservesAll()
    {
        var link = new LinkMetadata
        {
            Href = "https://example.com",
            Text = "Link",
            Rel = new List<string> { "nofollow", "external", "noopener" }
        };

        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var json = JsonSerializer.Serialize(link, options);
        var deserialized = JsonSerializer.Deserialize<LinkMetadata>(json, options);

        Assert.NotNull(deserialized);
        Assert.Equal(3, deserialized.Rel.Count);
        Assert.Contains("nofollow", deserialized.Rel);
        Assert.Contains("external", deserialized.Rel);
        Assert.Contains("noopener", deserialized.Rel);
    }

    [Fact]
    public void HtmlImageMetadata_With_ComplexAttributes_PreservesAll()
    {
        var image = new HtmlImageMetadata
        {
            Src = "image.jpg",
            Alt = "Test",
            Attributes = new Dictionary<string, string>
            {
                { "class", "responsive-image" },
                { "data-lazy", "true" },
                { "srcset", "image-small.jpg 480w, image-medium.jpg 1024w" },
                { "sizes", "(max-width: 600px) 100vw, 50vw" }
            }
        };

        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var json = JsonSerializer.Serialize(image, options);
        var deserialized = JsonSerializer.Deserialize<HtmlImageMetadata>(json, options);

        Assert.NotNull(deserialized);
        Assert.Equal(4, deserialized.Attributes.Count);
        Assert.True(deserialized.Attributes.ContainsKey("class") && deserialized.Attributes["class"] == "responsive-image");
        Assert.True(deserialized.Attributes.ContainsKey("data-lazy") && deserialized.Attributes["data-lazy"] == "true");
        Assert.True(deserialized.Attributes.ContainsKey("srcset") && deserialized.Attributes["srcset"] == "image-small.jpg 480w, image-medium.jpg 1024w");
        Assert.True(deserialized.Attributes.ContainsKey("sizes") && deserialized.Attributes["sizes"] == "(max-width: 600px) 100vw, 50vw");
    }

    [Fact]
    public void StructuredData_With_ComplexJson_PreservesRawJson()
    {
        var complexJson = @"{
            ""@context"": ""https://schema.org"",
            ""@type"": ""NewsArticle"",
            ""headline"": ""The Title of the Article"",
            ""image"": [
                ""https://example.com/photos/1x1/photo.jpg""
            ],
            ""datePublished"": ""2015-02-05T08:00:00+00:00"",
            ""author"": {
                ""@type"": ""Person"",
                ""name"": ""Jane Doe""
            }
        }";

        var data = new StructuredData
        {
            DataType = "json_ld",
            RawJson = complexJson,
            SchemaType = "NewsArticle"
        };

        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var json = JsonSerializer.Serialize(data, options);
        var deserialized = JsonSerializer.Deserialize<StructuredData>(json, options);

        Assert.NotNull(deserialized);
        Assert.Equal("json_ld", deserialized.DataType);
        Assert.Equal("NewsArticle", deserialized.SchemaType);
        Assert.Contains("NewsArticle", deserialized.RawJson);
        Assert.Contains("Jane Doe", deserialized.RawJson);
    }

    #endregion

    #region Edge Cases Tests

    [Fact]
    public void HtmlMetadata_With_SpecialCharactersInStrings_SerializesCorrectly()
    {
        var metadata = new HtmlMetadata
        {
            Title = "Test & \"Special\" <Characters>",
            Description = "Description with 'quotes' and \"double quotes\"",
            Author = "Author & Co."
        };

        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var json = JsonSerializer.Serialize(metadata, options);
        var deserialized = JsonSerializer.Deserialize<HtmlMetadata>(json, options);

        Assert.NotNull(deserialized);
        Assert.Equal(metadata.Title, deserialized.Title);
        Assert.Equal(metadata.Description, deserialized.Description);
        Assert.Equal(metadata.Author, deserialized.Author);
    }

    [Fact]
    public void HtmlImageMetadata_With_NullDimensions_HandlesCorrectly()
    {
        var image = new HtmlImageMetadata
        {
            Src = "image.jpg",
            Dimensions = null
        };

        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var json = JsonSerializer.Serialize(image, options);
        var deserialized = JsonSerializer.Deserialize<HtmlImageMetadata>(json, options);

        Assert.NotNull(deserialized);
        Assert.Null(deserialized.Dimensions!);
    }

    [Fact]
    public void HeaderMetadata_With_ZeroValues_SerializesCorrectly()
    {
        var header = new HeaderMetadata
        {
            Level = 0,
            Text = "Zero Header",
            Depth = 0,
            HtmlOffset = 0
        };

        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var json = JsonSerializer.Serialize(header, options);
        var deserialized = JsonSerializer.Deserialize<HeaderMetadata>(json, options);

        Assert.NotNull(deserialized);
        Assert.Equal(0, deserialized.Level);
        Assert.Equal(0, deserialized.Depth);
        Assert.Equal(0, deserialized.HtmlOffset);
    }

    [Fact]
    public void LinkMetadata_With_EmptyStringValues_SerializesCorrectly()
    {
        var link = new LinkMetadata
        {
            Href = "",
            Text = "",
            LinkType = ""
        };

        var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower };
        var json = JsonSerializer.Serialize(link, options);
        var deserialized = JsonSerializer.Deserialize<LinkMetadata>(json, options);

        Assert.NotNull(deserialized);
        Assert.Equal("", deserialized.Href);
        Assert.Equal("", deserialized.Text);
        Assert.Equal("", deserialized.LinkType);
    }

    #endregion
}

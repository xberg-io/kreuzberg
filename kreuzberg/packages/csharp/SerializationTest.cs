using System;
using System.Text.Json;
using Xunit;
using Kreuzberg;

namespace Kreuzberg.Tests
{
    /// <summary>
    /// Cross-language serialization tests for C# bindings.
    ///
    /// Validates that ExtractionConfig serializes consistently with other language bindings.
    /// </summary>
    public class SerializationTest
    {
        private readonly JsonSerializerOptions _jsonOptions = new()
        {
            PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
            WriteIndented = false
        };

        [Fact(DisplayName = "Should serialize minimal config to JSON")]
        public void TestMinimalSerialization()
        {
            var config = new ExtractionConfig();
            var json = JsonSerializer.Serialize(config, _jsonOptions);

            Assert.NotNull(json);
            Assert.Contains("useCache", json);
            Assert.Contains("enableQualityProcessing", json);
            Assert.Contains("forceOcr", json);
        }

        [Fact(DisplayName = "Should serialize config with custom values")]
        public void TestCustomValuesSerialization()
        {
            var config = new ExtractionConfig
            {
                UseCache = true,
                EnableQualityProcessing = false,
                ForceOcr = true
            };

            var json = JsonSerializer.Serialize(config, _jsonOptions);
            var restored = JsonSerializer.Deserialize<ExtractionConfig>(json, _jsonOptions);

            Assert.NotNull(restored);
            Assert.True(restored.UseCache);
            Assert.False(restored.EnableQualityProcessing);
            Assert.True(restored.ForceOcr);
        }

        [Fact(DisplayName = "Should preserve field values after serialization")]
        public void TestFieldPreservation()
        {
            var original = new ExtractionConfig
            {
                UseCache = false,
                EnableQualityProcessing = true
            };

            var json = JsonSerializer.Serialize(original, _jsonOptions);
            var restored = JsonSerializer.Deserialize<ExtractionConfig>(json, _jsonOptions);

            Assert.NotNull(restored);
            Assert.Equal(original.UseCache, restored.UseCache);
            Assert.Equal(original.EnableQualityProcessing, restored.EnableQualityProcessing);
        }

        [Fact(DisplayName = "Should handle round-trip serialization")]
        public void TestRoundTripSerialization()
        {
            var config1 = new ExtractionConfig
            {
                UseCache = true,
                EnableQualityProcessing = false
            };

            var json1 = JsonSerializer.Serialize(config1, _jsonOptions);
            var config2 = JsonSerializer.Deserialize<ExtractionConfig>(json1, _jsonOptions);
            var json2 = JsonSerializer.Serialize(config2, _jsonOptions);

            // Parse both JSONs and compare
            var element1 = JsonDocument.Parse(json1).RootElement;
            var element2 = JsonDocument.Parse(json2).RootElement;

            Assert.Equal(element1.GetRawText(), element2.GetRawText());
        }

        [Fact(DisplayName = "Should use camelCase field names")]
        public void TestCamelCaseFieldNames()
        {
            var config = new ExtractionConfig { UseCache = true };
            var json = JsonSerializer.Serialize(config, _jsonOptions);

            Assert.Contains("useCache", json);
            Assert.Contains("enableQualityProcessing", json);
            Assert.Contains("forceOcr", json);

            Assert.DoesNotContain("use_cache", json);
            Assert.DoesNotContain("enable_quality_processing", json);
            Assert.DoesNotContain("force_ocr", json);
        }

        [Fact(DisplayName = "Should serialize nested OCR config")]
        public void TestNestedOcrConfig()
        {
            var config = new ExtractionConfig
            {
                Ocr = new OcrConfig
                {
                    Backend = "tesseract",
                    Language = "eng"
                }
            };

            var json = JsonSerializer.Serialize(config, _jsonOptions);

            Assert.Contains("ocr", json.ToLower());
            Assert.Contains("tesseract", json);
            Assert.Contains("eng", json);
        }

        [Fact(DisplayName = "Should handle null values correctly")]
        public void TestNullValueHandling()
        {
            var config = new ExtractionConfig
            {
                Ocr = null,
                Chunking = null
            };

            var json = JsonSerializer.Serialize(config, _jsonOptions);
            var restored = JsonSerializer.Deserialize<ExtractionConfig>(json, _jsonOptions);

            Assert.NotNull(restored);
            Assert.Null(restored.Ocr);
            Assert.Null(restored.Chunking);
        }

        [Fact(DisplayName = "Should maintain immutability during serialization")]
        public void TestImmutabilityDuringSerialization()
        {
            var config = new ExtractionConfig { UseCache = true };

            var json1 = JsonSerializer.Serialize(config, _jsonOptions);
            var json2 = JsonSerializer.Serialize(config, _jsonOptions);
            var json3 = JsonSerializer.Serialize(config, _jsonOptions);

            Assert.Equal(json1, json2);
            Assert.Equal(json2, json3);
        }

        [Fact(DisplayName = "Should serialize all mandatory fields")]
        public void TestMandatoryFields()
        {
            var config = new ExtractionConfig();
            var json = JsonSerializer.Serialize(config, _jsonOptions);
            var element = JsonDocument.Parse(json).RootElement;

            Assert.True(element.TryGetProperty("useCache", out _), "useCache field is missing");
            Assert.True(element.TryGetProperty("enableQualityProcessing", out _), "enableQualityProcessing field is missing");
            Assert.True(element.TryGetProperty("forceOcr", out _), "forceOcr field is missing");
        }

        [Fact(DisplayName = "Should deserialize from JSON string")]
        public void TestDeserialization()
        {
            var json = """{"useCache":true,"enableQualityProcessing":false,"forceOcr":true}""";
            var config = JsonSerializer.Deserialize<ExtractionConfig>(json, _jsonOptions);

            Assert.NotNull(config);
            Assert.True(config.UseCache);
            Assert.False(config.EnableQualityProcessing);
            Assert.True(config.ForceOcr);
        }

        [Fact(DisplayName = "Should produce valid JSON")]
        public void TestValidJsonOutput()
        {
            var config = new ExtractionConfig { UseCache = true };
            var json = JsonSerializer.Serialize(config, _jsonOptions);

            // Should not throw
            var _ = JsonDocument.Parse(json);
            Assert.NotEmpty(json);
        }

        [Fact(DisplayName = "Should pretty-print JSON")]
        public void TestPrettyPrint()
        {
            var config = new ExtractionConfig { UseCache = true };
            var prettyOptions = new JsonSerializerOptions
            {
                PropertyNamingPolicy = JsonNamingPolicy.CamelCase,
                WriteIndented = true
            };

            var json = JsonSerializer.Serialize(config, prettyOptions);

            // Pretty JSON should have newlines
            Assert.Contains("\n", json);

            // Should still be valid JSON
            var _ = JsonDocument.Parse(json);
        }
    }
}

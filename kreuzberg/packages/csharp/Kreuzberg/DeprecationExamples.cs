using System;
using System.Collections.Generic;
using System.Text.Json.Serialization;
using System.Threading.Tasks;

namespace Kreuzberg;

/// <summary>
/// Legacy extraction API using deprecated boolean-based OCR configuration.
///
/// This class demonstrates how to mark deprecated methods in C#.
/// Users should migrate to the modern ExtractionConfig-based approach.
/// </summary>
public static class LegacyExtractionAPI
{
    /// <summary>
    /// Extract content with deprecated boolean OCR parameter.
    /// </summary>
    /// <remarks>
    /// This method uses the old boolean parameter pattern for OCR control.
    /// It will be removed in v2.0.0. Please use ExtractAsyncWithConfig() instead,
    /// which provides more flexible OCR configuration options.
    ///
    /// Migration example:
    ///
    /// OLD (deprecated):
    /// <c>
    /// await ExtractAsyncWithOcr(input, "application/pdf", enableOcr: true);
    /// </c>
    ///
    /// NEW (recommended):
    /// <c>
    /// var config = new ExtractionConfig
    /// {
    ///     Ocr = new OcrConfig { Backend = "tesseract", Language = "eng" }
    /// };
    /// await ExtractAsyncWithConfig(input, "application/pdf", config);
    /// </c>
    /// </remarks>
    [Obsolete(
        "Use ExtractAsyncWithConfig() with ExtractionConfig.Ocr instead. " +
        "This method will be removed in v2.0.0. " +
        "See migration guide: https://docs.kreuzberg.io/v1-to-v2-migration",
        error: false
    )]
    public static async Task<ExtractionResult> ExtractAsyncWithOcr(
        byte[] input,
        string mimeType,
        bool enableOcr = false
    )
    {
        var config = new ExtractionConfig
        {
            Ocr = enableOcr ? new OcrConfig { Backend = "tesseract" } : null
        };

        return await ExtractAsyncWithConfig(input, mimeType, config);
    }

    /// <summary>
    /// Extract content using the modern configuration-based approach.
    /// </summary>
    public static async Task<ExtractionResult> ExtractAsyncWithConfig(
        byte[] input,
        string mimeType,
        ExtractionConfig? config = null
    )
    {
        // Implementation delegated to KreuzbergClient
        return await KreuzbergClient.ExtractBytesAsync(input, mimeType, config);
    }
}

/// <summary>
/// Extension methods demonstrating deprecated patterns.
/// </summary>
public static class DeprecatedExtensions
{
    /// <summary>
    /// Enable quality processing using deprecated property approach.
    /// </summary>
    /// <remarks>
    /// Use the ExtractionConfig.EnableQualityProcessing field directly instead.
    /// This extension method will be removed in v2.0.0.
    /// </remarks>
    [Obsolete(
        "Set ExtractionConfig.EnableQualityProcessing directly instead. " +
        "This extension will be removed in v2.0.0.",
        error: false
    )]
    public static ExtractionConfig WithQualityProcessing(
        this ExtractionConfig config,
        bool enable
    )
    {
        return new ExtractionConfig
        {
            UseCache = config.UseCache,
            EnableQualityProcessing = enable,
            Ocr = config.Ocr,
            Chunking = config.Chunking,
            MaxConcurrentExtractions = config.MaxConcurrentExtractions,
            OutputFormat = config.OutputFormat,
            ResultFormat = config.ResultFormat
        };
    }

    /// <summary>
    /// Configure OCR using deprecated fluent API approach.
    /// </summary>
    [Obsolete(
        "Use the ExtractionConfig.Ocr property directly with ExtractionConfig with expression. " +
        "This extension will be removed in v2.0.0.",
        error: false
    )]
    public static ExtractionConfig WithOcrBackend(
        this ExtractionConfig config,
        string backend
    )
    {
        var ocr = config.Ocr ?? new OcrConfig();
        var newOcr = new OcrConfig
        {
            Backend = backend,
            Language = ocr.Language,
            TesseractConfig = ocr.TesseractConfig
        };
        return new ExtractionConfig
        {
            UseCache = config.UseCache,
            EnableQualityProcessing = config.EnableQualityProcessing,
            Ocr = newOcr,
            Chunking = config.Chunking,
            MaxConcurrentExtractions = config.MaxConcurrentExtractions,
            OutputFormat = config.OutputFormat,
            ResultFormat = config.ResultFormat
        };
    }
}

/// <summary>
/// Example class showing how to use [Obsolete] on properties and types.
/// </summary>
public sealed class DeprecatedConfigurationModel
{
    /// <summary>
    /// Deprecated: This property has been moved to ExtractionConfig.Ocr.Backend.
    /// </summary>
    [JsonPropertyName("ocr_backend")]
    [Obsolete("Use ExtractionConfig.Ocr.Backend instead", error: false)]
    public string? OcrBackend { get; init; }

    /// <summary>
    /// Deprecated: Use ExtractionConfig.Ocr configuration object instead.
    /// </summary>
    [JsonPropertyName("enable_ocr")]
    [Obsolete("Use ExtractionConfig.Ocr nested configuration instead", error: false)]
    public bool EnableOcr { get; init; }

    /// <summary>
    /// Deprecated: Language setting moved to OcrConfig.Language.
    /// </summary>
    [JsonPropertyName("ocr_language")]
    [Obsolete("Use ExtractionConfig.Ocr.Language instead", error: false)]
    public string? OcrLanguage { get; init; }
}

/// <summary>
/// Demonstrates validation logic that might generate deprecation warnings.
/// </summary>
public static class DeprecatedValidationLogic
{
    /// <summary>
    /// Validate using old-style boolean configuration.
    /// </summary>
    /// <remarks>
    /// This method validates a deprecated configuration format.
    /// Users should validate ExtractionConfig.Ocr field directly instead.
    /// </remarks>
    [Obsolete(
        "This validation logic is deprecated. Validate ExtractionConfig directly. " +
        "Removed in v2.0.0.",
        error: false
    )]
    public static bool IsOcrEnabledDeprecated(bool enableOcr)
    {
        return enableOcr;
    }

    /// <summary>
    /// Modern validation using configuration objects.
    /// </summary>
    public static bool IsOcrConfigured(ExtractionConfig? config)
    {
        return config?.Ocr != null;
    }
}

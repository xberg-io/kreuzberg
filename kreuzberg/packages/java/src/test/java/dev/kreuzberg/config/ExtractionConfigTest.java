package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive ExtractionConfig tests.
 *
 * <p>
 * Tests for the main extraction configuration that integrates all
 * sub-configurations including OCR, chunking, language detection, PDF options,
 * image extraction, preprocessing, post-processing, token reduction, HTML
 * options, keywords, and pages.
 */
@DisplayName("ExtractionConfig Tests")
final class ExtractionConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		ExtractionConfig config = ExtractionConfig.builder().build();

		assertThat(config.isUseCache()).isTrue();
		assertThat(config.isEnableQualityProcessing()).isTrue();
		assertThat(config.isForceOcr()).isFalse();
		assertNull(config.getOutputFormat());
		assertNull(config.getResultFormat());
		assertNull(config.getOcr());
		assertNull(config.getChunking());
		assertNull(config.getLanguageDetection());
		assertNull(config.getPdfOptions());
		assertNull(config.getImageExtraction());
		assertNull(config.getPostprocessor());
		assertNull(config.getTokenReduction());
		assertNull(config.getPages());
	}

	@Test
	@DisplayName("should disable use cache")
	void shouldDisableUseCache() {
		ExtractionConfig config = ExtractionConfig.builder().useCache(false).build();

		assertThat(config.isUseCache()).isFalse();
	}

	@Test
	@DisplayName("should enable quality processing")
	void shouldEnableQualityProcessing() {
		ExtractionConfig config = ExtractionConfig.builder().enableQualityProcessing(true).build();

		assertThat(config.isEnableQualityProcessing()).isTrue();
	}

	@Test
	@DisplayName("should enable force OCR")
	void shouldEnableForceOcr() {
		ExtractionConfig config = ExtractionConfig.builder().forceOcr(true).build();

		assertThat(config.isForceOcr()).isTrue();
	}

	@Test
	@DisplayName("should set output format to markdown")
	void shouldSetOutputFormatMarkdown() {
		ExtractionConfig config = ExtractionConfig.builder().outputFormat("markdown").build();

		assertThat(config.getOutputFormat()).isEqualTo("markdown");
	}

	@Test
	@DisplayName("should set output format to djot")
	void shouldSetOutputFormatDjot() {
		ExtractionConfig config = ExtractionConfig.builder().outputFormat("djot").build();

		assertThat(config.getOutputFormat()).isEqualTo("djot");
	}

	@Test
	@DisplayName("should set output format to html")
	void shouldSetOutputFormatHtml() {
		ExtractionConfig config = ExtractionConfig.builder().outputFormat("html").build();

		assertThat(config.getOutputFormat()).isEqualTo("html");
	}

	@Test
	@DisplayName("should set output format to plain")
	void shouldSetOutputFormatPlain() {
		ExtractionConfig config = ExtractionConfig.builder().outputFormat("plain").build();

		assertThat(config.getOutputFormat()).isEqualTo("plain");
	}

	@Test
	@DisplayName("should set result format to element_based")
	void shouldSetResultFormatElementBased() {
		ExtractionConfig config = ExtractionConfig.builder().resultFormat("element_based").build();

		assertThat(config.getResultFormat()).isEqualTo("element_based");
	}

	@Test
	@DisplayName("should set result format to unified")
	void shouldSetResultFormatUnified() {
		ExtractionConfig config = ExtractionConfig.builder().resultFormat("unified").build();

		assertThat(config.getResultFormat()).isEqualTo("unified");
	}

	@Test
	@DisplayName("should set both output and result formats")
	void shouldSetBothFormats() {
		ExtractionConfig config = ExtractionConfig.builder().outputFormat("markdown").resultFormat("element_based")
				.build();

		assertThat(config.getOutputFormat()).isEqualTo("markdown");
		assertThat(config.getResultFormat()).isEqualTo("element_based");
	}

	@Test
	@DisplayName("should include output_format in toMap")
	void shouldIncludeOutputFormatInMap() {
		ExtractionConfig config = ExtractionConfig.builder().outputFormat("markdown").build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("output_format", "markdown");
	}

	@Test
	@DisplayName("should include result_format in toMap")
	void shouldIncludeResultFormatInMap() {
		ExtractionConfig config = ExtractionConfig.builder().resultFormat("element_based").build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("result_format", "element_based");
	}

	@Test
	@DisplayName("should not include null output format in toMap")
	void shouldNotIncludeNullOutputFormatInMap() {
		ExtractionConfig config = ExtractionConfig.builder().build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).doesNotContainKey("output_format");
	}

	@Test
	@DisplayName("should not include null result format in toMap")
	void shouldNotIncludeNullResultFormatInMap() {
		ExtractionConfig config = ExtractionConfig.builder().build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).doesNotContainKey("result_format");
	}

	@Test
	@DisplayName("should set OCR config")
	void shouldSetOcrConfig() {
		OcrConfig ocrConfig = OcrConfig.builder().backend("tesseract").build();
		ExtractionConfig config = ExtractionConfig.builder().ocr(ocrConfig).build();

		assertNotNull(config.getOcr());
		assertThat(config.getOcr().getBackend()).isEqualTo("tesseract");
	}

	@Test
	@DisplayName("should set chunking config")
	void shouldSetChunkingConfig() {
		ChunkingConfig chunkingConfig = ChunkingConfig.builder().maxChars(2000).build();
		ExtractionConfig config = ExtractionConfig.builder().chunking(chunkingConfig).build();

		assertNotNull(config.getChunking());
		assertThat(config.getChunking().getMaxChars()).isEqualTo(2000);
	}

	@Test
	@DisplayName("should set language detection config")
	void shouldSetLanguageDetectionConfig() {
		LanguageDetectionConfig langConfig = LanguageDetectionConfig.builder().enabled(true).build();
		ExtractionConfig config = ExtractionConfig.builder().languageDetection(langConfig).build();

		assertNotNull(config.getLanguageDetection());
		assertThat(config.getLanguageDetection().isEnabled()).isTrue();
	}

	@Test
	@DisplayName("should set PDF options config")
	void shouldSetPdfOptionsConfig() {
		PdfConfig pdfConfig = PdfConfig.builder().extractImages(true).build();
		ExtractionConfig config = ExtractionConfig.builder().pdfOptions(pdfConfig).build();

		assertNotNull(config.getPdfOptions());
		assertThat(config.getPdfOptions().isExtractImages()).isTrue();
	}

	@Test
	@DisplayName("should set image extraction config")
	void shouldSetImageExtractionConfig() {
		ImageExtractionConfig imageExtConfig = ImageExtractionConfig.builder().targetDpi(600).build();
		ExtractionConfig config = ExtractionConfig.builder().imageExtraction(imageExtConfig).build();

		assertNotNull(config.getImageExtraction());
		assertThat(config.getImageExtraction().getTargetDpi()).isEqualTo(600);
	}

	@Test
	@DisplayName("should set post-processor config")
	void shouldSetPostProcessorConfig() {
		PostProcessorConfig postConfig = PostProcessorConfig.builder().enabled(true).build();
		ExtractionConfig config = ExtractionConfig.builder().postprocessor(postConfig).build();

		assertNotNull(config.getPostprocessor());
		assertThat(config.getPostprocessor().isEnabled()).isTrue();
	}

	@Test
	@DisplayName("should set token reduction config")
	void shouldSetTokenReductionConfig() {
		TokenReductionConfig tokenConfig = TokenReductionConfig.builder().mode("moderate").build();
		ExtractionConfig config = ExtractionConfig.builder().tokenReduction(tokenConfig).build();

		assertNotNull(config.getTokenReduction());
		assertThat(config.getTokenReduction().getMode()).isEqualTo("moderate");
	}

	@Test
	@DisplayName("should set keyword config")
	void shouldSetKeywordConfig() {
		KeywordConfig keywordConfig = KeywordConfig.builder().algorithm("yake").build();
		ExtractionConfig config = ExtractionConfig.builder().keywords(keywordConfig).build();

		assertNotNull(config.getKeywords());
		assertThat(config.getKeywords().toMap().get("algorithm")).isEqualTo("yake");
	}

	@Test
	@DisplayName("should set page config")
	void shouldSetPageConfig() {
		PageConfig pageConfig = PageConfig.builder().extractPages(true).build();
		ExtractionConfig config = ExtractionConfig.builder().pages(pageConfig).build();

		assertNotNull(config.getPages());
		assertThat(config.getPages().isExtractPages()).isTrue();
	}

	@Test
	@DisplayName("should set max concurrent extractions")
	void shouldSetMaxConcurrentExtractions() {
		ExtractionConfig config = ExtractionConfig.builder().maxConcurrentExtractions(4).build();

		assertThat(config.getMaxConcurrentExtractions()).isEqualTo(4);
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		OcrConfig ocrConfig = OcrConfig.builder().backend("tesseract").build();
		ChunkingConfig chunkingConfig = ChunkingConfig.builder().maxChars(2000).build();
		LanguageDetectionConfig langConfig = LanguageDetectionConfig.builder().enabled(true).build();
		PdfConfig pdfConfig = PdfConfig.builder().extractImages(true).build();
		ImageExtractionConfig imageExtConfig = ImageExtractionConfig.builder().targetDpi(600).build();
		PostProcessorConfig postConfig = PostProcessorConfig.builder().enabled(true).build();
		TokenReductionConfig tokenConfig = TokenReductionConfig.builder().mode("moderate").build();
		PageConfig pageConfig = PageConfig.builder().extractPages(true).build();

		ExtractionConfig config = ExtractionConfig.builder().useCache(true).enableQualityProcessing(true).forceOcr(true)
				.outputFormat("markdown").resultFormat("element_based").ocr(ocrConfig).chunking(chunkingConfig)
				.languageDetection(langConfig).pdfOptions(pdfConfig).imageExtraction(imageExtConfig)
				.postprocessor(postConfig).tokenReduction(tokenConfig).pages(pageConfig).maxConcurrentExtractions(4)
				.build();

		assertThat(config.isUseCache()).isTrue();
		assertThat(config.isEnableQualityProcessing()).isTrue();
		assertThat(config.isForceOcr()).isTrue();
		assertThat(config.getOutputFormat()).isEqualTo("markdown");
		assertThat(config.getResultFormat()).isEqualTo("element_based");
		assertNotNull(config.getOcr());
		assertNotNull(config.getChunking());
		assertNotNull(config.getLanguageDetection());
		assertNotNull(config.getPdfOptions());
		assertNotNull(config.getImageExtraction());
		assertNotNull(config.getPostprocessor());
		assertNotNull(config.getTokenReduction());
		assertNotNull(config.getPages());
		assertThat(config.getMaxConcurrentExtractions()).isEqualTo(4);
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		OcrConfig ocrConfig = OcrConfig.builder().backend("tesseract").build();
		ExtractionConfig config = ExtractionConfig.builder().useCache(true).forceOcr(false).ocr(ocrConfig)
				.outputFormat("html").resultFormat("unified").build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsKey("use_cache").containsKey("force_ocr").containsKey("output_format")
				.containsKey("result_format");
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		OcrConfig ocrConfig = OcrConfig.builder().backend("tesseract").build();
		ExtractionConfig config = ExtractionConfig.builder().useCache(false).enableQualityProcessing(true)
				.forceOcr(true).outputFormat("djot").resultFormat("element_based").ocr(ocrConfig).build();

		assertThat(config.isUseCache()).isFalse();
		assertThat(config.isEnableQualityProcessing()).isTrue();
		assertThat(config.isForceOcr()).isTrue();
		assertThat(config.getOutputFormat()).isEqualTo("djot");
		assertThat(config.getResultFormat()).isEqualTo("element_based");
		assertNotNull(config.getOcr());
	}

	@Test
	@DisplayName("should handle nested configuration objects")
	void shouldHandleNestedConfigs() {
		OcrConfig ocrConfig = OcrConfig.builder().backend("tesseract").build();
		PdfConfig pdfConfig = PdfConfig.builder().extractImages(true).build();
		ExtractionConfig config = ExtractionConfig.builder().ocr(ocrConfig).pdfOptions(pdfConfig).build();

		assertNotNull(config.getOcr());
		assertNotNull(config.getPdfOptions());
		assertThat(config.getOcr().getBackend()).isEqualTo("tesseract");
		assertThat(config.getPdfOptions().isExtractImages()).isTrue();
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		ExtractionConfig config1 = ExtractionConfig.builder().useCache(true).outputFormat("markdown").build();
		ExtractionConfig config2 = ExtractionConfig.builder().useCache(false).outputFormat("html").build();

		assertThat(config1.isUseCache()).isNotEqualTo(config2.isUseCache());
		assertThat(config1.getOutputFormat()).isNotEqualTo(config2.getOutputFormat());
	}

	@Test
	@DisplayName("should handle optional sub-configurations")
	void shouldHandleOptionalConfigs() {
		ExtractionConfig config = ExtractionConfig.builder().build();

		assertNull(config.getOcr());
		assertNull(config.getChunking());
		assertNull(config.getLanguageDetection());
		assertNull(config.getPdfOptions());
		assertNull(config.getImageExtraction());
		assertNull(config.getPostprocessor());
		assertNull(config.getTokenReduction());
		assertNull(config.getKeywords());
		assertNull(config.getPages());
	}

	@Test
	@DisplayName("should set output format independently")
	void shouldSetOutputFormatIndependently() {
		ExtractionConfig config = ExtractionConfig.builder().outputFormat("markdown").build();

		assertThat(config.getOutputFormat()).isEqualTo("markdown");
		assertNull(config.getResultFormat());
	}

	@Test
	@DisplayName("should set result format independently")
	void shouldSetResultFormatIndependently() {
		ExtractionConfig config = ExtractionConfig.builder().resultFormat("element_based").build();

		assertNull(config.getOutputFormat());
		assertThat(config.getResultFormat()).isEqualTo("element_based");
	}
}

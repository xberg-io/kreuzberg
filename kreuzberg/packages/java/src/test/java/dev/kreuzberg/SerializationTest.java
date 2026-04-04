package dev.kreuzberg;

import static org.junit.jupiter.api.Assertions.*;

import com.fasterxml.jackson.databind.ObjectMapper;
import dev.kreuzberg.config.ExtractionConfig;
import dev.kreuzberg.config.OcrConfig;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Cross-language serialization tests for Java bindings.
 *
 * Validates that ExtractionConfig serializes consistently with other language
 * bindings.
 */
@DisplayName("ExtractionConfig Serialization")
class SerializationTest {

	private ObjectMapper objectMapper;

	@BeforeEach
	void setUp() {
		objectMapper = new ObjectMapper();
	}

	@Test
	@DisplayName("should serialize minimal config to JSON")
	void testMinimalSerialization() throws Exception {
		ExtractionConfig config = ExtractionConfig.builder().build();
		String json = config.toJson();

		assertNotNull(json);
		assertTrue(json.contains("use_cache"));
		assertTrue(json.contains("enable_quality_processing"));
		assertTrue(json.contains("force_ocr"));
	}

	@Test
	@DisplayName("should serialize config with custom values")
	void testCustomValuesSerialization() throws Exception {
		ExtractionConfig config = ExtractionConfig.builder().useCache(true).enableQualityProcessing(false)
				.forceOcr(true).build();

		String json = config.toJson();

		ExtractionConfig restored = ExtractionConfig.fromJson(json);

		assertEquals(true, restored.isUseCache());
		assertEquals(false, restored.isEnableQualityProcessing());
		assertEquals(true, restored.isForceOcr());
	}

	@Test
	@DisplayName("should preserve field values after serialization")
	void testFieldPreservation() throws Exception {
		ExtractionConfig original = ExtractionConfig.builder().useCache(false).enableQualityProcessing(true).build();

		String json = original.toJson();
		ExtractionConfig restored = ExtractionConfig.fromJson(json);

		assertEquals(original.isUseCache(), restored.isUseCache());
		assertEquals(original.isEnableQualityProcessing(), restored.isEnableQualityProcessing());
	}

	@Test
	@DisplayName("should handle round-trip serialization")
	void testRoundTripSerialization() throws Exception {
		ExtractionConfig config1 = ExtractionConfig.builder().useCache(true).enableQualityProcessing(false).build();

		String json1 = config1.toJson();
		ExtractionConfig config2 = ExtractionConfig.fromJson(json1);
		String json2 = config2.toJson();

		// Should produce equivalent JSON
		ExtractionConfig parsed1 = ExtractionConfig.fromJson(json1);
		ExtractionConfig parsed2 = ExtractionConfig.fromJson(json2);

		assertEquals(parsed1.isUseCache(), parsed2.isUseCache());
		assertEquals(parsed1.isEnableQualityProcessing(), parsed2.isEnableQualityProcessing());
		assertEquals(parsed1.isForceOcr(), parsed2.isForceOcr());
	}

	@Test
	@DisplayName("should use snake_case field names in JSON")
	void testSnakeCaseFieldNames() throws Exception {
		ExtractionConfig config = ExtractionConfig.builder().build();
		String json = config.toJson();

		assertTrue(json.contains("use_cache"));
		assertTrue(json.contains("enable_quality_processing"));
		assertTrue(json.contains("force_ocr"));
	}

	@Test
	@DisplayName("should serialize nested OCR config")
	void testNestedOcrConfig() throws Exception {
		OcrConfig ocrConfig = OcrConfig.builder().backend("tesseract").language("eng").build();

		ExtractionConfig config = ExtractionConfig.builder().ocr(ocrConfig).build();

		String json = config.toJson();

		assertTrue(json.contains("\"ocr\""));
		assertTrue(json.contains("\"backend\""));
		assertTrue(json.contains("\"tesseract\""));
		assertTrue(json.contains("\"language\""));
		assertTrue(json.contains("\"eng\""));
	}

	@Test
	@DisplayName("should handle null values correctly")
	void testNullValueHandling() throws Exception {
		ExtractionConfig config = ExtractionConfig.builder().build();

		String json = config.toJson();
		ExtractionConfig restored = ExtractionConfig.fromJson(json);

		assertNull(restored.getOcr());
		assertNull(restored.getChunking());
	}

	@Test
	@DisplayName("should maintain stability during serialization")
	void testStabilityDuringSerialization() throws Exception {
		ExtractionConfig config = ExtractionConfig.builder().useCache(true).build();

		String json1 = config.toJson();
		String json2 = config.toJson();
		String json3 = config.toJson();

		assertEquals(json1, json2);
		assertEquals(json2, json3);
	}

	@Test
	@DisplayName("should serialize all mandatory fields")
	void testMandatoryFields() throws Exception {
		ExtractionConfig config = ExtractionConfig.builder().build();
		String json = config.toJson();

		ExtractionConfig parsed = ExtractionConfig.fromJson(json);

		assertNotNull(parsed.isUseCache());
		assertNotNull(parsed.isEnableQualityProcessing());
		assertNotNull(parsed.isForceOcr());
	}

	@Test
	@DisplayName("should deserialize from JSON string")
	void testDeserialization() throws Exception {
		String json = "{\"use_cache\":true,\"enable_quality_processing\":false,\"force_ocr\":true}";

		ExtractionConfig config = ExtractionConfig.fromJson(json);

		assertEquals(true, config.isUseCache());
		assertEquals(false, config.isEnableQualityProcessing());
		assertEquals(true, config.isForceOcr());
	}
}

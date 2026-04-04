package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive PageConfig tests.
 *
 * <p>
 * Tests for page configuration including page extraction, page marker
 * insertion, and marker format customization.
 */
@DisplayName("PageConfig Tests")
final class PageConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		PageConfig config = PageConfig.builder().build();

		assertThat(config.isExtractPages()).isFalse();
		assertThat(config.isInsertPageMarkers()).isFalse();
		assertThat(config.getMarkerFormat()).isNotNull();
	}

	@Test
	@DisplayName("should set extract pages flag")
	void shouldSetExtractPages() {
		PageConfig config = PageConfig.builder().extractPages(true).build();

		assertThat(config.isExtractPages()).isTrue();
	}

	@Test
	@DisplayName("should set insert page markers flag")
	void shouldSetInsertPageMarkers() {
		PageConfig config = PageConfig.builder().insertPageMarkers(true).build();

		assertThat(config.isInsertPageMarkers()).isTrue();
	}

	@Test
	@DisplayName("should set custom marker format")
	void shouldSetCustomMarkerFormat() {
		String customFormat = "\n--- Page {page_num} ---\n";
		PageConfig config = PageConfig.builder().markerFormat(customFormat).build();

		assertThat(config.getMarkerFormat()).isEqualTo(customFormat);
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		String customFormat = "\n=== Page {page_num} ===\n";
		PageConfig config = PageConfig.builder().extractPages(true).insertPageMarkers(true).markerFormat(customFormat)
				.build();

		assertThat(config.isExtractPages()).isTrue();
		assertThat(config.isInsertPageMarkers()).isTrue();
		assertThat(config.getMarkerFormat()).isEqualTo(customFormat);
	}

	@Test
	@DisplayName("should support marker format placeholder")
	void shouldSupportMarkerFormatPlaceholder() {
		String format = "\nPage Number: {page_num}\n";
		PageConfig config = PageConfig.builder().markerFormat(format).build();

		assertThat(config.getMarkerFormat()).contains("{page_num}");
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		PageConfig config = PageConfig.builder().extractPages(true).insertPageMarkers(true).build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("extract_pages", true).containsEntry("insert_page_markers", true)
				.containsKey("marker_format");
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		String format = "\n<!-- Page {page_num} -->\n";
		PageConfig config = PageConfig.builder().extractPages(true).insertPageMarkers(true).markerFormat(format)
				.build();

		assertThat(config.isExtractPages()).isTrue();
		assertThat(config.isInsertPageMarkers()).isTrue();
		assertThat(config.getMarkerFormat()).isEqualTo(format);
	}

	@Test
	@DisplayName("should use default marker format when null provided")
	void shouldUseDefaultMarkerFormat() {
		PageConfig config = PageConfig.builder().markerFormat(null).build();

		assertThat(config.getMarkerFormat()).isNotNull();
	}

	@Test
	@DisplayName("should test equality of configs")
	void shouldTestEquality() {
		PageConfig config1 = PageConfig.builder().extractPages(true).insertPageMarkers(true).build();
		PageConfig config2 = PageConfig.builder().extractPages(true).insertPageMarkers(true).build();

		assertThat(config1).isEqualTo(config2);
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		PageConfig config1 = PageConfig.builder().extractPages(true).build();
		PageConfig config2 = PageConfig.builder().extractPages(false).build();

		assertThat(config1.isExtractPages()).isNotEqualTo(config2.isExtractPages());
	}

	@Test
	@DisplayName("should support both extract and marker insertion")
	void shouldSupportBothFeatures() {
		PageConfig config = PageConfig.builder().extractPages(true).insertPageMarkers(true).build();

		assertThat(config.isExtractPages()).isTrue();
		assertThat(config.isInsertPageMarkers()).isTrue();
	}
}

package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import java.util.Arrays;
import java.util.List;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive PdfConfig tests.
 *
 * <p>
 * Tests for PDF-specific extraction options including image extraction,
 * password handling, metadata extraction, font configuration, and hierarchy
 * detection.
 */
@DisplayName("PdfConfig Tests")
final class PdfConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		PdfConfig config = PdfConfig.builder().build();

		assertThat(config.isExtractImages()).isFalse();
		assertNull(config.getPasswords());
		assertThat(config.isExtractMetadata()).isTrue();
		assertNull(config.getFontConfig());
		assertNull(config.getHierarchyConfig());
	}

	@Test
	@DisplayName("should enable image extraction")
	void shouldEnableImageExtraction() {
		PdfConfig config = PdfConfig.builder().extractImages(true).build();

		assertThat(config.isExtractImages()).isTrue();
	}

	@Test
	@DisplayName("should set single password")
	void shouldSetSinglePassword() {
		PdfConfig config = PdfConfig.builder().password("secret").build();

		assertThat(config.getPasswords()).isNotNull().containsExactly("secret");
	}

	@Test
	@DisplayName("should set multiple passwords")
	void shouldSetMultiplePasswords() {
		List<String> passwords = Arrays.asList("pass1", "pass2", "pass3");
		PdfConfig config = PdfConfig.builder().passwords(passwords).build();

		assertThat(config.getPasswords()).containsExactlyElementsOf(passwords);
	}

	@Test
	@DisplayName("should disable metadata extraction")
	void shouldDisableMetadataExtraction() {
		PdfConfig config = PdfConfig.builder().extractMetadata(false).build();

		assertThat(config.isExtractMetadata()).isFalse();
	}

	@Test
	@DisplayName("should set FontConfig")
	void shouldSetFontConfig() {
		FontConfig fontConfig = FontConfig.builder().enabled(true).build();
		PdfConfig config = PdfConfig.builder().fontConfig(fontConfig).build();

		assertNotNull(config.getFontConfig());
		assertThat(config.getFontConfig().isEnabled()).isTrue();
	}

	@Test
	@DisplayName("should set HierarchyConfig")
	void shouldSetHierarchyConfig() {
		HierarchyConfig hierarchyConfig = HierarchyConfig.builder().enabled(true).build();
		PdfConfig config = PdfConfig.builder().hierarchyConfig(hierarchyConfig).build();

		assertNotNull(config.getHierarchyConfig());
		assertThat(config.getHierarchyConfig().isEnabled()).isTrue();
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		FontConfig fontConfig = FontConfig.builder().enabled(true).build();
		HierarchyConfig hierarchyConfig = HierarchyConfig.builder().enabled(true).build();
		List<String> passwords = Arrays.asList("pass1", "pass2");

		PdfConfig config = PdfConfig.builder().extractImages(true).passwords(passwords).extractMetadata(true)
				.fontConfig(fontConfig).hierarchyConfig(hierarchyConfig).build();

		assertThat(config.isExtractImages()).isTrue();
		assertThat(config.getPasswords()).containsExactlyElementsOf(passwords);
		assertThat(config.isExtractMetadata()).isTrue();
		assertNotNull(config.getFontConfig());
		assertNotNull(config.getHierarchyConfig());
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		FontConfig fontConfig = FontConfig.builder().enabled(true).build();
		PdfConfig config = PdfConfig.builder().extractImages(true).extractMetadata(true).fontConfig(fontConfig).build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("extract_images", true).containsEntry("extract_metadata", true)
				.containsKey("font_config");
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		FontConfig fontConfig = FontConfig.builder().enabled(true).build();
		PdfConfig config = PdfConfig.builder().extractImages(true).password("pwd").extractMetadata(true)
				.fontConfig(fontConfig).build();

		assertThat(config.isExtractImages()).isTrue();
		assertThat(config.getPasswords()).contains("pwd");
		assertThat(config.isExtractMetadata()).isTrue();
		assertNotNull(config.getFontConfig());
	}

	@Test
	@DisplayName("should handle passwords immutability")
	void shouldHandlePasswordsImmutability() {
		List<String> passwords = Arrays.asList("pass1", "pass2");
		PdfConfig config = PdfConfig.builder().passwords(passwords).build();

		assertThat(config.getPasswords()).isUnmodifiable();
	}

	@Test
	@DisplayName("should support adding multiple individual passwords")
	void shouldSupportAddingMultiplePasswords() {
		PdfConfig config = PdfConfig.builder().password("pass1").password("pass2").password("pass3").build();

		assertThat(config.getPasswords()).hasSize(3).contains("pass1", "pass2", "pass3");
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		PdfConfig config1 = PdfConfig.builder().extractImages(true).build();
		PdfConfig config2 = PdfConfig.builder().extractImages(false).build();

		assertThat(config1.isExtractImages()).isNotEqualTo(config2.isExtractImages());
	}
}

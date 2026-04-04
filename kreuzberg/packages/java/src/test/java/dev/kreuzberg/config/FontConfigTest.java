package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Nested;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive FontConfig tests.
 *
 * <p>
 * Tests for FontConfig feature that allows users to enable/disable custom font
 * provider and add custom font directories.
 */
@DisplayName("FontConfig Tests")
final class FontConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void testFontConfigDefaults() {
		FontConfig config = FontConfig.builder().build();

		assertNotNull(config, "FontConfig should not be null");
		assertTrue(config.isEnabled(), "enabled should default to true");
		assertNull(config.getCustomFontDirs(), "customFontDirs should default to null");
	}

	@Test
	@DisplayName("should create config with enabled true")
	void testFontConfigBuilderWithEnabledTrue() {
		FontConfig config = FontConfig.builder().enabled(true).build();

		assertTrue(config.isEnabled(), "enabled should be true");
		assertNull(config.getCustomFontDirs(), "customFontDirs should be null");
	}

	@Test
	@DisplayName("should create config with enabled false")
	void testFontConfigBuilderWithEnabledFalse() {
		FontConfig config = FontConfig.builder().enabled(false).build();

		assertFalse(config.isEnabled(), "enabled should be false");
		assertNull(config.getCustomFontDirs(), "customFontDirs should be null");
	}

	@Test
	@DisplayName("should create config with custom font directories")
	void testFontConfigBuilderWithCustomDirs() {
		java.util.List<String> dirs = java.util.Arrays.asList("/usr/share/fonts/custom", "~/my-fonts");

		FontConfig config = FontConfig.builder().customFontDirs(dirs).build();

		assertTrue(config.isEnabled(), "enabled should default to true");
		assertNotNull(config.getCustomFontDirs(), "customFontDirs should not be null");
		assertThat(config.getCustomFontDirs()).hasSize(2).containsExactly("/usr/share/fonts/custom", "~/my-fonts");
	}

	@Test
	@DisplayName("should create config with all parameters")
	void testFontConfigBuilderWithAllParameters() {
		java.util.List<String> dirs = java.util.Arrays.asList("/path/to/fonts", "/another/path");

		FontConfig config = FontConfig.builder().enabled(true).customFontDirs(dirs).build();

		assertTrue(config.isEnabled(), "enabled should be true");
		assertNotNull(config.getCustomFontDirs(), "customFontDirs should not be null");
		assertThat(config.getCustomFontDirs()).hasSize(2).containsExactlyElementsOf(dirs);
	}

	@Test
	@DisplayName("should support builder method chaining")
	void testFontConfigBuilderChaining() {
		FontConfig config = FontConfig.builder().enabled(false).customFontDirs(java.util.Arrays.asList("/fonts"))
				.build();

		assertFalse(config.isEnabled(), "Method chaining should work");
		assertNotNull(config.getCustomFontDirs());
	}

	@Test
	@DisplayName("should handle empty custom font directories")
	void testFontConfigEmptyCustomDirs() {
		FontConfig config = FontConfig.builder().enabled(true).customFontDirs(new java.util.ArrayList<>()).build();

		assertTrue(config.isEnabled());
		assertNotNull(config.getCustomFontDirs());
		assertThat(config.getCustomFontDirs()).isEmpty();
	}

	@Test
	@DisplayName("should handle multiple custom font directories")
	void testFontConfigMultipleCustomDirs() {
		java.util.List<String> dirs = java.util.Arrays.asList("/path1", "/path2", "/path3", "~/fonts",
				"./relative-fonts");

		FontConfig config = FontConfig.builder().customFontDirs(dirs).build();

		assertThat(config.getCustomFontDirs()).hasSize(5).containsExactlyElementsOf(dirs);
	}

	@Test
	@DisplayName("should test equals and hash code")
	void testFontConfigEqualsAndHashCode() {
		java.util.List<String> dirs = java.util.Arrays.asList("/fonts");

		FontConfig config1 = FontConfig.builder().enabled(true).customFontDirs(dirs).build();

		FontConfig config2 = FontConfig.builder().enabled(true).customFontDirs(dirs).build();

		assertEquals(config1, config2, "Equal configs should be equal");
		assertEquals(config1.hashCode(), config2.hashCode(), "Equal configs should have same hash");
	}

	@Test
	@DisplayName("should have meaningful string representation")
	void testFontConfigToString() {
		FontConfig config = FontConfig.builder().enabled(true).customFontDirs(java.util.Arrays.asList("/fonts"))
				.build();

		String str = config.toString();

		assertNotNull(str);
		assertThat(str).contains("FontConfig");
	}

	@Test
	@DisplayName("should handle font directories immutability")
	void testFontConfigImmutability() {
		java.util.List<String> dirs = java.util.Arrays.asList("/fonts");
		FontConfig config = FontConfig.builder().customFontDirs(dirs).build();

		assertThat(config.getCustomFontDirs()).isUnmodifiable();
	}

	@Test
	@DisplayName("should create independent builder instances")
	void testIndependentBuilders() {
		FontConfig config1 = FontConfig.builder().enabled(true).build();
		FontConfig config2 = FontConfig.builder().enabled(false).build();

		assertThat(config1.isEnabled()).isNotEqualTo(config2.isEnabled());
	}

	@Nested
	@DisplayName("PdfConfig Integration")
	class PdfConfigIntegration {

		@Test
		@DisplayName("should integrate with PdfConfig")
		void testPdfConfigWithFontConfig() {
			FontConfig fontConfig = FontConfig.builder().enabled(true).customFontDirs(java.util.Arrays.asList("/fonts"))
					.build();

			PdfConfig pdfConfig = PdfConfig.builder().extractImages(true).fontConfig(fontConfig).build();

			assertNotNull(pdfConfig.getFontConfig());
			assertTrue(pdfConfig.getFontConfig().isEnabled());
			assertThat(pdfConfig.getFontConfig().getCustomFontDirs()).contains("/fonts");
		}

		@Test
		@DisplayName("should integrate with disabled FontConfig")
		void testPdfConfigWithFontConfigDisabled() {
			FontConfig fontConfig = FontConfig.builder().enabled(false)
					.customFontDirs(java.util.Arrays.asList("/custom")).build();

			PdfConfig pdfConfig = PdfConfig.builder().fontConfig(fontConfig).build();

			assertNotNull(pdfConfig.getFontConfig());
			assertFalse(pdfConfig.getFontConfig().isEnabled());
			assertThat(pdfConfig.getFontConfig().getCustomFontDirs()).contains("/custom");
		}

		@Test
		@DisplayName("should integrate with all FontConfig parameters")
		void testPdfConfigWithFontConfigAllParameters() {
			FontConfig fontConfig = FontConfig.builder().enabled(true)
					.customFontDirs(java.util.Arrays.asList("/custom-fonts")).build();

			PdfConfig pdfConfig = PdfConfig.builder().extractImages(true).passwords(java.util.Arrays.asList("pass1"))
					.extractMetadata(true).fontConfig(fontConfig).build();

			assertTrue(pdfConfig.isExtractImages());
			assertThat(pdfConfig.getPasswords()).contains("pass1");
			assertTrue(pdfConfig.isExtractMetadata());
			assertTrue(pdfConfig.getFontConfig().isEnabled());
		}

		@Test
		@DisplayName("should work without FontConfig")
		void testPdfConfigWithoutFontConfig() {
			PdfConfig pdfConfig = PdfConfig.builder().extractImages(true).build();

			assertNull(pdfConfig.getFontConfig(), "FontConfig should be null when not set");
		}

		@Test
		@DisplayName("should handle null FontConfig")
		void testPdfConfigFontConfigNull() {
			PdfConfig pdfConfig = PdfConfig.builder().fontConfig(null).build();

			assertNull(pdfConfig.getFontConfig());
		}
	}
}

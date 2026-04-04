package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive HierarchyConfig tests.
 *
 * <p>
 * Tests for hierarchy detection configuration including enable flag, clustering
 * parameters, bounding box handling, and OCR coverage thresholds.
 */
@DisplayName("HierarchyConfig Tests")
final class HierarchyConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		HierarchyConfig config = HierarchyConfig.builder().build();

		assertThat(config.isEnabled()).isTrue();
		assertThat(config.getKClusters()).isEqualTo(6);
		assertThat(config.isIncludeBbox()).isTrue();
		assertNull(config.getOcrCoverageThreshold());
	}

	@Test
	@DisplayName("should disable hierarchy detection")
	void shouldDisableHierarchyDetection() {
		HierarchyConfig config = HierarchyConfig.builder().enabled(false).build();

		assertThat(config.isEnabled()).isFalse();
	}

	@Test
	@DisplayName("should set k clusters")
	void shouldSetKClusters() {
		HierarchyConfig config = HierarchyConfig.builder().kClusters(8).build();

		assertThat(config.getKClusters()).isEqualTo(8);
	}

	@Test
	@DisplayName("should disable include bbox")
	void shouldDisableIncludeBbox() {
		HierarchyConfig config = HierarchyConfig.builder().includeBbox(false).build();

		assertThat(config.isIncludeBbox()).isFalse();
	}

	@Test
	@DisplayName("should set OCR coverage threshold")
	void shouldSetOcrCoverageThreshold() {
		HierarchyConfig config = HierarchyConfig.builder().ocrCoverageThreshold(0.75).build();

		assertThat(config.getOcrCoverageThreshold()).isEqualTo(0.75);
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		HierarchyConfig config = HierarchyConfig.builder().enabled(true).kClusters(10).includeBbox(true)
				.ocrCoverageThreshold(0.8).build();

		assertThat(config.isEnabled()).isTrue();
		assertThat(config.getKClusters()).isEqualTo(10);
		assertThat(config.isIncludeBbox()).isTrue();
		assertThat(config.getOcrCoverageThreshold()).isEqualTo(0.8);
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		HierarchyConfig config = HierarchyConfig.builder().enabled(true).kClusters(8).includeBbox(true)
				.ocrCoverageThreshold(0.7).build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("enabled", true).containsEntry("k_clusters", 8)
				.containsEntry("include_bbox", true).containsEntry("ocr_coverage_threshold", 0.7);
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		HierarchyConfig config = HierarchyConfig.builder().enabled(true).kClusters(12).includeBbox(false)
				.ocrCoverageThreshold(0.9).build();

		assertThat(config.isEnabled()).isTrue();
		assertThat(config.getKClusters()).isEqualTo(12);
		assertThat(config.isIncludeBbox()).isFalse();
		assertThat(config.getOcrCoverageThreshold()).isEqualTo(0.9);
	}

	@Test
	@DisplayName("should handle various k cluster values")
	void shouldHandleVariousKClusterValues() {
		HierarchyConfig config1 = HierarchyConfig.builder().kClusters(4).build();
		HierarchyConfig config2 = HierarchyConfig.builder().kClusters(6).build();
		HierarchyConfig config3 = HierarchyConfig.builder().kClusters(10).build();

		assertThat(config1.getKClusters()).isLessThan(config2.getKClusters());
		assertThat(config2.getKClusters()).isLessThan(config3.getKClusters());
	}

	@Test
	@DisplayName("should handle threshold values in valid range")
	void shouldHandleThresholdValues() {
		HierarchyConfig config1 = HierarchyConfig.builder().ocrCoverageThreshold(0.1).build();
		HierarchyConfig config2 = HierarchyConfig.builder().ocrCoverageThreshold(0.5).build();
		HierarchyConfig config3 = HierarchyConfig.builder().ocrCoverageThreshold(0.95).build();

		assertThat(config1.getOcrCoverageThreshold()).isGreaterThanOrEqualTo(0.0);
		assertThat(config2.getOcrCoverageThreshold()).isGreaterThanOrEqualTo(0.0);
		assertThat(config3.getOcrCoverageThreshold()).isLessThanOrEqualTo(1.0);
	}

	@Test
	@DisplayName("should test equality of configs")
	void shouldTestEquality() {
		HierarchyConfig config1 = HierarchyConfig.builder().enabled(true).kClusters(6).includeBbox(true).build();
		HierarchyConfig config2 = HierarchyConfig.builder().enabled(true).kClusters(6).includeBbox(true).build();

		assertThat(config1).isEqualTo(config2);
		assertThat(config1.hashCode()).isEqualTo(config2.hashCode());
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		HierarchyConfig config1 = HierarchyConfig.builder().kClusters(6).build();
		HierarchyConfig config2 = HierarchyConfig.builder().kClusters(8).build();

		assertThat(config1.getKClusters()).isNotEqualTo(config2.getKClusters());
	}
}

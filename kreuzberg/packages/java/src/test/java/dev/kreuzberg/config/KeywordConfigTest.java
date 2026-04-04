package dev.kreuzberg.config;

import static org.assertj.core.api.Assertions.*;
import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

/**
 * Comprehensive KeywordConfig tests.
 *
 * <p>
 * Tests for keyword extraction configuration including algorithm selection,
 * scoring parameters, n-gram ranges, and specialized params for YAKE/RAKE.
 */
@DisplayName("KeywordConfig Tests")
final class KeywordConfigTest {

	@Test
	@DisplayName("should create config with default values")
	void shouldCreateWithDefaults() {
		KeywordConfig config = KeywordConfig.builder().build();

		assertNull(config.toMap().get("algorithm"));
		assertNull(config.toMap().get("max_keywords"));
	}

	@Test
	@DisplayName("should set algorithm")
	void shouldSetAlgorithm() {
		KeywordConfig config = KeywordConfig.builder().algorithm("yake").build();

		assertThat(config.toMap()).containsEntry("algorithm", "yake");
	}

	@Test
	@DisplayName("should set max keywords")
	void shouldSetMaxKeywords() {
		KeywordConfig config = KeywordConfig.builder().maxKeywords(20).build();

		assertThat(config.toMap()).containsEntry("max_keywords", 20);
	}

	@Test
	@DisplayName("should set min score")
	void shouldSetMinScore() {
		KeywordConfig config = KeywordConfig.builder().minScore(0.3).build();

		assertThat(config.toMap()).containsEntry("min_score", 0.3);
	}

	@Test
	@DisplayName("should set n-gram range")
	void shouldSetNgramRange() {
		KeywordConfig config = KeywordConfig.builder().ngramRange(1, 3).build();

		assertThat(config.toMap()).containsKey("ngram_range");
	}

	@Test
	@DisplayName("should set language")
	void shouldSetLanguage() {
		KeywordConfig config = KeywordConfig.builder().language("en").build();

		assertThat(config.toMap()).containsEntry("language", "en");
	}

	@Test
	@DisplayName("should set YAKE parameters")
	void shouldSetYakeParams() {
		KeywordConfig.YakeParams yakeParams = KeywordConfig.YakeParams.builder().windowSize(3).build();
		KeywordConfig config = KeywordConfig.builder().algorithm("yake").yakeParams(yakeParams).build();

		assertThat(config.toMap()).containsKey("yake_params");
	}

	@Test
	@DisplayName("should set RAKE parameters")
	void shouldSetRakeParams() {
		KeywordConfig.RakeParams rakeParams = KeywordConfig.RakeParams.builder().minWordLength(3).maxWordsPerPhrase(3)
				.build();
		KeywordConfig config = KeywordConfig.builder().algorithm("rake").rakeParams(rakeParams).build();

		assertThat(config.toMap()).containsKey("rake_params");
	}

	@Test
	@DisplayName("should create config with all parameters")
	void shouldCreateWithAllParameters() {
		KeywordConfig.YakeParams yakeParams = KeywordConfig.YakeParams.builder().windowSize(3).build();
		KeywordConfig config = KeywordConfig.builder().algorithm("yake").maxKeywords(20).minScore(0.2).ngramRange(1, 3)
				.language("en").yakeParams(yakeParams).build();

		java.util.Map<String, Object> map = config.toMap();
		assertThat(map).containsEntry("algorithm", "yake").containsEntry("max_keywords", 20)
				.containsEntry("min_score", 0.2).containsEntry("language", "en");
	}

	@Test
	@DisplayName("should convert to map representation")
	void shouldConvertToMap() {
		KeywordConfig config = KeywordConfig.builder().algorithm("rake").maxKeywords(15).minScore(0.1).build();

		java.util.Map<String, Object> map = config.toMap();

		assertThat(map).containsEntry("algorithm", "rake").containsEntry("max_keywords", 15).containsEntry("min_score",
				0.1);
	}

	@Test
	@DisplayName("should support builder method chaining")
	void shouldSupportBuilderChaining() {
		KeywordConfig config = KeywordConfig.builder().algorithm("yake").maxKeywords(20).minScore(0.3).language("de")
				.build();

		java.util.Map<String, Object> map = config.toMap();
		assertThat(map).containsEntry("algorithm", "yake").containsEntry("max_keywords", 20);
	}

	@Test
	@DisplayName("should test YAKE parameters builder")
	void shouldTestYakeParamsBuilder() {
		KeywordConfig.YakeParams yakeParams = KeywordConfig.YakeParams.builder().windowSize(5).build();

		assertThat(yakeParams.toMap()).containsEntry("window_size", 5);
	}

	@Test
	@DisplayName("should test RAKE parameters builder")
	void shouldTestRakeParamsBuilder() {
		KeywordConfig.RakeParams rakeParams = KeywordConfig.RakeParams.builder().minWordLength(2).maxWordsPerPhrase(5)
				.build();

		assertThat(rakeParams.toMap()).containsEntry("min_word_length", 2).containsEntry("max_words_per_phrase", 5);
	}

	@Test
	@DisplayName("should create independent builder instances")
	void shouldCreateIndependentBuilderInstances() {
		KeywordConfig config1 = KeywordConfig.builder().algorithm("yake").build();
		KeywordConfig config2 = KeywordConfig.builder().algorithm("rake").build();

		assertThat(config1.toMap().get("algorithm")).isNotEqualTo(config2.toMap().get("algorithm"));
	}
}

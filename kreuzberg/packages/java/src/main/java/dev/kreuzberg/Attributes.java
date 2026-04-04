package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

/**
 * Element attributes in Djot.
 *
 * <p>
 * Represents attributes attached to elements using {.class #id key="value"}
 * syntax.
 */
public final class Attributes {
	private final Optional<String> id;
	private final List<String> classes;
	@JsonDeserialize(contentAs = KeyValue.class)
	private final List<KeyValue> keyValues;

	@JsonCreator
	public Attributes(@JsonProperty("id") String id, @JsonProperty("classes") List<String> classes,
			@JsonProperty("key_values") List<KeyValue> keyValues) {
		this.id = Optional.ofNullable(id);
		this.classes = Collections.unmodifiableList(classes != null ? classes : Collections.emptyList());
		this.keyValues = Collections.unmodifiableList(keyValues != null ? keyValues : Collections.emptyList());
	}

	public Optional<String> getId() {
		return id;
	}

	public List<String> getClasses() {
		return classes;
	}

	public List<KeyValue> getKeyValues() {
		return keyValues;
	}

	@Override
	public String toString() {
		return "Attributes{" + "id=" + id + ", classes=" + classes + ", keyValues=" + keyValues + '}';
	}

	/**
	 * Key-value pair for element attributes.
	 */
	public static final class KeyValue {
		private final String key;
		private final String value;

		@JsonCreator
		public KeyValue(@JsonProperty("0") String key, @JsonProperty("1") String value) {
			this.key = Objects.requireNonNull(key, "key must not be null");
			this.value = Objects.requireNonNull(value, "value must not be null");
		}

		public String getKey() {
			return key;
		}

		public String getValue() {
			return value;
		}

		@Override
		public String toString() {
			return key + "=" + value;
		}
	}
}

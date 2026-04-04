package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

/**
 * Inline element within a block in a Djot document.
 *
 * <p>
 * Represents text with formatting, links, images, etc.
 */
public final class InlineElement {
	private final InlineType elementType;
	private final String content;
	private final Optional<Attributes> attributes;
	private final Optional<Map<String, String>> metadata;

	@JsonCreator
	public InlineElement(@JsonProperty("element_type") InlineType elementType, @JsonProperty("content") String content,
			@JsonProperty("attributes") Attributes attributes, @JsonProperty("metadata") Map<String, String> metadata) {
		this.elementType = Objects.requireNonNull(elementType, "elementType must not be null");
		this.content = Objects.requireNonNull(content, "content must not be null");
		this.attributes = Optional.ofNullable(attributes);
		this.metadata = Optional.ofNullable(metadata != null ? Collections.unmodifiableMap(metadata) : null);
	}

	public InlineType getElementType() {
		return elementType;
	}

	public String getContent() {
		return content;
	}

	public Optional<Attributes> getAttributes() {
		return attributes;
	}

	public Optional<Map<String, String>> getMetadata() {
		return metadata;
	}

	@Override
	public String toString() {
		return "InlineElement{" + "elementType=" + elementType + ", contentLength=" + content.length() + '}';
	}
}

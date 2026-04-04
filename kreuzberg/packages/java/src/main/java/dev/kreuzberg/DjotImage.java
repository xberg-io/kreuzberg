package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * Image element in a Djot document.
 */
public final class DjotImage {
	private final String src;
	private final String alt;
	private final Optional<String> title;
	private final Optional<Attributes> attributes;

	@JsonCreator
	public DjotImage(@JsonProperty("src") String src, @JsonProperty("alt") String alt,
			@JsonProperty("title") String title, @JsonProperty("attributes") Attributes attributes) {
		this.src = Objects.requireNonNull(src, "src must not be null");
		this.alt = Objects.requireNonNull(alt, "alt must not be null");
		this.title = Optional.ofNullable(title);
		this.attributes = Optional.ofNullable(attributes);
	}

	public String getSrc() {
		return src;
	}

	public String getAlt() {
		return alt;
	}

	public Optional<String> getTitle() {
		return title;
	}

	public Optional<Attributes> getAttributes() {
		return attributes;
	}

	@Override
	public String toString() {
		return "DjotImage{" + "src='" + src + '\'' + ", alt='" + alt + '\'' + '}';
	}
}

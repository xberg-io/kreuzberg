package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * Link element in a Djot document.
 */
public final class DjotLink {
	private final String url;
	private final String text;
	private final Optional<String> title;
	private final Optional<Attributes> attributes;

	@JsonCreator
	public DjotLink(@JsonProperty("url") String url, @JsonProperty("text") String text,
			@JsonProperty("title") String title, @JsonProperty("attributes") Attributes attributes) {
		this.url = Objects.requireNonNull(url, "url must not be null");
		this.text = Objects.requireNonNull(text, "text must not be null");
		this.title = Optional.ofNullable(title);
		this.attributes = Optional.ofNullable(attributes);
	}

	public String getUrl() {
		return url;
	}

	public String getText() {
		return text;
	}

	public Optional<String> getTitle() {
		return title;
	}

	public Optional<Attributes> getAttributes() {
		return attributes;
	}

	@Override
	public String toString() {
		return "DjotLink{" + "url='" + url + '\'' + ", text='" + text + '\'' + '}';
	}
}

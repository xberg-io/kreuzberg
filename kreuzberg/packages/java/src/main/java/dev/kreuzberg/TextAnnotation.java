package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * Inline text annotation for formatting and links.
 *
 * <p>
 * Annotations reference byte offsets into a node's text content, enabling
 * precise identification of formatted regions. Each annotation has a type
 * (bold, italic, link, etc.) and byte range.
 *
 * @since 4.3.0
 */
@JsonIgnoreProperties(ignoreUnknown = true)
public final class TextAnnotation {
	private final int start;
	private final int end;
	private final String kind;
	private final String url;
	private final String title;

	/**
	 * Create a new TextAnnotation.
	 *
	 * @param start
	 *            start byte offset (inclusive)
	 * @param end
	 *            end byte offset (exclusive)
	 * @param kind
	 *            annotation type (must not be null)
	 * @param url
	 *            link URL (for Link annotations), or null
	 * @param title
	 *            link title (for Link annotations), or null
	 */
	@JsonCreator
	public TextAnnotation(@JsonProperty("start") int start, @JsonProperty("end") int end,
			@JsonProperty("annotation_type") String kind, @JsonProperty("url") String url,
			@JsonProperty("title") String title) {
		if (start < 0) {
			throw new IllegalArgumentException("start must be non-negative, got " + start);
		}
		if (end < start) {
			throw new IllegalArgumentException("end must be >= start, got end=" + end + " start=" + start);
		}
		this.start = start;
		this.end = end;
		this.kind = Objects.requireNonNull(kind, "kind must not be null");
		this.url = url;
		this.title = title;
	}

	/**
	 * Get the start byte offset of this annotation (inclusive).
	 *
	 * @return start offset
	 */
	@JsonProperty("start")
	public int getStart() {
		return start;
	}

	/**
	 * Get the end byte offset of this annotation (exclusive).
	 *
	 * @return end offset
	 */
	@JsonProperty("end")
	public int getEnd() {
		return end;
	}

	/**
	 * Get the annotation type.
	 *
	 * <p>
	 * Possible values: "bold", "italic", "underline", "strikethrough", "code",
	 * "subscript", "superscript", "link".
	 *
	 * @return annotation type (never null)
	 */
	@JsonProperty("annotation_type")
	public String getKind() {
		return kind;
	}

	/**
	 * Get the link URL if this is a Link annotation.
	 *
	 * @return URL, or empty if not a Link annotation
	 */
	@JsonProperty("url")
	public Optional<String> getUrl() {
		return Optional.ofNullable(url);
	}

	/**
	 * Get the link title if this is a Link annotation.
	 *
	 * @return link title, or empty if not a Link annotation or no title specified
	 */
	@JsonProperty("title")
	public Optional<String> getTitle() {
		return Optional.ofNullable(title);
	}

	/**
	 * Get the byte range of this annotation.
	 *
	 * @return length in bytes
	 */
	public int getLength() {
		return end - start;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof TextAnnotation)) {
			return false;
		}
		TextAnnotation other = (TextAnnotation) obj;
		return start == other.start && end == other.end && Objects.equals(kind, other.kind)
				&& Objects.equals(url, other.url) && Objects.equals(title, other.title);
	}

	@Override
	public int hashCode() {
		return Objects.hash(start, end, kind, url, title);
	}

	@Override
	public String toString() {
		return "TextAnnotation{" + "start=" + start + ", end=" + end + ", kind='" + kind + '\'' + '}';
	}
}

package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * A URI extracted from a document.
 *
 * <p>
 * Represents any link, reference, or resource pointer found during extraction.
 * The {@code kind} field classifies the URI semantically, while {@code label}
 * carries optional human-readable display text.
 *
 * @since 4.6.0
 */
@JsonIgnoreProperties(ignoreUnknown = true)
public final class Uri {
	private final String url;
	private final String label;
	private final Integer page;
	private final String kind;

	@JsonCreator
	public Uri(@JsonProperty("url") String url, @JsonProperty("label") String label,
			@JsonProperty("page") Integer page, @JsonProperty("kind") String kind) {
		this.url = Objects.requireNonNull(url, "url must not be null");
		this.label = label;
		this.page = page;
		this.kind = Objects.requireNonNull(kind, "kind must not be null");
	}

	/**
	 * Get the URL or path string.
	 *
	 * @return the URL (never null)
	 */
	public String getUrl() {
		return url;
	}

	/**
	 * Get the optional display text / label for the link.
	 *
	 * @return label text, or empty if not available
	 */
	public Optional<String> getLabel() {
		return Optional.ofNullable(label);
	}

	/**
	 * Get the optional page number where the URI was found (1-indexed).
	 *
	 * @return page number, or empty if not available
	 */
	public Optional<Integer> getPage() {
		return Optional.ofNullable(page);
	}

	/**
	 * Get the semantic classification of the URI.
	 *
	 * <p>
	 * Possible values: "hyperlink", "image", "anchor", "citation", "reference",
	 * "email".
	 *
	 * @return the kind string (never null)
	 */
	public String getKind() {
		return kind;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof Uri)) {
			return false;
		}
		Uri other = (Uri) obj;
		return Objects.equals(url, other.url) && Objects.equals(label, other.label)
				&& Objects.equals(page, other.page) && Objects.equals(kind, other.kind);
	}

	@Override
	public int hashCode() {
		return Objects.hash(url, label, page, kind);
	}

	@Override
	public String toString() {
		return "Uri{" + "url='" + url + '\'' + ", kind='" + kind + '\'' + '}';
	}
}

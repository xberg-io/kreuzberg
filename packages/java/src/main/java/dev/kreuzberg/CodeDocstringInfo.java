package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

/**
 * A documentation comment/docstring found in source code.
 */
public final class CodeDocstringInfo {
	private final String text;
	private final String format;
	private final String associatedItem;
	private final CodeSpan span;
	private final List<CodeDocSection> sections;

	@JsonCreator
	public CodeDocstringInfo(@JsonProperty("text") String text, @JsonProperty("format") String format,
			@JsonProperty("associated_item") String associatedItem, @JsonProperty("span") CodeSpan span,
			@JsonProperty("sections") List<CodeDocSection> sections) {
		this.text = Objects.requireNonNull(text, "text must not be null");
		this.format = Objects.requireNonNull(format, "format must not be null");
		this.associatedItem = associatedItem;
		this.span = Objects.requireNonNull(span, "span must not be null");
		this.sections = sections != null ? Collections.unmodifiableList(sections) : List.of();
	}

	public String getText() {
		return text;
	}

	public String getFormat() {
		return format;
	}

	public Optional<String> getAssociatedItem() {
		return Optional.ofNullable(associatedItem);
	}

	public CodeSpan getSpan() {
		return span;
	}

	public List<CodeDocSection> getSections() {
		return sections;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeDocstringInfo)) {
			return false;
		}
		CodeDocstringInfo other = (CodeDocstringInfo) obj;
		return Objects.equals(text, other.text) && Objects.equals(format, other.format)
				&& Objects.equals(associatedItem, other.associatedItem) && Objects.equals(span, other.span)
				&& Objects.equals(sections, other.sections);
	}

	@Override
	public int hashCode() {
		return Objects.hash(text, format, associatedItem, span, sections);
	}

	@Override
	public String toString() {
		return "CodeDocstringInfo{" + "format='" + format + '\'' + ", associatedItem='" + associatedItem
				+ "', sections=" + sections.size() + '}';
	}
}

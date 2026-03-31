package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;

/**
 * A comment found in source code.
 */
public final class CodeCommentInfo {
	private final String text;
	private final String kind;
	private final CodeSpan span;

	@JsonCreator
	public CodeCommentInfo(@JsonProperty("text") String text, @JsonProperty("kind") String kind,
			@JsonProperty("span") CodeSpan span) {
		this.text = Objects.requireNonNull(text, "text must not be null");
		this.kind = Objects.requireNonNull(kind, "kind must not be null");
		this.span = Objects.requireNonNull(span, "span must not be null");
	}

	public String getText() {
		return text;
	}

	public String getKind() {
		return kind;
	}

	public CodeSpan getSpan() {
		return span;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeCommentInfo)) {
			return false;
		}
		CodeCommentInfo other = (CodeCommentInfo) obj;
		return Objects.equals(text, other.text) && Objects.equals(kind, other.kind)
				&& Objects.equals(span, other.span);
	}

	@Override
	public int hashCode() {
		return Objects.hash(text, kind, span);
	}

	@Override
	public String toString() {
		return "CodeCommentInfo{" + "kind='" + kind + '\'' + ", textLength=" + text.length() + '}';
	}
}

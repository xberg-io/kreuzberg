package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * A chunk of source code with optional parent context.
 */
public final class CodeChunk {
	private final String content;
	private final String language;
	private final CodeSpan span;
	private final CodeChunkContext context;

	@JsonCreator
	public CodeChunk(@JsonProperty("content") String content, @JsonProperty("language") String language,
			@JsonProperty("span") CodeSpan span, @JsonProperty("context") CodeChunkContext context) {
		this.content = Objects.requireNonNull(content, "content must not be null");
		this.language = Objects.requireNonNull(language, "language must not be null");
		this.span = Objects.requireNonNull(span, "span must not be null");
		this.context = context;
	}

	public String getContent() {
		return content;
	}

	public String getLanguage() {
		return language;
	}

	public CodeSpan getSpan() {
		return span;
	}

	public Optional<CodeChunkContext> getContext() {
		return Optional.ofNullable(context);
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeChunk)) {
			return false;
		}
		CodeChunk other = (CodeChunk) obj;
		return Objects.equals(content, other.content) && Objects.equals(language, other.language)
				&& Objects.equals(span, other.span) && Objects.equals(context, other.context);
	}

	@Override
	public int hashCode() {
		return Objects.hash(content, language, span, context);
	}

	@Override
	public String toString() {
		return "CodeChunk{" + "language='" + language + '\'' + ", contentLength=" + content.length() + '}';
	}
}

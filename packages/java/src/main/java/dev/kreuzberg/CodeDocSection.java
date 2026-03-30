package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * A section within a docstring (e.g. @param, @returns, description).
 */
public final class CodeDocSection {
	private final String kind;
	private final String name;
	private final String content;

	@JsonCreator
	public CodeDocSection(@JsonProperty("kind") String kind, @JsonProperty("name") String name,
			@JsonProperty("content") String content) {
		this.kind = Objects.requireNonNull(kind, "kind must not be null");
		this.name = name;
		this.content = Objects.requireNonNull(content, "content must not be null");
	}

	public String getKind() {
		return kind;
	}

	public Optional<String> getName() {
		return Optional.ofNullable(name);
	}

	public String getContent() {
		return content;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeDocSection)) {
			return false;
		}
		CodeDocSection other = (CodeDocSection) obj;
		return Objects.equals(kind, other.kind) && Objects.equals(name, other.name)
				&& Objects.equals(content, other.content);
	}

	@Override
	public int hashCode() {
		return Objects.hash(kind, name, content);
	}

	@Override
	public String toString() {
		return "CodeDocSection{" + "kind='" + kind + '\'' + ", name='" + name + '\'' + '}';
	}
}

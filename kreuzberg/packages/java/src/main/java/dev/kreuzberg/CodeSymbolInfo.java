package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * A symbol (variable, constant, type alias, etc.) in source code.
 */
public final class CodeSymbolInfo {
	private final String name;
	private final String kind;
	private final String typeAnnotation;
	private final CodeSpan span;

	@JsonCreator
	public CodeSymbolInfo(@JsonProperty("name") String name, @JsonProperty("kind") String kind,
			@JsonProperty("type_annotation") String typeAnnotation, @JsonProperty("span") CodeSpan span) {
		this.name = Objects.requireNonNull(name, "name must not be null");
		this.kind = Objects.requireNonNull(kind, "kind must not be null");
		this.typeAnnotation = typeAnnotation;
		this.span = Objects.requireNonNull(span, "span must not be null");
	}

	public String getName() {
		return name;
	}

	public String getKind() {
		return kind;
	}

	public Optional<String> getTypeAnnotation() {
		return Optional.ofNullable(typeAnnotation);
	}

	public CodeSpan getSpan() {
		return span;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeSymbolInfo)) {
			return false;
		}
		CodeSymbolInfo other = (CodeSymbolInfo) obj;
		return Objects.equals(name, other.name) && Objects.equals(kind, other.kind)
				&& Objects.equals(typeAnnotation, other.typeAnnotation) && Objects.equals(span, other.span);
	}

	@Override
	public int hashCode() {
		return Objects.hash(name, kind, typeAnnotation, span);
	}

	@Override
	public String toString() {
		return "CodeSymbolInfo{" + "name='" + name + '\'' + ", kind='" + kind + '\'' + '}';
	}
}

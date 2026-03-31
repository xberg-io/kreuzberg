package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;

/**
 * An exported symbol from source code.
 */
public final class CodeExportInfo {
	private final String name;
	private final String kind;
	private final CodeSpan span;

	@JsonCreator
	public CodeExportInfo(@JsonProperty("name") String name, @JsonProperty("kind") String kind,
			@JsonProperty("span") CodeSpan span) {
		this.name = Objects.requireNonNull(name, "name must not be null");
		this.kind = Objects.requireNonNull(kind, "kind must not be null");
		this.span = Objects.requireNonNull(span, "span must not be null");
	}

	public String getName() {
		return name;
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
		if (!(obj instanceof CodeExportInfo)) {
			return false;
		}
		CodeExportInfo other = (CodeExportInfo) obj;
		return Objects.equals(name, other.name) && Objects.equals(kind, other.kind)
				&& Objects.equals(span, other.span);
	}

	@Override
	public int hashCode() {
		return Objects.hash(name, kind, span);
	}

	@Override
	public String toString() {
		return "CodeExportInfo{" + "name='" + name + '\'' + ", kind='" + kind + '\'' + '}';
	}
}

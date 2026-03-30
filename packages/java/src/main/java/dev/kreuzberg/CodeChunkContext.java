package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;
import java.util.Optional;

/**
 * Parent context for a code chunk.
 */
public final class CodeChunkContext {
	private final String parentName;
	private final String parentKind;

	@JsonCreator
	public CodeChunkContext(@JsonProperty("parent_name") String parentName,
			@JsonProperty("parent_kind") String parentKind) {
		this.parentName = parentName;
		this.parentKind = parentKind;
	}

	public Optional<String> getParentName() {
		return Optional.ofNullable(parentName);
	}

	public Optional<String> getParentKind() {
		return Optional.ofNullable(parentKind);
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeChunkContext)) {
			return false;
		}
		CodeChunkContext other = (CodeChunkContext) obj;
		return Objects.equals(parentName, other.parentName) && Objects.equals(parentKind, other.parentKind);
	}

	@Override
	public int hashCode() {
		return Objects.hash(parentName, parentKind);
	}

	@Override
	public String toString() {
		return "CodeChunkContext{" + "parentName='" + parentName + '\'' + ", parentKind='" + parentKind + '\'' + '}';
	}
}

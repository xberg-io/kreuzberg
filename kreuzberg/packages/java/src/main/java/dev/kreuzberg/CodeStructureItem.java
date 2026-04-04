package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

/**
 * A structural element (function, class, method, etc.) in source code.
 *
 * <p>
 * Represents a node in the code structure tree, with optional children for
 * nested structures (e.g., methods within a class).
 */
public final class CodeStructureItem {
	private final String kind;
	private final String name;
	private final String visibility;
	private final CodeSpan span;
	private final List<CodeStructureItem> children;
	private final List<String> decorators;
	private final String docComment;
	private final String signature;
	private final CodeSpan bodySpan;

	@JsonCreator
	public CodeStructureItem(@JsonProperty("kind") String kind, @JsonProperty("name") String name,
			@JsonProperty("visibility") String visibility, @JsonProperty("span") CodeSpan span,
			@JsonProperty("children") List<CodeStructureItem> children,
			@JsonProperty("decorators") List<String> decorators, @JsonProperty("doc_comment") String docComment,
			@JsonProperty("signature") String signature, @JsonProperty("body_span") CodeSpan bodySpan) {
		this.kind = Objects.requireNonNull(kind, "kind must not be null");
		this.name = name;
		this.visibility = visibility;
		this.span = Objects.requireNonNull(span, "span must not be null");
		this.children = children != null ? Collections.unmodifiableList(children) : List.of();
		this.decorators = decorators != null ? Collections.unmodifiableList(decorators) : List.of();
		this.docComment = docComment;
		this.signature = signature;
		this.bodySpan = bodySpan;
	}

	public String getKind() {
		return kind;
	}

	public Optional<String> getName() {
		return Optional.ofNullable(name);
	}

	public Optional<String> getVisibility() {
		return Optional.ofNullable(visibility);
	}

	public CodeSpan getSpan() {
		return span;
	}

	public List<CodeStructureItem> getChildren() {
		return children;
	}

	public List<String> getDecorators() {
		return decorators;
	}

	public Optional<String> getDocComment() {
		return Optional.ofNullable(docComment);
	}

	public Optional<String> getSignature() {
		return Optional.ofNullable(signature);
	}

	public Optional<CodeSpan> getBodySpan() {
		return Optional.ofNullable(bodySpan);
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeStructureItem)) {
			return false;
		}
		CodeStructureItem other = (CodeStructureItem) obj;
		return Objects.equals(kind, other.kind) && Objects.equals(name, other.name)
				&& Objects.equals(visibility, other.visibility) && Objects.equals(span, other.span)
				&& Objects.equals(children, other.children) && Objects.equals(decorators, other.decorators)
				&& Objects.equals(docComment, other.docComment) && Objects.equals(signature, other.signature)
				&& Objects.equals(bodySpan, other.bodySpan);
	}

	@Override
	public int hashCode() {
		return Objects.hash(kind, name, visibility, span, children, decorators, docComment, signature, bodySpan);
	}

	@Override
	public String toString() {
		return "CodeStructureItem{" + "kind='" + kind + '\'' + ", name='" + name + '\'' + ", children="
				+ children.size() + '}';
	}
}

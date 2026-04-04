package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

/**
 * An import/include/require statement found in source code.
 */
public final class CodeImportInfo {
	private final String source;
	private final List<String> items;
	private final String alias;
	private final boolean isWildcard;
	private final CodeSpan span;

	@JsonCreator
	public CodeImportInfo(@JsonProperty("source") String source, @JsonProperty("items") List<String> items,
			@JsonProperty("alias") String alias, @JsonProperty("is_wildcard") boolean isWildcard,
			@JsonProperty("span") CodeSpan span) {
		this.source = Objects.requireNonNull(source, "source must not be null");
		this.items = items != null ? Collections.unmodifiableList(items) : List.of();
		this.alias = alias;
		this.isWildcard = isWildcard;
		this.span = Objects.requireNonNull(span, "span must not be null");
	}

	public String getSource() {
		return source;
	}

	public List<String> getItems() {
		return items;
	}

	public Optional<String> getAlias() {
		return Optional.ofNullable(alias);
	}

	public boolean isWildcard() {
		return isWildcard;
	}

	public CodeSpan getSpan() {
		return span;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeImportInfo)) {
			return false;
		}
		CodeImportInfo other = (CodeImportInfo) obj;
		return Objects.equals(source, other.source) && Objects.equals(items, other.items)
				&& Objects.equals(alias, other.alias) && isWildcard == other.isWildcard
				&& Objects.equals(span, other.span);
	}

	@Override
	public int hashCode() {
		return Objects.hash(source, items, alias, isWildcard, span);
	}

	@Override
	public String toString() {
		return "CodeImportInfo{" + "source='" + source + '\'' + ", items=" + items + ", isWildcard=" + isWildcard + '}';
	}
}

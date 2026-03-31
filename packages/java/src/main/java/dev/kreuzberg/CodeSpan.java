package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;

/**
 * Byte/line/column range in source code.
 *
 * <p>
 * Represents a span of source text identified by byte offsets and
 * line/column positions, as reported by tree-sitter.
 */
public final class CodeSpan {
	private final int startByte;
	private final int endByte;
	private final int startLine;
	private final int startColumn;
	private final int endLine;
	private final int endColumn;

	@JsonCreator
	public CodeSpan(@JsonProperty("start_byte") int startByte, @JsonProperty("end_byte") int endByte,
			@JsonProperty("start_line") int startLine, @JsonProperty("start_column") int startColumn,
			@JsonProperty("end_line") int endLine, @JsonProperty("end_column") int endColumn) {
		this.startByte = startByte;
		this.endByte = endByte;
		this.startLine = startLine;
		this.startColumn = startColumn;
		this.endLine = endLine;
		this.endColumn = endColumn;
	}

	public int getStartByte() {
		return startByte;
	}

	public int getEndByte() {
		return endByte;
	}

	public int getStartLine() {
		return startLine;
	}

	public int getStartColumn() {
		return startColumn;
	}

	public int getEndLine() {
		return endLine;
	}

	public int getEndColumn() {
		return endColumn;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeSpan)) {
			return false;
		}
		CodeSpan other = (CodeSpan) obj;
		return startByte == other.startByte && endByte == other.endByte && startLine == other.startLine
				&& startColumn == other.startColumn && endLine == other.endLine && endColumn == other.endColumn;
	}

	@Override
	public int hashCode() {
		return Objects.hash(startByte, endByte, startLine, startColumn, endLine, endColumn);
	}

	@Override
	public String toString() {
		return "CodeSpan{" + "startByte=" + startByte + ", endByte=" + endByte + ", startLine=" + startLine
				+ ", startColumn=" + startColumn + ", endLine=" + endLine + ", endColumn=" + endColumn + '}';
	}
}

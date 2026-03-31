package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;

/**
 * Aggregate metrics for a parsed source file.
 *
 * <p>
 * Contains line counts, byte size, AST node statistics, and parse error
 * counts as reported by tree-sitter analysis.
 */
public final class CodeFileMetrics {
	private final int totalLines;
	private final int codeLines;
	private final int commentLines;
	private final int blankLines;
	private final int totalBytes;
	private final int nodeCount;
	private final int errorCount;
	private final int maxDepth;

	@JsonCreator
	public CodeFileMetrics(@JsonProperty("total_lines") int totalLines, @JsonProperty("code_lines") int codeLines,
			@JsonProperty("comment_lines") int commentLines, @JsonProperty("blank_lines") int blankLines,
			@JsonProperty("total_bytes") int totalBytes, @JsonProperty("node_count") int nodeCount,
			@JsonProperty("error_count") int errorCount, @JsonProperty("max_depth") int maxDepth) {
		this.totalLines = totalLines;
		this.codeLines = codeLines;
		this.commentLines = commentLines;
		this.blankLines = blankLines;
		this.totalBytes = totalBytes;
		this.nodeCount = nodeCount;
		this.errorCount = errorCount;
		this.maxDepth = maxDepth;
	}

	public int getTotalLines() {
		return totalLines;
	}

	public int getCodeLines() {
		return codeLines;
	}

	public int getCommentLines() {
		return commentLines;
	}

	public int getBlankLines() {
		return blankLines;
	}

	public int getTotalBytes() {
		return totalBytes;
	}

	public int getNodeCount() {
		return nodeCount;
	}

	public int getErrorCount() {
		return errorCount;
	}

	public int getMaxDepth() {
		return maxDepth;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeFileMetrics)) {
			return false;
		}
		CodeFileMetrics other = (CodeFileMetrics) obj;
		return totalLines == other.totalLines && codeLines == other.codeLines && commentLines == other.commentLines
				&& blankLines == other.blankLines && totalBytes == other.totalBytes && nodeCount == other.nodeCount
				&& errorCount == other.errorCount && maxDepth == other.maxDepth;
	}

	@Override
	public int hashCode() {
		return Objects.hash(totalLines, codeLines, commentLines, blankLines, totalBytes, nodeCount, errorCount,
				maxDepth);
	}

	@Override
	public String toString() {
		return "CodeFileMetrics{" + "totalLines=" + totalLines + ", codeLines=" + codeLines + ", commentLines="
				+ commentLines + ", blankLines=" + blankLines + ", totalBytes=" + totalBytes + ", nodeCount=" + nodeCount
				+ ", errorCount=" + errorCount + ", maxDepth=" + maxDepth + '}';
	}
}

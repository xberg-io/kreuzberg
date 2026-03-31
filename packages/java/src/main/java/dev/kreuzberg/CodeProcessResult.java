package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Complete result of tree-sitter code analysis.
 *
 * <p>
 * Contains the detected language, file-level metrics, structural items,
 * imports, exports, comments, docstrings, symbols, diagnostics, and code
 * chunks extracted by tree-sitter processing.
 */
public final class CodeProcessResult {
	private final String language;
	private final CodeFileMetrics metrics;
	private final List<CodeStructureItem> structure;
	private final List<CodeImportInfo> imports;
	private final List<CodeExportInfo> exports;
	private final List<CodeCommentInfo> comments;
	private final List<CodeDocstringInfo> docstrings;
	private final List<CodeSymbolInfo> symbols;
	private final List<CodeDiagnostic> diagnostics;
	private final List<CodeChunk> chunks;

	@JsonCreator
	public CodeProcessResult(@JsonProperty("language") String language,
			@JsonProperty("metrics") CodeFileMetrics metrics,
			@JsonProperty("structure") List<CodeStructureItem> structure,
			@JsonProperty("imports") List<CodeImportInfo> imports,
			@JsonProperty("exports") List<CodeExportInfo> exports,
			@JsonProperty("comments") List<CodeCommentInfo> comments,
			@JsonProperty("docstrings") List<CodeDocstringInfo> docstrings,
			@JsonProperty("symbols") List<CodeSymbolInfo> symbols,
			@JsonProperty("diagnostics") List<CodeDiagnostic> diagnostics,
			@JsonProperty("chunks") List<CodeChunk> chunks) {
		this.language = Objects.requireNonNull(language, "language must not be null");
		this.metrics = Objects.requireNonNull(metrics, "metrics must not be null");
		this.structure = structure != null ? Collections.unmodifiableList(structure) : List.of();
		this.imports = imports != null ? Collections.unmodifiableList(imports) : List.of();
		this.exports = exports != null ? Collections.unmodifiableList(exports) : List.of();
		this.comments = comments != null ? Collections.unmodifiableList(comments) : List.of();
		this.docstrings = docstrings != null ? Collections.unmodifiableList(docstrings) : List.of();
		this.symbols = symbols != null ? Collections.unmodifiableList(symbols) : List.of();
		this.diagnostics = diagnostics != null ? Collections.unmodifiableList(diagnostics) : List.of();
		this.chunks = chunks != null ? Collections.unmodifiableList(chunks) : List.of();
	}

	public String getLanguage() {
		return language;
	}

	public CodeFileMetrics getMetrics() {
		return metrics;
	}

	public List<CodeStructureItem> getStructure() {
		return structure;
	}

	public List<CodeImportInfo> getImports() {
		return imports;
	}

	public List<CodeExportInfo> getExports() {
		return exports;
	}

	public List<CodeCommentInfo> getComments() {
		return comments;
	}

	public List<CodeDocstringInfo> getDocstrings() {
		return docstrings;
	}

	public List<CodeSymbolInfo> getSymbols() {
		return symbols;
	}

	public List<CodeDiagnostic> getDiagnostics() {
		return diagnostics;
	}

	public List<CodeChunk> getChunks() {
		return chunks;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeProcessResult)) {
			return false;
		}
		CodeProcessResult other = (CodeProcessResult) obj;
		return Objects.equals(language, other.language) && Objects.equals(metrics, other.metrics)
				&& Objects.equals(structure, other.structure) && Objects.equals(imports, other.imports)
				&& Objects.equals(exports, other.exports) && Objects.equals(comments, other.comments)
				&& Objects.equals(docstrings, other.docstrings) && Objects.equals(symbols, other.symbols)
				&& Objects.equals(diagnostics, other.diagnostics) && Objects.equals(chunks, other.chunks);
	}

	@Override
	public int hashCode() {
		return Objects.hash(language, metrics, structure, imports, exports, comments, docstrings, symbols, diagnostics,
				chunks);
	}

	@Override
	public String toString() {
		return "CodeProcessResult{" + "language='" + language + '\'' + ", structure=" + structure.size() + ", imports="
				+ imports.size() + ", exports=" + exports.size() + ", comments=" + comments.size() + ", docstrings="
				+ docstrings.size() + ", symbols=" + symbols.size() + ", diagnostics=" + diagnostics.size()
				+ ", chunks=" + chunks.size() + '}';
	}
}

package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;

/**
 * A parse error or warning from tree-sitter.
 */
public final class CodeDiagnostic {
	private final String message;
	private final String severity;
	private final CodeSpan span;

	@JsonCreator
	public CodeDiagnostic(@JsonProperty("message") String message, @JsonProperty("severity") String severity,
			@JsonProperty("span") CodeSpan span) {
		this.message = Objects.requireNonNull(message, "message must not be null");
		this.severity = Objects.requireNonNull(severity, "severity must not be null");
		this.span = Objects.requireNonNull(span, "span must not be null");
	}

	public String getMessage() {
		return message;
	}

	public String getSeverity() {
		return severity;
	}

	public CodeSpan getSpan() {
		return span;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof CodeDiagnostic)) {
			return false;
		}
		CodeDiagnostic other = (CodeDiagnostic) obj;
		return Objects.equals(message, other.message) && Objects.equals(severity, other.severity)
				&& Objects.equals(span, other.span);
	}

	@Override
	public int hashCode() {
		return Objects.hash(message, severity, span);
	}

	@Override
	public String toString() {
		return "CodeDiagnostic{" + "severity='" + severity + '\'' + ", message='" + message + '\'' + '}';
	}
}

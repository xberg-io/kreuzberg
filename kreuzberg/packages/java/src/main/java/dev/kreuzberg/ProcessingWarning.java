package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.Objects;

/**
 * A warning generated during document processing.
 *
 * <p>
 * Contains the source component that generated the warning and a descriptive
 * message about the issue encountered.
 *
 * @since 4.5.0
 */
public final class ProcessingWarning {
	private final String source;
	private final String message;

	@JsonCreator
	public ProcessingWarning(@JsonProperty("source") String source, @JsonProperty("message") String message) {
		this.source = Objects.requireNonNull(source, "source must not be null");
		this.message = Objects.requireNonNull(message, "message must not be null");
	}

	/**
	 * Get the source component that generated the warning.
	 *
	 * @return the source identifier
	 */
	public String getSource() {
		return source;
	}

	/**
	 * Get the warning message.
	 *
	 * @return the descriptive warning message
	 */
	public String getMessage() {
		return message;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj) {
			return true;
		}
		if (!(obj instanceof ProcessingWarning)) {
			return false;
		}
		ProcessingWarning other = (ProcessingWarning) obj;
		return Objects.equals(source, other.source) && Objects.equals(message, other.message);
	}

	@Override
	public int hashCode() {
		return Objects.hash(source, message);
	}

	@Override
	public String toString() {
		return "ProcessingWarning{" + "source='" + source + '\'' + ", message='" + message + '\'' + '}';
	}
}

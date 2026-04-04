package dev.kreuzberg;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Footnote in a Djot document.
 */
public final class Footnote {
	private final String label;
	@JsonDeserialize(contentAs = FormattedBlock.class)
	private final List<FormattedBlock> content;

	@JsonCreator
	public Footnote(@JsonProperty("label") String label, @JsonProperty("content") List<FormattedBlock> content) {
		this.label = Objects.requireNonNull(label, "label must not be null");
		this.content = Collections.unmodifiableList(content != null ? content : Collections.emptyList());
	}

	public String getLabel() {
		return label;
	}

	public List<FormattedBlock> getContent() {
		return content;
	}

	@Override
	public String toString() {
		return "Footnote{" + "label='" + label + '\'' + ", contentBlocks=" + content.size() + '}';
	}
}

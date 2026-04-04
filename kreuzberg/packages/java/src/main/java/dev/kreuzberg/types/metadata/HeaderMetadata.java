package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jspecify.annotations.Nullable;

/** HTML header/heading metadata. */
public record HeaderMetadata(@JsonProperty("level") int level, @JsonProperty("text") String text,
		@JsonProperty("id") @Nullable String id, @JsonProperty("depth") int depth,
		@JsonProperty("html_offset") int htmlOffset) {
}

package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import java.util.List;
import org.jspecify.annotations.Nullable;

/** FictionBook (FB2) metadata. */
public record FictionBookMetadata(@JsonProperty("genres") List<String> genres,
		@JsonProperty("sequences") List<String> sequences,
		@JsonProperty("annotation") @Nullable String annotation) {
}

package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jspecify.annotations.Nullable;

/** EPUB metadata (Dublin Core extensions). */
public record EpubMetadata(@JsonProperty("coverage") @Nullable String coverage,
		@JsonProperty("dc_format") @Nullable String dcFormat,
		@JsonProperty("relation") @Nullable String relation, @JsonProperty("source") @Nullable String source,
		@JsonProperty("dc_type") @Nullable String dcType,
		@JsonProperty("cover_image") @Nullable String coverImage) {
}

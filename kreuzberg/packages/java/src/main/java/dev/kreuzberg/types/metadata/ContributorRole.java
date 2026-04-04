package dev.kreuzberg.types.metadata;

import com.fasterxml.jackson.annotation.JsonProperty;
import org.jspecify.annotations.Nullable;

/** JATS contributor with role. */
public record ContributorRole(@JsonProperty("name") String name,
		@JsonProperty("role") @Nullable String role) {
}
